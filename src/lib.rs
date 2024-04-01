#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::f32;
use std::hash::{Hash, Hasher};
pub mod key_transformer;
pub use key_transformer::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize, DbgFlags};

/// Holds a fuzzy match search result string, and its associated similarity
/// to the query text.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct SearchResult {
    /// The text of a fuzzy match
    pub text: String,
    /// A similarity value indicating how closely the other term matched
    pub similarity: f32,
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &SearchResult) -> Option<Ordering> {
        self.similarity.partial_cmp(&other.similarity)
    }
}

impl PartialEq for SearchResult {
    fn eq(&self, other: &SearchResult) -> bool {
        self.similarity == other.similarity
    }
}

impl SearchResult {
    /// Trivial constructor used internally to build search results
    pub(crate) fn new(text: String, similarity: f32) -> Self {
        SearchResult { text, similarity }
    }
}

/// Determines how strings are padded before calculating the grams.
/// Having some sort of padding is especially important for small words
/// Auto pad pre/appends `arity`-1 space chars
/// [Read more about the effect of ngram padding](http://journals.plos.org/plosone/article?id=10.1371/journal.pone.0107510)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub enum Pad {
    /// No padding should be added before generating ngrams.
    None,
    /// Automatically set the padding to `arity`-1 space chars.
    Auto,
    /// Use the supplied `String` as padding.
    Pad(String),
}

impl Default for Pad {
    /// Default padding is `Auto`, which pads the left and right with `arity`-1
    /// space characters, making for generally more accurate matching for most
    /// corpuses
    fn default() -> Self {
        Pad::Auto
    }
}

impl Pad {
    /// Render this `Pad` instance as a string
    pub(crate) fn to_string(&self, autopad_width: usize) -> String {
        match *self {
            Pad::Auto => " ".repeat(autopad_width),
            Pad::Pad(ref p) => p.to_string(),
            Pad::None => "".to_string(),
        }
    }

    /// Static method to render a given `&str` with the indicated `Pad`ding.
    pub(crate) fn pad_text(
        text: &str,
        pad_left: Pad,
        pad_right: Pad,
        autopad_width: usize,
    ) -> String {
        pad_left.to_string(autopad_width) + text + pad_right.to_string(autopad_width).as_ref()
    }
}

/// Stores a "word", with all its n-grams. The "arity" member determines the
/// value of "n" used in generating the n-grams.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct Ngram<const ARITY: usize> {
    /// The text for which ngrams were generated
    pub text: String,
    /// The text for which ngrams were generated, with the padding
    /// used for generating the ngrams
    pub text_padded: String,
    /// A collection of all generated ngrams for the text, with a
    /// count of how many times that ngram appears in the text
    pub grams: HashMap<String, usize>,
}

impl<const ARITY: usize> PartialEq for Ngram<ARITY> {
    fn eq(&self, other: &Self) -> bool {
        self.text_padded == other.text_padded
    }
}
impl<const ARITY: usize> Eq for Ngram<ARITY> {}

impl<const ARITY: usize> Hash for Ngram<ARITY> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text_padded.hash(state);
        ARITY.hash(state);
    }
}

// TODO: When rust adds const generics
// (see https://github.com/rust-lang/rust/issues/44580)
// switch Ngram's "arity" member to be a const generic
// on Ngram, and implement From(String) so that we can
// do things like Ngram::<3>::From(text) to construct
// new ngrams

impl<const ARITY: usize> Ngram<ARITY> {
    /// Static method to calculate `Ngram` similarity based on samegram count,
    /// allgram count, and a `warp` factor.
    pub(crate) fn similarity(samegram_count: usize, allgram_count: usize, warp: f32) -> f32 {
        let warp = warp.max(1.0).min(3.0);
        let samegrams = samegram_count as f32;
        let allgrams = allgram_count as f32;
        if (warp - 1.0).abs() < 0.0000000001 {
            samegrams / allgrams
        } else {
            let diffgrams = allgrams - samegrams;
            (allgrams.powf(warp) - diffgrams.powf(warp)) / (allgrams.powf(warp))
        }
    }

