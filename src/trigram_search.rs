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
    pub fn trigram_search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str> {
        self.trigram_search_with_warp(key, 2, threshold, limit)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    pub fn trigram_search_with_warp<W, F: Float>(
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
        let warp: Warp<W> = warp.try_into()?;
        Ok(self.search(
            key,
            threshold,
            limit,
            move |query: &QueryHashmap,
                  ngrams: NgramIdsAndCooccurrences<'_, G>| { warp.trigram_similarity(query, ngrams, NG::ARITY) },
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
    pub fn trigram_par_search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str> {
        self.trigram_par_search_with_warp(key, 2, threshold, limit)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    pub fn trigram_par_search_with_warp<W, F: Float>(
        &self,
        key: &KS::K,
        warp: W,
        threshold: F,
        limit: usize,
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
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                warp.trigram_similarity(query, ngrams, NG::ARITY)
            },
        ))
    }
}
