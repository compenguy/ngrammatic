//! This module provides a way to store the weights of a document in a compressed way.
//! The compression is highly dependent on **our** weights distribution and thus
//! it's not recommended to use this module for other purposes.

use dsi_bitstream::prelude::*;
#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};
use std::io::{Cursor, Write};
use sux::prelude::*;
use webgraph::prelude::*;

type Writer<W> = BufBitWriter<LittleEndian, WordAdapter<u32, W>>;
type Reader<R> = BufBitReader<LittleEndian, WordAdapter<u32, R>>;
type EF = EliasFano<SelectFixed2>;

/// A factory that can create a reader.
/// The factory own the data and the reader borrows it.
pub trait ReaderFactory {
    /// The reader type that we will pass to another struct.
    type Reader<'a>: GammaRead<LittleEndian> + BitRead<LittleEndian>
    where
        Self: 'a;
    /// Returns a reader that reads from the given offset.
    fn get_reader(&self, offset: usize) -> Self::Reader<'_>;
}

/// A factory that creates a reader from vec of u8.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct CursorReaderFactory {
    data: Vec<u8>,
}

impl CursorReaderFactory {
    /// Creates a new `CursorReaderFactory` that reads from the given data.
    pub fn new(data: Vec<u8>) -> Self {
        CursorReaderFactory { data }
    }

    /// Consumes the `CursorReaderFactory` and returns the inner data.
    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }
}

impl ReaderFactory for CursorReaderFactory {
    type Reader<'a> = Reader<std::io::Cursor<&'a [u8]>>;

    fn get_reader(&self, offset: usize) -> Self::Reader<'_> {
        let mut reader = std::io::Cursor::new(self.data.as_slice());
        reader.set_position(offset as u64);
        BufBitReader::<LittleEndian, _>::new(WordAdapter::<u32, _>::new(reader))
    }
}

/// A builder on which you can push the weights of a document.
/// The compression is highly dependent on **our** weights distribution and thus
/// it's not recommended to use this builder for other purposes.
#[derive(Debug)]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct WeightsBuilder<W: Write = std::io::Cursor<Vec<u8>>> {
    /// The bitstream
    writer: Writer<W>,
    /// A vec of offsets where each node data starts
    offsets: Vec<usize>,
    /// How many bits we wrote so far
    len: usize,
    /// how many nodes we have
    num_nodes: usize,
    /// how many weights we have
    num_weights: usize,
}

impl<W: Write> WeightsBuilder<W> {
    /// Creates a new `WeightsBuilder` that writes to the given writer.
    pub fn new() -> WeightsBuilder<Cursor<Vec<u8>>> {
        WeightsBuilder {
            writer: BufBitWriter::new(WordAdapter::new(Cursor::new(Vec::new()))),
            offsets: vec![],
            len: 0,
            num_nodes: 0,
            num_weights: 0,
        }
    }

    /// Writes the weights of the given node to the writer.
    pub fn push<WS>(&mut self, weights: WS) -> std::io::Result<usize>
    where
        WS: ExactSizeIterator<Item = usize>,
    {
        self.num_nodes += 1;
        self.num_weights += weights.len();
        self.offsets.push(self.len);
        let mut bits_written = 0;
        bits_written += self.writer.write_gamma(weights.len() as u64)?;

        let mut zeros_range = 0;
        for weight in weights {
            if weight == 0 {
                if zeros_range == 0 {
                    bits_written += self.writer.write_unary(0)?;
                }
                zeros_range += 1;
                continue;
            }

            if zeros_range > 0 {
                bits_written += self.writer.write_gamma(zeros_range as u64)?;
                zeros_range = 0;
            }

            bits_written += self.writer.write_unary(weight as u64)?;
        }

        Ok(bits_written)
    }
}

impl WeightsBuilder {
    /// Finishes the writing and returns the reader.
    pub fn build(self) -> Weights {
        let mut efb = EliasFanoBuilder::new(self.num_nodes, self.len);
        for offset in self.offsets {
            efb.push(offset).unwrap();
        }
        let ef = efb.build();

        Weights {
            num_nodes: self.num_nodes,
            num_weights: self.num_weights,
            offsets: ef.convert_to().unwrap(),
            reader_factory: CursorReaderFactory::new(
                self.writer.into_inner().unwrap().into_inner().into_inner(),
            ),
        }
    }

