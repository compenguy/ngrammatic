//! Submodule providing a bidirectional weighted bipartite graph implementation based on Webgraph.
use crate::traits::graph::WeightedBipartiteGraph;
use webgraph::graphs::BVGraph;
use sux::prelude::BitFieldVec;
use sux::traits::BitFieldSlice;
use sux::traits::BitFieldSliceCore;

/// A weighted bipartite graph implementation based on Webgraph.
pub struct BiWebgraph {
    /// Webgraph graph.
    graph: BVGraph,
    /// Vector containing the number of times a given gram appears in a given key.
    /// This is a descriptor of an edge from a Key to a Gram.
    srcs_to_dsts_weights: BitFieldVec,
    /// Number of source nodes.
    number_of_source_nodes: usize,
    /// Number of destination nodes.
    number_of_destination_nodes: usize,
}

impl WeightedBipartiteGraph for BiWebgraph {
    #[inline(always)]
    fn number_of_source_nodes(&self) -> usize {
        self.number_of_source_nodes
    }

    #[inline(always)]
    fn number_of_destination_nodes(&self) -> usize {
        self.number_of_destination_nodes
    }

    #[inline(always)]
    fn number_of_edges(&self) -> usize {
        self.srcs_to_dsts_weights.len()
    }

    #[inline(always)]
    fn src_degree(&self, src_id: usize) -> usize {
        self.graph.out_degree(src_id)
    }

    #[inline(always)]
    fn dst_degree(&self, dst_id: usize) -> usize {
        self.graph.out_degree(dst_id + self.number_of_source_nodes())
    }

    type Srcs<'a>: ExactSizeIterator<Item = usize>
    where
        Self: 'a;

    fn srcs_from_dst(&self, dst_id: usize) -> Self::Srcs<'_> {
        self.graph.in_neighbours(dst_id as u32).map(|x| x as usize)
    }

    type Dsts<'a>: ExactSizeIterator<Item = usize>
    where
        Self: 'a;

    fn dsts_from_src(&self, src_id: usize) -> Self::Dsts<'_> {
        self.graph.out_neighbours(src_id as u32).map(|x| x as usize)
    }

    type Weights<'a>: ExactSizeIterator<Item = usize>
    where
        Self: 'a;

    fn weights_from_src(&self, src_id: usize) -> Self::Weights<'_> {
        self.srcs_to_dsts_weights.get(src_id)
    }
}