# Ngrammatic
[![Build status](https://github.com/compenguy/ngrammatic/actions/workflows/clippy.yml/badge.svg)](https://github.com/compenguy/ngrammatic/actions)
[![Crates.io](https://img.shields.io/crates/v/ngrammatic.svg)](https://crates.io/crates/ngrammatic)
[![Documentation](https://docs.rs/ngrammatic/badge.svg)](https://docs.rs/ngrammatic/)

This crate provides fuzzy search/string matching using N-grams.

This implementation is character-based, rather than word based, matching
solely based on string similarity. It is modelled somewhat after the
[python ngram module](https://pythonhosted.org/ngram/ngram.html) with some inspiration from
[chappers' blog post on fuzzy matching with ngrams](http://chappers.github.io/web%20micro%20log/2015/04/29/comparison-of-ngram-fuzzy-matching-approaches/).

The crate is implemented in three parts: the `Corpus`, which is an
index connecting strings (words, symbols, whatever) to their `Ngrams`,
and `SearchResult`s, which contains a fuzzy match result, with the
word and a similarity measure in the range of 0.0 to 1.0.

The general usage pattern is to construct a `Corpus`, `.add()` your
list of valid symbols to it, and then perform `.search()`es of valid,
unknown, misspelled, etc symbols on the `Corpus`. The results come
back as a vector of up to 10 results, sorted from highest similarity
to lowest.

Licensed under the MIT license.

## Installation

This crate is published on [crates.io](https://crates.io/crates/).

To use it, add this to your Cargo.toml:

```toml
[dependencies]
ngrammatic = "0.4.0"
```

## Usage example
To do fuzzy matching, build up your corpus of valid symbols like this:

```rust
use ngrammatic::{CorpusBuilder, Pad};

let mut corpus = CorpusBuilder::default()
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