    #[cfg(feature = "rayon")]
    /// Finishes the writing and returns the reader.
    pub fn par_build(self) -> Weights {
        use rayon::iter::IndexedParallelIterator;
        use rayon::iter::IntoParallelIterator;
        use rayon::iter::ParallelIterator;

        let efb = EliasFanoConcurrentBuilder::new(self.num_nodes, self.len);
        self.offsets
            .into_par_iter()
            .enumerate()
            .for_each(|(index, offset)| unsafe {
                efb.set(index, offset, std::sync::atomic::Ordering::SeqCst);
            });
        let ef = efb.build();

        Weights {
            num_nodes: self.num_nodes,
            num_weights: self.num_weights,
            offsets: ef.convert_to().unwrap(),
            reader_factory: CursorReaderFactory::new(
                self.writer.into_inner().unwrap().into_inner().into_inner(),
            ),
        }
    }
}

/// A builder on which you can push the weights of a document.
/// The compression is highly dependent on **our** weights distribution and thus
/// it's not recommended to use this builder for other purposes.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct Weights<RF = CursorReaderFactory, OFF = EF> {
    /// The factory of bitstream readers
    reader_factory: RF,
    /// A vec of offsets gaps
    offsets: OFF,
    /// how many nodes we have
    num_nodes: usize,
    /// how many weights we have
    num_weights: usize,
}

impl<RF, OFF> Weights<RF, OFF> {
    /// Creates a new `WeightsBuilder` that writes to the given writer.
    pub fn new(reader_factory: RF, offsets: OFF, num_nodes: usize, num_weights: usize) -> Self {
        Weights {
            reader_factory,
            offsets,
            num_nodes,
            num_weights,
        }
    }

    /// Returns the number of weights.
    pub fn num_weights(&self) -> usize {
        self.num_weights
    }

    /// Returns the number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    /// Consumes the `Weights` and returns the inner reader and offsets.
    pub fn into_inner(self) -> (RF, OFF) {
        (self.reader_factory, self.offsets)
    }
}

/// A lender
#[derive(Clone, Debug)]
pub struct Lender<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> {
    /// The bitstream
    reader: R,
    /// how many nodes left to decode
    num_nodes: usize,
    /// at which node we are at
    start_node: usize,
}

impl<'lend, R: GammaRead<LittleEndian> + BitRead<LittleEndian>>
    webgraph::traits::NodeLabelsLender<'lend> for Lender<R>
{
    type Label = usize;
    type IntoIterator = Vec<usize>;
}

impl<'lend, R: GammaRead<LittleEndian> + BitRead<LittleEndian>> lender::Lending<'lend>
    for Lender<R>
{
    type Lend = (usize, Vec<usize>);
}

impl<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> lender::ExactSizeLender for Lender<R> {
    fn len(&self) -> usize {
        self.num_nodes - self.start_node
    }
}

impl<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> lender::Lender for Lender<R> {
    fn next(&mut self) -> Option<lender::prelude::Lend<'_, Self>> {
        if self.start_node == self.num_nodes - 1 {
            return None;
        }

        let node = self.start_node;
        self.start_node += 1;

        let mut weights_to_decode = self.reader.read_gamma().unwrap() as usize;
        let mut successors = Vec::with_capacity(weights_to_decode);

        while weights_to_decode != 0 {
            let weight = self.reader.read_unary().unwrap() as usize;
            if weight == 0 {
                let zeros_range = self.reader.read_gamma().unwrap() as usize;
                successors.resize(successors.len() + zeros_range, 0);
                weights_to_decode -= zeros_range;
                continue;
            }

            successors.push(weight);
            weights_to_decode -= 1;
        }

        Some((node, successors))
    }
}

/// The iterator over all the weights of the successors of all nodes
pub struct WeightsIter<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> {
    num_nodes: usize,
    succ: Succ<R>,
}

