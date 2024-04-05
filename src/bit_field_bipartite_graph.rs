//! Submodule providing a bitfield bipartite graph which provides a structure
//! storing a bipartite graph into two CSR-like structures composed of bitfields.

use std::iter::Skip;
use std::iter::Take;
use std::iter::Zip;

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};
use sux::bits::BitFieldVec;
use sux::dict::EliasFano;
use sux::prelude::BitFieldVecIterator;
use sux::rank_sel::SelectFixed2;
use sux::traits::IndexedDict;
use sux::traits::BitFieldSliceCore;
use sux::traits::Pred;

use crate::WeightedBipartiteGraph;

#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
#[derive(Debug, Clone)]
/// A bipartite graph stored in two CSR-like structures composed of bitfields.
pub struct WeightedBitFieldBipartiteGraph {
    /// Vector containing the number of times a given gram appears in a given key.
    /// This is a descriptor of an edge from a Key to a Gram.
    srcs_to_dsts_weights: BitFieldVec,
    /// Vector containing the comulative outbound degree from a given key to grams.
    /// This is a vector with the same length as the keys vector PLUS ONE, and the value at
    /// index `i` is the sum of the oubound degrees before index `i`. The last element of this
    /// vector is the total number of edges in the bipartite graph from keys to grams.
    /// We use this vector alongside the `cooccurrences` vector to find the weighted edges
    /// of a given key. The destinations, i.e. the grams, are found in the `grams` vector.
    srcs_offsets: EliasFano<SelectFixed2>,
    /// Vector contain the comulative inbound degree from a given gram to keys.
    /// This is a vector with the same length as the grams vector PLUS ONE, and the value at
    /// index `i` is the sum of the inbound degrees before index `i`. The last element of this
    /// vector is the total number of edges in the bipartite graph from grams to keys.
    /// These edges are NOT weighted, as the weights are stored in the `cooccurrences` vector and
    /// solely refer to the edges from keys to grams.
    dsts_offsets: EliasFano<SelectFixed2>,
    /// Vector containing the destinations of the edges from keys to grams.
    srcs_to_dsts: BitFieldVec,
    /// Vector containing the sources of the edges from grams to keys.
    dsts_to_srcs: BitFieldVec,
}

impl WeightedBitFieldBipartiteGraph {
    /// Creates a new `WeightedBitFieldBipartiteGraph`.
    ///
    /// # Arguments
    /// * `srcs_to_dsts_weights` - The weights of the edges from keys to grams.
    /// * `srcs_offsets` - The comulative outbound degree from a given key to grams.
    /// * `dsts_offsets` - The comulative inbound degree from a given gram to keys.
    /// * `srcs_to_dsts` - The destinations of the edges from keys to grams.
    /// * `dsts_to_srcs` - The sources of the edges from grams to keys.
    ///
    pub fn new(
        srcs_to_dsts_weights: BitFieldVec,
        srcs_offsets: EliasFano<SelectFixed2>,
        dsts_offsets: EliasFano<SelectFixed2>,
        srcs_to_dsts: BitFieldVec,
        dsts_to_srcs: BitFieldVec,
    ) -> Self {
        assert_eq!(srcs_to_dsts.len(), srcs_to_dsts_weights.len());
        assert_eq!(srcs_to_dsts.len(), dsts_to_srcs.len());

        WeightedBitFieldBipartiteGraph {
            srcs_to_dsts_weights,
            srcs_offsets,
            dsts_offsets,
            srcs_to_dsts,
            dsts_to_srcs,
        }
    }

    /// Returns the comulative outbound degree from a source id.
    ///
    /// # Arguments
    /// * `src_id` - The source id.
    #[inline(always)]
    pub fn src_comulative_outbound_degree(&self, src_id: usize) -> usize {
        self.srcs_offsets.get(src_id)
    }

    /// Returns the comulative inbound degree from a destination id.
    ///
    /// # Arguments
    /// * `dst_id` - The destination id.
    #[inline(always)]
    pub fn dst_comulative_inbound_degree(&self, dst_id: usize) -> usize {
        self.dsts_offsets.get(dst_id)
    }

    /// Returns the src_id from a given edge_id from src to dst.
    ///
    /// # Arguments
    /// * `edge_id` - The edge id.
    ///
    /// # Implementative details
    /// Since the source comulative outbound degree is stored in a bitfield, we can
    /// use the `rank` method to find the source id of a given edge id.
    #[inline(always)]
    pub fn src_id_from_edge_id(&self, edge_id: usize) -> usize {
        self.srcs_offsets.pred(&edge_id).unwrap().0
    }

