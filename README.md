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

