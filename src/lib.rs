/*!
This crate provides fuzzy search/string matching using N-grams.

This implementation is character-based, rather than word based, matching
solely based on string similarity. It is modelled somewhat after the
[python ngram module][1] with some inspiration from [chappers' blog post on
fuzzy matching with ngrams][2].

The crate is implemented in three parts: the `Corpus`, which is an
index connecting strings (words, symbols, whatever) to their `Ngrams`,
and `SearchResult`s, which contains a fuzzy match result, with the
word and a similarity measure in the range of 0.0 to 1.0.

The general usage pattern is to construct a `Corpus`, `.add()` your
list of valid symbols to it, and then perform `.search()`es of valid,
unknown, misspelled, etc symbols on the `Corpus`. The results come
back as a vector of up to 10 results, sorted from highest similarity
to lowest.

# Examples

```rust
use ngrammatic::{CorpusBuilder, Pad};

let mut corpus = CorpusBuilder::new()
    .arity(2)
    .pad_full(Pad::Auto)
    .finish();

// Build up the list of known words
corpus.add_text("pie");
corpus.add_text("animal");
corpus.add_text("tomato");
corpus.add_text("seven");
corpus.add_text("carbon");

// Now we can try an unknown/misspelled word, and find a similar match
// in the corpus
let results = corpus.search("tomacco", 0.25);
let top_match = results.first();

assert!(top_match.is_some());
assert!(top_match.unwrap().similarity > 0.5);
assert_eq!(top_match.unwrap().text,String::from("tomato"));
```

[1]: https://pythonhosted.org/ngram/ngram.html
[2]: http://chappers.github.io/web%20micro%20log/2015/04/29/comparison-of-ngram-fuzzy-matching-approaches/
*/

//#![deny(missing_docs)]

use std::collections::HashMap;
use std::cmp::Ordering;
use std::f32;

/// Holds a fuzzy match search result string, and its associated similarity
/// to the query text.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub text: String,
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
    pub fn new(text: String, similarity: f32) -> Self {
        SearchResult {
            text: text,
            similarity: similarity,
        }
    }
}

/// Determines how strings are padded before calculating the ngrams.
/// Having some sort of padding is especially important for small words
/// to the query text.
// Auto pad pre/appends arity-1 space chars
// http://journals.plos.org/plosone/article?id=10.1371/journal.pone.0107510
#[derive(Debug)]
#[derive(Clone)]
pub enum Pad {
    None,
    Auto,
    Pad(String),
}

impl Default for Pad {
    fn default() -> Self {
        Pad::Auto
    }
}

impl Pad {
    pub fn to_string(&self, autopad_width: usize) -> String {
        match *self {
            Pad::Auto => std::iter::repeat(" ").take(autopad_width).collect::<String>(),
            Pad::Pad(ref p) => p.to_owned(),
            Pad::None => "".to_owned(),
        }
    }

