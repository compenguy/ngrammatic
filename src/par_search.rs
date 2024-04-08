//! This module contains the search functionality for the `Corpus` struct.

use crate::search::QueryHashmap;
use crate::search::SearchConfig;
use crate::traits::key::Key;
use crate::NgramIdsAndCooccurrences;
use crate::SearchResults;
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
    <<KS as Keys<NG>>::K as Key<NG, <NG as Ngram>::G>>::Ref: Send + Sync,
    G: WeightedBipartiteGraph + Send + Sync,
{
    #[inline(always)]
    /// Perform a fuzzy search of the `Corpus` for `Ngrams` with a custom `warp` for
    /// results above some `threshold` of similarity to the supplied `key`.  Returns
    /// up to `maximum_number_of_results` results, sorted by highest similarity to lowest.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `config` - The configuration for the search.
    /// * `similarity` - A function that computes the similarity between the query hashmap
    /// and the ngram ids and cooccurrences.
    pub(crate) fn par_search<KR, F: Float>(
        &self,
        key: KR,
        config: SearchConfig<F>,
        similarity: impl Fn(&QueryHashmap, NgramIdsAndCooccurrences<'_, G>) -> F + Send + Sync,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K> + Send + Sync,
    {
        let key: &K = key.as_ref();
        let query_hashmap = self.ngram_ids_from_ngram_counts(key.counts());
        let query_hashmap_ref = &query_hashmap;
        let max_ngram_degree = config.max_ngram_degree(self.number_of_keys());

        // We identify all of the ngrams to be considered in the search, which
        // are the set of ngrams that contain any of the grams in the ngram
        let mut matches = query_hashmap_ref
            .par_ngram_ids()
            .enumerate()
            .flat_map(|(ngram_number, ngram_id)| {
                // If this term is too common, we can skip it as it does not provide
                // much information associated to the rarity of this term.
                if self.number_of_keys_from_ngram_id(ngram_id) > max_ngram_degree {
                    return Vec::new();
                }
                let mut heap = SearchResultsHeap::new(config.maximum_number_of_results());
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
                    if score >= config.minimum_similarity_score() {
                        heap.push(SearchResult::new(self.key_from_id(key_id), score));
                    }
                });
                heap.into_sorted_vec()
            })
            .collect::<Vec<SearchResult<'_, <<KS as Keys<NG>>::K as Key<NG, <NG as Ngram>::G>>::Ref, F>>>();

        // Sort highest similarity to lowest
        matches.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        matches.truncate(config.maximum_number_of_results());
        matches
    }
}
