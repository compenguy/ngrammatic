//! Contains the `SearchResult` struct, which holds a fuzzy match search result string, and its associated similarity to the query text.

use crate::traits::Float;
use std::cmp::{Ordering, Reverse};

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
    similarity: F,
}

impl<'a, K, F: Float> Eq for SearchResult<'a, K, F> {}

impl<'a, K, F: Float> Ord for SearchResult<'a, K, F> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.similarity.partial_cmp(&other.similarity).unwrap()
    }
}

impl<'a, K, F: Float> PartialOrd for SearchResult<'a, K, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

/// Holds the top n best search results.
pub(crate) struct SearchResultsHeap<'a, K, F: Float> {
    /// The k best search results
    heap: std::collections::BinaryHeap<Reverse<SearchResult<'a, K, F>>>,
    /// The maximum number of results to return
    n: usize,
}

impl<'a, K, F: Float> SearchResultsHeap<'a, K, F> {
    /// Creates a new `SearchResultsHeap` with a maximum number of results to return
    ///
    /// # Arguments
    /// * `n` - The maximum number of results to return
    pub fn new(n: usize) -> Self {
        Self {
            heap: std::collections::BinaryHeap::with_capacity(n),
            n,
        }
    }

    /// Pushes a new search result onto the heap
    ///
    /// # Arguments
    /// * `search_result` - The search result to push onto the heap
    pub fn push(&mut self, search_result: SearchResult<'a, K, F>) {
        if self.heap.len() < self.n {
            self.heap.push(Reverse(search_result));
        } else if let Some(min) = self.heap.peek() {
            if search_result > min.0 {
                self.heap.pop();
                self.heap.push(Reverse(search_result));
            }
        }
    }

    /// Returns the top n best search results
    pub fn into_sorted_vec(self) -> Vec<SearchResult<'a, K, F>> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse(x)| x)
            .collect()
    }
}
