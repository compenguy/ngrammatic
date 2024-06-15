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

use crate::bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph;
use crate::traits::*;
use lender::prelude::*;
use sux::prelude::BitFieldVecIterator;
use webgraph::traits::NodeLabelsLender;

/// A trait to create a struct from the graph and the range of nodes to iterate over.
pub trait FromGraphRange {
    /// The iterator type to return.
    type Iter<'a>: Iterator;

    /// Returns a new struct from the graph and the range of nodes to iterate over.
    fn from_graph_range(
        graph: &WeightedBitFieldBipartiteGraph,
        start: usize,
        end: usize,
    ) -> Self::Iter<'_>;
}

/// A struct iterating across the nodes and their neighbours in a ragged list.
pub struct RaggedListIter<'a> {
    graph: &'a WeightedBitFieldBipartiteGraph,
    start: usize,
    end: usize,
}

impl<'a> FromGraphRange for RaggedListIter<'a> {
    type Iter<'b> = RaggedListIter<'b>;

    fn from_graph_range(
        graph: &WeightedBitFieldBipartiteGraph,
        start: usize,
        end: usize,
    ) -> Self::Iter<'_> {
        RaggedListIter { graph, start, end }
    }
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
    type Item = (usize, Offset<BitFieldVecIterator<'a, usize, Vec<usize>>>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        if self.start < self.graph.number_of_source_nodes() {
            let offset = Offset::new(
                self.graph.number_of_source_nodes() as isize,
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
                self.graph.number_of_source_nodes() as isize,
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
    type Label = <Offset<BitFieldVecIterator<'a, usize, Vec<usize>>> as Iterator>::Item;
    type IntoIterator = Offset<BitFieldVecIterator<'a, usize, Vec<usize>>>;
}

/// A struct iterating across the nodes and their neighbours in a ragged list.
pub struct RaggedWeightListIter<'a> {
    graph: &'a WeightedBitFieldBipartiteGraph,
    start: usize,
    end: usize,
}

impl<'a> FromGraphRange for RaggedWeightListIter<'a> {
    type Iter<'b> = RaggedWeightListIter<'b>;

    fn from_graph_range(
        graph: &WeightedBitFieldBipartiteGraph,
        start: usize,
        end: usize,
    ) -> Self::Iter<'_> {
        RaggedWeightListIter { graph, start, end }
    }
}

impl<'a> From<&'a WeightedBitFieldBipartiteGraph> for RaggedWeightListIter<'a> {
    fn from(graph: &'a WeightedBitFieldBipartiteGraph) -> Self {
        RaggedWeightListIter {
            graph,
            start: 0,
            end: graph.number_of_source_nodes(),
        }
    }
}

impl<'a> Iterator for RaggedWeightListIter<'a> {
    type Item = (
        usize,
        <WeightedBitFieldBipartiteGraph as WeightedBipartiteGraph>::WeightsSrc<'a>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let iterator = self.graph.weights_from_src(self.start);
        self.start += 1;
        Some((self.start - 1, iterator))
    }
}

impl<'a> DoubleEndedIterator for RaggedWeightListIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        self.end -= 1;
        Some((self.end, self.graph.weights_from_src(self.end)))
    }
}

impl<'a> ExactSizeIterator for RaggedWeightListIter<'a> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'a, 'b> Lending<'b> for RaggedWeightListIter<'a> {
    type Lend = <Self as Iterator>::Item;
}

impl<'a> Lender for RaggedWeightListIter<'a> {
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        <RaggedWeightListIter as Iterator>::next(self)
    }
}

impl<'a> DoubleEndedLender for RaggedWeightListIter<'a> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        <RaggedWeightListIter as DoubleEndedIterator>::next_back(self)
    }
}

impl<'a> ExactSizeLender for RaggedWeightListIter<'a> {
    fn len(&self) -> usize {
        <RaggedWeightListIter as ExactSizeIterator>::len(self)
    }
}

