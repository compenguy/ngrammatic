//! Submodule defining the weighted bipartite graph trait.

/// Trait defining a weighted bipartite graph.
pub trait WeightedBipartiteGraph {
    /// Returns the number of source nodes.
    fn number_of_source_nodes(&self) -> usize;

    /// Returns the number of destination nodes.
    fn number_of_destination_nodes(&self) -> usize;

    /// Returns the number of edges.
    /// 
    /// # Implementation details
    /// The number of edges is the same as the number of weights,
    /// and counts the bidirectional edges only once. Note that
    /// all edges in this graph are bidirectional, as in our use
    /// case all edges are undirected as they represent an occurrence
    /// of a destination (an ngram) in a source (a key).
    fn number_of_edges(&self) -> usize;

    /// Returns the degree of a given source node id.
    /// 
    /// # Arguments
    /// * `src_id` - The source node id.
    fn src_degree(&self, src_id: usize) -> usize;

    /// Returns the degree of a given destination node id.
    /// 
    /// # Arguments
    /// * `dst_id` - The destination node id.
    fn dst_degree(&self, dst_id: usize) -> usize;

    /// Type of the src iterator.
    type Srcs<'a>: ExactSizeIterator<Item = usize>
    where
        Self: 'a;

    /// Returns srcs assocated to a given dst.
    /// 
    /// # Arguments
    /// * `dst_id` - The destination node id.
    fn srcs_from_dst(&self, dst_id: usize) -> Self::Srcs<'_>;

    /// Type of the dst iterator.
    type Dsts<'a>: ExactSizeIterator<Item = usize> + Clone
    where
        Self: 'a;
    
    /// Returns dsts assocated to a given src.
    /// 
    /// # Arguments
    /// * `src_id` - The source node id.
    fn dsts_from_src(&self, src_id: usize) -> Self::Dsts<'_>;

    /// Type of the weights iterator.
    type WeightsSrc<'a>: ExactSizeIterator<Item = usize> + Clone
    where
        Self: 'a;

    /// Returns weights assocated to a given src.
    ///
    /// # Arguments
    /// * `src_id` - The source node id.
    fn weights_from_src(&self, src_id: usize) -> Self::WeightsSrc<'_>;

    /// Type of the weights iterator.
    type Weights<'a>: ExactSizeIterator<Item = usize>
    where
        Self: 'a;

    /// Returns weights associated to the links between a given dst and its srcs.
    fn weights(&self) -> Self::Weights<'_>;

    /// Type of the degrees iterator.
    type Degrees<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Returns the degrees of all the nodes.
    /// 
    /// The first part are the degrees of the source nodes, the second part
    /// are the degrees of the destination nodes.
    fn degrees(&self) -> Self::Degrees<'_>;
}