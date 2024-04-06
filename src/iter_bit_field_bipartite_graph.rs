//! This module implements an iterator over the edges of a `WeightedBitFieldBipartiteGraph`.

use std::iter::{Skip, Zip};

use crate::bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph;
use crate::traits::graph::WeightedBipartiteGraph;
use sux::bits::BitFieldVecIterator;

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

type SrcIterator<'a> = (
    usize,
    Zip<
        Skip<BitFieldVecIterator<'a, usize, Vec<usize>>>,
        Skip<BitFieldVecIterator<'a, usize, Vec<usize>>>,
    >,
);

type DstIterator<'a> = (usize, Skip<BitFieldVecIterator<'a, usize, Vec<usize>>>);

#[derive(Debug, Clone)]
/// An iterator over the edges of a `WeightedBitFieldBipartiteGraph`.
///
/// # Implementative details
/// This iterator iterates across all edges of the graph, including both the
/// edges from keys to grams and the edges from grams to keys. The edges are
/// returned in the order they are stored in the graph, which is not necessarily
/// the order in which they were added to the graph.
pub struct EdgesIterator<'a> {
    graph: &'a WeightedBitFieldBipartiteGraph,
    src_iterator: Option<SrcIterator<'a>>,
    dst_iterator: Option<DstIterator<'a>>,
    start: usize,
    end: usize,
}

impl<'a> EdgesIterator<'a> {
    /// Returns whether a provided edge id refers to a key-to-gram edge.
    ///
    /// # Arguments
    /// * `edge_id` - The edge id to check.
    fn is_key_to_gram_edge(&self, edge_id: usize) -> bool {
        edge_id < self.graph.number_of_edges()
    }

    #[cfg(any(feature = "rayon", feature = "webgraph"))]
    /// Splits the iterator into two at the given index.
    ///
    /// # Arguments
    /// * `index` - The index at which to split the iterator.
    fn split_at(self, index: usize) -> (Self, Self) {
        let mid = self.start + index;
        let (left_start, left_end) = (self.start, mid);
        let (right_start, right_end) = (mid, self.end);
        (
            EdgesIterator {
                graph: self.graph,
                src_iterator: None,
                dst_iterator: None,
                start: left_start,
                end: left_end,
            },
            EdgesIterator {
                graph: self.graph,
                src_iterator: None,
                dst_iterator: None,
                start: right_start,
                end: right_end,
            },
        )
    }
}

/// An edge in a `WeightedBitFieldBipartiteGraph`.
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Edge {
    /// The source of the edge.
    src: usize,
    /// The destination of the edge.
    dst: usize,
    /// The weight of the edge, when available. When the edge
    /// goes from a key to a gram, this value is the number of
    /// times the gram appears in the key. When the edge goes
    /// from a gram to a key, this value is None.
    weight: Option<usize>,
}

impl Edge {
    /// Returns the source of the edge.
    #[inline(always)]
    pub fn src(&self) -> usize {
        self.src
    }

    /// Returns the destination of the edge.
    #[inline(always)]
    pub fn dst(&self) -> usize {
        self.dst
    }

    /// Returns the weight of the edge.
    #[inline(always)]
    pub fn weight(&self) -> Option<usize> {
        self.weight
    }

    /// Returns the tuple (src, dst) of the edge.
    #[inline(always)]
    pub fn endpoints(self) -> (usize, usize) {
        (self.src, self.dst)
    }
}

impl<'a> From<&'a WeightedBitFieldBipartiteGraph> for EdgesIterator<'a> {
    fn from(graph: &'a WeightedBitFieldBipartiteGraph) -> Self {
        EdgesIterator {
            graph,
            src_iterator: None,
            dst_iterator: None,
            start: 0,
            end: graph.number_of_edges() * 2,
        }
    }
}

impl<'a> Iterator for EdgesIterator<'a> {
    type Item = Edge;

