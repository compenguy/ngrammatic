//! This module contains the search functionality for the `Corpus` struct.

use crate::search::QueryHashmap;
use crate::traits::key::Key;
use crate::NgramIdsAndCooccurrences;
use crate::SearchResultsHeap;
use crate::{Corpus, Float, Keys, Ngram, SearchResult, WeightedBipartiteGraph};
use rayon::prelude::*;

impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram + Send + Sync,
    <NG as Ngram>::G: Send + Sync,
    <NG as Ngram>::SortedStorage: Send + Sync,
    KS: Keys<NG> + Send + Sync,
    KS::K: AsRef<K> + Send + Sync,
    K: Key<NG, NG::G> + ?Sized + Send + Sync,
    G: WeightedBipartiteGraph + Send + Sync,
{
    #[inline(always)]
    /// Perform a fuzzy search of the `Corpus` for `Ngrams` with a custom `warp` for
    /// results above some `threshold` of similarity to the supplied `key`.  Returns
    /// up to `limit` results, sorted by highest similarity to lowest.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    /// * `max_counts` - Excludes ngrams with counts above this value. By default, equal to the maximum between 1/10 of the number of keys and 100.
    /// * `similarity` - A function that computes the similarity between the query hashmap
    /// and the ngram ids and cooccurrences.
    pub fn par_search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        max_counts: Option<usize>,
        similarity: impl Fn(&QueryHashmap, NgramIdsAndCooccurrences<'_, G>) -> F + Send + Sync,
    ) -> Vec<SearchResult<'_, KS::K, F>> {
        let key: &K = key.as_ref();
        let query_hashmap = self.ngram_ids_from_ngram_counts(key.counts());
        let query_hashmap_ref = &query_hashmap;
        let max_counts =
            max_counts.unwrap_or_else(|| if self.keys.len() < 1_000 { 100 } else { self.keys.len() / 10 });

        // We identify all of the ngrams to be considered in the search, which
        // are the set of ngrams that contain any of the grams in the ngram
        let mut matches = query_hashmap_ref
            .par_ngram_ids()
            .enumerate()
            .flat_map(|(ngram_number, ngram_id)| {
                // If this term is too common, we can skip it as it does not provide
                // much information associated to the rarity of this term.
                if self.number_of_keys_from_ngram_id(ngram_id) > max_counts {
                    return Vec::new();
                }
                let mut heap = SearchResultsHeap::new(limit);
                self.key_ids_from_ngram_id(ngram_id).for_each(|key_id| {
                    if self.contains_any_ngram_ids(
                        query_hashmap_ref.ngram_ids().take(ngram_number),
                        key_id,
                    ) {
                        // If it has found any gram in the ngram, excluding the one we are currently
                        // looking at, then we can exclude it as it will be included by the other
                        // ngrams
                        return;
                    }
                    // At this point, we can compute the similarity.
                    let score = similarity(
                        query_hashmap_ref,
                        self.ngram_ids_and_cooccurrences_from_key(key_id),
                    );
                    if score >= threshold {
                        heap.push(SearchResult::new(self.key_from_id(key_id), score));
                    }
                });
                heap.into_sorted_vec()
            })
            .collect::<Vec<SearchResult<'_, KS::K, F>>>();

        // Sort highest similarity to lowest
        matches.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        matches.truncate(limit);
        matches
    }
}
