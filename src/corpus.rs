#![deny(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::f32;

use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

use crate::ngram::{Ngram, NgramBuilder};
use crate::{Pad, SearchResult};

// Import traits for rayon parallelization
#[cfg(feature = "rayon")]
use rayon::{
    iter::IntoParallelIterator, iter::IntoParallelRefIterator, iter::ParallelIterator,
    slice::ParallelSliceMut,
};

/// Holds a corpus of words and their ngrams, allowing fuzzy matches of
/// candidate strings against known strings in the corpus.
pub struct Corpus {
    arity: usize,
    pad_left: Pad,
    pad_right: Pad,
    strings: StringInterner<DefaultBackend>,
    ngrams: HashMap<DefaultSymbol, Ngram>,
    gram_to_words: HashMap<DefaultSymbol, Vec<DefaultSymbol>>,
    key_trans: Box<dyn Fn(&str) -> String + Send + Sync>,
}

impl std::fmt::Debug for Corpus {
    /// Debug format for a `Corpus`. Omits any representation of the
    /// `key_trans` field, as there's no meaningful representation we could
    /// give.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Corpus {{")?;
        writeln!(f, "  arity: {:?},", self.arity)?;
        writeln!(f, "  pad_left: {:?},", self.pad_left)?;
        writeln!(f, "  pad_right: {:?},", self.pad_right)?;
        writeln!(f, "  ngrams: {:?},", self.ngrams)?;
        writeln!(f, "}}")
    }
}

impl Corpus {
    /// Add the supplied `ngram` to the `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::new().finish();
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
    pub fn add_ngram(&mut self, ngram: Ngram) {
        let word_sym = self.strings.get_or_intern(ngram.text.as_str());
        self.ngrams.insert(word_sym, ngram.clone());
        for gram_str in ngram.grams.keys() {
            let gram_sym = self.strings.get_or_intern(gram_str.as_str());
            self.gram_to_words
                .entry(gram_sym)
                .or_insert_with(Vec::new)
                .push(word_sym);
        }
    }

    /// Generate an `Ngram` for the supplied `text`, and add it to the
    /// `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::new().finish();
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
        let arity = self.arity;
        let pad_left = self.pad_left.clone();
        let pad_right = self.pad_right.clone();
        let new_key = &(self.key_trans)(text);
        self.add_ngram(
            NgramBuilder::new(new_key)
                .arity(arity)
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
        let transformed = (self.key_trans)(text);
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
    /// let mut corpus = CorpusBuilder::new().finish();
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
    /// let mut corpus = CorpusBuilder::new().finish();
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
    /// let mut corpus = CorpusBuilder::new().finish();
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
        let item = NgramBuilder::new(&(self.key_trans)(text))
            .arity(self.arity)
            .pad_left(self.pad_left.clone())
            .pad_right(self.pad_right.clone())
            .finish();
        let ngrams_to_consider: HashSet<&Ngram> = item
            .grams
            .keys()
            .filter_map(|gram_str| self.strings.get(gram_str.as_str()))
            .filter_map(|gram_sym| self.gram_to_words.get(&gram_sym))
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
    /// let mut corpus = CorpusBuilder::new().finish();
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
        let item = NgramBuilder::new(&(self.key_trans)(text))
            .arity(self.arity)
            .pad_left(self.pad_left.clone())
            .pad_right(self.pad_right.clone())
            .finish();
        let ngrams_to_consider: HashSet<&Ngram> = item
            .grams
            .keys()
            .collect::<Vec<_>>()
            .par_iter()
            .filter_map(|gram_str| self.strings.get(gram_str.as_str()))
            .filter_map(|gram_sym| self.gram_to_words.get(&gram_sym))
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
pub struct CorpusBuilder {
    arity: usize,
    pad_left: Pad,
    pad_right: Pad,
    strings: StringInterner<DefaultBackend>,
    texts: Vec<DefaultSymbol>,
    key_trans: Box<dyn Fn(&str) -> String + Send + Sync>,
}

impl std::fmt::Debug for CorpusBuilder {
    /// Debug format for a `CorpusBuilder`. Omits any representation of the
    /// `key_trans` field, as there's no meaningful representation we could
    /// give.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "CorpusBuilder {{")?;
        writeln!(f, "  arity: {:?},", self.arity)?;
        writeln!(f, "  pad_left: {:?},", self.pad_left)?;
        writeln!(f, "  pad_right: {:?},", self.pad_right)?;
        writeln!(f, "  texts: {:?},", self.texts)?;
        writeln!(f, "}}")
    }
}

impl Default for CorpusBuilder {
    /// Fowards to `CorpusBuilder`'s `new` method.
    fn default() -> Self {
        Self::new()
    }
}

impl CorpusBuilder {
    /// Initialize a new instance of an `CorpusBuilder`, with a default `arity`
    /// of 2, padding set to `Auto`, for the given `texts`. The default key_trans
    /// function is a pass-through, leaving the keys unmodified.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::new().finish();
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
        CorpusBuilder {
            arity: 2,
            pad_left: Pad::Auto,
            pad_right: Pad::Auto,
            strings: StringInterner::default(),
            texts: Vec::new(),
            key_trans: Box::new(|x| x.into()),
        }
    }

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

