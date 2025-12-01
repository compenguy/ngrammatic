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
ngrammatic = "0.5"
```

Or, for a possible improvement search speeds, enable the "rayon" feature:

```toml
[dependencies]
ngrammatic = { version = "0.5", features = ["rayon"] }
```

Benchmark results show 0-80% performance improvement in search, but a 0-3%
performance decline for corpus creation. With such a wide variation in
performance impact, it was decided not to enable it by default. When enabled,
it is possible to use any combination of serialized or parallelized methods
depending on which shows better performance for your use cases.

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

Here's a sample of data collected from the 0.5 version on my development machine:

| Test                                             | lower bound (ms) | typical (ms) | upper bound (ms) |
| ------------------------------------------------ | ---------------- | ------------ | ---------------- |
| novel parallel insertion case sensitive          |   84.477         |   84.857     |   85.236         |
| novel parallel insertion case insensitive        |   83.806         |   84.185     |   84.581         |
| novel serial insertion case sensitive            |   82.052         |   82.356     |   82.676         |
| novel serial insertion case insensitive          |   81.476         |   81.793     |   82.135         |
| random text parallel insertion case sensitive    |  154.46          |  154.95      |  155.45          |
| random text parallel insertion case insensitive  |  156.2           |  156.74      |  157.31          |
| random text serial insertion case sensitive      |  152.56          |  153.03      |  153.54          |
| random text serial insertion case insensitive    |  153.9           |  154.28      |  154.67          |
| domain names parallel insertion case sensitive   |  154.16          |  154.64      |  155.16          |
| domain names parallel insertion case insensitive |  154.61          |  155.58      |  156.71          |
| domain names serial insertion case sensitive     |  151.89          |  152.29      |  152.69          |
| domain names serial insertion case insensitive   |  152.76          |  153.28      |  153.83          |
| novel parallel search no match                   |    0.51049       |    0.51999   |    0.5306        |
| novel parallel search match                      |    3.1019        |    3.1362    |    3.1746        |
| novel serial search no match                     |    0.77879       |    0.77967   |    0.78066       |
| novel serial search match                        |    6.9894        |    7.0099    |    7.0331        |
| random text parallel search no match             |    0.50239       |    0.51616   |    0.53169       |
| random text parallel search match                |    2.8231        |    2.8733    |    2.9281        |
| random text serial search no match               |    1.4228        |    1.4238    |    1.4248        |
| random text serial search match                  |   16.335         |   16.594     |   16.864         |
| domain names parallel search no match            |   21.821         |   22.029     |   22.25          |
| domain names parallel search match               | 2068.7           | 2071.8       | 2075.1           |
| domain names serial search no match              |   81.061         |   81.687     |   82.348         |
| domain names serial search match                 | 5898.3           | 5906.6       | 5914.9           |

Do note that those search times against the top domain names corpus were taking
several seconds to complete in the case where a perfect match exists. It's unclear
at the moment why search results with perfect matches always take significantly longer.

### Areas for future improvement

Adding string interning to the corpus was a really big performance and memory
win. Unfortunately, updating `Ngram` with interning is quite a bit more
complicated, and I'm open to proposals for how to accomplish it.

Interning the grams themselves (`HashMap<String, usize>`) might be less of a
win, though. The default symbol for StringInterner is a u32, while the string
itself is the same size or small for `arity` <= 4. Because we don't actually
care about the 'stringness' of the grams, something like a tinyvec<u8>
pre-sized to `arity` could be a win.
