//! Submodule implementing a struct able to implement the Lending and Lender traits
//! and iterate over the neighbours of a given node. Furthermore, this module implements
//! the method iter_ragged_list for the WeightedBitFieldBipartiteGraph struct.
//!
//! A lender, also called a lending iterator, is an iterator that lends mutable borrows
//! to the items it returns. In particular, this means that the reference to an item
//! is invalidated by the subsequent call to next.
//!
//! Concretely, for this specific implementation, the lender is not strictly necessary
//! as the WeightedBitFieldBipartiteGraph struct is already able to iterate over the
//! neighbours of a given node several times without having to execute expensive
//! operations, but it is required by the current implementation of Webgraph.

use crate::{bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph, WeightedBipartiteGraph};
use lender::prelude::*;
use sux::prelude::BitFieldVecIterator;
use webgraph::traits::NodeLabelsLender;

/// A struct handling the offsetting of ngram nodes in the WeightedBitFieldBipartiteGraph.
pub struct Offset<'a> {
    /// The offset to apply to the nodes. It is expected to be zero for
    /// the source nodes representing the keys and equal to the number of
    /// source nodes for the destination nodes representing the ngrams.
    offset: usize,
    /// The iterator over the nodes.
    iterator: BitFieldVecIterator<'a, usize, Vec<usize>>,
}

impl<'a> From<BitFieldVecIterator<'a, usize, Vec<usize>>> for Offset<'a> {
    fn from(iterator: BitFieldVecIterator<'a, usize, Vec<usize>>) -> Self {
        Self::new(0, iterator)
    }
}

impl<'a> Offset<'a> {
    /// Returns a new Offset struct.
    pub fn new(offset: usize, iterator: BitFieldVecIterator<'a, usize, Vec<usize>>) -> Self {
        Offset { offset, iterator }
    }
}

impl<'a> Iterator for Offset<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|node| node + self.offset)
    }
}

impl<'a> DoubleEndedIterator for Offset<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator.next_back().map(|node| node + self.offset)
    }
}

impl<'a> ExactSizeIterator for Offset<'a> {
    fn len(&self) -> usize {
        self.iterator.len()
    }
}

/// A struct iterating across the nodes and their neighbours in a ragged list.
pub struct RaggedListIter<'a> {
    graph: &'a WeightedBitFieldBipartiteGraph,
    start: usize,
    end: usize,
}

impl<'a> From<&'a WeightedBitFieldBipartiteGraph> for RaggedListIter<'a> {
    fn from(graph: &'a WeightedBitFieldBipartiteGraph) -> Self {
        RaggedListIter {
            graph,
            start: 0,
            end: graph.number_of_source_nodes() + graph.number_of_destination_nodes(),
        }
    }
}

impl<'a> Iterator for RaggedListIter<'a> {
    type Item = (usize, Offset<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        if self.start < self.graph.number_of_source_nodes() {
            let offset = Offset::new(
                self.graph.number_of_source_nodes(),
                self.graph.dsts_from_src(self.start),
            );
            self.start += 1;
            Some((self.start - 1, offset))
        } else {
            let offset = Offset::from(
                self.graph
                    .srcs_from_dst(self.start - self.graph.number_of_source_nodes()),
            );
            self.start += 1;
            Some((self.start - 1, offset))
        }
    }
}

impl<'a> DoubleEndedIterator for RaggedListIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        self.end -= 1;
        if self.end < self.graph.number_of_source_nodes() {
            let offset = Offset::new(
                self.graph.number_of_source_nodes(),
                self.graph.dsts_from_src(self.end),
            );
            Some((self.end, offset))
        } else {
            let offset = Offset::from(
                self.graph
                    .srcs_from_dst(self.end - self.graph.number_of_source_nodes()),
            );
            Some((self.end, offset))
        }
    }
}

impl<'a> ExactSizeIterator for RaggedListIter<'a> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'a, 'b> Lending<'b> for RaggedListIter<'a> {
    type Lend = <Self as Iterator>::Item;
}

