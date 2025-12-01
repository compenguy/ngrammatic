#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::cmp::Ordering;

mod corpus;
mod ngram;

pub use crate::corpus::{Corpus, CorpusBuilder};
pub use crate::ngram::{Ngram, NgramBuilder};

/// Holds a fuzzy match search result string, and its associated similarity
/// to the query text.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
            Pad::None => String::new(),
        }
    }

    /// Static method to render a given `&str` with the indicated `Pad`ding.
    pub(crate) fn pad_text(
        text: &str,
        pad_left: Pad,
        pad_right: Pad,
        autopad_width: usize,
    ) -> String {
        let mut s = pad_left.to_string(autopad_width);
        s.push_str(text);
        s.push_str(&pad_right.to_string(autopad_width));
        s
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Trait for key transformers.
///
/// Key transformers are used to transform keys before they are used in a map,
/// and are designed to be chained together.
pub trait KeyTransformer {
    /// Transform a key.
    ///
    /// # Arguments
    /// * `key` - The key to transform.
    fn transform(&self, key: &str) -> String;

    /// Link this key transformer to another.
    ///
    /// # Arguments
    /// * `dst` - The key transformer to link to itself.
    fn link<Dst>(self, dst: Dst) -> LinkedKeyTransformer<Self, Dst>
    where
        Self: Sized,
    {
        LinkedKeyTransformer::new(self, dst)
    }

    /// Chain this key transformer with a lowercasing key transformer.
    fn lower(self) -> LinkedKeyTransformer<Self, LowerKeyTransformer>
    where
        Self: Sized,
    {
        self.link(LowerKeyTransformer)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Identity key transformer.
pub struct IdentityKeyTransformer;

impl KeyTransformer for IdentityKeyTransformer {
    fn transform(&self, key: &str) -> String {
        key.to_string()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Lowercasing key transformer.
pub struct LowerKeyTransformer;

impl KeyTransformer for LowerKeyTransformer {
    fn transform(&self, key: &str) -> String {
        key.to_lowercase()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Linked key transformer.
pub struct LinkedKeyTransformer<Src, Dst> {
    src: Src,
    dst: Dst,
}

impl<Src, Dst> LinkedKeyTransformer<Src, Dst> {
    /// Create a new linked key transformer.
    ///
    /// # Arguments
    /// * `src` - The source key transformer.
    /// * `dst` - The destination key transformer.
    pub fn new(src: Src, dst: Dst) -> Self {
        LinkedKeyTransformer { src, dst }
    }
}

impl<Src, Dst> KeyTransformer for LinkedKeyTransformer<Src, Dst>
where
    Src: KeyTransformer,
    Dst: KeyTransformer,
{
    fn transform(&self, key: &str) -> String {
        self.dst.transform(&self.src.transform(key))
    }
}
