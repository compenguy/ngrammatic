//! This module contains the search functionality for the `Corpus` struct.

use crate::search::QueryHashmap;
use crate::traits::key::Key;
use crate::SearchResultsHeap;
use crate::NgramIdsAndCooccurrences;
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
    /// * `warp` - The warp factor to use in the similarity calculation. This value
    ///  should be in the range 1.0 to 3.0, with 2.0 being the default.
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    pub fn par_search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        similarity: impl Fn(&QueryHashmap, NgramIdsAndCooccurrences<'_, G>) -> F + Send + Sync,
    ) -> Vec<SearchResult<'_, KS::K, F>> {
        let key: &K = key.as_ref();
        let query_hashmap = self.ngram_ids_from_ngram_counts(key.counts());
        let query_hashmap_ref = &query_hashmap;

        // We identify all of the ngrams to be considered in the search, which
        // are the set of ngrams that contain any of the grams in the ngram
        let mut matches = query_hashmap_ref
            .par_ngram_ids()
            .enumerate()
            .flat_map(|(ngram_number, ngram_id)| {
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