    /// Calculate the similarity of this `Ngram` and an `other`, for a given `warp`
    /// factor (clamped to the range 1.0 to 3.0).
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").finish();
    /// let b = NgramBuilder::<2>::new("tomacco").finish();
    /// println!("Similarity factor for {} and {}: {:.0}%", a.text, b.text, a.similarity_to(&b, 2.0) *
    /// 100.0);
    /// # }
    /// ```
    pub fn similarity_to(&self, other: &Self, warp: f32) -> f32 {
        let warp = warp.max(1.0).min(3.0);
        let samegram_count = self.count_samegrams(other);
        let allgram_count = self.count_allgrams(other);
        Self::similarity(samegram_count, allgram_count, warp)
    }

    /// Determines if this `Ngram` matches a given `other` `Ngram`, for a given
    /// `threshold` of certainty. This is equivalent to `matches_with_warp` and a warp
    /// of 2.0.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").finish();
    /// let b = NgramBuilder::<2>::new("tomacco").finish();
    /// if let Some(word_match) = a.matches(&b, 0.40) {
    ///     println!("{} matches {} with {:.0}% certainty", a.text, b.text, word_match.similarity *
    ///     100.0);
    /// } else {
    ///     println!("{} doesn't look anything like {}.", a.text, b.text);
    /// }
    /// # }
    /// ```
    pub fn matches(&self, other: &Self, threshold: f32) -> Option<SearchResult> {
        self.matches_with_warp(other, 2.0, threshold)
    }

    /// Determines if this `Ngram` matches a given `other` `Ngram`, with the specified warp
    /// (clamped to the range 1.0 to 3.0), and for a given `threshold` of certainty.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").finish();
    /// let b = NgramBuilder::<2>::new("tomacco").finish();
    /// if let Some(word_match) = a.matches_with_warp(&b, 2.0, 0.40) {
    ///     println!("{} matches {} with {:.0}% certainty", a.text, b.text, word_match.similarity *
    ///     100.0);
    /// } else {
    ///     println!("{} doesn't look anything like {}.", a.text, b.text);
    /// }
    /// # }
    /// ```
    pub fn matches_with_warp(
        &self,
        other: &Self,
        warp: f32,
        threshold: f32,
    ) -> Option<SearchResult> {
        let similarity = self.similarity_to(other, warp);
        if similarity >= threshold {
            Some(SearchResult::new(other.text.clone(), similarity))
        } else {
            None
        }
    }

    /// Returns the count of symmetrically differing grams between this
    /// `Ngram` and the `other` `Ngram`.
    #[allow(dead_code)]
    pub(crate) fn count_diffgrams(&self, other: &Self) -> usize {
        self.count_allgrams(other) - self.count_samegrams(other)
    }

    /// Returns the total number of unique grams between this
    /// `Ngram` and the `other` `Ngram`.
    pub(crate) fn count_allgrams(&self, other: &Self) -> usize {
        // This is a shortcut that counts all grams between both ngrams
        // Then subtracts out one instance of the grams that are in common
        let self_length = self.text_padded.chars().count();
        let other_length = other.text_padded.chars().count();
        if self_length < ARITY || other_length < ARITY {
            0 // if either ngram is too small, they can't share a common gram
        } else {
            self_length + other_length - (2 * ARITY) + 2 - self.count_samegrams(other)
        }
    }

    /// Returns a count of grams that are common between this
    /// `Ngram` and the `other` `Ngram`.
    pub(crate) fn count_samegrams(&self, other: &Self) -> usize {
        let mut sames: usize = 0;
        for key in self.grams.keys() {
            let selfcount = self.count_gram(key.as_ref());
            let othercount = other.count_gram(key.as_ref());
            sames += selfcount.min(othercount);
        }
        sames
    }

    /// Return the number of times a particular `gram` appears in the `Ngram`
    /// text.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").finish();
    /// println!("Number of times the 'to' bigram appears in {}: {}", a.text, a.count_gram("to"));
    /// # }
    /// ```
    pub fn count_gram(&self, gram: &str) -> usize {
        match self.grams.get(gram) {
            Some(count) => *count,
            None => 0,
        }
    }

