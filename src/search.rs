//! This module contains the search functionality for the `Corpus` struct.
use crate::NgramIdsAndCooccurrences;
use crate::SearchResultsHeap;
use core::slice::Iter;
use fxhash::FxBuildHasher;
use std::collections::HashMap;
use std::iter::{Copied, Map};

use crate::traits::key::Key;
use crate::{Corpus, Float, Keys, Ngram, SearchResult, WeightedBipartiteGraph};

/// A struct representing a query hashmap, with several values precomputed.
pub struct QueryHashmap {
    /// The hashmap with the identified ngram ids as keys and their counts as values.
    ngram_ids: Vec<(usize, usize)>,
    /// A total count of the unknown ngrams.
    total_unknown_count: usize,
    /// A total count of the identified ngrams.
    total_identified_count: usize,
}

/// A parallel iterator over the identified ngram ids.
pub type ParNgramIds<'a> =
    rayon::iter::Map<rayon::slice::Iter<'a, (usize, usize)>, fn(&(usize, usize)) -> usize>;

/// A sequential iterator over the identified ngram ids.
pub type NgramIds<'a> = Map<Iter<'a, (usize, usize)>, fn(&(usize, usize)) -> usize>;

impl QueryHashmap {
    #[inline(always)]
    /// Returns the identified ngram ids.
    pub fn ngram_ids(&self) -> NgramIds<'_> {
        self.ngram_ids.iter().map(|(ngram_id, _)| *ngram_id)
    }

    #[cfg(feature = "rayon")]
    #[inline(always)]
    /// Returns a parallel iterator over the identified ngram ids.
    pub fn par_ngram_ids(&self) -> ParNgramIds<'_> {
        use rayon::iter::IntoParallelRefIterator;
        use rayon::iter::ParallelIterator;
        self.ngram_ids.par_iter().map(|(ngram_id, _)| *ngram_id)
    }

    #[inline(always)]
    /// Returns the ngram ids and their counts.
    pub fn ngram_ids_and_counts(&self) -> Copied<Iter<'_, (usize, usize)>> {
        self.ngram_ids.iter().copied()
    }

    #[inline(always)]
    /// Returns the total number of unique ngrams, including the unknown ngrams.
    pub fn total_count(&self) -> usize {
        self.total_unknown_count + self.total_identified_count
    }
}

/// We test that the QueryHashmap struct is working as expected.
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_query_hashmap() {
        let corpus: Corpus<[&str; 699], TriGram<char>> = Corpus::from(ANIMALS);
        let query_hashmap = corpus.ngram_ids_from_ngram_counts("cat".counts());
        let ngram_ids: Vec<_> = query_hashmap.ngram_ids().collect();
        // We check that the ngram ids are sorted.
        let mut sorted_ngram_ids = ngram_ids.clone();
        sorted_ngram_ids.sort_unstable();
        assert_eq!(ngram_ids, sorted_ngram_ids, "The ngram ids are not sorted");
        // We check that the total sum of the counts is correct.
        let total_count: usize = query_hashmap
            .ngram_ids_and_counts()
            .map(|(_, count)| count)
            .sum();
        assert_eq!(
            total_count, query_hashmap.total_identified_count,
            "The total count is incorrect"
        );
    }
}

impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph,
{
    #[inline(always)]
    /// Converts the provided hashmap of ngram counts into an hashmap of ngram ids.
    ///
    /// # Arguments
    /// * `ngram_counts` - The hashmap of ngram counts.
    ///
    /// # Implementation details
    /// This function is used to convert the ngram counts into ngram ids, which are used
    /// to search the corpus. Some ngrams may not be present in the corpus, and in that
    /// case they are converted into a None value, which is used to store all of the
    /// unknown ngrams.
    ///
    /// # Returns
    /// A triple containing:
    /// * The hashmap with the identified ngram ids as keys and their counts as values.
    /// * A total of the unknown ngrams.
    /// * A total of the identified ngrams.
    pub(crate) fn ngram_ids_from_ngram_counts(
        &self,
        ngram_counts: HashMap<NG, usize, FxBuildHasher>,
    ) -> QueryHashmap {
        let number_of_ngrams = ngram_counts.len();
        let mut total_unknown_count = 0;
        let mut total_identified_count = 0;
        let mut ngram_ids = Vec::with_capacity(number_of_ngrams);

        for (ngram, count) in ngram_counts {
            if let Some(ngram_id) = self.ngram_id_from_ngram(ngram) {
                ngram_ids.push((ngram_id, count));
                total_identified_count += count;
            } else {
                total_unknown_count += count;
            }
        }

        // We sort the ngram_ids inplace by the first element of the tuple
        ngram_ids.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        QueryHashmap {
            ngram_ids,
            total_unknown_count,
            total_identified_count,
        }
    }

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
    pub(crate) fn search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        max_counts: Option<usize>,
        similarity: impl Fn(&QueryHashmap, NgramIdsAndCooccurrences<'_, G>) -> F,
    ) -> Vec<SearchResult<'_, KS::K, F>> {
        let key: &K = key.as_ref();
        let query_hashmap = self.ngram_ids_from_ngram_counts(key.counts());
        let query_hashmap_ref = &query_hashmap;
        let mut heap = SearchResultsHeap::new(limit);
        let max_counts =
            max_counts.unwrap_or_else(|| if self.keys.len() < 1_000 { 100 } else { self.keys.len() / 10 });

        // We identify all of the ngrams to be considered in the search, which
        // are the set of ngrams that contain any of the grams in the ngram
        query_hashmap_ref
            .ngram_ids()
            .enumerate()
            .for_each(|(ngram_number, ngram_id)| {
                // If this term is too common, we can skip it as it does not provide
                // much information associated to the rarity of this term.
                if self.number_of_keys_from_ngram_id(ngram_id) > max_counts {
                    return;
                }
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
            });

        // Sort highest similarity to lowest
        heap.into_sorted_vec()
    }
}