    pub fn pad_text(text: &str, pad_left: Pad, pad_right: Pad, autopad_width: usize) -> String {
        pad_left.to_string(autopad_width) + text + pad_right.to_string(autopad_width).as_ref()
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct Ngram {
    arity: usize,
    text: String,
    text_padded: String,
    grams: HashMap<String, usize>,
}

// TODO: When rust adds const generics
// (see https://github.com/rust-lang/rust/issues/44580)
// switch Ngram's "arity" member to be a const generic
// on Ngram, and implement From(String) so that we can
// do things like Ngram::<3>::From(text) to construct
// new ngrams

impl Ngram {
    pub fn similarity(samegram_count: usize, allgram_count: usize, warp: f32) -> f32 {
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

    pub fn similarity_to(&self, other: &Ngram, warp: f32) -> f32 {
        let warp = warp.max(1.0).min(3.0);
        let samegram_count = self.count_samegrams(other);
        let allgram_count = self.count_allgrams(other);
        Ngram::similarity(samegram_count, allgram_count, warp)
    }

    pub fn matches(&self, other: &Ngram, threshold: f32) -> Option<SearchResult> {
        let similarity = self.similarity_to(other, 2.0);
        if similarity >= threshold {
            Some(SearchResult::new(other.text.clone(), similarity))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn count_diffgrams(&self, other: &Ngram) -> usize {
        self.count_allgrams(other) - self.count_samegrams(other)
    }

    pub fn count_allgrams(&self, other: &Ngram) -> usize {
        // This is a shortcut that counts all grams between both ngrams
        // Then subtracts out one instance of the grams that are in common
        self.text_padded.chars().count() + other.text_padded.chars().count() -
        (2 * self.arity) + 2 - self.count_samegrams(other)
    }

    pub fn count_samegrams(&self, other: &Ngram) -> usize {
        let mut sames: usize = 0;
        for key in self.grams.keys() {
            let selfcount = self.count_gram(key.as_ref());
            let othercount = other.count_gram(key.as_ref());
            sames += selfcount.min(othercount);
        }
        sames
    }

    pub fn count_gram(&self, gram: &str) -> usize {
        match self.grams.get(gram) {
            Some(count) => *count,
            None => 0,
        }
    }

    pub fn count_grams(&self) -> usize {
        self.grams.values().sum()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.count_grams() == 0
    }

    #[allow(dead_code)]
    pub fn contains(&self, gram: &str) -> bool {
        self.count_gram(gram) > 0
    }

    fn init(&mut self) {
        if self.arity > self.text_padded.len() {
            return;
        }
        let chars_padded: Vec<char> = self.text_padded.chars().collect();
        for window in chars_padded.windows(self.arity) {
            let count = self.grams.entry(window.iter().collect()).or_insert(0);
            *count += 1;
        }
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct NgramBuilder {
    arity: usize,
    pad_left: Pad,
    pad_right: Pad,
    text: String,
}

impl NgramBuilder {
    pub fn new(text: &str) -> Self {
        NgramBuilder {
            arity: 2,
            pad_left: Pad::Auto,
            pad_right: Pad::Auto,
            text: text.to_owned(),
        }
    }

    pub fn pad_left(mut self, pad_left: Pad) -> Self {
        self.pad_left = pad_left;
        self
    }

    pub fn pad_right(mut self, pad_right: Pad) -> Self {
        self.pad_right = pad_right;
        self
    }

    pub fn pad_full(mut self, pad: Pad) -> Self {
        self.pad_left = pad.clone();
        self.pad_right = pad;
        self
    }

    pub fn arity(mut self, arity: usize) -> Self {
        self.arity = arity.max(1);
        self
    }

    pub fn finish(self) -> Ngram {
        let mut ngram = Ngram {
            arity: self.arity,
            text: self.text.clone(),
            text_padded: Pad::pad_text(&self.text, self.pad_left, self.pad_right, self.arity - 1),
            grams: HashMap::new(),
        };
        ngram.init();
        ngram
    }
}

pub struct Corpus {
    arity: usize,
    pad_left: Pad,
    pad_right: Pad,
    ngrams: HashMap<String, Ngram>,
    key_trans: Box<Fn(&str) -> String>,
}

impl std::fmt::Debug for Corpus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Corpus {{\n")?;
        write!(f, "  arity: {:?},\n", self.arity)?;
        write!(f, "  pad_left: {:?},\n", self.pad_left)?;
        write!(f, "  pad_right: {:?},\n", self.pad_right)?;
        write!(f, "  ngrams: {:?},\n", self.ngrams)?;
        write!(f, "}}\n")
    }
}

impl Corpus {
    #[allow(dead_code)]
    pub fn add_ngram(&mut self, ngram: Ngram) {
        self.ngrams.insert(ngram.text.to_owned(), ngram);
    }

    #[allow(dead_code)]
    pub fn add_text(&mut self, text: &str) {
        let arity = self.arity;
        let pad_left = self.pad_left.clone();
        let pad_right = self.pad_right.clone();
        let new_key = &(self.key_trans)(text);
        self.add_ngram(NgramBuilder::new(new_key)
            .arity(arity)
            .pad_left(pad_left)
            .pad_right(pad_right)
            .finish());
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.ngrams.is_empty()
    }

    #[allow(dead_code)]
    pub fn get_key(&self, text: &str) -> Option<String> {
        if self.ngrams.contains_key(&(self.key_trans)(text)) {
            Some(text.to_owned())
        } else {
            self.ngrams
                .values()
                .find(|x| (self.key_trans)(&x.text) == (self.key_trans)(text))
                .map(|x| x.text.clone())
        }
    }

    #[allow(dead_code)]
    pub fn search(&self, text: &str, threshold: f32) -> Vec<SearchResult> {
        let item = NgramBuilder::new(&(self.key_trans)(text))
            .arity(self.arity)
            .pad_left(self.pad_left.clone())
            .pad_right(self.pad_right.clone())
            .finish();
        let mut results: Vec<SearchResult> =
            self.ngrams.values().filter_map(|n| item.matches(n, threshold)).collect();

        // Sort highest similarity to lowest
        results.sort_by(|a, b| b.partial_cmp(a).unwrap());
        results.truncate(10);
        results
    }
}

pub struct CorpusBuilder {
    arity: usize,
    pad_left: Pad,
    pad_right: Pad,
    texts: Vec<String>,
    key_trans: Box<Fn(&str) -> String>,
}

impl std::fmt::Debug for CorpusBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CorpusBuilder {{\n")?;
        write!(f, "  arity: {:?},\n", self.arity)?;
        write!(f, "  pad_left: {:?},\n", self.pad_left)?;
        write!(f, "  pad_right: {:?},\n", self.pad_right)?;
        write!(f, "  texts: {:?},\n", self.texts)?;
        write!(f, "}}\n")
    }
}

impl Default for CorpusBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CorpusBuilder {
    pub fn new() -> Self {
        CorpusBuilder {
            arity: 2,
            pad_left: Pad::Auto,
            pad_right: Pad::Auto,
            key_trans: Box::new(|x| x.into()),
            texts: Vec::new(),
        }
    }

    pub fn pad_left(mut self, pad_left: Pad) -> Self {
        self.pad_left = pad_left;
        self
    }

    pub fn pad_right(mut self, pad_right: Pad) -> Self {
        self.pad_right = pad_right;
        self
    }

    pub fn pad_full(mut self, pad: Pad) -> Self {
        self.pad_left = pad.clone();
        self.pad_right = pad;
        self
    }

    pub fn arity(mut self, arity: usize) -> Self {
        self.arity = arity.max(1);
        self
    }

    pub fn fill<'it, It>(mut self, iterable: It) -> Self
        where It: IntoIterator<Item = &'it str>
    {
        self.texts.extend(iterable.into_iter().map(|x| x.to_owned()));
        self
    }

    pub fn set_key_trans(mut self, key_trans: Box<Fn(&str) -> String>) -> Self {
        self.key_trans = key_trans;
        self
    }

    pub fn set_case_insensitive(self) -> Self {
        self.set_key_trans(Box::new(|x| x.to_lowercase()))
    }

    pub fn finish(self) -> Corpus {
        let mut corpus = Corpus {
            arity: self.arity,
            ngrams: HashMap::new(),
            pad_left: self.pad_left,
            pad_right: self.pad_right,
            key_trans: self.key_trans,
        };
        for text in self.texts {
            corpus.add_text(&text);
        }
        corpus
    }
}



#[cfg(test)]
mod tests {
    use  super::*;

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
        let ngram = NgramBuilder::new("").arity(1).pad_full(Pad::None).finish();
        assert!(ngram.is_empty());
    }

    #[test]
    fn arity_clamp_empty_string_padded() {
        let ngram = NgramBuilder::new("")
            .arity(2)
            .pad_left(Pad::Pad("--".to_owned()))
            .pad_right(Pad::Pad("++".to_owned()))
            .finish();
        assert!(ngram.contains("--"));
        assert!(ngram.contains("-+"));
        assert!(ngram.contains("++"));
    }

    #[test]
    fn empty_string_nopad() {
        let ngram = NgramBuilder::new("").arity(2).pad_full(Pad::None).finish();
        assert!(ngram.is_empty());
    }

    #[test]
    fn empty_string_autopad() {
        let ngram = NgramBuilder::new("").arity(2).finish();
        assert!(ngram.contains("  "));
    }

    #[test]
    fn empty_string_strpad() {
        let ngram = NgramBuilder::new("")
            .arity(2)
            .pad_left(Pad::Pad("--".to_owned()))
            .pad_right(Pad::Pad("++".to_owned()))
            .finish();
        assert!(ngram.contains("--"));
        assert!(ngram.contains("-+"));
        assert!(ngram.contains("++"));
    }

    #[test]
    fn short_string_nopad() {
        let ngram = NgramBuilder::new("ab").arity(2).pad_full(Pad::None).finish();
        assert!(ngram.contains("ab"));
    }

    #[test]
    fn short_string_autopad() {
        let ngram = NgramBuilder::new("ab").arity(2).finish();
        assert!(ngram.contains(" a"));
        assert!(ngram.contains("ab"));
        assert!(ngram.contains("b "));
    }

    #[test]
    fn short_string_strpad() {
        let ngram = NgramBuilder::new("ab")
            .arity(2)
            .pad_left(Pad::Pad("--".to_owned()))
            .pad_right(Pad::Pad("++".to_owned()))
            .finish();
        assert!(ngram.contains("--"));
        assert!(ngram.contains("-a"));
        assert!(ngram.contains("ab"));
        assert!(ngram.contains("b+"));
        assert!(ngram.contains("++"));
    }

    #[test]
    fn ngram_similarity_raw() {
        assert!(float_approx_eq(Ngram::similarity(5, 10, 1.0), 0.5, None));
        assert!(float_approx_eq(Ngram::similarity(5, 10, 2.0), 0.75, None));
        assert!(float_approx_eq(Ngram::similarity(5, 10, 3.0), 0.875, None));
        assert!(float_approx_eq(Ngram::similarity(2, 4, 2.0), 0.75, None));
        assert!(float_approx_eq(Ngram::similarity(3, 4, 1.0), 0.75, None));
    }

    #[test]
    fn similarity_identical() {
        let ngram0 = NgramBuilder::new("ab").arity(2).finish();
        let ngram1 = NgramBuilder::new("ab").arity(2).finish();
        assert!(float_approx_eq(ngram0.similarity_to(&ngram1, 3.0), 1.0, None));
    }

    #[test]
    fn similarity_completelydifferent() {
        let ngram0 = NgramBuilder::new("ab").arity(2).finish();
        let ngram1 = NgramBuilder::new("cd").arity(2).finish();
        assert!(float_approx_eq(ngram0.similarity_to(&ngram1, 3.0), 0.0, None));
    }

    #[test]
    fn corpus_add_text_before_setting_arity() {
        let corpus = CorpusBuilder::new().fill(vec!["ab", "ba"]).finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_set_arity_after_adding_text() {
        let corpus = CorpusBuilder::new().arity(2).fill(vec!["ab", "ba"]).arity(3).finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_set_padding_after_adding_text() {
        let corpus =
            CorpusBuilder::new().arity(2).fill(vec!["ab", "ba"]).pad_full(Pad::None).finish();
        println!("{:?}", corpus);
    }

    #[test]
    fn corpus_add_multiple() {
        let corpus =
            CorpusBuilder::new().arity(2).pad_full(Pad::Auto).fill(vec!["ab", "ba"]).finish();
        assert_eq!(corpus.is_empty(), false);
        assert_eq!(corpus.get_key("ab"), Some("ab".to_owned()));
        assert_eq!(corpus.get_key("ba"), Some("ba".to_owned()));
        assert_eq!(corpus.get_key("zabba"), None);
    }

    #[test]
    fn corpus_search() {
        let corpus =
            CorpusBuilder::new().arity(1).pad_full(Pad::None).fill(vec!["ab", "ba", "cd"]).finish();
        assert_eq!(corpus.search("ce", 0.3).len(), 1);
        assert_eq!(corpus.search("ec", 0.3).len(), 1);
        assert_eq!(corpus.search("b", 0.5).len(), 2);
    }

    #[test]
    fn corpus_case_insensitive_corpus_search() {
        let corpus = CorpusBuilder::new()
            .arity(1)
            .pad_full(Pad::None)
            .fill(vec!["Ab", "Ba", "Cd"])
            .set_case_insensitive()
            .finish();
        assert_eq!(corpus.search("ce", 0.3).len(), 1);
        assert_eq!(corpus.search("ec", 0.3).len(), 1);
        assert_eq!(corpus.search("b", 0.5).len(), 2);
    }

    #[test]
    fn corpus_case_insensitive_corpus_search_terms() {
        let corpus = CorpusBuilder::new()
            .arity(1)
            .pad_full(Pad::None)
            .fill(vec!["Ab", "Ba", "Cd"])
            .set_case_insensitive()
            .finish();
        assert_eq!(corpus.search("cE", 0.3).len(), 1);
        assert_eq!(corpus.search("eC", 0.3).len(), 1);
        assert_eq!(corpus.search("b", 0.5).len(), 2);
    }

    #[test]
    fn corpus_search_emoji() {
        let corpus = CorpusBuilder::new()
            .arity(1)
            .pad_full(Pad::None)
            .fill(vec!["\u{1f60f}\u{1f346}", "ba", "cd"])
            .finish();

        assert_eq!(corpus.search("ac", 0.3).len(), 2);
        assert_eq!(corpus.search("\u{1f346}d", 0.3).len(), 2);
    }
}
