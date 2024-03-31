#![deny(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::f32;

use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ngram::{Ngram, NgramBuilder};
use crate::{
    IdentityKeyTransformer, KeyTransformer, LinkedKeyTransformer, LowerKeyTransformer, Pad,
    SearchResult,
};

// Import traits for rayon parallelization
#[cfg(feature = "rayon")]
use rayon::{
    iter::IntoParallelIterator, iter::IntoParallelRefIterator, iter::ParallelIterator,
    slice::ParallelSliceMut,
};

/// Holds a corpus of words and their ngrams, allowing fuzzy matches of
/// candidate strings against known strings in the corpus.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Corpus<KT, const ARITY: usize = 2>
where
    KT: KeyTransformer,
{
    pad_left: Pad,
    pad_right: Pad,
    strings: StringInterner<DefaultBackend>,
    ngrams: HashMap<DefaultSymbol, Ngram<ARITY>>,
    gram_to_words: HashMap<String, Vec<DefaultSymbol>>,
    key_transformer: KT,
}

impl<KT, const ARITY: usize> std::fmt::Debug for Corpus<KT, ARITY>
where
    KT: KeyTransformer,
{
    /// Debug format for a `Corpus`. Omits any representation of the
    /// `key_trans` field, as there's no meaningful representation we could
    /// give.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Corpus {{")?;
        writeln!(f, "  arity: {ARITY:?},")?;
        writeln!(f, "  pad_left: {:?},", self.pad_left)?;
        writeln!(f, "  pad_right: {:?},", self.pad_right)?;
        writeln!(f, "  ngrams: {:?},", self.ngrams)?;
        writeln!(f, "}}")
    }
}

impl<KT, const ARITY: usize> Corpus<KT, ARITY>
where
    KT: KeyTransformer + std::marker::Sync,
{
    /// Add the supplied `ngram` to the `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().finish();
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
    #[allow(clippy::unwrap_or_default)]
    pub fn add_ngram(&mut self, ngram: Ngram<ARITY>) {
        let word_sym = self.strings.get_or_intern(ngram.text.as_str());
        for gram in ngram.grams.keys() {
            self.gram_to_words
                .entry(gram.clone())
                .or_insert_with(Vec::new)
                .push(word_sym);
        }
        self.ngrams.insert(word_sym, ngram);
    }

    /// Generate an `Ngram` for the supplied `text`, and add it to the
    /// `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().finish();
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
        let transformed = self.key_transformer.transform(text);
        self.strings
            .get(transformed.as_str())
            .and_then(|sym| self.ngrams.get(&sym))
            .map(|_| text.to_string())
    }

    /// Perform a fuzzy search of the `Corpus` for `Ngrams` above some
    /// `threshold` of similarity to the supplied `text`.  Returns up to `limit`
    /// results, sorted by highest similarity to lowest.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().finish();
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

    /// Perform a parallelized fuzzy search of the `Corpus` for `Ngrams` above
    /// some `threshold` of similarity to the supplied `text`.  Returns up to
    /// `limit` results, sorted by highest similarity to lowest.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search_par("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    #[cfg(feature = "rayon")]
    pub fn search_par(&self, text: &str, threshold: f32, limit: usize) -> Vec<SearchResult> {
        self.search_with_warp_par(text, 2.0, threshold, limit)
    }

    /// Perform a fuzzy search of the `Corpus` for `Ngrams` with a custom `warp` for
    /// results above some `threshold` of similarity to the supplied `text`.  Returns
    /// up to `limit` results, sorted by highest similarity to lowest.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().finish();
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
        let ngrams_to_consider: HashSet<&Ngram<ARITY>> = item
            .grams
            .keys()
            .filter_map(|gram| self.gram_to_words.get(gram))
            // Fetch ngrams from raw words
            .flat_map(|word_syms| word_syms.iter().filter_map(|ws| self.ngrams.get(ws)))
            .collect();
        let mut results: Vec<SearchResult> = ngrams_to_consider
            .iter()
            .filter_map(|n| item.matches_with_warp(n, warp, threshold))
            .collect();

        // Sort highest similarity to lowest
        results.sort_by(|a, b| b.partial_cmp(a).unwrap());
        results.truncate(limit);
        results
    }

    /// Perform a parallelized fuzzy search of the `Corpus` for `Ngrams` with a custom
    /// `warp` for results above some `threshold` of similarity to the supplied `text`.
    /// Returns up to `limit` results, sorted by highest similarity to lowest.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search_with_warp_par("tomacco", 2.0, 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    #[cfg(feature = "rayon")]
    pub fn search_with_warp_par(
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
        let ngrams_to_consider: HashSet<&Ngram<ARITY>> = item
            .grams
            .keys()
            .collect::<Vec<_>>()
            .par_iter()
            .filter_map(|gram| self.gram_to_words.get(*gram))
            .flat_map_iter(|word_syms| word_syms.iter().filter_map(|ws| self.ngrams.get(ws)))
            .collect();
        let mut results: Vec<SearchResult> = ngrams_to_consider
            .into_par_iter()
            .filter_map(|n| item.matches_with_warp(n, warp, threshold))
            .collect();

        // Sort highest similarity to lowest
        results.par_sort_by(|a, b| b.partial_cmp(a).unwrap());
        results.truncate(limit);
        results
    }
}