    /// Return the total number of grams generated for the `Ngram` text.
    pub fn count_grams(&self) -> usize {
        self.grams.values().sum()
    }

    /// If the set of grams is empty.
    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.count_grams() == 0
    }

    /// If the set of grams contains the specified `gram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").finish();
    /// if a.contains("to") {
    ///     println!("{} contains the bigram 'to'!", a.text);
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn contains(&self, gram: &str) -> bool {
        self.count_gram(gram) > 0
    }

    /// Private method that initializes an `Ngram` by calculating all of its
    /// grams.
    fn init(&mut self) {
        if ARITY > self.text_padded.len() {
            return;
        }
        let chars_padded: Vec<char> = self.text_padded.chars().collect();
        for window in chars_padded.windows(ARITY) {
            let count = self.grams.entry(window.iter().collect()).or_insert(0);
            *count += 1;
        }
    }
}

/// Build an `Ngram`, one setting at a time.
// We provide a builder for ngrams to ensure initialization operations are
// performed in the correct order, without requiring an extensive parameter list
// to a constructor method, and allowing default values by omission.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
pub struct NgramBuilder<const ARITY: usize> {
    pad_left: Pad,
    pad_right: Pad,
    text: String,
}

impl<const ARITY: usize> NgramBuilder<ARITY> {
    /// Initialize a new instance of an `NgramBuilder`, with a default `arity`
    /// of 2, padding set to `Auto`, for the given `text`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").finish();
    /// if a.contains("to") {
    ///     println!("{} contains the bigram 'to'!", a.text);
    /// }
    /// # }
    /// ```
    pub fn new(text: &str) -> Self {
        NgramBuilder {
            pad_left: Pad::Auto,
            pad_right: Pad::Auto,
            text: text.to_string(),
        }
    }

    /// Set the left padding to build into the `Ngram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # use ngrammatic::Pad;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").pad_left(Pad::Pad(" ".to_string())).finish();
    /// if a.contains(" t") {
    ///     println!("{}, when padded, contains the bigram ' t'!", a.text);
    /// }
    /// # }
    /// ```
    pub fn pad_left(mut self, pad_left: Pad) -> Self {
        self.pad_left = pad_left;
        self
    }

    /// Set the right padding to build into the `Ngram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # use ngrammatic::Pad;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").pad_right(Pad::Pad(" ".to_string())).finish();
    /// if a.contains("o ") {
    ///     println!("{}, when padded, contains the bigram 'o '!", a.text);
    /// }
    /// # }
    /// ```
    pub fn pad_right(mut self, pad_right: Pad) -> Self {
        self.pad_right = pad_right;
        self
    }

    /// Set both the left and right padding to build into the `Ngram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # use ngrammatic::Pad;
    /// # fn main() {
    /// let a = NgramBuilder::<2>::new("tomato").pad_full(Pad::Pad(" ".to_string())).finish();
    /// if a.contains(" t") {
    ///     println!("{}, when padded, contains the bigram ' t'!", a.text);
    /// }
    /// if a.contains("o ") {
    ///     println!("{}, when padded, contains the bigram 'o '!", a.text);
    /// }
    /// # }
    /// ```
    pub fn pad_full(mut self, pad: Pad) -> Self {
        self.pad_left = pad.clone();
        self.pad_right = pad;
        self
    }