    /// Returns the dst_id from a given edge_id from src to dst.
    ///
    /// # Arguments
    /// * `edge_id` - The edge id.
    ///
    /// # Implementative details
    /// Since the destination comulative inbound degree is stored in a bitfield, we can
    /// use the `rank` method to find the destination id of a given edge id.
    #[inline(always)]
    pub fn dst_id_from_edge_id(&self, edge_id: usize) -> usize {
        self.dsts_offsets.pred(&edge_id).unwrap().0
    }
}

impl WeightedBipartiteGraph for WeightedBitFieldBipartiteGraph {
    #[inline(always)]
    fn number_of_source_nodes(&self) -> usize {
        self.srcs_offsets.len() - 1
    }

    #[inline(always)]
    fn number_of_destination_nodes(&self) -> usize {
        self.dsts_offsets.len() - 1
    }

    #[inline(always)]
    fn number_of_edges(&self) -> usize {
        self.srcs_to_dsts_weights.len()
    }

    #[inline(always)]
    fn src_degree(&self, src_id: usize) -> usize {
        let start = self.srcs_offsets.get(src_id);
        let end = self.srcs_offsets.get(src_id + 1);
        end - start
    }

    #[inline(always)]
    fn dst_degree(&self, dst_id: usize) -> usize {
        let start = self.dsts_offsets.get(dst_id);
        let end = self.dsts_offsets.get(dst_id + 1);
        end - start
    }

    type Srcs<'a> = Take<BitFieldVecIterator<'a, usize, Vec<usize>>>;

    #[inline(always)]
    fn srcs_from_dst(&self, dst_id: usize) -> Self::Srcs<'_> {
        let start = self.dsts_offsets.get(dst_id);
        let end = self.dsts_offsets.get(dst_id + 1);
        self.srcs_to_dsts.iter_from(start).take(end - start)
    }

    type Dsts<'a> = Take<BitFieldVecIterator<'a, usize, Vec<usize>>>;

    #[inline(always)]
    fn dsts_from_src(&self, src_id: usize) -> Self::Dsts<'_> {
        let start = self.srcs_offsets.get(src_id);
        let end = self.srcs_offsets.get(src_id + 1);
        self.dsts_to_srcs.iter_from(start).take(end - start)
    }

    type Weights<'a> = Take<BitFieldVecIterator<'a, usize, Vec<usize>>>;

    #[inline(always)]
    fn weights_from_src(&self, src_id: usize) -> Self::Weights<'_> {
        let start = self.srcs_offsets.get(src_id);
        let end = self.srcs_offsets.get(src_id + 1);
        self.srcs_to_dsts_weights
            .iter_from(start)
            .take(end - start)
    }
}

type SrcIterator<'a> = (
    usize,
    Zip<
        Skip<Take<BitFieldVecIterator<'a, usize, Vec<usize>>>>,
        Skip<Take<BitFieldVecIterator<'a, usize, Vec<usize>>>>,
    >,
);

