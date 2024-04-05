//! This module contains the search functionality for the `Corpus` struct.
use crate::traits::key::Key;
use crate::{Corpus, Float, Keys, Ngram, SearchResult, Similarity, Warp};

impl<KS, NG, K> Corpus<KS, NG, K>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
{
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
    ///
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
    ///
    pub fn search_with_warp<W, F: Float>(
        &self,
        key: &KS::K,
        warp: W,
        threshold: F,
        limit: usize,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str>
    where
        W: TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: Similarity + Copy,
    {
        let warp = warp.try_into()?;
        let key: &K = key.as_ref();
        let ngram_counts = key.counts();
        let ngram_counts_ref = &ngram_counts;

        // We identify all of the ngrams to be considered in the search, which
        // are the set of ngrams that contain any of the grams in the ngram
        let mut matches = ngram_counts_ref
            .keys()
            .enumerate()
            .filter_map(|(gram_number, gram)| {
                self.ngram_id_from_ngram(gram)
                    .ok()
                    .map(|ngram_id| (gram_number, ngram_id))
            })
            .flat_map(|(gram_number, ngram_id)| {
                self.key_ids_from_ngram_id(ngram_id)
                    .filter_map(move |key_id| {
                        if self
                            .contains_any_ngrams(ngram_counts_ref.keys().take(gram_number), key_id)
                        {
                            // If it has found any gram in the ngram, excluding the one we are currently
                            // looking at, then we can exclude it as it will be included by the other
                            // ngrams
                            return None;
                        }
                        // At this point, we can compute the similarity.
                        let similarity = warp.similarity(
                            ngram_counts_ref
                                .iter()
                                .map(|(ngram, count)| (*ngram, *count)),
                            self.ngrams_and_cooccurrences_from_key(key_id),
                        );
                        if similarity >= threshold {
                            Some(SearchResult::new(self.key_from_id(key_id), similarity))
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<SearchResult<'_, KS::K, F>>>();

        // Sort highest similarity to lowest
        matches.sort_by(|a, b| b.partial_cmp(a).unwrap());
        matches.truncate(limit);
        Ok(matches)
    }
}