    /// Yield an `Ngram` instance with all the properties set with this builder.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::<3>::new("tomato").finish();
    /// if a.contains("tom") {
    ///     println!("{} contains the trigram 'tom'!", a.text);
    /// }
    /// # }
    /// ```
    pub fn finish(self) -> Ngram<ARITY> {
        let mut ngram = Ngram {
            text: self.text.clone(),
            text_padded: Pad::pad_text(&self.text, self.pad_left, self.pad_right, ARITY - 1),
            grams: HashMap::new(),
        };
        ngram.init();
        ngram
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Holds a corpus of words and their ngrams, allowing fuzzy matches of
/// candidate strings against known strings in the corpus.
pub struct Corpus<KT, const ARITY: usize>
where
    KT: KeyTransformer,
{
    pad_left: Pad,
    pad_right: Pad,
    ngrams: HashMap<String, Ngram<ARITY>>,
    gram_to_words: HashMap<String, Vec<String>>,
    key_transformer: KT,
}

impl<const ARITY: usize, KT> std::fmt::Debug for Corpus<KT, ARITY>
where
    KT: KeyTransformer,
{
    /// Debug format for a `Corpus`. Omits any representation of the
    /// `key_trans` field, as there's no meaningful representation we could
    /// give.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Corpus<{}> {{", ARITY)?;
        writeln!(f, "  pad_left: {:?},", self.pad_left)?;
        writeln!(f, "  pad_right: {:?},", self.pad_right)?;
        writeln!(f, "  ngrams: {:?},", self.ngrams)?;
        writeln!(f, "}}")
    }
}

impl<const ARITY: usize, KT> Corpus<KT, ARITY>
where
    KT: KeyTransformer,
{
    /// Add the supplied `ngram` to the `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<2>::default().finish();
    /// corpus.add_ngram(NgramBuilder::new("tomato").finish());
    /// let results = corpus.search("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn add_ngram(&mut self, ngram: Ngram<ARITY>) {
        self.ngrams.insert(ngram.text.to_string(), ngram.clone());
        for gram in ngram.grams.keys() {
            let ngram_list = self.gram_to_words.entry(gram.clone()).or_default();
            ngram_list.push(ngram.text.to_string());
        }
    }

    /// Generate an `Ngram` for the supplied `text`, and add it to the
    /// `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<2>::default().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn add_text(&mut self, text: &str) {
        let pad_left = self.pad_left.clone();
        let pad_right = self.pad_right.clone();
        let new_key = self.key_transformer.transform(text);
        self.add_ngram(
            NgramBuilder::<ARITY>::new(&new_key)
                .pad_left(pad_left)
                .pad_right(pad_right)
                .finish(),
        );
    }

    /// If the corpus is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.ngrams.is_empty()
    }

    /// Determines whether an exact match exists for the supplied `text` in the
    /// `Corpus` index, after processing it with the `Corpus`'s `key_trans`
    /// function.
    #[allow(dead_code)]
    pub fn key(&self, text: &str) -> Option<String> {
        if self
            .ngrams
            .contains_key(&self.key_transformer.transform(text))
        {
            Some(text.to_string())
        } else {
            None
        }
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
    /// let mut corpus = CorpusBuilder::<2>::default().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn search(&self, text: &str, threshold: f32, limit: usize) -> Vec<SearchResult> {
        self.search_with_warp(text, 2.0, threshold, limit)
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
    /// let mut corpus = CorpusBuilder::<2>::default().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search_with_warp("tomacco", 2.0, 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn search_with_warp(
        &self,
        text: &str,
        warp: f32,
        threshold: f32,
        limit: usize,
    ) -> Vec<SearchResult> {
        let item = NgramBuilder::new(&self.key_transformer.transform(text))
            .pad_left(self.pad_left.clone())
            .pad_right(self.pad_right.clone())
            .finish();
        let mut ngrams_to_consider: HashSet<&Ngram<ARITY>> = HashSet::new();
        for gram in item.grams.keys() {
            if let Some(words) = self.gram_to_words.get(gram) {
                // Fetch ngrams from raw words
                ngrams_to_consider.extend(words.iter().filter_map(|word| self.ngrams.get(word)));
            }
        }
        let mut results: Vec<SearchResult> = ngrams_to_consider
            .iter()
            .filter_map(|n| item.matches_with_warp(n, warp, threshold))
            .collect();

        // Sort highest similarity to lowest
        results.sort_by(|a, b| b.partial_cmp(a).unwrap());
        results.truncate(limit);
        results
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Build an Ngram Corpus, one setting at a time.
// We provide a builder for Corpus to ensure initialization operations are
// performed in the correct order, without requiring an extensive parameter list
// to a constructor method, and allowing default values by omission.
pub struct CorpusBuilder<const ARITY: usize, KT = IdentityKeyTransformer>
where
    KT: KeyTransformer,
{
    pad_left: Pad,
    pad_right: Pad,
    texts: Vec<String>,
    key_transformer: KT,
}

impl<const ARITY: usize, KT> std::fmt::Debug for CorpusBuilder<ARITY, KT>
where
    KT: KeyTransformer,
{
    /// Debug format for a `CorpusBuilder`. Omits any representation of the
    /// `key_trans` field, as there's no meaningful representation we could
    /// give.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "CorpusBuilder<{}> {{", ARITY)?;
        writeln!(f, "  pad_left: {:?},", self.pad_left)?;
        writeln!(f, "  pad_right: {:?},", self.pad_right)?;
        writeln!(f, "  texts: {:?},", self.texts)?;
        writeln!(f, "}}")
    }
}

