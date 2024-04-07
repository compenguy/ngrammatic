//! Submodule providing the trigram search implementation.

use crate::{prelude::*, search::QueryHashmap};

impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph,
{
    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    /// 
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    /// * `max_counts` - Excludes ngrams with counts above this value. By default, equal to the maximum between 1/10 of the number of keys and 100.
    pub fn trigram_search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        max_counts: Option<usize>,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str> {
        self.trigram_search_with_warp(key, 2, threshold, limit, max_counts)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    /// 
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `warp` - The warp value to use in the trigram similarity calculation
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    /// * `max_counts` - Excludes ngrams with counts above this value. By default, equal to the maximum between 1/10 of the number of keys and 100.
    pub fn trigram_search_with_warp<W, F: Float>(
        &self,
        key: &KS::K,
        warp: W,
        threshold: F,
        limit: usize,
        max_counts: Option<usize>,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str>
    where
        W: TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: TrigramSimilarity + Copy,
    {
        let warp: Warp<W> = warp.try_into()?;
        Ok(self.search(
            key,
            threshold,
            limit,
            max_counts,
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                warp.trigram_similarity(query, ngrams, NG::ARITY)
            },
        ))
    }
}

#[cfg(feature = "rayon")]
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
    /// Returns the number of ngrams from a given key.
    /// 
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    /// * `max_counts` - Excludes ngrams with counts above this value. By default, equal to the maximum between 1/10 of the number of keys and 100.
    pub fn trigram_par_search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        max_counts: Option<usize>,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str> {
        self.trigram_par_search_with_warp(key, 2, threshold, limit, max_counts)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    /// 
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `warp` - The warp value to use in the trigram similarity calculation
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    /// * `max_counts` - Excludes ngrams with counts above this value. By default, equal to the maximum between 1/10 of the number of keys and 100.
    pub fn trigram_par_search_with_warp<W, F: Float>(
        &self,
        key: &KS::K,
        warp: W,
        threshold: F,
        limit: usize,
        max_counts: Option<usize>,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str>
    where
        W: TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: TrigramSimilarity + Copy + Send + Sync,
    {
        let warp: Warp<W> = warp.try_into()?;
        Ok(self.par_search(
            key,
            threshold,
            limit,
            max_counts,
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                warp.trigram_similarity(query, ngrams, NG::ARITY)
            },
        ))
    }
}