impl<'a, 'b> NodeLabelsLender<'b> for RaggedWeightListIter<'a> {
    type Label = <BitFieldVecIterator<'a, usize, Vec<usize>> as Iterator>::Item;
    type IntoIterator = <WeightedBitFieldBipartiteGraph as WeightedBipartiteGraph>::WeightsSrc<'a>;
}

/// A struct implementing an iterator over ragged list iterators that only
/// iterate over a given fraction of the nodes.
pub struct FractionalRaggedListIter<'a, I> {
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
    /// The total number of nodes.
    number_of_nodes: usize,
    /// The marker to specify the type of iterator to return.
    _marker: std::marker::PhantomData<I>,
}

impl<'a, I> FractionalRaggedListIter<'a, I> {
    /// Returns a new FractionalRaggedListIter struct.
    pub fn new(
        graph: &'a WeightedBitFieldBipartiteGraph,
        number_of_chunks: usize,
    ) -> FractionalRaggedListIter<'a, RaggedListIter<'a>> {
        FractionalRaggedListIter {
            graph,
            number_of_chunks,
            start: 0,
            end: number_of_chunks,
            number_of_nodes: graph.number_of_source_nodes() + graph.number_of_destination_nodes(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns a new FractionalRaggedListIter struct.
    pub fn new_sources(
        graph: &'a WeightedBitFieldBipartiteGraph,
        number_of_chunks: usize,
    ) -> FractionalRaggedListIter<'a, RaggedWeightListIter<'a>> {
        FractionalRaggedListIter {
            graph,
            number_of_chunks,
            start: 0,
            end: number_of_chunks,
            number_of_nodes: graph.number_of_source_nodes(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the start and end index of the nodes in the current chunk.
    ///
    /// # Arguments
    /// * `index` - The index of the current chunk.
    fn chunk_range(&self, index: usize) -> (usize, usize) {
        let chunk_size = self.number_of_nodes / self.number_of_chunks;
        if index == self.number_of_chunks - 1 {
            (index * chunk_size, self.number_of_nodes)
        } else {
            (index * chunk_size, (index + 1) * chunk_size)
        }
    }
}

impl<'a, I: FromGraphRange> Iterator for FractionalRaggedListIter<'a, I> {
    type Item = <I as FromGraphRange>::Iter<'a>;

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
        Some(I::from_graph_range(self.graph, start, end))
    }
}

impl<'a, I: FromGraphRange> DoubleEndedIterator for FractionalRaggedListIter<'a, I> {
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
        Some(I::from_graph_range(self.graph, start, end))
    }
}

impl<'a, I> ExactSizeIterator for FractionalRaggedListIter<'a, I>
where
    Self: Iterator,
{
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl WeightedBitFieldBipartiteGraph {
    /// Returns an iterator over the nodes and their neighbours in a ragged list.
    pub fn iter_ragged_list(&self) -> RaggedListIter<'_> {
        RaggedListIter::from(self)
    }

    /// Returns an iterator over the nodes and their neighbours in a ragged list.
    pub fn iter_ragged_weight_list(&self) -> RaggedWeightListIter<'_> {
        RaggedWeightListIter::from(self)
    }

    /// Returns an iterator over fractioned ragged list iterators.
    ///
    /// # Arguments
    /// * `number_of_chunks` - The number of chunks to split the nodes into.
    pub fn iter_fractional_ragged_list(
        &self,
        number_of_chunks: usize,
    ) -> FractionalRaggedListIter<'_, RaggedListIter<'_>> {
        FractionalRaggedListIter::<RaggedListIter<'_>>::new(self, number_of_chunks)
    }

    /// Returns an iterator over fractioned ragged weight list iterators.
    ///
    /// # Arguments
    /// * `number_of_chunks` - The number of chunks to split the nodes into.
    pub fn iter_fractional_ragged_weight_list(
        &self,
        number_of_chunks: usize,
    ) -> FractionalRaggedListIter<'_, RaggedWeightListIter<'_>> {
        FractionalRaggedListIter::<RaggedWeightListIter<'_>>::new_sources(self, number_of_chunks)
    }
}
