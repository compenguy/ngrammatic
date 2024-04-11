//! Submodule providing a term frequency-inverse document frequency (TF-IDF) implementation.
use crate::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Struct providing an tf-idf search configuration.
pub struct TFIDFSearchConfig<W: Copy = i32, F: Float = f32> {
    /// The trigram search configuration.
    search_config: NgramSearchConfig<W, F>,
    /// The K1 constant.
    k1: F,
    /// The B constant.
    b: F,
}

impl<W: Copy, F: Float> From<TFIDFSearchConfig<W, F>> for SearchConfig<F> {
    #[inline(always)]
    /// Returns the search configuration.
    fn from(config: TFIDFSearchConfig<W, F>) -> Self {
        config.search_config.into()
    }
}

impl<F: Float> Default for TFIDFSearchConfig<i32, F> {
    #[inline(always)]
    /// Returns the default search configuration.
    fn default() -> Self {
        Self {
            search_config: NgramSearchConfig::default(),
            k1: F::from_f64(1.2),
            b: F::from_f64(0.75),
        }
    }
}

impl<W: Copy, F: Float> TFIDFSearchConfig<W, F> {
    #[inline(always)]
    /// Returns the minimum similarity value for a result to be included in the output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let config = TFIDFSearchConfig::default();
    /// let minimum_similarity_score: f32 = config.minimum_similarity_score();
    ///
    /// assert_eq!(minimum_similarity_score, 0.7_f32);
    /// ```
    pub fn minimum_similarity_score(&self) -> F {
        self.search_config.minimum_similarity_score()
    }

    #[inline(always)]
    /// Returns the maximum number of results to return.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let config: TFIDFSearchConfig<i32, f32> = TFIDFSearchConfig::default();
    /// let maximum_number_of_results = config.maximum_number_of_results();
    ///
    /// assert_eq!(maximum_number_of_results, 10);
    /// ```
    pub fn maximum_number_of_results(&self) -> usize {
        self.search_config.maximum_number_of_results()
    }

