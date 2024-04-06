//! Submodule providing a bidirectional weighted bipartite graph implementation based on Webgraph.
use std::iter::Empty;
use std::iter::Map;
use std::iter::Take;

use crate::bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph;
use crate::lender_bit_field_bipartite_graph::RaggedListIter;
use crate::traits::graph::WeightedBipartiteGraph;
use crate::Corpus;
use crate::Key;
use crate::Keys;
use crate::Ngram;
use dsi_bitstream::traits::BigEndian;
use sux::bits::BitFieldVecIterator;
use sux::traits::BitFieldSliceCore;
use tempfile::Builder;
use webgraph::prelude::*;

// #[cfg(feature = "mem_dbg")]
// use mem_dbg::{MemDbg, MemSize};

type DecoderFactoryType = DynCodesDecoderFactory<
    BigEndian,
    MemoryFactory<BigEndian, MmapHelper<u32>>,
    epserde::deser::DeserType<'static, webgraph::graphs::bvgraph::EF>,
>;

type LoadedGraph = BVGraph<DecoderFactoryType>;

// #[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// A weighted bipartite graph implementation based on Webgraph.
pub struct BiWebgraph {
    /// Webgraph graph.
    graph: LoadedGraph,
    /// Vector containing the number of times a given gram appears in a given key.
    /// This is a descriptor of an edge from a Key to a Gram.
    srcs_to_dsts_weights: sux::prelude::BitFieldVec,
    /// Number of source nodes.
    number_of_source_nodes: usize,
    /// Number of destination nodes.
    number_of_destination_nodes: usize,
}

impl<KS, NG, K> From<Corpus<KS, NG, K, WeightedBitFieldBipartiteGraph>>
    for Corpus<KS, NG, K, BiWebgraph>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
{
    fn from(corpus: Corpus<KS, NG, K, WeightedBitFieldBipartiteGraph>) -> Self {
        Self::new(
            corpus.keys,
            corpus.ngrams,
            corpus.average_key_length,
            corpus.graph.into(),
        )
    }
}

impl From<WeightedBitFieldBipartiteGraph> for BiWebgraph {
    fn from(graph: WeightedBitFieldBipartiteGraph) -> Self {
        let number_of_nodes = graph.number_of_source_nodes() + graph.number_of_destination_nodes();

        let dir = Builder::new()
            .prefix("CompressSimplified")
            .tempdir()
            .unwrap();
        BVComp::parallel_iter::<BigEndian, RaggedListIter>(
            "ngrams",
            graph.iter_fractional_ragged_list(64),
            number_of_nodes,
            CompFlags::default(),
            Threads::Default,
            dir,
        )
        .unwrap();

        let gino = BVGraph::with_basename("ngrams")
            .offsets_mode::<LoadMmap>()
            .mode::<LoadMmap>()
            .load()
            .unwrap();

        Self {
            graph: gino,
            number_of_source_nodes: graph.number_of_source_nodes(),
            number_of_destination_nodes: graph.number_of_destination_nodes(),
            srcs_to_dsts_weights: graph.srcs_to_dsts_weights,
        }
    }
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
        self.graph.outdegree(src_id)
    }

    #[inline(always)]
    fn dst_degree(&self, dst_id: usize) -> usize {
        self.graph.outdegree(dst_id + self.number_of_source_nodes())
    }

    type Srcs<'a> = <LoadedGraph as RandomAccessLabeling>::Labels<'a>;

    #[inline(always)]
    fn srcs_from_dst(&self, dst_id: usize) -> Self::Srcs<'_> {
        self.graph
            .successors(dst_id + self.number_of_source_nodes())
    }

    type Dsts<'a> = Empty<usize>;
    // type Dsts<'a> = <LoadedGraph as RandomAccessLabeling>::Labels<'a>;

    #[inline(always)]
    fn dsts_from_src(&self, src_id: usize) -> Self::Dsts<'_> {
        todo!()
        // self.graph.successors(src_id)
    }

    type WeightsSrc<'a> = Take<BitFieldVecIterator<'a, usize, Vec<usize>>>;

    #[inline(always)]
    fn weights_from_src(&self, src_id: usize) -> Self::WeightsSrc<'_> {
        // let start = self.srcs_offsets.get(src_id);
        // let end = self.srcs_offsets.get(src_id + 1);
        // self.srcs_to_dsts_weights.iter_from(start).take(end - start)
        todo!()
    }

    type Weights<'a> = BitFieldVecIterator<'a, usize, Vec<usize>>;

    #[inline(always)]
    fn weights(&self) -> Self::Weights<'_> {
        self.srcs_to_dsts_weights.iter()
    }

    type Degrees<'a> = Map<
        OffsetDegIter<<DecoderFactoryType as RandomAccessDecoderFactory>::Decoder<'a>>,
        fn((u64, usize)) -> usize,
    >;

    #[inline(always)]
    fn degrees(&self) -> Self::Degrees<'_> {
        todo!()
        // self.graph.offset_deg_iter().map(|(_, deg)| deg)
    }
}
