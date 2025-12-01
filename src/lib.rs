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
let results = corpus.search("tomacco", 0.25, 10);
let top_match = results.first();

assert!(top_match.is_some());
assert!(top_match.unwrap().similarity > 0.5);
assert_eq!(top_match.unwrap().text,String::from("tomato"));
```

[1]: https://pythonhosted.org/ngram/ngram.html
[2]: http://chappers.github.io/web%20micro%20log/2015/04/29/comparison-of-ngram-fuzzy-matching-approaches/
*/

#![deny(missing_docs)]

use std::cmp::Ordering;

mod corpus;
mod ngram;

pub use crate::corpus::{Corpus, CorpusBuilder};
pub use crate::ngram::{Ngram, NgramBuilder};

/// Holds a fuzzy match search result string, and its associated similarity
/// to the query text.
#[derive(Debug, Clone)]
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