    #[inline(always)]
    /// Set the minimum similarity value for a result to be included in the output.
    ///
    /// # Arguments
    /// * `minimum_similarity_score` - The minimum similarity value for a result to be included in the output.
    ///
    /// # Raises
    /// * If the minimum similarity score is not a valid float or is not in the range 0.0 to 1.0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let config = TFIDFSearchConfig::default();
    /// assert_eq!(config.minimum_similarity_score(), 0.7_f32);
    /// assert_eq!(
    ///     config.set_minimum_similarity_score(f32::NAN),
    ///     Err("The minimum similarity score must not be NaN")
    /// );
    /// let config = config.set_minimum_similarity_score(0.5_f32).unwrap();
    ///
    /// assert_eq!(config.minimum_similarity_score(), 0.5_f32);
    /// ```
    pub fn set_minimum_similarity_score(
        mut self,
        minimum_similarity_score: F,
    ) -> Result<Self, &'static str> {
        self.search_config = self
            .search_config
            .set_minimum_similarity_score(minimum_similarity_score)?;
        Ok(self)
    }

    #[inline(always)]
    /// Set the maximum number of results to return.
    ///
    /// # Arguments
    /// * `maximum_number_of_results` - The maximum number of results to return.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let config: TFIDFSearchConfig<i32, f32> = TFIDFSearchConfig::default();
    /// assert_eq!(config.maximum_number_of_results(), 10);
    /// let config = config.set_maximum_number_of_results(5);
    ///
    /// assert_eq!(config.maximum_number_of_results(), 5);
    /// ```
    pub fn set_maximum_number_of_results(mut self, maximum_number_of_results: usize) -> Self {
        self.search_config = self
            .search_config
            .set_maximum_number_of_results(maximum_number_of_results);
        self
    }

    #[inline(always)]
    /// Set the maximum degree of the ngrams to consider in the search.
    ///
    /// # Arguments
    /// * `max_ngram_degree` - The maximum degree of the ngrams to consider in the search.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let config: TFIDFSearchConfig<i32, f32> = TFIDFSearchConfig::default();
    /// assert_eq!(config.max_ngram_degree(), MaxNgramDegree::Default);
    /// let config = config.set_max_ngram_degree(MaxNgramDegree::None);
    ///
    /// assert_eq!(config.max_ngram_degree(), MaxNgramDegree::None);
    /// ```
    pub fn set_max_ngram_degree(mut self, max_ngram_degree: MaxNgramDegree) -> Self {
        self.search_config = self.search_config.set_max_ngram_degree(max_ngram_degree);
        self
    }

    #[inline(always)]
    /// Returns the maximum degree of the ngrams to consider in the search.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let config: TFIDFSearchConfig<i32, f32> = TFIDFSearchConfig::default();
    /// assert_eq!(config.max_ngram_degree(), MaxNgramDegree::Default);
    /// ```
    pub fn max_ngram_degree(&self) -> MaxNgramDegree {
        self.search_config.max_ngram_degree()
    }

    #[inline(always)]
    /// Set the K1 constant.
    ///
    /// # Arguments
    /// * `k1` - The K1 constant.
    ///
    /// # Raises
    /// * If the K1 constant is not a valid float or is not in the range 1.2 to 2.0.
    pub fn set_k1(mut self, k1: F) -> Result<Self, &'static str> {
        if k1.is_nan() || !(1.2..=2.0).contains(&k1.to_f64()) {
            return Err("The K1 constant must be a float in the range 1.2 to 2.0.");
        }
        self.k1 = k1;
        Ok(self)
    }

    #[inline(always)]
    /// Returns the K1 constant.
    pub fn k1(&self) -> F {
        self.k1
    }

    #[inline(always)]
    /// Set the B constant.
    ///
    /// # Arguments
    /// * `b` - The B constant.
    ///
    /// # Raises
    /// * If the B constant is not a valid float or is not in the range 0.0 to 1.0.
    pub fn set_b(mut self, b: F) -> Result<Self, &'static str> {
        if b.is_nan() || !(0.0..=1.0).contains(&b.to_f64()) {
            return Err("The B constant must be a float in the range 0.0 to 1.0.");
        }
        self.b = b;
        Ok(self)
    }

    #[inline(always)]
    /// Returns the B constant.
    pub fn b(&self) -> F {
        self.b
    }

    #[inline(always)]
    /// Set the warp factor to use in the trigram similarity calculation.
    ///
    /// # Arguments
    /// * `warp` - The warp factor to use in the trigram similarity calculation.
    pub fn set_warp<W2>(self, warp: W2) -> Result<TFIDFSearchConfig<W2, F>, &'static str>
    where
        W2: Copy + TryInto<Warp<W2>, Error = &'static str>,
    {
        Ok(TFIDFSearchConfig {
            search_config: self.search_config.set_warp(warp)?,
            k1: self.k1,
            b: self.b,
        })
    }

    #[inline(always)]
    /// Returns the warp factor.
    pub fn warp(&self) -> Warp<W> {
        self.search_config.warp()
    }
}

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
    for<'a> KS::KeyRef<'a>: AsRef<K>,
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
    ///
    /// # Examples
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<[&str; 699], TriGram<char>> = Corpus::from(ANIMALS);
    /// let average_key_length = corpus.average_key_length();
    ///
    /// assert_eq!(average_key_length, 12.962804005722461_f64);
    /// ```
    pub fn average_key_length(&self) -> f64 {
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
    for<'a> KS::KeyRef<'a>: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph,
{
    #[inline(always)]
    /// Returns the best matches using the TF-IDF similarity metric.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus.
    /// * `config` - The TF-IDF search configuration.
    ///
    /// # Examples
    /// We can use the ANIMALS dataset shipped with the library to search for similar keys using
    /// the TF-IDF similarity metric.
    /// We use as unit of the ngram a `char`, and we search for trigrams similar to the key "cat".
    /// Using a `char` is an `u32`, so four times more expensive than using a `u8` or a `ASCIIChar`,
    /// but it allows us to support any character, including emojis. In the following examples we
    /// will also illustrate how to use `ASCIIChar` and `u8` as ngram units.
    /// Note that the search, defined as in this example, is case-sensitive. In the next example we wil
    /// see how to make it case-insensitive and introduce additional normalizations.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<&&str, f32>> =
    ///     corpus.tf_idf_search("Cat", TFIDFSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), &"Cat");
    ///
    /// let results: Vec<SearchResult<&&str, f32>> =
    ///     corpus.tf_idf_search("Catt", TFIDFSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), &"Cat");
    /// ```
    pub fn tf_idf_search<KR, F: Float>(
        &self,
        key: KR,
        config: TFIDFSearchConfig<i32, F>,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K>,
    {
        let k1 = config.k1().to_f64();
        let b = config.b().to_f64();

        self.search(
            key,
            config.into(),
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                F::from_f64(self.tf_idf(query, ngrams, k1, b))
            },
        )
    }

    #[inline(always)]
    /// Returns the best matches using the combined warped ngram + TF-IDF similarity metric.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus.
    /// * `config` - The TF-IDF search configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::par_from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<&&str, f32>> =
    ///     corpus.warped_tf_idf_search("Cat", TFIDFSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), &"Cat");
    ///
    /// let results: Vec<SearchResult<&&str, f32>> =
    ///     corpus.warped_tf_idf_search("Catt", TFIDFSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), &"Cat");
    /// ```
    pub fn warped_tf_idf_search<KR, W, F: Float>(
        &self,
        key: KR,
        config: TFIDFSearchConfig<W, F>,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K>,
        W: Copy + TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: TrigramSimilarity + Copy,
    {
        let k1 = config.k1().to_f64();
        let b = config.b().to_f64();

        let warp: Warp<W> = config.warp();

        self.search(
            key,
            config.into(),
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                F::from_f64(self.tf_idf(query, ngrams.clone(), k1, b))
                    * warp.ngram_similarity(query, ngrams)
            },
        )
    }
}

#[cfg(feature = "rayon")]
impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram + Send + Sync,
    <NG as Ngram>::G: Send + Sync,
    <NG as Ngram>::SortedStorage: Send + Sync,
    KS: Keys<NG> + Send + Sync,
    for<'a> KS::KeyRef<'a>: AsRef<K> + Send + Sync,
    K: Key<NG, NG::G> + ?Sized + Send + Sync,
    <<KS as Keys<NG>>::K as Key<NG, <NG as Ngram>::G>>::Ref: Send + Sync,
    G: WeightedBipartiteGraph + Send + Sync,
{
    #[inline(always)]
    /// Returns the best matches using the TF-IDF similarity metric in parallel.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus.
    /// * `config` - The TF-IDF search configuration.
    ///
    /// # Examples
    /// This is the concurrent version of the example in the `tf_idf_search` method.
    /// If you need a more detailed version of the example, please refer to the documentation of the
    /// sequential `tf_idf_search` method.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<&&str, f32>> =
    ///     corpus.tf_idf_par_search("Cat", TFIDFSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), &"Cat");
    ///
    /// let results: Vec<SearchResult<&&str, f32>> =
    ///     corpus.tf_idf_par_search("Catt", TFIDFSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), &"Cat");
    /// ```
    pub fn tf_idf_par_search<KR, F: Float>(
        &self,
        key: KR,
        config: TFIDFSearchConfig<i32, F>,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K> + Send + Sync,
    {
        let k1 = config.k1.to_f64();
        let b = config.b.to_f64();
        self.par_search(
            key,
            config.into(),
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                F::from_f64(self.tf_idf(query, ngrams, k1, b))
            },
        )
    }

    #[inline(always)]
    /// Returns the best matches using the combined warped ngram + TF-IDF similarity metric in parallel.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus.
    /// * `config` - The TF-IDF search configuration.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::par_from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<&&str, f32>> =
    ///     corpus.warped_tf_idf_par_search("Cat", TFIDFSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), &"Cat");
    ///
    /// let results: Vec<SearchResult<&&str, f32>> =
    ///     corpus.warped_tf_idf_par_search("Catt", TFIDFSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), &"Cat");
    /// ```
    pub fn warped_tf_idf_par_search<KR, W, F: Float>(
        &self,
        key: KR,
        config: TFIDFSearchConfig<W, F>,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K> + Send + Sync,
        W: Copy + TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: TrigramSimilarity + Copy + Send + Sync,
    {
        let k1 = config.k1().to_f64();
        let b = config.b().to_f64();

        let warp: Warp<W> = config.warp();

        self.par_search(
            key,
            config.into(),
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                F::from_f64(self.tf_idf(query, ngrams.clone(), k1, b))
                    * warp.ngram_similarity(query, ngrams)
            },
        )
    }
}
