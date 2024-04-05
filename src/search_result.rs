//! Contains the `SearchResult` struct, which holds a fuzzy match search result string, and its associated similarity to the query text.

use std::cmp::Ordering;
use crate::traits::Float;

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

/// Holds a fuzzy match search result string, and its associated similarity
/// to the query text.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct SearchResult<'a, K, F: Float> {
    /// The key of a fuzzy match
    key: &'a K,
    /// A similarity value indicating how closely the other term matched
    similarity: F
}

impl<'a, K, F: Float> PartialOrd for SearchResult<'a, K, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.similarity.partial_cmp(&other.similarity)
    }
}

impl<'a, K, F: Float> PartialEq for SearchResult<'a, K, F> {
    fn eq(&self, other: &Self) -> bool {
        self.similarity == other.similarity
    }
}

impl<'a, K, F: Float> SearchResult<'a, K, F> {
    /// Trivial constructor used internally to build search results
    /// 
    /// # Arguments
    /// * `key` - The key of a fuzzy match
    /// * `similarity` - A similarity value indicating how closely the other term matched 
    pub(crate) fn new(key: &'a K, similarity: F) -> Self {
        Self { key, similarity }
    }

    /// Returns the key of a fuzzy match
    pub fn key(&self) -> &'a K {
        self.key
    }

    /// Returns a similarity value indicating how closely the other term matched
    pub fn similarity(&self) -> F {
        self.similarity
    }
}