    #[allow(clippy::iter_skip_zero)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        // We check whether we are iterating over the edges from keys to grams or from grams to keys.
        if self.is_key_to_gram_edge(self.start) {
            // If we are in the first portion of the bipartite graph, the one dealing with the edges
            // from keys to grams, we check whether we have already started iterating over the edges
            // from a given source node.
            if let Some((src, ref mut dsts)) = self.src_iterator {
                // If so, we check whether this iterator has more elements to return.
                if let Some((dst, weight)) = dsts.next() {
                    // If it does, we increase the edge counter from the left side, and return the edge.
                    self.start += 1;
                    return Some(Edge {
                        src,
                        // We offset the destination by the number of source nodes, so that while the
                        // bipartite graph is stored in two CSR-like structures, we can return the edges
                        // in a single iterator.
                        dst: dst + self.graph.number_of_source_nodes(),
                        weight: Some(weight),
                    });
                } else {
                    // If it does not have any more edges, it means that we have finished the edges of
                    // this specific source node and we need to move to the next one.
                    let src = src + 1;
                    // We retrieve the destination nodes and weights from the next source node.
                    let dsts = self.graph.dsts_from_src(src).skip(0);
                    let weights = self.graph.weights_from_src(src).skip(0);
                    // And we create the new iterator.
                    self.src_iterator = Some((src, dsts.zip(weights)));
                    // At this point, we can call the next method again, as we have a new iterator.
                    return self.next();
                }
            }
            // If we got here, it means that we do not know the source node of the edge we are
            // currently iterating over. We need to find it. To find it, we need to run a binary
            // search on the source comulative outbound degree, to find the source node of the edge.
            let src = self.graph.src_id_from_edge_id(self.start);
            // Since we may have gotten here from an operation such as the split_at, it may be the
            // case that the start value is greater than the offset of the source node. In this case,
            // we need to adjust the start value to the offset.
            let src_offset = self.graph.src_comulative_outbound_degree(src);

            // The start value may not be exactly equal to the start of the offset, as for instance
            // during a parallel iteration, the start value may be greater than the offset. In this
            // case, we need to adjust the start value to the offset.
            let number_of_edges_to_skip = self.start - src_offset;

            // We retrieve the destination nodes and weights from the source node.
            let dsts = self.graph.dsts_from_src(src).skip(number_of_edges_to_skip);
            let weights = self
                .graph
                .weights_from_src(src)
                .skip(number_of_edges_to_skip);
            // And we create the new iterator.
            self.src_iterator = Some((src, dsts.zip(weights)));
            // At this point, we can call the next method again, as we have a new iterator.
            self.next()
        } else {
            // If we are in the second portion of the bipartite graph, the one dealing with the edges
            // from grams to keys, we check whether we have already started iterating over the edges
            // from a given destination node.
            if let Some((dst, ref mut srcs)) = self.dst_iterator {
                // If so, we check whether this iterator has more elements to return.
                if let Some(src) = srcs.next() {
                    // If it does, we increase the edge counter from the left side, and return the edge.
                    self.start += 1;
                    return Some(Edge {
                        // We offset the source by the number of source nodes, so that while the
                        // bipartite graph is stored in two CSR-like structures, we can return the edges
                        // in a single iterator.
                        src: dst + self.graph.number_of_source_nodes(),
                        dst: src,
                        weight: None,
                    });
                } else {
                    // If it does not have any more edges, it means that we have finished the edges of
                    // this specific destination node and we need to move to the next one.
                    let dst = dst + 1;
                    // We retrieve the source nodes from the next destination node.
                    let srcs = self.graph.srcs_from_dst(dst).skip(0);
                    // And we create the new iterator.
                    self.dst_iterator = Some((dst, srcs));
                    // At this point, we can call the next method again, as we have a new iterator.
                    return self.next();
                }
            }
            // If we got here, it means that we do not know the destination node of the edge we are
            // currently iterating over. We need to find it. To find it, we need to run a binary
            // search on the destination comulative inbound degree, to find the destination node of the edge.

            // Since we have stacked the edges from keys to grams before the edges from grams to keys,
            // we need to adjust the start value to the correct index in the destination nodes.
            let adjusted_start = self.start - self.graph.number_of_edges();
            let dst = self.graph.dst_id_from_edge_id(adjusted_start);
            // Since we may have gotten here from an operation such as the split_at, it may be the
            // case that the start value is greater than the offset of the destination node. In this case,
            // we need to adjust the start value to the offset.
            let dst_offset = self.graph.dst_comulative_inbound_degree(dst);

            // The start value may not be exactly equal to the start of the offset, as for instance
            // during a parallel iteration, the start value may be greater than the offset. In this
            // case, we need to adjust the start value to the offset.
            let number_of_edges_to_skip = adjusted_start - dst_offset;

            // We retrieve the source nodes from the destination node.
            let srcs = self.graph.srcs_from_dst(dst).skip(number_of_edges_to_skip);
            // And we create the new iterator.
            self.dst_iterator = Some((dst, srcs));
            // At this point, we can call the next method again, as we have a new iterator.
            self.next()
        }
    }
}

