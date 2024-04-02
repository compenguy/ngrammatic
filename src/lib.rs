#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::cmp::Ordering;
use std::collections::HashMap;
pub mod traits;
use std::fmt::Debug;
pub use traits::*;
pub mod key_transformers;
pub use key_transformers::*;
mod ngrams;
use ngrams::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

/// Holds a fuzzy match search result string, and its associated similarity
/// to the query text.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct SearchResult<'a, K> {
    /// The key of a fuzzy match
    pub key: &'a K,
    /// A similarity value indicating how closely the other term matched
    pub similarity: f32,
}

impl<'a, K> PartialOrd for SearchResult<'a, K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.similarity.partial_cmp(&other.similarity)
    }
}

impl<'a, K> PartialEq for SearchResult<'a, K> {
    fn eq(&self, other: &Self) -> bool {
        self.similarity == other.similarity
    }
}

impl<'a, K> SearchResult<'a, K> {
    /// Trivial constructor used internally to build search results
    pub(crate) fn new(key: &'a K, similarity: f32) -> Self {
        Self { key, similarity }
    }
}

#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Holds a corpus of words and their ngrams, allowing fuzzy matches of
/// candidate strings against known strings in the corpus.
pub struct Corpus<'a, A: Arity = ArityTwo, KT = Lower, K = String, Counter = usize>
where
    Counter: UnsignedInteger,
    KT: KeyTransformer<K::Key>,
    K: Key,
{
    keys_to_ngrams: HashMap<Normalizer<K, KT>, Ngram<A, Counter>>,
    ngrams_to_keys: HashMap<A::Gram, Vec<&'a Normalizer<K, KT>>>,
}

impl<'a, A: Arity, KT, K, Counter> std::fmt::Debug for Corpus<'a, A, KT, K, Counter>
where
    K: Key + Debug,
    KT: KeyTransformer<K::Key>,
    Counter: UnsignedInteger,
{
    /// Debug format for a `Corpus`. Omits any representation of the
    /// `key_trans` field, as there's no meaningful representation we could
    /// give.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Corpus<{}> {{", A::ARITY)?;
        writeln!(f, "  keys_to_ngrams: {:?},", self.keys_to_ngrams)?;
        writeln!(f, "  ngrams_to_keys: {:?},", self.ngrams_to_keys)?;
        writeln!(f, "}}")
    }
}