impl<'a> Lender for RaggedListIter<'a> {
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        <RaggedListIter as Iterator>::next(self)
    }
}

impl<'a> DoubleEndedLender for RaggedListIter<'a> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        <RaggedListIter as DoubleEndedIterator>::next_back(self)
    }
}

impl<'a> ExactSizeLender for RaggedListIter<'a> {
    fn len(&self) -> usize {
        <RaggedListIter as ExactSizeIterator>::len(self)
    }
}

impl<'a, 'b> NodeLabelsLender<'b> for RaggedListIter<'a> {
    type Label = <Offset<'a> as Iterator>::Item;
    type IntoIterator = Offset<'a>;
}

/// A struct implementing an iterator over ragged list iterators that only
/// iterate over a given fraction of the nodes.
pub struct FractionalRaggedListIter<'a> {
    /// The graph to iterate over.
    graph: &'a WeightedBitFieldBipartiteGraph,
    /// The number of chunks to split the nodes into.
    /// In each chunk, we have number of nodes / number of chunks nodes.
    /// The last chunk may have a different number of nodes when the
    /// number of nodes is not divisible by the number of chunks.
    number_of_chunks: usize,
    /// The index of the first chunk.
    start: usize,
    /// The index of the last chunk.
    end: usize,
}

impl<'a> FractionalRaggedListIter<'a> {
    /// Returns a new FractionalRaggedListIter struct.
    pub fn new(graph: &'a WeightedBitFieldBipartiteGraph, number_of_chunks: usize) -> Self {
        FractionalRaggedListIter {
            graph,
            number_of_chunks,
            start: 0,
            end: number_of_chunks,
        }
    }

    /// Returns the start and end index of the nodes in the current chunk.
    ///
    /// # Arguments
    /// * `index` - The index of the current chunk.
    fn chunk_range(&self, index: usize) -> (usize, usize) {
        let chunk_size = (self.graph.number_of_source_nodes()
            + self.graph.number_of_destination_nodes())
            / self.number_of_chunks;
        if index == self.number_of_chunks - 1 {
            (
                index * chunk_size,
                self.graph.number_of_source_nodes() + self.graph.number_of_destination_nodes(),
            )
        } else {
            (index * chunk_size, (index + 1) * chunk_size)
        }
    }
}

impl<'a> Iterator for FractionalRaggedListIter<'a> {
    type Item = RaggedListIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have already reached the end, we return None.
        if self.start >= self.end {
            return None;
        }

        // We calculate the number of nodes in the current chunk.
        let (start, end) = self.chunk_range(self.start);

        // We update the start index for the next chunk.
        self.start += 1;

        // We return the iterator over the nodes in the current chunk.
        Some(RaggedListIter {
            graph: self.graph,
            start,
            end,
        })
    }
}

impl<'a> DoubleEndedIterator for FractionalRaggedListIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // If we have already reached the end, we return None.
        if self.start >= self.end {
            return None;
        }

        // We update the end index for the next chunk.
        self.end -= 1;

        // We calculate the number of nodes in the current chunk.
        let (start, end) = self.chunk_range(self.end);

        // We return the iterator over the nodes in the current chunk.
        Some(RaggedListIter {
            graph: self.graph,
            start,
            end,
        })
    }
}

impl<'a> ExactSizeIterator for FractionalRaggedListIter<'a> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl WeightedBitFieldBipartiteGraph {
    /// Returns an iterator over the nodes and their neighbours in a ragged list.
    pub fn iter_ragged_list(&self) -> RaggedListIter<'_> {
        RaggedListIter::from(self)
    }

    /// Returns an iterator over fractioned ragged list iterators.
    ///
    /// # Arguments
    /// * `number_of_chunks` - The number of chunks to split the nodes into.
    pub fn iter_fractional_ragged_list(
        &self,
        number_of_chunks: usize,
    ) -> FractionalRaggedListIter<'_> {
        FractionalRaggedListIter::new(self, number_of_chunks)
    }
}