impl<'a> DoubleEndedIterator for EdgesIterator<'a> {
    #[allow(clippy::iter_skip_zero)]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        // We check whether we are iterating over the edges from keys to grams or from grams to keys.
        if self.is_key_to_gram_edge(self.end - 1) {
            // If we are in the first portion of the bipartite graph, the one dealing with the edges
            // from keys to grams, we check whether we have already started iterating over the edges
            // from a given source node.
            if let Some((src, ref mut dsts)) = self.src_iterator {
                // If so, we check whether this iterator has more elements to return.
                if let Some((dst, weight)) = dsts.next_back() {
                    // If it does, we decrease the edge counter from the right side, and return the edge.
                    self.end -= 1;
                    return Some(Edge {
                        src,
                        // We offset the destination by the number of source nodes, so that while the
                        // bipartite graph is stored in two CSR-like structures, we can return the edges
                        // in a single iterator.
                        dst: dst + self.graph.number_of_source_nodes(),
                        weight: Some(weight),
                    });
                } else {
                    // If it does not have any more edges, it means that we have finished the edges of
                    // this specific source node and we need to move to the next one.
                    let src = src - 1;
                    // We retrieve the destination nodes and weights from the next source node.
                    let dsts = self.graph.dsts_from_src(src).skip(0);
                    let weights = self.graph.weights_from_src(src).skip(0);
                    // And we create the new iterator.
                    self.src_iterator = Some((src, dsts.zip(weights)));
                    // At this point, we can call the next method again, as we have a new iterator.
                    return self.next_back();
                }
            }
            // If we got here, it means that we do not know the source node of the edge we are
            // currently iterating over. We need to find it. To find it, we need to run a binary
            // search on the source comulative outbound degree, to find the source node of the edge.
            let src = self.graph.src_id_from_edge_id(self.end - 1);
            // Since we may have gotten here from an operation such as the split_at, it may be the
            // case that the start value is greater than the offset of the source node. In this case,
            // we need to adjust the start value to the offset.
            let src_offset = self.graph.src_comulative_outbound_degree(src);

            // The start value may not be exactly equal to the start of the offset, as for instance
            // during a parallel iteration, the start value may be greater than the offset. In this
            // case, we need to adjust the start value to the offset.
            let number_of_edges_to_skip = self.end - src_offset;

            // We retrieve the destination nodes and weights from the source node.
            let dsts = self.graph.dsts_from_src(src).skip(number_of_edges_to_skip);
            let weights = self
                .graph
                .weights_from_src(src)
                .skip(number_of_edges_to_skip);
            // And we create the new iterator.
            self.src_iterator = Some((src, dsts.zip(weights)));
            // At this point, we can call the next method again, as we have a new iterator.
            self.next_back()
        } else {
            // If we are in the second portion of the bipartite graph, the one dealing with the edges
            // from grams to keys, we check whether we have already started iterating over the edges
            // from a given destination node.
            if let Some((dst, ref mut srcs)) = self.dst_iterator {
                // If so, we check whether this iterator has more elements to return.
                if let Some(src) = srcs.next_back() {
                    // If it does, we decrease the edge counter from the right side, and return the edge.
                    self.end -= 1;
                    return Some(Edge {
                        // We offset the source by the number of source nodes, so that while the
                        // bipartite graph is stored in two CSR-like structures, we can return the edges
                        // in a single iterator.
                        src: dst + self.graph.number_of_source_nodes(),
                        dst: src,
                        weight: None,
                    });
                } else {
                    // If it does not have any more edges, it means that we have finished the edges of
                    // this specific destination node and we need to move to the next one.
                    let dst = dst - 1;
                    // We retrieve the source nodes from the next destination node.
                    let srcs = self.graph.srcs_from_dst(dst).skip(0);
                    // And we create the new iterator.
                    self.dst_iterator = Some((dst, srcs));
                    // At this point, we can call the next method again, as we have a new iterator.
                    return self.next_back();
                }
            }
            // If we got here, it means that we do not know the destination node of the edge we are
            // currently iterating over. We need to find it. To find it, we need to run a binary
            // search on the destination comulative inbound degree, to find the destination node of the edge.

            // Since we have stacked the edges from keys to grams before the edges from grams to keys,
            // we need to adjust the start value to the correct index in the destination nodes.
            let adjusted_start = self.end - self.graph.number_of_edges();
            let dst = self.graph.dst_id_from_edge_id(adjusted_start);
            // Since we may have gotten here from an operation such as the split_at, it may be the
            // case that the start value is greater than the offset of the destination node. In this case,
            // we need to adjust the start value to the offset.
            let dst_offset = self.graph.dst_comulative_inbound_degree(dst);

            // The start value may not be exactly equal to the start of the offset, as for instance
            // during a parallel iteration, the start value may be greater than the offset. In this
            // case, we need to adjust the start value to the offset.
            let number_of_edges_to_skip = adjusted_start - dst_offset;

            // We retrieve the source nodes from the destination node.
            let srcs = self.graph.srcs_from_dst(dst).skip(number_of_edges_to_skip);
            // And we create the new iterator.
            self.dst_iterator = Some((dst, srcs));
            // At this point, we can call the next method again, as we have a new iterator.
            self.next_back()
        }
    }
}

impl<'a> ExactSizeIterator for EdgesIterator<'a> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

#[cfg(feature = "rayon")]
/// Implementation of the Producer trait for the EdgesIterator.
impl<'a> rayon::iter::plumbing::Producer for EdgesIterator<'a> {
    type Item = Edge;
    type IntoIter = Self;

    fn into_iter(self) -> Self {
        self
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        self.split_at(index)
    }
}

#[cfg(feature = "rayon")]
/// Implementation of the ParallelIterator trait for the EdgesIterator.
impl<'a> rayon::prelude::ParallelIterator for EdgesIterator<'a> {
    type Item = Edge;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        rayon::iter::plumbing::bridge(self, consumer)
    }
}

#[cfg(feature = "rayon")]
/// Implementation of the IndexedParallelIterator trait for the EdgesIterator.
impl<'a> rayon::prelude::IndexedParallelIterator for EdgesIterator<'a> {
    fn len(&self) -> usize {
        self.end - self.start
    }

    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::Consumer<Self::Item>,
    {
        rayon::iter::plumbing::bridge(self, consumer)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: rayon::iter::plumbing::ProducerCallback<Self::Item>,
    {
        callback.callback(self)
    }
}
