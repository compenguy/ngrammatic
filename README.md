This crate provides fuzzy search/string matching using N-grams.

This implementation is character-based, rather than word based,
matching solely based on string similarity.

Licensed under the MIT license.


### Documentation

https://docs.rs/ngrammatic/latest/ngrammatic/

### Installation

This crate is published on [crates.io](https://crates.io/crates/).

To use it, add this to your Cargo.toml:

```toml
[dependencies]
ngrammatic = "0.4.1"
```

Or, to reduce memory usage for large corpuses, enable the "trie" feature:

```toml
[dependencies]
ngrammatic = { version = "0.4.1", features = ["trie"] }
```

Benchmarking suggests that trie is 30-45% slower for corpus creation and 25-45%
slower for search.

The `rayon` feature is available, and benchmark results show 0-3% performance
improvement in search, but a 0-3% performance decline for corpus creation. It
is possible to use serialized corpus creation and parallel search, which might
yield the best overall results.

### Usage
To do fuzzy matching, build up your corpus of valid symbols like this:

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
let word = String::from("tomacco");
if let Some(top_result) = corpus.search(word, 0.25).first() {
    if top_result.similarity > 0.99 {
        println!("✔ {}", top_result.text);
    } else {
        println!("❓{} (did you mean {}? [{:.0}% match])",
                 word,
                 top_result.text,
                 top_result.similarity * 100.0);
    }
} else {
    println!("🗙 {}", word);
}
```