/// Build an Ngram Corpus, one setting at a time.
// We provide a builder for Corpus to ensure initialization operations are
// performed in the correct order, without requiring an extensive parameter list
// to a constructor method, and allowing default values by omission.
pub struct CorpusBuilder<const ARITY: usize = 2, KT = IdentityKeyTransformer>
where
    KT: KeyTransformer,
{
    pad_left: Pad,
    pad_right: Pad,
    strings: StringInterner<DefaultBackend>,
    texts: Vec<DefaultSymbol>,
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
        writeln!(f, "CorpusBuilder {{")?;
        writeln!(f, "  arity: {ARITY:?},")?;
        writeln!(f, "  pad_left: {:?},", self.pad_left)?;
        writeln!(f, "  pad_right: {:?},", self.pad_right)?;
        writeln!(f, "  texts: {:?},", self.texts)?;
        // TODO: inspect key transforms for type names, and print?
        writeln!(f, "}}")
    }
}

impl Default for CorpusBuilder {
    /// Initialize a new instance of an `CorpusBuilder`, with a default `arity`
    /// of 2, padding set to `Auto`, for the given `texts`. The default key_trans
    /// function is a pass-through, leaving the keys unmodified.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().finish();
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
            strings: StringInterner::default(),
            key_transformer: IdentityKeyTransformer,
        }
    }
}

impl<const ARITY: usize> CorpusBuilder<ARITY> {
    /// Initialize a new instance of an `CorpusBuilder`, with a configurable `arity`
    /// padding set to `Auto`, for the given `texts`. The default key_trans
    /// function is a pass-through, leaving the keys unmodified.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::<2>::new().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search("tomacco", 0.40, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'tomacco' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'tomacco'.");
    /// }
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            pad_left: Pad::Auto,
            pad_right: Pad::Auto,
            texts: Vec::new(),
            strings: StringInterner::default(),
            key_transformer: IdentityKeyTransformer,
        }
    }
}

impl<const ARITY: usize, KT> CorpusBuilder<ARITY, KT>
where
    KT: KeyTransformer + std::marker::Sync,
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
        It::Item: AsRef<str>,
    {
        self.texts.extend(
            iterable
                .into_iter()
                .map(|s| self.strings.get_or_intern(s.as_ref())),
        );
        self
    }

    /// Provide an iterator that will be parallelized that yields strings to
    /// be added to the `Corpus`.
    #[cfg(feature = "rayon")]
    pub fn fill_par<FillIt>(mut self, iterable: FillIt) -> Self
    where
        FillIt: rayon::iter::IntoParallelIterator,
        String: From<<FillIt as IntoParallelIterator>::Item>,
    {
        let tmp: Vec<String> = iterable.into_par_iter().map(<_>::into).collect();
        self.texts
            .extend(tmp.into_iter().map(|s| self.strings.get_or_intern(s)));
        self
    }

    /// A key transformation function, supplied as a boxed Fn that takes a
    /// &str and returns a String, applied to all strings that will be added
    /// to the `Corpus`. Searches on the `Corpus` will be similarly
    /// transformed.
    /// ```rust
    /// use ngrammatic::CorpusBuilder;
    /// use ngrammatic::LowerKeyTransformer;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().link_key_transformer(LowerKeyTransformer::default()).finish();
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
            strings: self.strings,
            key_transformer: self.key_transformer.link(key_trans),
        }
    }

    /// Convenience function that calls `link_key_transformer` with a
    /// transformer that lowercases all keys added to the `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::default().case_insensitive().finish();
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
        self.link_key_transformer(LowerKeyTransformer)
    }

    /// Yield a `Corpus` instance with all the properties set with this builder.
    pub fn finish(self) -> Corpus<KT, ARITY> {
        let mut corpus = Corpus {
            ngrams: HashMap::new(),
            gram_to_words: HashMap::new(),
            strings: self.strings,
            pad_left: self.pad_left,
            pad_right: self.pad_right,
            key_transformer: self.key_transformer,
        };
        for sym in self.texts {
            if let Some(owned) = corpus.strings.resolve(sym).map(str::to_owned) {
                corpus.add_text(&owned);
            }
        }
        corpus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_add_text() {
        let corpus = CorpusBuilder::default().fill(vec!["ab", "ba"]).finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_set_padding_after_adding_text() {
        let corpus = CorpusBuilder::default()
            .fill(vec!["ab", "ba"])
            .pad_full(Pad::None)
            .finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_add_multiple() {
        let corpus = CorpusBuilder::default()
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
        let corpus = CorpusBuilder::<1>::new()
            .pad_full(Pad::None)
            .fill(vec!["ab", "ba", "cd"])
            .finish();
        assert_eq!(corpus.search("ce", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("ec", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("b", 0.5, 10).len(), 2);
    }

    #[test]
    fn corpus_case_insensitive_corpus_search() {
        let corpus = CorpusBuilder::<1>::new()
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
        let corpus = CorpusBuilder::<1>::new()
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
        let corpus = CorpusBuilder::<1>::new()
            .pad_full(Pad::None)
            .fill(vec!["\u{1f60f}\u{1f346}", "ba", "cd"])
            .finish();

        assert_eq!(corpus.search("ac", 0.3, 10).len(), 2);
        assert_eq!(corpus.search("\u{1f346}d", 0.3, 10).len(), 2);
    }

    #[test]
    fn corpus_search_small_word() {
        let corpus = CorpusBuilder::<5>::new()
            .pad_full(Pad::Pad(" ".to_string()))
            .fill(vec!["ab"])
            .case_insensitive()
            .finish();
        assert!(corpus.search("a", 0., 10).is_empty());
    }

    #[test]
    fn corpus_search_empty_string() {
        let corpus = CorpusBuilder::<3>::new()
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
        let _ = CorpusBuilder::default().fill(provider);
    }

    #[test]
    fn accept_iterator_of_string_slices() {
        let provider = Vec::<String>::new();
        // The test is only meant to verify that `fill` accepts an iterator that
        // yields `&str`s or `&String`s.
        let _ = CorpusBuilder::default()
            .fill(&provider)
            .fill(provider.iter().map(String::as_str));
    }
}
