//! Submodule providing a bidirectional weighted bipartite graph implementation based on Webgraph.
use std::iter::Map;

use crate::bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph;
use crate::lender_bit_field_bipartite_graph::RaggedListIter;
use crate::traits::graph::WeightedBipartiteGraph;
use crate::weights::Weights;
use crate::Corpus;
use crate::Key;
use crate::Keys;
use crate::Ngram;
use crate::Offset;
use crate::Offsettable;
use dsi_bitstream::traits::BigEndian;
use std::hash::Hash;
use std::hash::Hasher;
use tempfile::Builder;
use webgraph::prelude::*;

use mem_dbg::MemSize;

#[cfg(feature = "rayon")]
fn num_threads() -> usize {
    rayon::current_num_threads()
}

#[cfg(not(feature = "rayon"))]
fn num_threads() -> usize {
    1
}

type DecoderFactoryType = DynCodesDecoderFactory<
    BigEndian,
    MemoryFactory<BigEndian, MmapHelper<u32>>,
    epserde::deser::DeserType<'static, webgraph::graphs::bvgraph::EF>,
>;

struct LoadedGraph {
    bvgraph: BVGraph<DecoderFactoryType>,
}

impl MemSize for LoadedGraph {
    fn mem_size(&self, _flags: mem_dbg::SizeFlags) -> usize {
        todo!(
            concat!(
                "The trait MemSize is not yet implemented for the ",
                "published version of webgraph. When the new version ",
                "is published, we can replace this todo with a simple ",
                "derive of the MemSize and MemDbg traits." 
            )
        )
    }
}

#[derive(MemSize)]
/// A weighted bipartite graph implementation based on Webgraph.
pub struct BiWebgraph {
    /// Webgraph graph.
    graph: LoadedGraph,
    /// Vector containing the number of times a given gram appears in a given key.
    /// This is a descriptor of an edge from a Key to a Gram.
    srcs_to_dsts_weights: Weights,
    /// Number of source nodes.
    number_of_source_nodes: usize,
    /// Number of destination nodes.
    number_of_destination_nodes: usize,
}

impl<KS, NG, K> TryFrom<Corpus<KS, NG, K, WeightedBitFieldBipartiteGraph>>
    for Corpus<KS, NG, K, BiWebgraph>
where
    NG: Ngram,
    KS: Keys<NG>,
    for<'a> KS::KeyRef<'a>: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
{
    type Error = &'static str;
    fn try_from(
        corpus: Corpus<KS, NG, K, WeightedBitFieldBipartiteGraph>,
    ) -> Result<Self, Self::Error> {
        Ok(Self::new(
            corpus.keys,
            corpus.ngrams,
            corpus.average_key_length,
            corpus.graph.try_into()?,
        ))
    }
}

impl TryFrom<WeightedBitFieldBipartiteGraph> for BiWebgraph {
    type Error = &'static str;

    fn try_from(graph: WeightedBitFieldBipartiteGraph) -> Result<Self, Self::Error> {
        let number_of_nodes = graph.number_of_source_nodes() + graph.number_of_destination_nodes();

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        graph.number_of_source_nodes().hash(&mut hasher);
        graph.number_of_destination_nodes().hash(&mut hasher);
        graph.number_of_edges().hash(&mut hasher);

        // We also introduce a random seed to avoid conflicts with other
        // temporary directories.
        let random_seed: u64 = rand::random();
        random_seed.hash(&mut hasher);

        let seed = hasher.finish();

        let dir = Builder::new()
            .prefix(&seed.to_string())
            .tempdir()
            .map_err(|_| "Could not create temporary directory")?;

        let basename = seed.to_string();

        BVComp::parallel_iter::<BigEndian, RaggedListIter>(
            &basename,
            // We use a number of chunks equal to the number of threads
            // available on this device.
            graph.iter_fractional_ragged_list(num_threads()),
            number_of_nodes,
            CompFlags::default(),
            Threads::Default,
            dir,
        )
        .map_err(|_| "Could not create BVComp")?;

        // Next, we need to create the offset elias fano.
        let cli_args = webgraph::cli::build::ef::CliArgs {
            basename: (&basename).into(),
            n: None,
        };

        webgraph::cli::build::ef::build_eliasfano::<BigEndian>(cli_args)
            .map_err(|_| "Could not build Elias Fano")?;

        let bvgraph = BVGraph::with_basename(&basename)
            .offsets_mode::<LoadMmap>()
            .mode::<LoadMmap>()
            .load()
            .map_err(|_| "Could not load BVGraph")?;

        // For the time being, we delete the files associated with the graph.
        std::fs::remove_file(format!("{}.graph", &basename))
            .map_err(|_| "Could not remove graph (.graph) file")?;
        std::fs::remove_file(format!("{}.properties", &basename))
            .map_err(|_| "Could not remove property (.properties) file")?;
        std::fs::remove_file(format!("{}.ef", &basename))
            .map_err(|_| "Could not remove elias-fano (.ef) file")?;

        Ok(Self {
            graph: LoadedGraph { bvgraph },
            number_of_source_nodes: graph.number_of_source_nodes(),
            number_of_destination_nodes: graph.number_of_destination_nodes(),
            srcs_to_dsts_weights: graph.srcs_to_dsts_weights,
        })
    }
}

