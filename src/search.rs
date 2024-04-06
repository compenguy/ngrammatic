//! This module contains the search functionality for the `Corpus` struct.
use core::slice::Iter;
use std::collections::HashMap;
use std::iter::{Copied, Map};

use crate::SearchResultsHeap;
use fxhash::FxBuildHasher;

use crate::traits::key::Key;
use crate::{
    Corpus, Float, Keys, Ngram, SearchResult, TrigramSimilarity, Warp, WeightedBipartiteGraph,
};

/// A struct representing a query hashmap, with several values precomputed.
pub struct QueryHashmap {
    /// The hashmap with the identified ngram ids as keys and their counts as values.
    ngram_ids: Vec<(usize, usize)>,
    /// A total count of the unknown ngrams.
    total_unknown_count: usize,
    /// The number of total unique unknown ngrams.
    total_unique_unknown: usize,
    /// A total count of the identified ngrams.
    total_identified_count: usize,
}

impl QueryHashmap {
    #[inline(always)]
    /// Returns the identified ngram ids.
    pub fn ngram_ids(&self) -> Map<Iter<'_, (usize, usize)>, fn(&(usize, usize)) -> usize> {
        self.ngram_ids.iter().map(|(ngram_id, _)| *ngram_id)
    }

    #[cfg(feature = "rayon")]
    #[inline(always)]
    /// Returns a parallel iterator over the identified ngram ids.
    pub fn par_ngram_ids(
        &self,
    ) -> rayon::iter::Map<rayon::slice::Iter<'_, (usize, usize)>, fn(&(usize, usize)) -> usize>
    {
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
        let mut total_unique_unknown = 0;
        let mut total_identified_count = 0;
        let mut ngram_ids = Vec::with_capacity(number_of_ngrams);

        for (ngram, count) in ngram_counts {
            if let Some(ngram_id) = self.ngram_id_from_ngram(ngram) {
                ngram_ids.push((ngram_id, count));
                total_identified_count += count;
            } else {
                total_unknown_count += count;
                total_unique_unknown += 1;
            }
        }

        // We sort the ngram_ids inplace by the first element of the tuple
        ngram_ids.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        QueryHashmap {
            ngram_ids,
            total_unknown_count,
            total_unique_unknown,
            total_identified_count,
        }
    }

    #[inline(always)]
    /// Perform a fuzzy search of the `Corpus` for `Ngrams` above some
    /// `threshold` of similarity to the supplied `key`.  Returns up to `limit`
    /// results, sorted by highest similarity to lowest.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    pub fn search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
    ) -> Vec<SearchResult<'_, KS::K, F>> {
        self.search_with_warp(key, 2_i32, threshold, limit).unwrap()
    }

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
    pub fn search_with_warp<W, F: Float>(
        &self,
        key: &KS::K,
        warp: W,
        threshold: F,
        limit: usize,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str>
    where
        W: TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: TrigramSimilarity + Copy,
    {
        let warp = warp.try_into()?;
        let key: &K = key.as_ref();
        let query_hashmap = self.ngram_ids_from_ngram_counts(key.counts());
        let query_hashmap_ref = &query_hashmap;
        let mut heap = SearchResultsHeap::new(limit);

        // We identify all of the ngrams to be considered in the search, which
        // are the set of ngrams that contain any of the grams in the ngram
        query_hashmap_ref
            .ngram_ids()
            .enumerate()
            .for_each(|(ngram_number, ngram_id)| {
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
                    let similarity = warp.trigram_similarity(
                        query_hashmap_ref,
                        self.ngram_ids_and_cooccurrences_from_key(key_id),
                        NG::ARITY,
                    );
                    if similarity >= threshold {
                        heap.push(SearchResult::new(self.key_from_id(key_id), similarity));
                    }
                });
            });

        // Sort highest similarity to lowest
        Ok(heap.into_sorted_vec())
    }
}
