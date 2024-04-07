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
    pub(crate) fn new(n: usize) -> Self {
        Self {
            heap: std::collections::BinaryHeap::with_capacity(n),
            n,
        }
    }

    /// Pushes a new search result onto the heap
    ///
    /// # Arguments
    /// * `search_result` - The search result to push onto the heap
    pub(crate) fn push(&mut self, search_result: SearchResult<'a, K, F>) {
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
    pub(crate) fn into_sorted_vec(self) -> Vec<SearchResult<'a, K, F>> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse(x)| x)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result() {
        let key = "key";
        let similarity = 0.5;
        let search_result = SearchResult::new(&key, similarity);

        assert_eq!(search_result.key(), &key);
        assert_eq!(search_result.similarity(), similarity);
    }

    #[test]
    fn test_search_results_heap() {
        let mut search_results_heap = SearchResultsHeap::new(3);

        let search_result1 = SearchResult::new(&"key1", 0.1);
        let search_result2 = SearchResult::new(&"key2", 0.2);
        let search_result3 = SearchResult::new(&"key3", 0.3);
        let search_result4 = SearchResult::new(&"key4", 0.4);
        let search_result5 = SearchResult::new(&"key5", 0.5);

        search_results_heap.push(search_result1);
        search_results_heap.push(search_result2);
        search_results_heap.push(search_result3);
        search_results_heap.push(search_result4);
        search_results_heap.push(search_result5);

        let sorted_search_results = search_results_heap.into_sorted_vec();

        assert_eq!(sorted_search_results.len(), 3);
        assert_eq!(sorted_search_results[0].key(), &"key5");
        assert_eq!(sorted_search_results[1].key(), &"key4");
        assert_eq!(sorted_search_results[2].key(), &"key3");
    }
}