impl WeightedBipartiteGraph for BiWebgraph {
    #[inline(always)]
    /// Returns the number of source nodes.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we check that the number of source nodes as provided from
    /// the webgraph method matches the expected value as provided
    /// from the dataset itself.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus).unwrap();
    ///
    /// assert_eq!(webgraph_corpus.graph().number_of_source_nodes(), 699);
    /// ```
    fn number_of_source_nodes(&self) -> usize {
        self.number_of_source_nodes
    }

    #[inline(always)]
    /// Returns the number of destination nodes.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we check that the number of destination nodes as provided from
    /// the webgraph method matches the expected value as provided
    /// from the dataset itself.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus).unwrap();
    ///
    /// assert_eq!(webgraph_corpus.graph().number_of_destination_nodes(), 2534);
    /// ```
    fn number_of_destination_nodes(&self) -> usize {
        self.number_of_destination_nodes
    }

    #[inline(always)]
    /// Returns the number of edges.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we check that the number of edges as provided from
    /// the webgraph method matches the expected value as provided
    /// from the dataset itself.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus).unwrap();
    ///
    /// assert_eq!(webgraph_corpus.graph().number_of_edges(), 9040);
    /// ```
    fn number_of_edges(&self) -> usize {
        self.srcs_to_dsts_weights.num_weights()
    }

    #[inline(always)]
    /// Returns the degree of a given source node.
    ///
    /// # Arguments
    /// * `src_id`: A `usize` which is the source node identifier.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we compare the degree of the key nodes from the first corpus
    /// with the degree of the source nodes from the webgraph corpus,
    /// and we check that they are equal.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus.clone()).unwrap();
    ///
    /// for key_id in 0..corpus.number_of_keys() {
    ///     assert_eq!(
    ///         corpus.graph().src_degree(key_id),
    ///         webgraph_corpus.graph().src_degree(key_id)
    ///     );
    /// }
    /// ```
    fn src_degree(&self, src_id: usize) -> usize {
        self.graph.bvgraph.outdegree(src_id)
    }

    #[inline(always)]
    /// Returns the degree of a given destination node.
    ///
    /// # Arguments
    /// * `dst_id`: A `usize` which is the destination node identifier.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we compare the degree of the gram nodes from the first corpus
    /// with the degree of the destination nodes from the webgraph corpus,
    /// and we check that they are equal.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus.clone()).unwrap();
    ///
    /// for gram_id in 0..corpus.number_of_ngrams() {
    ///     assert_eq!(
    ///         corpus.graph().dst_degree(gram_id),
    ///         webgraph_corpus.graph().dst_degree(gram_id)
    ///     );
    /// }
    /// ```
    fn dst_degree(&self, dst_id: usize) -> usize {
        self.graph
            .bvgraph
            .outdegree(dst_id + self.number_of_source_nodes())
    }

    type Srcs<'a> = <BVGraph<DecoderFactoryType> as RandomAccessLabeling>::Labels<'a>;