impl<'a, A: Arity, KT, K, Counter> Corpus<'a, A, KT, K, Counter>
where
    K: Key,
    KT: KeyTransformer<K::Key>,
    Counter: UnsignedInteger,
{
    /// Generate an `Ngram` for the supplied `text`, and add it to the
    /// `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {a
    /// let mut corpus = CorpusBuilder::<ArityTwo>::default().finish();
    /// corpus.push("tomato");
    /// let results = corpus.search("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn push(&mut self, key: K) {
        let new_key = KT::default().transform(&key.key());
        let bytes: &[u8] = new_key.as_ref();
        let ngram: Ngram<A, Counter> = Ngram::from(bytes);
        self.keys_to_ngrams.insert(key.clone().into(), ngram);

        match self.keys_to_ngrams.entry(key.into()) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                let transmuted_key_ref = unsafe {
                    std::mem::transmute::<&Normalizer<K, KT>, &'a Normalizer<K, KT>>(entry.key())
                };
                for gram in entry.get().iter_grams() {
                    self.ngrams_to_keys
                        .entry(*gram)
                        .or_default()
                        .push(transmuted_key_ref);
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                unreachable!();
            }
        }
    }

    /// If the corpus is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.keys_to_ngrams.is_empty()
    }

    /// Perform a fuzzy search of the `Corpus` for `Ngrams` above some
    /// `threshold` of similarity to the supplied `text`.  Returns up to `limit`
    /// results, sorted by highest similarity to lowest.
    ///
    /// # Arguments
    /// * `text` - The text to search for in the corpus
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    ///
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<ArityTwo>::default().finish();
    /// corpus.push("tomato");
    /// let results = corpus.search("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn search(&self, key: &K, threshold: f32, limit: usize) -> Vec<SearchResult<'_, K>>
    where
        Ngram<A, Counter>: Similarity<i32>,
    {
        self.search_with_warp(key, 2, threshold, limit)
    }

    /// Perform a fuzzy search of the `Corpus` for `Ngrams` with a custom `warp` for
    /// results above some `threshold` of similarity to the supplied `text`.  Returns
    /// up to `limit` results, sorted by highest similarity to lowest.
    ///
    /// # Arguments
    /// * `text` - The text to search for in the corpus
    /// * `warp` - The warp factor to use in the similarity calculation. This value
    ///  should be in the range 1.0 to 3.0, with 2.0 being the default.
    /// * `threshold` - The minimum similarity value for a result to be included in the
    /// output. This value should be in the range 0.0 to 1.0.
    /// * `limit` - The maximum number of results to return.
    ///
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<ArityTwo>::default().finish();
    /// corpus.push("tomato");
    /// let results = corpus.search_with_warp("tomacco", 2.0, 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn search_with_warp<Warp>(
        &self,
        key: &K,
        warp: Warp,
        threshold: f32,
        limit: usize,
    ) -> Vec<SearchResult<'_, K>>
    where
        Warp: Copy,
        Ngram<A, Counter>: Similarity<Warp>,
    {
        let new_key = KT::default().transform(&key.key());
        let bytes: &[u8] = new_key.as_ref();
        let ngram: Ngram<A, Counter> = Ngram::from(bytes);

        // We identify all of the ngrams to be considered in the search, which
        // are the set of ngrams that contain any of the grams in the ngram
        let mut matches = ngram
            .iter_grams()
            .enumerate()
            .filter_map(|(gram_number, gram)| {
                self.ngrams_to_keys
                    .get(gram)
                    .map(|keys| (gram_number, keys))
            })
            .flat_map(|(gram_number, keys)| {
                keys.iter()
                        .map(move |key| (gram_number, key))
                        .filter_map(|(gram_number, key)| {
                            let ngram_candidate = self.keys_to_ngrams.get(key)?;
                            if ngram_candidate
                                .contains_any_grams(ngram.iter_grams().take(gram_number).copied())
                            {
                                // If it has found any gram in the ngram, excluding the one we are currently
                                // looking at, then we can exclude it as it will be included by the other
                                // ngrams
                                None
                            } else {
                                Some((
                                    unsafe {
                                        std::mem::transmute::<
                                            &Normalizer<K, KT>,
                                            &'a Normalizer<K, KT>,
                                        >(key)
                                    },
                                    ngram_candidate,
                                ))
                            }
                        })
                        // At this point, we can compute the similarity.
                        .filter_map(
                            |(key, ngram_candidate): (&Normalizer<K, KT>, &Ngram<A, Counter>)| {
                                let similarity = ngram_candidate.similarity(&ngram, warp);
                                if similarity >= threshold {
                                    let key = key.key();
                                    Some(SearchResult::new(key, similarity))
                                } else {
                                    None
                                }
                            },
                        )
            })
            .collect::<Vec<SearchResult<'a, K>>>();

        // Sort highest similarity to lowest
        matches.sort_by(|a, b| b.partial_cmp(a).unwrap());
        matches.truncate(limit);
        matches
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Build an Ngram Corpus, one setting at a time.
// We provide a builder for Corpus to ensure initialization operations are
// performed in the correct order, without requiring an extensive parameter list
// to a constructor method, and allowing default values by omission.
pub struct CorpusBuilder<A: Arity, K = String, KT = Identity, Counter: UnsignedInteger = usize>
where
    KT: KeyTransformer<K::Key>,
    K: Key,
{
    key_transformer: KT,
    _phantom: std::marker::PhantomData<(K, A, Counter)>,
}

impl<A: Arity, K: Key, Counter: UnsignedInteger> Default for CorpusBuilder<A, K, Identity, Counter>
where
    Identity: KeyTransformer<K::Key>,
{
    /// Initialize a new instance of an `CorpusBuilder`, with a default `arity`
    /// of 2, padding set to `Auto`, for the given `texts`. The default key_trans
    /// function is a pass-through, leaving the keys unmodified.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<ArityTwo>::default().finish();
    /// corpus.push("tomato");
    /// let results = corpus.search("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    fn default() -> Self {
        Self {
            key_transformer: Identity,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<A: Arity, K, KT, Counter> CorpusBuilder<A, K, KT, Counter>
where
    K: Key,
    KT: KeyTransformer<K::Key>,
    Counter: UnsignedInteger,
{
    // /// Set the left padding to build into the `Corpus`.
    // pub fn left_padding(mut self, left_padding: Pad<A, S>) -> Self {
    //     self.left_padding = left_padding;
    //     self
    // }

    // /// Set the right padding to build into the `Corpus`.
    // pub fn right_padding(mut self, right_padding: Pad<A, S>) -> Self {
    //     self.right_padding = right_padding;
    //     self
    // }

    // /// Set both the left and right padding to build into the `Corpus`.
    // pub fn pad_full(mut self, pad: Pad<A, S>) -> Self {
    //     self.left_padding = pad.clone();
    //     self.right_padding = pad;
    //     self
    // }

    /// A key transformation function, supplied as a boxed Fn that takes a
    /// &str and returns a String, applied to all strings that will be added
    /// to the `Corpus`. Searches on the `Corpus` will be similarly
    /// transformed.
    /// ```rust
    /// use ngrammatic::CorpusBuilder;
    /// use ngrammatic::key_transformer::Lower;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<ArityTwo>::default().link_key_transformer(Lower::default()).finish();
    /// corpus.push("tomato");
    /// let results = corpus.search("ToMaTo", 0.90, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'ToMaTo' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'ToMaTo'.");
    /// }
    /// # }
    /// ```
    pub fn link_key_transformer<KT2>(
        self,
        key_transformer: KT2,
    ) -> CorpusBuilder<A, K, <KT as key_transformers::KeyTransformer<K::Key>>::Linked<KT2>, Counter>
    where
        KT2: KeyTransformer<KT::Target>,
    {
        CorpusBuilder {
            key_transformer: self.key_transformer.link(key_transformer),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Convenience function that calls `key_trans` with a closure that
    /// lowercases all keys added to the `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<ArityTwo>::default().lower().finish();
    /// corpus.push("tomato");
    /// let results = corpus.search("ToMaTo", 0.90, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'ToMaTo' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'ToMaTo'.");
    /// }
    /// # }
    /// ```
    pub fn lower(
        self,
    ) -> CorpusBuilder<A, K, <KT as key_transformers::KeyTransformer<K::Key>>::Linked<Lower>, Counter>
    where
        Lower: KeyTransformer<KT::Target>,
    {
        self.link_key_transformer(Lower)
    }

    /// Trim the keys added to the `Corpus` of leading and trailing whitespace.
    ///
    /// # Method availability
    /// This method is solely available for string keys.
    ///
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<ArityTwo>::default().trim().finish();
    /// corpus.push("tomato");
    /// let results = corpus.search("ToMaTo", 0.90, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'ToMaTo' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'ToMaTo'.");
    /// }
    /// # }
    /// ```
    pub fn trim(
        self,
    ) -> CorpusBuilder<A, K, <KT as key_transformers::KeyTransformer<K::Key>>::Linked<Trim>, Counter>
    where
        Trim: KeyTransformer<KT::Target>,
    {
        self.link_key_transformer(Trim)
    }

    /// Yield a `Corpus` instance with all the properties set with this builder.
    pub fn finish<'a>(self) -> Corpus<'a, A, KT, K, Counter> {
        Corpus {
            keys_to_ngrams: HashMap::new(),
            ngrams_to_keys: HashMap::new(),
        }
    }
}