impl<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> WeightsIter<R> {
    /// Creates a new `WeightsIter` that reads from the given reader.
    pub fn new(reader: R, num_nodes: usize) -> Self {
        WeightsIter {
            num_nodes,
            succ: Succ::new(reader),
        }
    }
}

impl<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> Iterator for WeightsIter<R> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_nodes == 0 {
            return None;
        }

        let mut next = None;
        while next.is_none() {
            next = self.succ.next();
            self.num_nodes -= 1;
            self.succ.reset();
        }

        next
    }
}

/// The iterator over the weights of the successors of a node
#[derive(Clone, Debug)]
pub struct Succ<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> {
    /// The bitstream
    reader: R,
    /// how many weights left to decode
    weights_to_decode: usize,
    /// zeros_range
    zeros_range: usize,
}

impl<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> Succ<R> {
    /// Creates a new `Succ` that reads from the given reader.
    pub fn new(reader: R) -> Self {
        let mut res = Succ {
            reader,
            weights_to_decode: 0,
            zeros_range: 0,
        };
        res.reset();
        res
    }

    /// Consumes the `Succ` and returns the inner reader.
    pub fn into_inner(self) -> R {
        self.reader
    }

    /// Resets the iterator so it can decode the weights of the next node.
    pub fn reset(&mut self) {
        let weights_to_decode = self.reader.read_gamma().unwrap() as usize;
        self.weights_to_decode = weights_to_decode;
        self.zeros_range = 0;
    }
}

impl<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> ExactSizeIterator for Succ<R> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.weights_to_decode
    }
}

impl<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> Iterator for Succ<R> {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<usize> {
        debug_assert!(
            self.weights_to_decode >= self.zeros_range,
            concat!(
                "Expected weights_to_decode >= zeros_range, but got ",
                "weights_to_decode = {:?}, zeros_range = {:?}"
            ),
            self.weights_to_decode,
            self.zeros_range
        );
        if self.weights_to_decode == 0 {
            return None;
        }

        if self.zeros_range > 0 {
            self.zeros_range -= 1;
            return Some(0);
        }

        let weight = self.reader.read_unary().unwrap() as usize;

        if weight == 0 {
            self.zeros_range = self.reader.read_gamma().unwrap() as usize;
            self.zeros_range -= 1;
        }

        self.weights_to_decode -= 1;
        Some(weight)
    }
}

impl<RF: ReaderFactory, OFF: IndexedDict<Input = usize, Output = usize>> SequentialLabeling
    for Weights<RF, OFF>
{
    type Label = usize;

    type Lender<'node> = Lender<<RF as ReaderFactory>::Reader<'node>> where RF: 'node, OFF: 'node;

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    fn iter_from(&self, from: usize) -> Self::Lender<'_> {
        let offset = self.offsets.get(from);
        Lender {
            reader: self.reader_factory.get_reader(offset),
            num_nodes: self.num_nodes - from,
            start_node: from,
        }
    }
}

impl<RF: ReaderFactory, OFF: IndexedDict<Input = usize, Output = usize>> RandomAccessLabeling
    for Weights<RF, OFF>
{
    type Labels<'succ> = Succ<<RF as ReaderFactory>::Reader<'succ>> where RF: 'succ, OFF: 'succ;

    fn num_arcs(&self) -> u64 {
        self.num_weights as u64
    }

    fn labels(&self, node_id: usize) -> <Self as RandomAccessLabeling>::Labels<'_> {
        let offset = self.offsets.get(node_id);
        Succ::new(self.reader_factory.get_reader(offset))
    }

    fn outdegree(&self, node_id: usize) -> usize {
        let offset = self.offsets.get(node_id);
        let mut reader = self.reader_factory.get_reader(offset);
        reader.read_gamma().unwrap() as usize
    }
}

impl<RF: ReaderFactory, OFF: IndexedDict<Input = usize, Output = usize>> Weights<RF, OFF> {
    /// Returns an iterator over all the weights of the successors of all nodes.
    pub fn weights(&self) -> WeightsIter<<RF as ReaderFactory>::Reader<'_>> {
        WeightsIter::new(self.reader_factory.get_reader(0), self.num_nodes)
    }
}
