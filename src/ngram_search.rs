//! Submodule providing the trigram search implementation.

use crate::{
    prelude::*,
    search::{MaxNgramDegree, QueryHashmap, SearchConfig},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Struct providing an ngram search configuration.
pub struct NgramSearchConfig<W: Copy = i32, F: Float = f32> {
    /// The search configuration.
    search_config: SearchConfig<F>,
    /// The warp factor to use in the trigram similarity calculation.
    warp: Warp<W>,
}

impl<W: Copy, F: Float> From<NgramSearchConfig<W, F>> for SearchConfig<F> {
    #[inline(always)]
    /// Returns the search configuration.
    fn from(config: NgramSearchConfig<W, F>) -> Self {
        config.search_config
    }
}

impl<F: Float> Default for NgramSearchConfig<i32, F> {
    #[inline(always)]
    /// Returns the default search configuration.
    fn default() -> Self {
        Self {
            search_config: SearchConfig::default(),
            warp: Warp::try_from(2).unwrap(),
        }
    }
}

impl<W: Copy, F: Float> NgramSearchConfig<W, F> {
    #[inline(always)]
    /// Returns the minimum similarity value for a result to be included in the output.
    pub fn minimum_similarity_score(&self) -> F {
        self.search_config.minimum_similarity_score()
    }

    #[inline(always)]
    /// Returns the maximum number of results to return.
    pub fn maximum_number_of_results(&self) -> usize {
        self.search_config.maximum_number_of_results()
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
    pub fn set_max_ngram_degree(mut self, max_ngram_degree: MaxNgramDegree) -> Self {
        self.search_config = self.search_config.set_max_ngram_degree(max_ngram_degree);
        self
    }

    #[inline(always)]
    /// Set the warp factor to use in the trigram similarity calculation.
    ///
    /// # Arguments
    /// * `warp` - The warp factor to use in the trigram similarity calculation.
    pub fn set_warp<W2>(self, warp: W2) -> Result<NgramSearchConfig<W2, F>, &'static str>
    where
        W2: Copy + TryInto<Warp<W2>, Error = &'static str>,
    {
        Ok(NgramSearchConfig {
            search_config: self.search_config,
            warp: warp.try_into()?,
        })
    }

    #[inline(always)]
    /// Returns the warp factor.
    pub fn warp(&self) -> Warp<W> {
        self.warp
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
    /// Returns the number of ngrams from a given key.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    /// * `max_counts` - Excludes ngrams with counts above this value. By default, equal to the maximum between 1/10 of the number of keys and 100.
    ///
    /// # Example
    /// We can use the ANIMALS dataset shipped with the library to search for similar keys.
    /// We use as unit of the ngram a `char`, and we search for trigrams similar to the key "cat".
    /// Using a `char` is an `u32`, so four times more expensive than using a `u8` or a `ASCIIChar`,
    /// but it allows us to support any character, including emojis. In the following examples we
    /// will also illustrate how to use `ASCIIChar` and `u8` as ngram units.
    /// Note that the search, defined as in this example, is case-sensitive. In the next example we wil
    /// see how to make it case-insensitive and introduce additional normalizations.
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<'_, str, f32>> =
    ///     corpus.ngram_search("Cat", NgramSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), "Cat");
    /// ```
    ///
    /// Now let's proceed with an example to highlight the importance of normalizing the input.
    /// In this case, always using the default search configurations which set the minimum similarity
    /// score to 0.7 and the maximum number of results to 10, we observe that the search for "catt"
    /// does not return any results. This is because while there are matches, none of them are above
    /// the minimum similarity score.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<'_, str, f32>> =
    ///     corpus.ngram_search("catt", NgramSearchConfig::default());
    ///
    /// assert!(results.is_empty());
    /// ```
    ///
    /// If we introduce some normalization to the input, we can see that the search returns the
    /// expected results. In this case, we normalize the input to lowercase by using the struct
    /// marker `Lowercase<str>`. Now that the input is normalized, the search for "catt" returns
    /// the expected result "Cat".
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>, Lowercase<str>> = Corpus::from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<'_, str, f32>> =
    ///     corpus.ngram_search("catt", NgramSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), "Cat");
    /// ```
    ///
    /// In the next example we will see how to use `ASCIIChar` as ngram unit. When
    /// a non-ASCII character is found, it will be ignored and filtered out from the
    /// ngram. This is useful when we want to support only ASCII characters.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<ASCIIChar>> = Corpus::from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<'_, str, f32>> =
    ///     corpus.ngram_search("Cat", NgramSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), "Cat");
    /// ```
    ///
    /// In the next example we will see how to use `u8` as ngram unit. The key difference between
    /// using `u8` and `ASCIIChar` is that `u8` will NOT filter out UTF-8 characters that are not
    /// ASCII, while `ASCIIChar` will. In fact, when using `u8` as ngram unit, the search will fraction
    /// into 4 `u8` any UTF-8 character that is not ASCII.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<u8>> = Corpus::from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<'_, str, f32>> =
    ///     corpus.ngram_search("Cat", NgramSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), "Cat");
    /// ```
    pub fn ngram_search<KR, F: Float>(
        &self,
        key: KR,
        mut config: NgramSearchConfig<i32, F>,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K>,
    {
        config = config.set_warp(2).unwrap();
        self.ngram_search_with_warp(key, config)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `config` - The configuration for the search.
    ///
    /// # Example
    /// In this example we use the ANIMALS dataset shipped with the library to search for similar keys,
    /// using the version of the search with a custom warp factor.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::par_from(&ANIMALS);
    ///
    /// let config = NgramSearchConfig::default().set_warp(2.5).unwrap();
    ///
    /// let results: Vec<SearchResult<'_, str, f32>> = corpus.ngram_search_with_warp("Cat", config);
    ///
    /// assert_eq!(results[0].key(), "Cat");
    /// ```
    pub fn ngram_search_with_warp<KR, W: Copy, F: Float>(
        &self,
        key: KR,
        config: NgramSearchConfig<W, F>,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K>,
        Warp<W>: TrigramSimilarity + Copy,
    {
        let warp: Warp<W> = config.warp();
        self.search(
            key,
            config.into(),
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                warp.ngram_similarity(query, ngrams)
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
    KS::K: AsRef<K> + Send + Sync,
    <<KS as Keys<NG>>::K as Key<NG, <NG as Ngram>::G>>::Ref: Send + Sync,
    K: Key<NG, NG::G> + ?Sized + Send + Sync,
    G: WeightedBipartiteGraph + Send + Sync,
{
    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `config` - The configuration for the search.
    ///
    /// # Example
    /// This is the concurrent version of the `ngram_search` method.
    /// Please look at the documentation of the `ngram_search` method for the extended
    /// documentation.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::par_from(&ANIMALS);
    ///
    /// let results: Vec<SearchResult<'_, str, f32>> =
    ///     corpus.ngram_par_search("Cat", NgramSearchConfig::default());
    ///
    /// assert_eq!(results[0].key(), "Cat");
    /// ```
    pub fn ngram_par_search<KR, F: Float>(
        &self,
        key: KR,
        mut config: NgramSearchConfig<i32, F>,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K> + Send + Sync,
    {
        config = config.set_warp(2).unwrap();
        self.ngram_par_search_with_warp(key, config)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    ///
    /// # Arguments
    /// * `key` - The key to search for in the corpus
    /// * `config` - The configuration for the search.
    ///
    /// # Example
    /// This is the concurrent version of the `ngram_search_with_warp` method.
    /// Please look at the documentation of the `ngram_search_with_warp` method for the extended
    /// documentation.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::par_from(&ANIMALS);
    ///
    /// let config = NgramSearchConfig::default().set_warp(2.5).unwrap();
    ///
    /// let results: Vec<SearchResult<'_, str, f32>> = corpus.ngram_par_search_with_warp("Cat", config);
    ///
    /// assert_eq!(results[0].key(), "Cat");
    /// ```
    pub fn ngram_par_search_with_warp<KR, W, F: Float>(
        &self,
        key: KR,
        config: NgramSearchConfig<W, F>,
    ) -> SearchResults<'_, KS, NG, F>
    where
        KR: AsRef<K> + Send + Sync,
        W: Copy + TryInto<Warp<W>, Error = &'static str>,
        Warp<W>: TrigramSimilarity + Copy + Send + Sync,
    {
        let warp: Warp<W> = config.warp();
        self.par_search(
            key,
            config.into(),
            move |query: &QueryHashmap, ngrams: NgramIdsAndCooccurrences<'_, G>| {
                warp.ngram_similarity(query, ngrams)
            },
        )
    }
}
