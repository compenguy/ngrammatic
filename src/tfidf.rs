//! Submodule providing a term frequency-inverse document frequency (TF-IDF) implementation.
use crate::{prelude::*, search::QueryHashmap};
use std::cmp::Ordering;

/// Returns the Term Frequency (TF) of the provided ngram in the provided key.
///
/// # Arguments
/// * `number_of_ngrams_in_key` - The number of ngrams in the key.
/// * `k1_numerator` - The numerator of the K1 constant.
/// * `k1_denominator` - The denominator of the K1 constant.
fn term_frequency(number_of_ngrams_in_key: usize, k1_numerator: f64, k1_denominator: f64) -> f64 {
    number_of_ngrams_in_key as f64 * k1_numerator
        / (k1_denominator + number_of_ngrams_in_key as f64)
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
    /// Returns the Inverse Document Frequency (IDF) of the provided ngram.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get the IDF of.
    pub(crate) fn inverse_document_frequency(&self, ngram_id: usize) -> f64 {
        let number_of_keys = self.number_of_keys() as f64;
        ((number_of_keys - self.number_of_keys_from_ngram_id(ngram_id) as f64 + 0.5_f64)
            / (self.number_of_keys_from_ngram_id(ngram_id) as f64 + 0.5_f64)
            + 1.0_f64)
            .ln()
    }

    #[inline(always)]
    /// Returns the average document length of the corpus.
    pub(crate) fn average_key_length(&self) -> f64 {
        self.average_key_length
    }

    #[inline(always)]
    /// Returns the TF-IDF of the provided ngram in the provided key.
    ///
    /// # Arguments
    /// * `query` - The query hashmap.
    /// * `key_id` - The id of the key to get the TF-IDF of.
    /// * `k1` - The K1 constant.
    /// * `b` - The B constant.
    pub(crate) fn tf_idf(
        &self,
        query: &QueryHashmap,
        mut ngrams: NgramIdsAndCooccurrences<'_, G>,
        k1: f64,
        b: f64,
    ) -> f64 {
        let document_length = ngrams.clone().map(|(_, weight)| weight).sum::<usize>() as f64;
        let k1_numerator = k1 + 1.0;
        let k1_denominator = k1 * (1.0 - b + b * document_length / self.average_key_length());

        let mut ngram_next = ngrams.next();
        let mut query_ids_and_counts = query.ngram_ids_and_counts();
        let mut query_next = query_ids_and_counts.next();
        let mut total = 0.0;

        while let (Some((ngram_id, cooccurrence)), Some((query_id, count))) =
            (ngram_next, query_next)
        {
            match ngram_id.cmp(&query_id) {
                Ordering::Less => {
                    ngram_next = ngrams.next();
                }
                Ordering::Equal => {
                    total += term_frequency(cooccurrence, k1_numerator, k1_denominator)
                        * self.inverse_document_frequency(ngram_id)
                        * count as f64;
                    ngram_next = ngrams.next();
                    query_next = query_ids_and_counts.next();
                }
                Ordering::Greater => {
                    query_next = query_ids_and_counts.next();
                }
            }
        }

        total
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
    /// Returns the best matches using the TF-IDF similarity metric.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus.
    /// * `threshold` - The minimum similarity value for a result to be included in the output.
    /// * `limit` - The maximum number of results to return.
    /// * `k1` - The K1 constant.
    /// * `b` - The B constant.
    pub fn tf_idf_search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        k1: F,
        b: F,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str> {
        let k1 = k1.to_f64();
        let b = b.to_f64();

        // We check that k1 is a valid float and appears in the range 1.2 to 2.0,
        // with extreme values being allowed.
        if k1.is_nan() || !(1.2..=2.0).contains(&k1) {
            return Err("The K1 constant must be a float in the range 1.2 to 2.0.");
        }

        // We check that b is a valid float and appears in the range 0.0 to 1.0,
        // with extreme values being allowed.
        if b.is_nan() || !(0.0..=1.0).contains(&b) {
            return Err("The B constant must be a float in the range 0.0 to 1.0.");
        }

        Ok(self.search(
            key,
            threshold,
            limit,
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                F::from_f64(self.tf_idf(query, ngrams, k1, b))
            },
        ))
    }

    #[inline(always)]
    /// Returns the best matches using the combined warped ngram + TF-IDF similarity metric.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus.
    /// * `threshold` - The minimum similarity value for a result to be included in the output.
    /// * `limit` - The maximum number of results to return.
    /// * `warp` - The warp factor.
    /// * `k1` - The K1 constant.
    /// * `b` - The B constant.
    pub fn warped_tf_idf_search<W, F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        warp: W,
        k1: F,
        b: F,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str>
    where
        W: TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: TrigramSimilarity + Copy,
    {
        let k1 = k1.to_f64();
        let b = b.to_f64();

        // We check that k1 is a valid float and appears in the range 1.2 to 2.0,
        // with extreme values being allowed.
        if k1.is_nan() || !(1.2..=2.0).contains(&k1) {
            return Err("The K1 constant must be a float in the range 1.2 to 2.0.");
        }

        // We check that b is a valid float and appears in the range 0.0 to 1.0,
        // with extreme values being allowed.
        if b.is_nan() || !(0.0..=1.0).contains(&b) {
            return Err("The B constant must be a float in the range 0.0 to 1.0.");
        }

        let warp: Warp<W> = warp.try_into()?;

        Ok(self.search(
            key,
            threshold,
            limit,
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                F::from_f64(self.tf_idf(query, ngrams.clone(), k1, b))
                    * warp.trigram_similarity(query, ngrams, NG::ARITY)
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
    /// Returns the best matches using the TF-IDF similarity metric in parallel.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus.
    /// * `threshold` - The minimum similarity value for a result to be included in the output.
    /// * `limit` - The maximum number of results to return.
    /// * `k1` - The K1 constant.
    /// * `b` - The B constant.
    pub fn tf_idf_par_search<F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        k1: F,
        b: F,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str> {
        let k1 = k1.to_f64();
        let b = b.to_f64();

        // We check that k1 is a valid float and appears in the range 1.2 to 2.0,
        // with extreme values being allowed.
        if k1.is_nan() || !(1.2..=2.0).contains(&k1) {
            return Err("The K1 constant must be a float in the range 1.2 to 2.0.");
        }

        // We check that b is a valid float and appears in the range 0.0 to 1.0,
        // with extreme values being allowed.
        if b.is_nan() || !(0.0..=1.0).contains(&b) {
            return Err("The B constant must be a float in the range 0.0 to 1.0.");
        }

        Ok(self.par_search(
            key,
            threshold,
            limit,
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                F::from_f64(self.tf_idf(query, ngrams, k1, b))
            },
        ))
    }

    #[inline(always)]
    /// Returns the best matches using the combined warped ngram + TF-IDF similarity metric in parallel.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus.
    /// * `threshold` - The minimum similarity value for a result to be included in the output.
    /// * `limit` - The maximum number of results to return.
    /// * `warp` - The warp factor.
    /// * `k1` - The K1 constant.
    /// * `b` - The B constant.
    pub fn warped_tf_idf_par_search<W, F: Float>(
        &self,
        key: &KS::K,
        threshold: F,
        limit: usize,
        warp: W,
        k1: F,
        b: F,
    ) -> Result<Vec<SearchResult<'_, KS::K, F>>, &'static str>
    where
        W: TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: TrigramSimilarity + Copy + Send + Sync,
    {
        let k1 = k1.to_f64();
        let b = b.to_f64();

        // We check that k1 is a valid float and appears in the range 1.2 to 2.0,
        // with extreme values being allowed.
        if k1.is_nan() || !(1.2..=2.0).contains(&k1) {
            return Err("The K1 constant must be a float in the range 1.2 to 2.0.");
        }

        // We check that b is a valid float and appears in the range 0.0 to 1.0,
        // with extreme values being allowed.
        if b.is_nan() || !(0.0..=1.0).contains(&b) {
            return Err("The B constant must be a float in the range 0.0 to 1.0.");
        }

        let warp: Warp<W> = warp.try_into()?;

        Ok(self.par_search(
            key,
            threshold,
            limit,
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                F::from_f64(self.tf_idf(query, ngrams.clone(), k1, b))
                    * warp.trigram_similarity(query, ngrams, NG::ARITY)
            },
        ))
    }
}