    /// Set `arity` (the _n_ in _ngram_) to use for the resulting `Corpus`.
    pub fn arity(mut self, arity: usize) -> Self {
        self.arity = arity.max(1);
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
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::new().key_trans(Box::new(|x| x.to_lowercase())).finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search("ToMaTo", 0.90, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'ToMaTo' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'ToMaTo'.");
    /// }
    /// # }
    /// ```
    pub fn key_trans(mut self, key_trans: Box<dyn Fn(&str) -> String + Send + Sync>) -> Self {
        self.key_trans = key_trans;
        self
    }

    /// Convenience function that calls `key_trans` with a closure that
    /// lowercases all keys added to the `Corpus`.
    /// ```rust
    /// # use ngrammatic::CorpusBuilder;
    /// # fn main() {
    /// let mut corpus = CorpusBuilder::new().case_insensitive().finish();
    /// corpus.add_text("tomato");
    /// let results = corpus.search("ToMaTo", 0.90, 10);
    /// if let Some(result) = results.first() {
    ///     println!("Closest match to 'ToMaTo' in the corpus was {}", result.text);
    /// } else {
    ///     println!("The corpus contained no words similar to 'ToMaTo'.");
    /// }
    /// # }
    /// ```
    pub fn case_insensitive(self) -> Self {
        self.key_trans(Box::new(|x| x.to_lowercase()))
    }

    /// Yield a `Corpus` instance with all the properties set with this builder.
    pub fn finish(self) -> Corpus {
        let mut corpus = Corpus {
            arity: self.arity,
            ngrams: HashMap::new(),
            gram_to_words: HashMap::new(),
            strings: self.strings,
            pad_left: self.pad_left,
            pad_right: self.pad_right,
            key_trans: self.key_trans,
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
    fn corpus_add_text_before_setting_arity() {
        let corpus = CorpusBuilder::new().fill(vec!["ab", "ba"]).finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_set_arity_after_adding_text() {
        let corpus = CorpusBuilder::new()
            .arity(2)
            .fill(vec!["ab", "ba"])
            .arity(3)
            .finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_set_padding_after_adding_text() {
        let corpus = CorpusBuilder::new()
            .arity(2)
            .fill(vec!["ab", "ba"])
            .pad_full(Pad::None)
            .finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_add_multiple() {
        let corpus = CorpusBuilder::new()
            .arity(2)
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
        let corpus = CorpusBuilder::new()
            .arity(1)
            .pad_full(Pad::None)
            .fill(vec!["ab", "ba", "cd"])
            .finish();
        assert_eq!(corpus.search("ce", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("ec", 0.3, 10).len(), 1);
        assert_eq!(corpus.search("b", 0.5, 10).len(), 2);
    }

    #[test]
    fn corpus_case_insensitive_corpus_search() {
        let corpus = CorpusBuilder::new()
            .arity(1)
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
        let corpus = CorpusBuilder::new()
            .arity(1)
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
        let corpus = CorpusBuilder::new()
            .arity(1)
            .pad_full(Pad::None)
            .fill(vec!["\u{1f60f}\u{1f346}", "ba", "cd"])
            .finish();

        assert_eq!(corpus.search("ac", 0.3, 10).len(), 2);
        assert_eq!(corpus.search("\u{1f346}d", 0.3, 10).len(), 2);
    }

    #[test]
    fn corpus_search_small_word() {
        let corpus = CorpusBuilder::new()
            .arity(5)
            .pad_full(Pad::Pad(" ".to_string()))
            .fill(vec!["ab"])
            .case_insensitive()
            .finish();
        assert!(corpus.search("a", 0., 10).is_empty());
    }

    #[test]
    fn corpus_search_empty_string() {
        let corpus = CorpusBuilder::new()
            .arity(3)
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
        let _ = CorpusBuilder::new().fill(provider);
    }

    #[test]
    fn accept_iterator_of_string_slices() {
        let provider = Vec::<String>::new();
        // The test is only meant to verify that `fill` accepts an iterator that
        // yields `&str`s or `&String`s.
        let _ = CorpusBuilder::new()
            .fill(&provider)
            .fill(provider.iter().map(String::as_str));
    }
}
