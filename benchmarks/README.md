# Memory benchmarks
The goal of this benchmark is to accurately measure how much memory is required by the `ngrammatic` library to load the taxons dataset into memory.
The taxons dataset contains the `2_571_000` taxons from NCBI Taxons. While compressed in gzip, it is a merely 12MBs file.

## How to run the benchmarks
To run the memory benchmarks, navigate to the `benchmarks` directory and run the following command:

```bash
cargo run --release
```

## Benchmarks 2 April 2024, 09:00 AM
The first benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.

### Time required
Altough the time required to load the dataset into memory was not accurately measured as we did not do several runs, for this specific run it was `36.779114884s`

### Memory required
The memory requirements for the dataset were:

```text
7.875 GB 100.00% ⏺: ngrammatic::Corpus<ngrammatic::key_transformer::LowerKeyTransformer, 2>
   24  B   0.00% ├╴pad_left: ngrammatic::Pad
                 │ ╰╴Variant: Auto
   24  B   0.00% ├╴pad_right: ngrammatic::Pad
                 │ ╰╴Variant: Auto
4.365 GB  55.43% ├╴ngrams: std::collections::hash::map::HashMap<alloc::string::String, ngrammatic::Ngram<2>>
3.510 GB  44.57% ├╴gram_to_words: std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<alloc::string::String>>
    0  B   0.00% ╰╴key_transformer: ngrammatic::key_transformer::LowerKeyTransformer
```