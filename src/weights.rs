//! This module provides a way to store the weights of a document in a compressed way.
//! The compression is highly dependent on **our** weights distribution and thus
//! it's not recommended to use this module for other purposes.

use dsi_bitstream::prelude::*;
#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};
use std::io::Write;
use sux::prelude::*;
use webgraph::prelude::*;

type Writer<W> = BufBitWriter<LittleEndian, WordAdapter<u32, W>>;
type Reader<R> = BufBitReader<LittleEndian, WordAdapter<u32, R>>;
type EF = EliasFano<SelectFixed2>;

/// A factory that can create a reader.
/// The factory own the data and the reader borrows it.
pub trait ReaderFactory {
    type Reader<'a>: GammaRead<LittleEndian> + BitRead<LittleEndian>
    where
        Self: 'a;
    /// Returns a reader that reads from the given offset.
    fn get_reader(&self, offset: usize) -> Self::Reader<'_>;
}

/// A factory that creates a reader from vec of u8.
pub struct CursorReaderFactory {
    data: Vec<u8>,
}

impl CursorReaderFactory {
    pub fn new(data: Vec<u8>) -> Self {
        CursorReaderFactory { data }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }
}

impl ReaderFactory for CursorReaderFactory {
    type Reader<'a> = Reader<std::io::Cursor<&'a [u8]>>;

    fn get_reader(&self, offset: usize) -> Self::Reader<'_> {
        BufBitReader::<LittleEndian, _>::new(WordAdapter::<u32, _>::new(std::io::Cursor::new(
            &self.data,
        )))
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
    pub fn new(writer: W) -> Self {
        WeightsBuilder {
            writer: BufBitWriter::new(WordAdapter::new(writer)),
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

        let mut ones_range = 0;
        for weight in weights {
            if weight == 1 {
                if ones_range == 0 {
                    bits_written += self.writer.write_unary(0)?;
                }
                ones_range += 1;
                continue;
            }

            if ones_range > 0 {
                bits_written += self.writer.write_gamma(ones_range as u64)?;
                ones_range = 0;
            }

            bits_written += self.writer.write_unary(weight as u64 - 1)?;
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

    /// Consumes the `Weights` and returns the inner reader and offsets.
    pub fn into_inner(self) -> (RF, OFF) {
        (self.reader_factory, self.offsets)
    }
}

/// A lender
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
            let weight = self.reader.read_unary().unwrap() as usize + 1;
            if weight == 1 {
                let ones_range = self.reader.read_gamma().unwrap() as usize;
                successors.resize(successors.len() + ones_range, 1);
                weights_to_decode -= ones_range;
                continue;
            }

            successors.push(weight);
            weights_to_decode -= 1;
        }

        Some((node, successors))
    }
}

/// The iterator over the weights of the successors of a node
pub struct Succ<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> {
    /// The bitstream
    reader: R,
    /// how many weights left to decode
    weights_to_decode: usize,
    /// ones_range
    ones_range: usize,
}

impl<R: GammaRead<LittleEndian> + BitRead<LittleEndian>> Succ<R> {
    /// Creates a new `Succ` that reads from the given reader.
    pub fn new(mut reader: R) -> Self {
        let weights_to_decode = reader.read_gamma().unwrap() as usize;
        Succ {
            reader,
            weights_to_decode,
            ones_range: 0,
        }
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
        debug_assert!(self.weights_to_decode <= self.ones_range);
        if self.weights_to_decode == 0 {
            return None;
        }

        if self.ones_range > 0 {
            self.ones_range -= 1;
            return Some(1);
        }

        let weight = self.reader.read_unary().unwrap() as usize + 1;

        if weight == 1 {
            self.ones_range = self.reader.read_gamma().unwrap() as usize;
            self.ones_range -= 1;
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