    #[inline(always)]
    /// Returns the source nodes of a given destination node.
    ///
    /// # Arguments
    /// * `dst_id`: A `usize` which is the destination node identifier.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we compare the source nodes of the gram nodes from the first corpus
    /// with the source nodes of the destination nodes from the webgraph corpus,
    /// and we check that they are equal.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus.clone()).unwrap();
    ///
    /// for gram_id in 0..corpus.number_of_ngrams() {
    ///     let srcs = corpus.graph().srcs_from_dst(gram_id);
    ///     let webgraph_srcs = webgraph_corpus.graph().srcs_from_dst(gram_id);
    ///
    ///     for (src, webgraph_src) in srcs.zip(webgraph_srcs) {
    ///         assert_eq!(src, webgraph_src);
    ///     }
    /// }
    /// ```
    fn srcs_from_dst(&self, dst_id: usize) -> Self::Srcs<'_> {
        self.graph
            .bvgraph
            .successors(dst_id + self.number_of_source_nodes())
    }

    type Dsts<'a> = Offset<<BVGraph<DecoderFactoryType> as RandomAccessLabeling>::Labels<'a>>;

    #[inline(always)]
    /// Returns the destination nodes of a given source node.
    ///
    /// # Arguments
    /// * `src_id`: A `usize` which is the source node identifier.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we compare the destination nodes of the key nodes from the first corpus
    /// with the destination nodes of the source nodes from the webgraph corpus,
    /// and we check that they are equal.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus.clone()).unwrap();
    ///
    /// for key_id in 0..corpus.number_of_keys() {
    ///     let dsts = corpus.graph().dsts_from_src(key_id);
    ///     let webgraph_dsts = webgraph_corpus.graph().dsts_from_src(key_id);
    ///
    ///     for (dst, webgraph_dst) in dsts.zip(webgraph_dsts) {
    ///         assert_eq!(dst, webgraph_dst);
    ///     }
    /// }
    /// ```
    fn dsts_from_src(&self, src_id: usize) -> Self::Dsts<'_> {
        self.graph
            .bvgraph
            .successors(src_id)
            .offset(-(self.number_of_source_nodes as isize))
    }

    type WeightsSrc<'a> = crate::weights::Succ<
        <crate::weights::CursorReaderFactory as crate::weights::ReaderFactory>::Reader<'a>,
    >;

    #[inline(always)]
    /// Returns the weights of the source nodes of a given destination node.
    ///
    /// # Arguments
    /// * `dst_id`: A `usize` which is the destination node identifier.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we compare the weights of the source nodes of the gram nodes from the first corpus
    /// with the weights of the source nodes of the destination nodes from the webgraph corpus,
    /// and we check that they are equal.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus.clone()).unwrap();
    ///
    /// for key_id in 0..corpus.number_of_keys() {
    ///     let weights = corpus.graph().weights_from_src(key_id);
    ///     let webgraph_weights = webgraph_corpus.graph().weights_from_src(key_id);
    ///
    ///     for (weight, webgraph_weight) in weights.zip(webgraph_weights) {
    ///         assert_eq!(weight, webgraph_weight);
    ///     }
    /// }
    /// ```
    fn weights_from_src(&self, src_id: usize) -> Self::WeightsSrc<'_> {
        self.srcs_to_dsts_weights.labels(src_id)
    }

    type Weights<'a> = crate::weights::WeightsIter<
        <crate::weights::CursorReaderFactory as crate::weights::ReaderFactory>::Reader<'a>,
    >;

    #[inline(always)]
    /// Returns the weights of the edges.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we compare the weights of the edges from the first corpus
    /// with the weights of the edges from the webgraph corpus,
    /// and we check that they are equal.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus.clone()).unwrap();
    ///
    /// let weights = corpus.graph().weights();
    /// let webgraph_weights = webgraph_corpus.graph().weights();
    ///
    /// for (weight, webgraph_weight) in weights.zip(webgraph_weights) {
    ///     assert_eq!(weight, webgraph_weight);
    /// }
    /// ```
    fn weights(&self) -> Self::Weights<'_> {
        self.srcs_to_dsts_weights.weights()
    }

    type Degrees<'a> = Map<
        OffsetDegIter<<DecoderFactoryType as RandomAccessDecoderFactory>::Decoder<'a>>,
        fn((u64, usize)) -> usize,
    >;

    #[inline(always)]
    /// Returns the degrees of the nodes.
    ///
    /// # Examples
    /// In this example, we create the trigram corpus associated
    /// to the ANIMALS dataset which we provide within this crate,
    /// and then we convert it to webgraph format. Secondarily,
    /// we compare the degrees of the nodes from the first corpus
    /// with the degrees of the nodes from the webgraph corpus,
    /// and we check that they are equal.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::from(&ANIMALS);
    /// let webgraph_corpus: Corpus<&[&str; 699], TriGram<char>, str, BiWebgraph> =
    ///     Corpus::try_from(corpus.clone()).unwrap();
    ///
    /// let degrees = corpus.graph().degrees();
    /// let webgraph_degrees = webgraph_corpus.graph().degrees();
    ///
    /// for (degree, webgraph_degree) in degrees.zip(webgraph_degrees) {
    ///     assert_eq!(degree, webgraph_degree);
    /// }
    /// ```
    fn degrees(&self) -> Self::Degrees<'_> {
        self.graph.bvgraph.offset_deg_iter().map(|(_, deg)| deg)
    }
}
