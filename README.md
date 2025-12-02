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
ngrammatic = "0.7"
```

Or, for a possible improvement search speeds, enable the "rayon" feature:

```toml
[dependencies]
ngrammatic = { version = "0.7", features = ["rayon"] }
```

Benchmark results show rayon offers a 30-80% performance improvement in search,
but about a 2% performance decline for corpus creation. When enabling rayon,
you'll likely see better results using serial corpus creation and parallel
search, but this is no substitute for running your own benchmarks.

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

## Benchmarking

Some benchmarks exist to compare the performance of various scenarios.

In order to run them, rayon must be enabled, e.g.:

```ignore
$ cargo bench --features rayon
```

The benchmarks of the top-domains.txt file can take quite a long time to
complete, as they're working against a very large dataset.

Here's a sample of data collected from the 0.7 version on my development machine (approx. 15-20% faster than 0.5!):

| Test                                             | lower bound (ms) | typical (ms) | upper bound (ms) |
| ------------------------------------------------ | ---------------- | ------------ | ---------------- |
| novel parallel insertion case sensitive          |   69.424         |   69.680     |   69.951         |
| novel parallel insertion case insensitive        |   69.865         |   70.118     |   70.392         |
| novel serial insertion case sensitive            |   67.468         |   67.715     |   67.976         |
| novel serial insertion case insensitive          |   68.839         |   69.253     |   69.698         |
| random text parallel insertion case sensitive    |  107.67          |  107.94      |  108.22          |
| random text parallel insertion case insensitive  |  109.89          |  110.41      |  110.97          |
| random text serial insertion case sensitive      |  107.46          |  107.90      |  108.41          |
| random text serial insertion case insensitive    |  108.04          |  108.39      |  108.77          |
| domain names parallel insertion case sensitive   |  108.76          |  109.15      |  109.56          |
| domain names parallel insertion case insensitive |  108.96          |  109.27      |  109.59          |
| domain names serial insertion case sensitive     |  107.00          |  107.23      |  107.46          |
| domain names serial insertion case insensitive   |  107.70          |  107.93      |  108.19          |
| novel parallel search no match                   |    0.49750       |    0.50972   |    0.52333       |
| novel parallel search match                      |    2.9885        |    3.0266    |    3.0700        |
| novel serial search no match                     |    0.75857       |    0.75919   |    0.75993       |
| novel serial search match                        |    6.7814        |    6.8057    |    6.8324        |
| random text parallel search no match             |    0.47633       |    0.49054   |    0.50553       |
| random text parallel search match                |    2.6893        |    2.7269    |    2.7673        |
| random text serial search no match               |    1.3826        |    1.3860    |    1.3901        |
| random text serial search match                  |   13.709         |   13.940     |   14.182         |
| domain names parallel search no match            |   15.018         |   15.194     |   15.379         |
| domain names parallel search match               | 1590.0           | 1593.5       | 1597.1           |
| domain names serial search no match              |   61.813         |   62.431     |   63.085         |
| domain names serial search match                 | 4466.2           | 4475.0       | 4498.9           |

Do note that those search times against the top domain names corpus were taking
several seconds to complete in the case where a perfect match exists. It's unclear
at the moment why search results with perfect matches always take significantly longer.

### Areas for future improvement

Adding string interning to the corpus was a really big performance and memory
win. Unfortunately, updating `Ngram` with interning is quite a bit more
complicated, and I'm open to proposals for how to accomplish it.

In the meantime, replacing Strings in `Ngram` with SmolStrs netted about a 15%
performance win vs without.

New major gains will likely require some thoughtful cpu and memory profiling.