impl<const ARITY: usize> Default for CorpusBuilder<ARITY> {
    /// Initialize a new instance of an `CorpusBuilder`, with a default `arity`
    /// of 2, padding set to `Auto`, for the given `texts`. The default key_trans
    /// function is a pass-through, leaving the keys unmodified.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<2>::default().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    fn default() -> Self {
        CorpusBuilder {
            pad_left: Pad::Auto,
            pad_right: Pad::Auto,
            texts: Vec::new(),
            key_transformer: IdentityKeyTransformer::default(),
        }
    }
}

impl<const ARITY: usize, KT> CorpusBuilder<ARITY, KT>
where
    KT: KeyTransformer,
{
    /// Set the left padding to build into the `Corpus`.
    pub fn pad_left(mut self, pad_left: Pad) -> Self {
        self.pad_left = pad_left;
        self
    }

    /// Set the right padding to build into the `Corpus`.
    pub fn pad_right(mut self, pad_right: Pad) -> Self {
        self.pad_right = pad_right;
        self
    }

    /// Set both the left and right padding to build into the `Corpus`.
    pub fn pad_full(mut self, pad: Pad) -> Self {
        self.pad_left = pad.clone();
        self.pad_right = pad;
        self
    }

    /// Provide an iterator that will yield strings to be added to the
    /// `Corpus`.
    pub fn fill<It>(mut self, iterable: It) -> Self
    where
        It: IntoIterator,
        It::Item: Into<String>,
    {
        self.texts.extend(iterable.into_iter().map(<_>::into));
        self
    }

    /// A key transformation function, supplied as a boxed Fn that takes a
    /// &str and returns a String, applied to all strings that will be added
    /// to the `Corpus`. Searches on the `Corpus` will be similarly
    /// transformed.
    /// ```rust
    /// use ngrammatic::CorpusBuilder;
    /// use ngrammatic::key_transformer::LowerKeyTransformer;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<2>::default().link_key_transformer(LowerKeyTransformer::default()).finish();
    /// corpus.add_text("tomato");
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
        key_trans: KT2,
    ) -> CorpusBuilder<ARITY, LinkedKeyTransformer<KT, KT2>>
    where
        KT2: KeyTransformer,
    {
        CorpusBuilder {
            pad_left: self.pad_left,
            pad_right: self.pad_right,
            texts: self.texts,
            key_transformer: self.key_transformer.link(key_trans),
        }
    }

    /// Convenience function that calls `key_trans` with a closure that
    /// lowercases all keys added to the `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<2>::default().case_insensitive().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search("ToMaTo", 0.90, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'ToMaTo' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'ToMaTo'.");
    /// }
    /// # }
    /// ```
    pub fn case_insensitive(
        self,
    ) -> CorpusBuilder<ARITY, LinkedKeyTransformer<KT, LowerKeyTransformer>> {
        self.link_key_transformer(LowerKeyTransformer::default())
    }

    /// Yield a `Corpus` instance with all the properties set with this builder.
    pub fn finish(self) -> Corpus<KT, ARITY> {
        let mut corpus = Corpus {
            ngrams: HashMap::new(),
            gram_to_words: HashMap::new(),
            pad_left: self.pad_left,
            pad_right: self.pad_right,
            key_transformer: self.key_transformer,
        };
        for text in self.texts {
            corpus.add_text(&text);
        }
        corpus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn float_approx_eq(a: f32, b: f32, epsilon: Option<f32>) -> bool {
        let abs_a = a.abs();
        let abs_b = b.abs();
        let diff = (a - b).abs();
        let epsilon = epsilon.unwrap_or(f32::EPSILON);

        if a == b {
            // infinity/NaN/exactly equal
            true
        } else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
            // one or both is very close to zero, or they're very close to each other
            diff < (epsilon * f32::MIN_POSITIVE)
        } else {
            // relative error
            (diff / f32::min(abs_a + abs_b, f32::MAX)) < epsilon
        }
    }

    #[test]
    fn arity_clamp_empty_string_nopad() {
        let ngram = NgramBuilder::<1>::new("").pad_full(Pad::None).finish();
        assert!(ngram.is_empty());
    }

    #[test]
    fn arity_clamp_empty_string_padded() {
        let ngram = NgramBuilder::<2>::new("")
            .pad_left(Pad::Pad("--".to_string()))
            .pad_right(Pad::Pad("++".to_string()))
            .finish();
        assert!(ngram.contains("--"));
        assert!(ngram.contains("-+"));
        assert!(ngram.contains("++"));
    }

    #[test]
    fn empty_string_nopad() {
        let ngram = NgramBuilder::<2>::new("").pad_full(Pad::None).finish();
        assert!(ngram.is_empty());
    }

    #[test]
    fn empty_string_autopad() {
        let ngram = NgramBuilder::<2>::new("").finish();
        assert!(ngram.contains("  "));
    }

    #[test]
    fn empty_string_strpad() {
        let ngram = NgramBuilder::<2>::new("")
            .pad_left(Pad::Pad("--".to_string()))
            .pad_right(Pad::Pad("++".to_string()))
            .finish();
        assert!(ngram.contains("--"));
        assert!(ngram.contains("-+"));
        assert!(ngram.contains("++"));
    }

    #[test]
    fn short_string_nopad() {
        let ngram = NgramBuilder::<2>::new("ab").pad_full(Pad::None).finish();
        assert!(ngram.contains("ab"));
    }

    #[test]
    fn short_string_autopad() {
        let ngram = NgramBuilder::<2>::new("ab").finish();
        assert!(ngram.contains(" a"));
        assert!(ngram.contains("ab"));
        assert!(ngram.contains("b "));
    }

    #[test]
    fn short_string_strpad() {
        let ngram = NgramBuilder::<2>::new("ab")
            .pad_left(Pad::Pad("--".to_string()))
            .pad_right(Pad::Pad("++".to_string()))
            .finish();
        assert!(ngram.contains("--"));
        assert!(ngram.contains("-a"));
        assert!(ngram.contains("ab"));
        assert!(ngram.contains("b+"));
        assert!(ngram.contains("++"));
    }

    #[test]
    fn ngram_similarity_raw() {
        assert!(float_approx_eq(
            Ngram::<2>::similarity(5, 10, 1.0),
            0.5,
            None
        ));
        assert!(float_approx_eq(
            Ngram::<2>::similarity(5, 10, 2.0),
            0.75,
            None
        ));
        assert!(float_approx_eq(
            Ngram::<2>::similarity(5, 10, 3.0),
            0.875,
            None
        ));
        assert!(float_approx_eq(
            Ngram::<2>::similarity(2, 4, 2.0),
            0.75,
            None
        ));
        assert!(float_approx_eq(
            Ngram::<2>::similarity(3, 4, 1.0),
            0.75,
            None
        ));
    }

    #[test]
    fn similarity_identical() {
        let ngram0 = NgramBuilder::<2>::new("ab").finish();
        let ngram1 = NgramBuilder::<2>::new("ab").finish();
        assert!(float_approx_eq(
            ngram0.similarity_to(&ngram1, 3.0),
            1.0,
            None,
        ));
    }

    #[test]
    fn similarity_completelydifferent() {
        let ngram0 = NgramBuilder::<2>::new("ab").finish();
        let ngram1 = NgramBuilder::<2>::new("cd").finish();
        assert!(float_approx_eq(
            ngram0.similarity_to(&ngram1, 3.0),
            0.0,
            None,
        ));
    }

    #[test]
    fn corpus_add_text_before_setting_arity() {
        let corpus = CorpusBuilder::<2>::default()
            .fill(vec!["ab", "ba"])
            .finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_set_padding_after_adding_text() {
        let corpus = CorpusBuilder::<2>::default()
            .fill(vec!["ab", "ba"])
            .pad_full(Pad::None)
            .finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_add_multiple() {
        let corpus = CorpusBuilder::<2>::default()
            .pad_full(Pad::Auto)
            .fill(vec!["ab", "ba"])
            .finish();
        assert_eq!(corpus.is_empty(), false);
        assert_eq!(corpus.key("ab"), Some("ab".to_string()));
        assert_eq!(corpus.key("ba"), Some("ba".to_string()));
        assert_eq!(corpus.key("zabba"), None);
    }

    #[test]
    fn corpus_search() {
        let corpus = CorpusBuilder::<1>::default()
            .pad_full(Pad::None)
            .fill(vec!["ab", "ba", "cd"])
            .finish();
        assert_eq!(corpus.search("ce", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("ec", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("b", 0.5, 10).len(), 2);
    }

    #[test]
    fn corpus_case_insensitive_corpus_search() {
        let corpus = CorpusBuilder::<1>::default()
            .pad_full(Pad::None)
            .fill(vec!["Ab", "Ba", "Cd"])
            .case_insensitive()
            .finish();
        assert_eq!(corpus.search("ce", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("ec", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("b", 0.5, 10).len(), 2);
    }

    #[test]
    fn corpus_case_insensitive_corpus_search_terms() {
        let corpus = CorpusBuilder::<1>::default()
            .pad_full(Pad::None)
            .fill(vec!["Ab", "Ba", "Cd"])
            .case_insensitive()
            .finish();
        assert_eq!(corpus.search("cE", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("eC", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("b", 0.5, 10).len(), 2);
    }

    #[test]
    fn corpus_search_emoji() {
        let corpus = CorpusBuilder::<1>::default()
            .pad_full(Pad::None)
            .fill(vec!["\u{1f60f}\u{1f346}", "ba", "cd"])
            .finish();

        assert_eq!(corpus.search("ac", 0.3, 10).len(), 2);
        assert_eq!(corpus.search("\u{1f346}d", 0.3, 10).len(), 2);
    }

    #[test]
    fn corpus_search_small_word() {
        let corpus = CorpusBuilder::<5>::default()
            .pad_full(Pad::Pad(" ".to_string()))
            .fill(vec!["ab"])
            .case_insensitive()
            .finish();
        assert!(corpus.search("a", 0., 10).is_empty());
    }

    #[test]
    fn corpus_search_empty_string() {
        let corpus = CorpusBuilder::<3>::default()
            .pad_full(Pad::Pad(" ".to_string()))
            .fill(vec!["a"])
            .case_insensitive()
            .finish();
        assert!(corpus.search("", 0., 10).is_empty());
    }

    #[test]
    fn accept_iterator_of_strings() {
        let provider = Vec::<String>::new().into_iter();
        // The test is only meant to verify that `fill` accepts an iterator that
        // yields `String`s.
        let _ = CorpusBuilder::<2>::default().fill(provider);
    }

    #[test]
    fn accept_iterator_of_string_slices() {
        let provider = Vec::<String>::new();
        // The test is only meant to verify that `fill` accepts an iterator that
        // yields `&str`s or `&String`s.
        let _ = CorpusBuilder::<2>::default()
            .fill(&provider)
            .fill(provider.iter().map(String::as_str));
    }
}