type DstIterator<'a> = (
    usize,
    Skip<Take<BitFieldVecIterator<'a, usize, Vec<usize>>>>,
);

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
        let (start, end) = (self.start, self.end);
        let (left_start, left_end) = (start, mid);
        let (right_start, right_end) = (mid, end);
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

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        // We check whether we are iterating over the edges from keys to grams or from grams to keys.
        if self.is_key_to_gram_edge(self.start) {
            let src = self.graph.src_id_from_edge_id(self.start);
            let src_offset = self.graph.src_comulative_outbound_degree(src);

            // The start value may not be exactly equal to the start of the offset, as for instance
            // during a parallel iteration, the start value may be greater than the offset. In this
            // case, we need to adjust the start value to the offset.
            let number_of_edges_to_skip = self.start - src_offset;

            let dsts = self.graph.dsts_from_src(src).skip(number_of_edges_to_skip);
            let weights = self
                .graph
                .weights_from_src(src)
                .skip(number_of_edges_to_skip);
            self.src_iterator = Some((src, dsts.zip(weights)));
            if let Some((src, ref mut dsts)) = self.src_iterator {
                if let Some((dst, weight)) = dsts.next() {
                    self.start += 1;
                    return Some(Edge {
                        src,
                        // We offset the destination by the number of source nodes, so that while the
                        // bipartite graph is stored in two CSR-like structures, we can return the edges
                        // in a single iterator.
                        dst: dst + self.graph.number_of_source_nodes(),
                        weight: Some(weight),
                    });
                }
            }
        } else {
            let adjusted_start = self.start - self.graph.number_of_edges();
            let dst = self.graph.dst_id_from_edge_id(adjusted_start);
            let dst_offset = self.graph.dst_comulative_inbound_degree(dst);

            // The start value may not be exactly equal to the start of the offset, as for instance
            // during a parallel iteration, the start value may be greater than the offset. In this
            // case, we need to adjust the start value to the offset.
            let number_of_edges_to_skip = adjusted_start - dst_offset;

            let srcs = self.graph.srcs_from_dst(dst).skip(number_of_edges_to_skip);
            self.dst_iterator = Some((dst, srcs));

            if let Some((dst, ref mut srcs)) = self.dst_iterator {
                if let Some(src) = srcs.next() {
                    self.start += 1;
                    return Some(Edge {
                        // We offset the source by the number of source nodes, so that while the
                        // bipartite graph is stored in two CSR-like structures, we can return the edges
                        // in a single iterator.
                        src: dst + self.graph.number_of_source_nodes(),
                        dst: src,
                        weight: None,
                    });
                }
            }
        }

        // If we reach this point, it means that we have iterated over all edges.
        assert_eq!(
            self.start, self.end,
            concat!(
                "The start value ({}) is not equal to the end value ({}) ",
                "after iterating over all edges."
            ),
            self.start, self.end
        );

        None
    }
}

impl<'a> DoubleEndedIterator for EdgesIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        // We check whether we are iterating over the edges from keys to grams or from grams to keys.
        if self.is_key_to_gram_edge(self.end - 1) {
            let src = self.graph.src_id_from_edge_id(self.end - 1);
            let src_offset = self.graph.src_comulative_outbound_degree(src);

            // The start value may not be exactly equal to the start of the offset, as for instance
            // during a parallel iteration, the start value may be greater than the offset. In this
            // case, we need to adjust the start value to the offset.
            let number_of_edges_to_skip = self.end - src_offset;

            let dsts = self.graph.dsts_from_src(src).skip(number_of_edges_to_skip);
            let weights = self
                .graph
                .weights_from_src(src)
                .skip(number_of_edges_to_skip);
            self.src_iterator = Some((src, dsts.zip(weights)));

            if let Some((src, ref mut dsts)) = self.src_iterator {
                if let Some((dst, weight)) = dsts.next_back() {
                    self.end -= 1;
                    return Some(Edge {
                        src,
                        // We offset the destination by the number of source nodes, so that while the
                        // bipartite graph is stored in two CSR-like structures, we can return the edges
                        // in a single iterator.
                        dst: dst + self.graph.number_of_source_nodes(),
                        weight: Some(weight),
                    });
                }
            }
        } else {
            let adjusted_end = self.end - 1 - self.graph.number_of_edges();
            let dst = self.graph.dst_id_from_edge_id(adjusted_end);
            let dst_offset = self.graph.dst_comulative_inbound_degree(dst);

            // The start value may not be exactly equal to the start of the offset, as for instance
            // during a parallel iteration, the start value may be greater than the offset. In this
            // case, we need to adjust the start value to the offset.
            let number_of_edges_to_skip = adjusted_end - dst_offset;

            let srcs = self.graph.srcs_from_dst(dst).skip(number_of_edges_to_skip);
            self.dst_iterator = Some((dst, srcs));

            if let Some((dst, ref mut srcs)) = self.dst_iterator {
                if let Some(src) = srcs.next_back() {
                    self.end -= 1;
                    return Some(Edge {
                        // We offset the source by the number of source nodes, so that while the
                        // bipartite graph is stored in two CSR-like structures, we can return the edges
                        // in a single iterator.
                        src: dst + self.graph.number_of_source_nodes(),
                        dst: src,
                        weight: None,
                    });
                }
            }
        }

        // If we reach this point, it means that we have iterated over all edges.
        assert_eq!(
            self.start, self.end,
            concat!(
                "The start value ({}) is not equal to the end value ({}) ",
                "after iterating over all edges."
            ),
            self.start, self.end
        );

        None
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
