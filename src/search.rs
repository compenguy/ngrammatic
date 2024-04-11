//! This module contains the search functionality for the `Corpus` struct.
use crate::NgramIdsAndCooccurrences;
use crate::SearchResults;
use crate::SearchResultsHeap;
use core::slice::Iter;
use fxhash::FxBuildHasher;
use std::collections::HashMap;
use std::iter::{Copied, Map};

use crate::traits::key::Key;
use crate::{Corpus, Float, Keys, Ngram, SearchResult, WeightedBipartiteGraph};

use mem_dbg::{MemDbg, MemSize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
#[derive(MemSize, MemDbg)]
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

/// Test that ngram_similarity works correctly.
#[cfg(test)]
mod test_ngram_similarity {
    use crate::{TrigramSimilarity, Warp};

    use super::*;

    #[test]
    /// Test that the trigram similarity of a series with itself is 1.
    fn test_simmetric_ngram_similarity() {
        let ngrams = vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)];
        let total_identified_count = ngrams.iter().map(|(_, count)| count).sum();
        let query = QueryHashmap {
            ngram_ids: ngrams.clone(),
            total_unknown_count: 0,
            total_identified_count,
        };

        for warp in 1..=3 {
            let warp = Warp::try_from(warp).unwrap();
            let similarity: f64 = warp.ngram_similarity(&query, ngrams.iter().copied());
            assert_eq!(similarity, 1.0);
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// The maximum degree of the ngrams to consider in the search.
///
/// Ngrams with a degree that is exceeding this value will be excluded from the search.
/// The reasoning is, when an ngram is too common, it does not provide much information
/// about the rarity of the key, while adding a significant amount of computation time
/// since it will be present in a large number of keys.
pub enum MaxNgramDegree {
    /// Leave it to the default value, which is either 100 or 1/10 of the number of keys.
    Default,
    /// Do not exclude any ngrams, no matter how frequent they are.
    None,
    /// Exclude ngrams with counts above the provided value.
    Custom(usize),
    /// Exclude ngrams with counts above a percentage of the total number of keys.
    Percentage(f64),
}

impl MaxNgramDegree {
    #[inline(always)]
    /// Returns the maximum number of ngrams to consider in the search.
    ///
    /// # Arguments
    /// * `number_of_keys` - The number of keys in the corpus.
    fn max_ngram_degree(&self, number_of_keys: usize) -> usize {
        match self {
            Self::Default => {
                if number_of_keys < 1_000 {
                    100
                } else {
                    number_of_keys / 10
                }
            }
            Self::None => usize::MAX,
            Self::Custom(value) => *value,
            Self::Percentage(percentage) => (number_of_keys as f64 * percentage) as usize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Struct providing a search configuration.
pub(crate) struct SearchConfig<F: Float = f64> {
    /// The maximum number of results to return.
    maximum_number_of_results: usize,
    /// The minimum similarity value for a result to be included in the output.
    minimum_similarity_score: F,
    /// The maximum number of ngrams to consider in the search.
    max_ngram_degree: MaxNgramDegree,
}

impl<F: Float> Default for SearchConfig<F> {
    #[inline(always)]
    /// Returns the default search configuration.
    fn default() -> Self {
        Self {
            maximum_number_of_results: 10,
            minimum_similarity_score: F::from_f64(0.7_f64),
            max_ngram_degree: MaxNgramDegree::Default,
        }
    }
}

impl<F: Float> SearchConfig<F> {
    #[inline(always)]
    /// Returns the maximum number of ngrams to consider in the search.
    ///
    /// # Arguments
    /// * `number_of_keys` - The number of keys in the corpus.
    pub(crate) fn compute_max_ngram_degree(&self, number_of_keys: usize) -> usize {
        self.max_ngram_degree.max_ngram_degree(number_of_keys)
    }

    #[inline(always)]
    /// Returns the max ngram degree.
    pub fn max_ngram_degree(&self) -> MaxNgramDegree {
        self.max_ngram_degree
    }

    #[inline(always)]
    /// Returns the minimum similarity value for a result to be included in the output.
    pub fn minimum_similarity_score(&self) -> F {
        self.minimum_similarity_score
    }

    #[inline(always)]
    /// Returns the maximum number of results to return.
    pub fn maximum_number_of_results(&self) -> usize {
        self.maximum_number_of_results
    }

    #[inline(always)]
    /// Set the minimum similarity value for a result to be included in the output.
    ///
    /// # Arguments
    /// * `minimum_similarity_score` - The minimum similarity value for a result to be included in the output.
    pub fn set_minimum_similarity_score(
        mut self,
        minimum_similarity_score: F,
    ) -> Result<Self, &'static str> {
        if minimum_similarity_score < F::from_f64(0.0) {
            return Err("The minimum similarity score must be greater than or equal to 0.0");
        }
        if minimum_similarity_score.is_nan() {
            return Err("The minimum similarity score must not be NaN");
        }
        self.minimum_similarity_score = minimum_similarity_score;
        Ok(self)
    }

    #[inline(always)]
    /// Set the maximum number of results to return.
    ///
    /// # Arguments
    /// * `maximum_number_of_results` - The maximum number of results to return.
    pub fn set_maximum_number_of_results(mut self, maximum_number_of_results: usize) -> Self {
        self.maximum_number_of_results = maximum_number_of_results;
        self
    }

    #[inline(always)]
    /// Set the maximum degree of the ngrams to consider in the search.
    ///
    /// # Arguments
    /// * `max_ngram_degree` - The maximum degree of the ngrams to consider in the search.
    pub fn set_max_ngram_degree(mut self, max_ngram_degree: MaxNgramDegree) -> Self {
        self.max_ngram_degree = max_ngram_degree;
        self
    }
}

impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram,
    KS: Keys<NG>,
    for<'a> KS::KeyRef<'a>: AsRef<K>,
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
    /// * `config` - The configuration for the search.
    /// * `similarity` - A function that computes the similarity between the query hashmap
    pub(crate) fn search<KR, F: Float>(
        &self,
        key: KR,
        config: SearchConfig<F>,
        similarity: impl Fn(&QueryHashmap, NgramIdsAndCooccurrences<'_, G>) -> F,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K>,
    {
        let key: &K = key.as_ref();
        let query_hashmap = self.ngram_ids_from_ngram_counts(key.counts());

        let query_hashmap_ref = &query_hashmap;
        let mut heap = SearchResultsHeap::new(config.maximum_number_of_results());
        let max_ngram_degree = config.compute_max_ngram_degree(self.number_of_keys());

        // We identify all of the ngrams to be considered in the search, which
        // are the set of ngrams that contain any of the grams in the ngram
        query_hashmap_ref
            .ngram_ids()
            .enumerate()
            .for_each(|(ngram_number, ngram_id)| {
                // If this term is too common, we can skip it as it does not provide
                // much information associated to the rarity of this term.
                if self.number_of_keys_from_ngram_id(ngram_id) > max_ngram_degree {
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
                    if score >= config.minimum_similarity_score() {
                        heap.push(SearchResult::new(self.key_from_id(key_id), score));
                    }
                });
            });

        // Sort highest similarity to lowest
        heap.into_sorted_vec()
    }
}
