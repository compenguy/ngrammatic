# Memory benchmarks
The goal of this benchmark is to accurately measure how much memory is required by the `ngrammatic` library to load the taxons dataset into memory.
The taxons dataset contains the `2_571_000` taxons from NCBI Taxons. While compressed in gzip, it is a merely 12MBs file.

## How to run the benchmarks
To run the memory benchmarks, navigate to the `benchmarks` directory and run the following command:

```bash
cargo run --release
```

## Benchmarks 2 april 2024, 11:00 PM
The third benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.

## Time required
Altough the time required to load the dataset into memory was not accurately measured as we did not do several runs, for this specific run it was `13.639367419s`. This is still an improvement, but I am rather confident that we can do better.

## Memory required
The memory requirements for the dataset were:

```text
1.010 GB 100.00% ⏺: ngrammatic::Corpus<ngrammatic::traits::arity::ArityTwo, ngrammatic::key_transformers::Lower, alloc::string::String, u16>
517.7 MB  51.25% ├╴keys_to_ngrams: std::collections::hash::map::HashMap<ngrammatic::traits::key::Normalizer<alloc::string::String, ngrammatic::key_transformers::Lower>, ngrammatic::ngrams::Ngram<ngrammatic::traits::arity::ArityTwo, u16>>
492.3 MB  48.75% ╰╴ngrams_to_keys: std::collections::hash::map::HashMap<[u8; 2], alloc::vec::Vec<&ngrammatic::traits::key::Normalizer<alloc::string::String, ngrammatic::key_transformers::Lower>>>
```

This is a further improvement compared to the previous run, as it is requires 300MBs less memory.

## Benchmarks 2 april 2024, 10:00 PM
The second benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.

### Time required
Altough the time required to load the dataset into memory was not accurately measured as we did not do several runs, for this specific run it was `14.457731947s`. This is a significant improvement over the previous run, as it is more than twice as fast.

### Memory required
The memory requirements for the dataset were:

```text
1.378 GB 100.00% ⏺: ngrammatic::Corpus
886.1 MB  64.28% ├╴keys_to_ngrams: std::collections::hash::map::HashMap<ngrammatic::traits::key::Normalizer<alloc::string::String, ngrammatic::key_transformer::Lower>, ngrammatic::ngrams::Ngram>
492.3 MB  35.72% ╰╴ngrams_to_keys: std::collections::hash::map::HashMap<[u8; 2], alloc::vec::Vec<&ngrammatic::traits::key::Normalizer<alloc::string::String, ngrammatic::key_transformer::Lower>>>
```

This is a significant improvement over the previous run, as it is more than 5 times less memory required.

## Benchmarks 2 April 2024, 09:00 AM
The first benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.

### Time required
Altough the time required to load the dataset into memory was not accurately measured as we did not do several runs, for this specific run it was `36.779114884s`

### Memory required
The memory requirements for the dataset were:

```text
7.875 GB 100.00% ⏺: ngrammatic::Corpus<ngrammatic::key_transformer::Lower, 2>
   24  B   0.00% ├╴pad_left: ngrammatic::Pad
                 │ ╰╴Variant: Auto
   24  B   0.00% ├╴pad_right: ngrammatic::Pad
                 │ ╰╴Variant: Auto
4.365 GB  55.43% ├╴ngrams: std::collections::hash::map::HashMap<alloc::string::String, ngrammatic::Ngram<2>>
3.510 GB  44.57% ├╴gram_to_words: std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<alloc::string::String>>
    0  B   0.00% ╰╴key_transformer: ngrammatic::key_transformer::Lower
```