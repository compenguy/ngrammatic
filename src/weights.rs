//! This module provides a way to store the weights of a document in a compressed way.
//! The compression is highly dependent on **our** weights distribution and thus
//! it's not recommended to use this module for other purposes.

use dsi_bitstream::prelude::*;
#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};
use std::io::{Read, Seek, Write};
use sux::prelude::*;
use webgraph::prelude::*;

type Writer<W> = BufBitWriter<LittleEndian, WordAdapter<u32, W>>;
type Reader<R> = BufBitReader<LittleEndian, WordAdapter<u32, R>>;
type EF = EliasFano<SelectFixed2>;

/// A builder on which you can push the weights of a document.
/// The compression is highly dependent on **our** weights distribution and thus
/// it's not recommended to use this builder for other purposes.
#[derive(Debug)]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct WeightsBuilder<W: Write> {
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

impl<RW: Write + Read + Seek> WeightsBuilder<RW> {
    /// Finishes the writing and returns the reader.
    pub fn build(self) -> Weights<RW, EF> {
        let mut efb = EliasFanoBuilder::new(self.num_nodes, self.len);
        for offset in self.offsets {
            efb.push(offset).unwrap();
        }
        let ef = efb.build();

        Weights {
            num_nodes: self.num_nodes,
            num_weights: self.num_weights,
            offsets: ef.convert_to().unwrap(),
            reader: self.writer.into_inner().unwrap().into_inner(),
        }
    }
}

/// A builder on which you can push the weights of a document.
/// The compression is highly dependent on **our** weights distribution and thus
/// it's not recommended to use this builder for other purposes.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct Weights<R: Read, OFF> {
    /// The bitstream
    reader: R,
    /// A vec of offsets gaps
    offsets: OFF,
    /// how many nodes we have
    num_nodes: usize,
    /// how many weights we have
    num_weights: usize,
}

impl<R: Read, OFF> Weights<R, OFF> {
    /// Creates a new `WeightsBuilder` that writes to the given writer.
    pub fn new(reader: R, offsets: OFF, num_nodes: usize, num_weights: usize) -> Self {
        Weights {
            reader: reader,
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
    pub fn into_inner(self) -> (R, OFF) {
        (self.reader, self.offsets)
    }
}

/// A lender
pub struct Lender<R: Read> {
    /// The bitstream
    reader: Reader<R>,
    /// how many nodes left to decode
    num_nodes: usize,
    /// at which node we are at
    start_node: usize,
}

impl<'lend, R: Read> webgraph::traits::NodeLabelsLender<'lend> for Lender<R> {
    type Label = usize;
    type IntoIterator = Vec<usize>;
}

impl<'lend, R: Read> lender::Lending<'lend> for Lender<R> {
    type Lend = (usize, Vec<usize>);
}

impl<R: Read> lender::Lender for Lender<R> {
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

impl<R: Read + Seek + Clone, OFF: IndexedDict<Input = usize, Output = usize>> SequentialLabeling
    for Weights<R, OFF>
{
    type Label = usize;

    type Lender<'node> = Lender<R> where R: 'node, OFF: 'node;

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    fn iter_from(&self, from: usize) -> Self::Lender<'_> {
        let offset = self.offsets.get(from);
        let mut reader = self.reader.clone();
        reader
            .seek(std::io::SeekFrom::Start(offset as u64))
            .unwrap();
        Lender {
            reader: BufBitReader::new(WordAdapter::new(reader)),
            num_nodes: self.num_nodes - from,
            start_node: from,
        }
    }
}

impl<R: Read + Seek + Clone, OFF: IndexedDict<Input = usize, Output = usize>> RandomAccessLabeling
    for Weights<R, OFF>
{
    type Labels<'succ> = Vec<usize> where R: 'succ, OFF: 'succ;

    fn num_arcs(&self) -> u64 {
        self.num_weights as u64
    }

    fn labels(&self, node_id: usize) -> <Self as RandomAccessLabeling>::Labels<'_> {
        let offset = self.offsets.get(node_id);
        let mut reader = self.reader.clone();
        reader
            .seek(std::io::SeekFrom::Start(offset as u64))
            .unwrap();
        let mut reader = BufBitReader::<LittleEndian, _>::new(WordAdapter::<u32, _>::new(reader));

        let mut weights_to_decode = reader.read_gamma().unwrap() as usize;
        let mut successors = Vec::with_capacity(weights_to_decode);

        while weights_to_decode != 0 {
            let weight = reader.read_unary().unwrap() as usize + 1;
            if weight == 1 {
                let ones_range = reader.read_gamma().unwrap() as usize;
                successors.resize(successors.len() + ones_range, 1);
                weights_to_decode -= ones_range;
                continue;
            }

            successors.push(weight);
            weights_to_decode -= 1;
        }

        successors
    }

    fn outdegree(&self, node_id: usize) -> usize {
        let offset = self.offsets.get(node_id);
        let mut reader = self.reader.clone();
        reader
            .seek(std::io::SeekFrom::Start(offset as u64))
            .unwrap();
        let mut reader = BufBitReader::<LittleEndian, _>::new(WordAdapter::<u32, _>::new(reader));
        reader.read_gamma().unwrap() as usize
    }
}
