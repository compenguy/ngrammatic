# Ngrammatic
[![Build status](https://github.com/compenguy/ngrammatic/actions/workflows/clippy.yml/badge.svg)](https://github.com/compenguy/ngrammatic/actions)
[![Crates.io](https://img.shields.io/crates/v/ngrammatic.svg)](https://crates.io/crates/ngrammatic)
[![Documentation](https://docs.rs/ngrammatic/badge.svg)](https://docs.rs/ngrammatic/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

Rust crate providin n-gram based fuzzy matching, with support for [Okapi BM25 TF-IDF](https://en.wikipedia.org/wiki/Okapi_BM25) search, cutting-edge memory efficient data structures such as [Rear Coded Lists](https://docs.rs/sux/0.3.1/sux/dict/rear_coded_list/struct.RearCodedList.html), [BVGraph](https://github.com/vigna/webgraph-rs) and [Elias-Fano](https://docs.rs/sux/latest/sux/dict/elias_fano/struct.EliasFano.html) baked in, plus [Rayon](https://github.com/rayon-rs/rayon)-based parallelism.

## Installation

This crate is available from [crates.io](https://crates.io/crates/ngrammatic).

To use it, either add it to your Cargo.toml or run `cargo add ngrammatic`.

```toml
[dependencies]
ngrammatic = "0.5.0"
```

## Usage examples
Depending on your use case, you may want to select different data structures and search algorithms. While in the [documentation](https://docs.rs/ngrammatic/) you can find more detailed information, here are some examples to get you started.

To make an informed choice for your use case, please do check out the [memory requirement benchmarks here](https://github.com/LucaCappelletti94/ngrammatic/tree/master/benchmarks) and the [time requirement benchmarks here](https://github.com/LucaCappelletti94/ngrammatic/tree/master/benches) which compare the different data structures and search algorithms.

### Basic usage
We will go in the details of the different data structures and search algorithms later in this README, but first let's get started with an example you can immediately run. In this case, we create a trigram index to search across the [`ANIMALS`] names list which we ship with the crate.

```rust
use ngrammatic::prelude::*;

// We create a corpus from the list of animals in parallel.
let corpus: Corpus<[&str; 699], TriGram<char>> = Corpus::par_from(ANIMALS);

// We setup the search configuration
let search_config = NgramSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_maximum_number_of_results(5);

// We search for a word similar to "catt"
let search_results: Vec<SearchResult<&&str, f32>> = corpus.ngram_search("Cattos", search_config);

assert!(!search_results.is_empty());

// We print the search results
for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}
```

### Text normalization
Natural language processing is notoriously difficult, and one of the first steps is to normalize the text. You can add any normalization you want by creating new struct markers that implement [`std::convert::AsRef`] to the type of the keys you want to use, which may be for instance [`str`] or [`String`]. In this case, we use the [`Lowercase`] struct marker to normalize the text to lowercase. By default, text represented in [`str`] or [`String`] is padded with [`NULL`](https://theasciicode.com.ar/ascii-control-characters/null-character-ascii-code-0.html) characters to ensure that the n-grams minimum length is respected by default, we drop all non-alphanumeric characters, remove duplicated spaces and trim both spaces and [`NULL`](https://theasciicode.com.ar/ascii-control-characters/null-character-ascii-code-0.html) characters from the sides of the text. You can use struct markers to customize the normalization process to remove or add any other normalization steps you may need.

```rust
use ngrammatic::prelude::*;

// We build the corpus sequentially - there is no particular reason to use one or ther other
// in this example, just to show that you can use both.
let corpus: Corpus<&[&str; 699], TriGram<char>, Lowercase> = Corpus::from(&ANIMALS);

// We setup the search configuration
let search_config = NgramSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_maximum_number_of_results(5);

// We search for a word similar to "catt"
let search_results: Vec<SearchResult<&&str, f32>> = corpus.ngram_search("Cattos", search_config);

assert!(!search_results.is_empty());

// We print the search results
for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}
```

#### ASCII characters
If you are working with ASCII characters or want to remove the UTF8 characters from your text, you can use the [`ASCIIChar`] struct as your gram to ensure that the text is normalized to ASCII characters. Note that a [`char`] in Rust is represented by a [`u32`], while an ASCII character is represented by a [`u8`]. This means that by using the [`ASCIIChar`] as your gram, your will reduce by default the memory usage of your n-grams by 4 times.

```rust
use ngrammatic::prelude::*;

let corpus: Corpus<&[&str; 699], TriGram<ASCIIChar>, Lowercase<str>> = Corpus::par_from(&ANIMALS);

// We setup the search configuration
let search_config = NgramSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_maximum_number_of_results(5);

// We search for a word similar to "catt"
let search_results: Vec<SearchResult<&&str, f32>> = corpus.ngram_search("Cattos", search_config);

assert!(!search_results.is_empty());

// We print the search results
for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}
```

#### Using bytes
If you are working with bytes, you can use [`u8`] as your gram. Note that this means that you can make n-grams out of anything that you can represent as a sequence of bytes. By default, [`u8`] are padded with zeros to ensure that the n-grams minimum length is respected.

```rust
use ngrammatic::prelude::*;

let corpus: Corpus<&[&str; 699], TriGram<u8>, Lowercase<str>> = Corpus::par_from(&ANIMALS);

// We setup the search configuration
let search_config = NgramSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_maximum_number_of_results(5);

// We search for a word similar to "catt"
let search_results: Vec<SearchResult<&&str, f32>> = corpus.ngram_search("Cattos", search_config);

assert!(!search_results.is_empty());

// We print the search results
for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}
```

### What are n-grams?
An n-gram is a contiguous sequence of n items from a given sequence. In the case of text, the n-gram are commonly used to create a sequence of characters. Concretely, in this crate, we use arrays for the n-grams, which means that the n-grams are fixed length sequences of items. In the following picture, we illustrate how the word "cat" is transformed into trigrams.

![Trigrams of a cat](split.jpg)

#### Which n-grams are available?
While you can create your own n-grams by implementing the [`Ngram`] trait, we provide ngrams from size one to eight. The n-grams are named as follows: [`UniGram`], [`BiGram`], [`TriGram`], [`TetraGram`], [`PentaGram`], [`HexaGram`], [`HeptaGram`] and [`OctaGram`]. The reason we stop at eight is that eight is the maximum number of [`u8`] can be stored in a [`u64`], and generally speaking using more than eight characters for n-grams is already overkill for most use cases.

Here follows an example of how you can create corpus with different n-grams:

```rust
use ngrammatic::prelude::*;

let corpus: Corpus<&[&str; 699], UniGram<char>> = Corpus::par_from(&ANIMALS);
let corpus: Corpus<&[&str; 699], BiGram<char>> = Corpus::par_from(&ANIMALS);
let corpus: Corpus<&[&str; 699], TriGram<char>> = Corpus::par_from(&ANIMALS);
let corpus: Corpus<&[&str; 699], TetraGram<char>> = Corpus::par_from(&ANIMALS);
let corpus: Corpus<&[&str; 699], PentaGram<char>> = Corpus::par_from(&ANIMALS);
let corpus: Corpus<&[&str; 699], HexaGram<char>> = Corpus::par_from(&ANIMALS);
let corpus: Corpus<&[&str; 699], HeptaGram<char>> = Corpus::par_from(&ANIMALS);
let corpus: Corpus<&[&str; 699], OctaGram<char>> = Corpus::par_from(&ANIMALS);
```

#### Which n-gram should I use?
The optimal size for ngrams can vary depending on the specific task and dataset you're working with. Generally, smaller ngram sizes (like [`UniGram`] or [`BiGram`]) capture more local patterns and are useful for tasks like text classification or sentiment analysis. On the other hand, larger ngram sizes (like [`TriGram`] or higher) capture more global patterns and are useful for tasks like machine translation or language modeling.

In practice, it's often beneficial to experiment with different ngram sizes to see what works best for your particular application. Keep in mind that larger ngram sizes can lead to sparsity issues, especially with smaller datasets, while smaller ngram sizes may not capture enough context for certain tasks. So, it's often a trade-off between capturing enough context and avoiding sparsity.

### Search algorithms
This struct provide two search algorithms: n-gram search and Okapi BM25 TF-IDF search. The n-gram search is a fuzzy search algorithm that uses n-grams to find similar strings in a corpus. The Okapi BM25 TF-IDF search is a more advanced search algorithm that uses term frequency-inverse document frequency (TF-IDF) to rank search results based on their relevance to a query. You can also combine the weighting schema from the two search algorithms. The search algorithms are implemented as methods on the [`Corpus`] struct. All search algorithms come with search configurations, and are available in both sequential and parallel versions.

#### N-gram search
The n-gram search algorithm is a fuzzy search algorithm that uses n-grams to find similar strings in a corpus. The algorithm works by creating n-grams from the query string and comparing them to the n-grams in the corpus. The search results are ranked based on the [Jaccard-like similarity](https://en.wikipedia.org/wiki/Jaccard_index) between the query string and the strings in the corpus. The n-gram search algorithm is implemented as the [`ngram_search`] and [`ngram_par_search`] methods on the [`Corpus`] struct, with the configuration provided by the [`NgramSearchConfig`] struct. By default, these method use a warp of 2.

```rust
use ngrammatic::prelude::*;

let corpus: Corpus<&[&str; 699], TriGram<char>, Lowercase> = Corpus::par_from(&ANIMALS);

let search_config = NgramSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_maximum_number_of_results(5);

let search_results: Vec<SearchResult<&&str, f32>> = corpus.ngram_search("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}

let search_results: Vec<SearchResult<&&str, f32>> = corpus.ngram_par_search("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}
```

##### Warp
The warp n-gram search algorithm uses a parameter called warp which defines the expontiation of the numerator and denumerator of the Jaccard-like similarity. The warp parameter is useful to give increase the similarity of shorter string pairs. The warp parameter is defined in the [`NgramSearchConfig`] struct. This search algorithm is implemented as the [`ngram_search_with_warp`] and [`ngram_par_search_with_warp`] methods on the [`Corpus`] struct. Note that using an integer warp is faster than using a float warp as we can use the [`powi`] method instead of the [`powf`] method.

```rust
use ngrammatic::prelude::*;

let corpus: Corpus<&[&str; 699], TriGram<char>, Lowercase> = Corpus::par_from(&ANIMALS);

let search_config = NgramSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_maximum_number_of_results(5)
    .set_warp(1.5).unwrap();

let search_results: Vec<SearchResult<&&str, f32>> = corpus.ngram_search_with_warp("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}

let search_results: Vec<SearchResult<&&str, f32>> = corpus.ngram_par_search_with_warp("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}
```

#### Okapi BM25 TF-IDF search
The [Okapi BM25 TF-IDF](https://en.wikipedia.org/wiki/Okapi_BM25) search algorithm is a more advanced ranking algorithm that uses [term frequency-inverse document frequency (TF-IDF)](https://en.wikipedia.org/wiki/Tf%E2%80%93idf) to rank search results based on their relevance to a query. The algorithm works by calculating the TF-IDF score for each term in the query and the documents in the corpus, and then ranking the documents based on their TF-IDF scores. The Okapi BM25 TF-IDF search algorithm is implemented as the [`tf_idf_search`] and [`tf_idf_par_search`] methods on the [`Corpus`] struct, with the configuration provided by the [`TFIDFSearchConfig`] struct.

```rust
use ngrammatic::prelude::*;

let corpus: Corpus<&[&str; 699], TriGram<char>, Lowercase> = Corpus::par_from(&ANIMALS);

let search_config = TFIDFSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_maximum_number_of_results(5);

let search_results: Vec<SearchResult<&&str, f32>> = corpus.tf_idf_search("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}

let search_results: Vec<SearchResult<&&str, f32>> = corpus.tf_idf_par_search("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}
```

#### Combined search
You can use the two combined weighting schema to combine the results of the n-gram search and the Okapi BM25 TF-IDF search. The combined search algorithm is implemented as the [`warped_tf_idf_search`] and [`warped_tf_idf_par_search`] methods on the [`Corpus`] struct, with the configuration provided by the [`TFIDFSearchConfig`] struct.

```rust
use ngrammatic::prelude::*;

let corpus: Corpus<&[&str; 699], TriGram<char>, Lowercase> = Corpus::par_from(&ANIMALS);

let search_config = TFIDFSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_warp(1.5).unwrap()
    .set_maximum_number_of_results(5);

let search_results: Vec<SearchResult<&&str, f32>> = corpus.warped_tf_idf_search("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}

let search_results: Vec<SearchResult<&&str, f32>> = corpus.warped_tf_idf_par_search("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}
```

### BVGraph or Webgraph
[Webgraph](https://github.com/vigna/webgraph-rs/) is a Rust library for succinctly representing graphs ([the very interesting paper describing this effort is available here](https://hal.science/hal-04494627/document)). The BVGraph is an efficient representation of a graph that uses a bit vector to represent the adjacency list of each node. The BVGraph is a compressed representation of the graph that uses substantially less memory than a traditional adjacency list or adjacency matrix.

By default, this crate uses a combination of [Elias-Fano](https://core.ac.uk/download/pdf/79617357.pdf) and [bit vectors](https://en.wikipedia.org/wiki/Bit_array) to store efficiently the graph representing the connections between the documents and the n-grams. In some settings, you may want to use instead the BVGraph representation, which is [somewhat slower](https://github.com/LucaCappelletti94/ngrammatic/tree/master/benches) but [it employs significantly less memory](https://github.com/LucaCappelletti94/ngrammatic/blob/master/benchmarks/README.md).

Here follows an example of how you can create a corpus with the BVGraph representation:

```rust
use ngrammatic::prelude::*;

let corpus: Corpus<&[&str; 699], TriGram<char>, Lowercase> = Corpus::par_from(&ANIMALS);
let corpus_webgraph: Corpus<&[&str; 699], TriGram<char>, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

```

#### Using native compilation targets
The [Webgraph](https://github.com/vigna/webgraph-rs/) rust library benefits significantly from compiling the code to the native target. This is because the Webgraph library extensively uses [SIMD instructions](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data) to speed up the computation. To compile the code to the native target, you can use the following command:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Rear Coded List
The [Rear Coded List](https://docs.rs/sux/0.3.1/sux/dict/rear_coded_list/struct.RearCodedList.html) is a memory efficient data structure to store an immutable list of strings compressed by prefix omission via rear coding. Prefix omission compresses a list of strings omitting the common prefixes of consecutive strings. To do so, it stores the length of what remains after the common prefix (hence, rear coding). It is usually applied to lists strings sorted in ascending order. The encoding is done in blocks of k strings: in each block the first string is encoded without compression, wheres the other strings are encoded with the common prefix removed.

Here follows an example of how you can create a corpus with the Rear Coded List representation:

```rust
use ngrammatic::prelude::*;
use rayon::slice::ParallelSliceMut;

let mut animals: Vec<&str> = ANIMALS.into();
animals.par_sort_unstable();

let mut builder = RearCodedListBuilder::new(8);
for animal in animals {
    builder.push(&animal);
}
let rear_coded_list: RearCodedList = builder.build();

let corpus: Corpus<RearCodedList, TriGram<char>, Lowercase> = Corpus::par_from(rear_coded_list);

// You can still convert this into a Webgraph representation to save even more memory
let corpus_webgraph: Corpus<RearCodedList, TriGram<char>, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

// We define a search config
let search_config = NgramSearchConfig::default()
    .set_minimum_similarity_score(0.3).unwrap()
    .set_maximum_number_of_results(5);

// And now you can use the corpus as you would normally do, with the catch that the search
// results will necessarily be of type SearchResult<String, f32> instead of references since
// the RearCodedList cannot provide references to the strings.
let search_results: Vec<SearchResult<String, f32>> = corpus_webgraph.ngram_search("Cattos", search_config);

assert!(!search_results.is_empty());

for search_result in search_results {
    println!("{}: {}", search_result.key(), search_result.score());
}

```

## Contributing
Contributions from the community are highly appreciated and can help improve this project. If you have any suggestions, feature requests, or bugs to report, please open an issue on GitHub. Additionally, if you want to contribute to the project, you can open a pull request with your proposed changes. Before making any substantial changes, please discuss them with the project maintainers in the issue tracker.

If you appreciate this project and would like to support its development, you can star the repository on GitHub. 
