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
ngrammatic = "0.3.4"
```

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

