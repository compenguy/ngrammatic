//!#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod traits;
pub use traits::*;
pub mod search_result;
pub use search_result::*;
pub mod corpus;
pub use corpus::*;
mod trigram_similarity;
pub use trigram_similarity::*;
pub mod adaptative_vector;
pub mod search;
pub use adaptative_vector::*;
pub mod animals;
pub mod bit_field_bipartite_graph;
pub mod corpus_from;
pub mod iter_bit_field_bipartite_graph;
pub mod lender_bit_field_bipartite_graph;
pub mod report;
pub mod tfidf;
pub mod trigram_search;

#[cfg(feature = "rayon")]
pub mod corpus_par_from;

// #[cfg(feature = "webgraph")]
pub mod bi_webgraph;

#[cfg(feature = "rayon")]
pub mod par_search;

/// Re-export of the most commonly used traits and structs.
pub mod prelude {
    pub use crate::adaptative_vector::*;
    pub use crate::corpus::*;
    pub use crate::search_result::*;
    pub use crate::traits::*;
    pub use crate::trigram_similarity::*;
    // #[cfg(feature = "webgraph")]
    pub use crate::bi_webgraph::*;
    pub use crate::animals::*;
}
