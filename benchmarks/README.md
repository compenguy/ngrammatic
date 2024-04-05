# Memory benchmarks
The goal of this benchmark is to accurately measure how much memory is required by the `ngrammatic` library to load the taxons dataset into memory.
The taxons dataset contains the `2_571_000` taxons from NCBI Taxons. While compressed in gzip, it is a merely 12MBs file.

## How to run the benchmarks
To run the memory benchmarks, navigate to the `benchmarks` directory and run the following command:

```bash
cargo run --release
```

## Benchmarks 5 April 2023, 08:00 PM
The sixth benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.
The innovation of this run is the use of the `EliasFano` data structure to store the ngrams, which can be more efficient than
Vec we were using before. The vec does not make any assumptions about the data, while the `EliasFano` data structure does, and
since in the vast majority of cases we want to store monotonically increasing data which we can generally convert to small integers,
this is a good fit. For all cases where the ngrams are too large to fit within an u64, we fallback to the Vec data structure.

### Time required
The time required to load the dataset into memory was `17.328862785s`. There seems to be a slight slow down compared to the previous run,
and this is likely due to the fact that we are now using the `EliasFano` data structure to store the ngrams which requires somewhat more
computation than the `Vec` data structure.

### Memory required
The memory requirements for the dataset are nearly identical to the previous run overall, but if we focus to specific field we modified, we can see that the `ngrams` field is now using the `EliasFano` data structure, which is more efficient than the `Vec` data structure we were using before.
Specifically, the `ngrams` field is now using 2.072kB of memory, while before it was using 5.196kB of memory. This is a significant improvement,
which will allow us to reasonable scale to much larger dictionaries.

```text
401.6 MB 100.00% ⏺: ngrammatic::corpus::Corpus<alloc::vec::Vec<alloc::string::String>, [ngrammatic::traits::ascii_char::ASCIIChar; 2], ngrammatic::traits::char_normalizer::Lowercase<str>>
128.4 MB  31.97% ├╴keys: alloc::vec::Vec<alloc::string::String>
2.072 kB   0.00% ├╴ngrams: sux::dict::elias_fano::EliasFano<sux::rank_sel::select_fixed2::SelectFixed2>
    8  B   0.00% │ ├╴u: usize
    8  B   0.00% │ ├╴n: usize
    8  B   0.00% │ ├╴l: usize
1.024 kB   0.00% │ ├╴low_bits: sux::bits::bit_field_vec::BitFieldVec
 1000  B   0.00% │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ ├╴bit_width: usize
    8  B   0.00% │ │ ├╴mask: usize
    8  B   0.00% │ │ ╰╴len: usize
1.024 kB   0.00% │ ╰╴high_bits: sux::rank_sel::select_fixed2::SelectFixed2
  872  B   0.00% │   ├╴bits: sux::bits::bit_vec::CountBitVec
  856  B   0.00% │   │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │   │ ├╴len: usize
    8  B   0.00% │   │ ╰╴number_of_ones: usize
  152  B   0.00% │   ╰╴inventory: alloc::vec::Vec<u64>
273.2 MB  68.03% ├╴graph: ngrammatic::bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph
28.53 MB   7.10% │ ├╴srcs_to_dsts_weights: sux::bits::bit_field_vec::BitFieldVec
28.53 MB   7.10% │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ ├╴bit_width: usize
    8  B   0.00% │ │ ├╴mask: usize
    8  B   0.00% │ │ ╰╴len: usize
2.153 MB   0.54% │ ├╴srcs_offsets: sux::dict::elias_fano::EliasFano<sux::rank_sel::select_fixed2::SelectFixed2>
    8  B   0.00% │ │ ├╴u: usize
    8  B   0.00% │ │ ├╴n: usize
    8  B   0.00% │ │ ├╴l: usize
1.286 MB   0.32% │ │ ├╴low_bits: sux::bits::bit_field_vec::BitFieldVec
1.286 MB   0.32% │ │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ │ ├╴bit_width: usize
    8  B   0.00% │ │ │ ├╴mask: usize
    8  B   0.00% │ │ │ ╰╴len: usize
867.7 kB   0.22% │ │ ╰╴high_bits: sux::rank_sel::select_fixed2::SelectFixed2
767.2 kB   0.19% │ │   ├╴bits: sux::bits::bit_vec::CountBitVec
767.2 kB   0.19% │ │   │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │   │ ├╴len: usize
    8  B   0.00% │ │   │ ╰╴number_of_ones: usize
100.5 kB   0.03% │ │   ╰╴inventory: alloc::vec::Vec<u64>
5.552 kB   0.00% │ ├╴dsts_offsets: sux::dict::elias_fano::EliasFano<sux::rank_sel::select_fixed2::SelectFixed2>
    8  B   0.00% │ │ ├╴u: usize
    8  B   0.00% │ │ ├╴n: usize
    8  B   0.00% │ │ ├╴l: usize
4.576 kB   0.00% │ │ ├╴low_bits: sux::bits::bit_field_vec::BitFieldVec
4.552 kB   0.00% │ │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ │ ├╴bit_width: usize
    8  B   0.00% │ │ │ ├╴mask: usize
    8  B   0.00% │ │ │ ╰╴len: usize
  952  B   0.00% │ │ ╰╴high_bits: sux::rank_sel::select_fixed2::SelectFixed2
  800  B   0.00% │ │   ├╴bits: sux::bits::bit_vec::CountBitVec
  784  B   0.00% │ │   │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │   │ ├╴len: usize
    8  B   0.00% │ │   │ ╰╴number_of_ones: usize
  152  B   0.00% │ │   ╰╴inventory: alloc::vec::Vec<u64>
156.9 MB  39.08% │ ├╴srcs_to_dsts: sux::bits::bit_field_vec::BitFieldVec
156.9 MB  39.08% │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ ├╴bit_width: usize
    8  B   0.00% │ │ ├╴mask: usize
    8  B   0.00% │ │ ╰╴len: usize
85.60 MB  21.31% │ ╰╴dsts_to_srcs: sux::bits::bit_field_vec::BitFieldVec
85.60 MB  21.31% │   ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │   ├╴bit_width: usize
    8  B   0.00% │   ├╴mask: usize
    8  B   0.00% │   ╰╴len: usize
    0  B   0.00% ╰╴_phantom: core::marker::PhantomData<ngrammatic::traits::char_normalizer::Lowercase<str>>
```

## Benchmarks 5 April 2024, 04:00 PM
The fifth benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.
The innovation of this run is that we are using a `EliasFano` data structure to store the offsets. This is more efficient than
the `BitFieldVec` that we were using before because we are exploiting the fact that the offsets are monotonically increasing.

### Time required
The time required to load the dataset into memory was `14.245963367s`. 

### Memory required
The memory requirements for the dataset were:

```text
401.5 MB 100.00% ⏺: ngrammatic::corpus::Corpus<alloc::vec::Vec<alloc::string::String>, [ngrammatic::traits::ascii_char::ASCIIChar; 2], ngrammatic::traits::char_normalizer::Lowercase<str>>
128.4 MB  31.98% ├╴keys: alloc::vec::Vec<alloc::string::String>
5.196 kB   0.00% ├╴ngrams: alloc::vec::Vec<[ngrammatic::traits::ascii_char::ASCIIChar; 2]>
273.1 MB  68.02% ├╴graph: ngrammatic::bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph
28.53 MB   7.11% │ ├╴srcs_to_dsts_weights: sux::bits::bit_field_vec::BitFieldVec
28.53 MB   7.11% │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ ├╴bit_width: usize
    8  B   0.00% │ │ ├╴mask: usize
    8  B   0.00% │ │ ╰╴len: usize
2.053 MB   0.51% │ ├╴srcs_offsets: sux::dict::elias_fano::EliasFano
    8  B   0.00% │ │ ├╴u: usize
    8  B   0.00% │ │ ├╴n: usize
    8  B   0.00% │ │ ├╴l: usize
1.286 MB   0.32% │ │ ├╴low_bits: sux::bits::bit_field_vec::BitFieldVec
1.286 MB   0.32% │ │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ │ ├╴bit_width: usize
    8  B   0.00% │ │ │ ├╴mask: usize
    8  B   0.00% │ │ │ ╰╴len: usize
767.2 kB   0.19% │ │ ╰╴high_bits: sux::bits::bit_vec::CountBitVec
767.2 kB   0.19% │ │   ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │   ├╴len: usize
    8  B   0.00% │ │   ╰╴number_of_ones: usize
5.400 kB   0.00% │ ├╴dsts_offsets: sux::dict::elias_fano::EliasFano
    8  B   0.00% │ │ ├╴u: usize
    8  B   0.00% │ │ ├╴n: usize
    8  B   0.00% │ │ ├╴l: usize
4.576 kB   0.00% │ │ ├╴low_bits: sux::bits::bit_field_vec::BitFieldVec
4.552 kB   0.00% │ │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ │ ├╴bit_width: usize
    8  B   0.00% │ │ │ ├╴mask: usize
    8  B   0.00% │ │ │ ╰╴len: usize
  800  B   0.00% │ │ ╰╴high_bits: sux::bits::bit_vec::CountBitVec
  784  B   0.00% │ │   ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │   ├╴len: usize
    8  B   0.00% │ │   ╰╴number_of_ones: usize
156.9 MB  39.09% │ ├╴srcs_to_dsts: sux::bits::bit_field_vec::BitFieldVec
156.9 MB  39.09% │ │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ │ ├╴bit_width: usize
    8  B   0.00% │ │ ├╴mask: usize
    8  B   0.00% │ │ ╰╴len: usize
85.60 MB  21.32% │ ╰╴dsts_to_srcs: sux::bits::bit_field_vec::BitFieldVec
85.60 MB  21.32% │   ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │   ├╴bit_width: usize
    8  B   0.00% │   ├╴mask: usize
    8  B   0.00% │   ╰╴len: usize
    0  B   0.00% ╰╴_phantom: core::marker::PhantomData<ngrammatic::traits::char_normalizer::Lowercase<str>>
```

This is a slight improvement over the previous run, as it is requires 40MBs less memory. Most of this improvement comes from
the introduction of an easy-to-use, compile-time-defined type marker for the normalization, which is a `PhantomData` field.

## Benchmarks 5 April 2024, 10:00 AM
The fourth benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.
The innovation of this iteration is the use of an explicit weighted bipartite graph connecting the keys to the ngrams, which is
represented using two CSR data structures. These CSRs have their offsets and destinations stored in a `BitFieldVec`. Also the
cooccurrences are stored in a `BitFieldVec`.

### Time required
Altough the time required to load the dataset into memory was not accurately measured as we did not do several runs, for this specific run it was `14.882637729s`. This is still an improvement, but I am rather confident that we can do better. Primarily, the construction of the BitFieldVecs is something that can be reasonably vastly improved upon. I am currently working with the author of the `sux` library to see if we can improve the performance of the BitFieldVecs.

### Memory required
The memory requirements for the dataset were:

```text
439.6 MB 100.00% ⏺: ngrammatic::corpus::Corpus<alloc::vec::Vec<alloc::string::String>, [u8; 2]>
128.4 MB  29.21% ├╴keys: alloc::vec::Vec<alloc::string::String>
11.33 kB   0.00% ├╴ngrams: alloc::vec::Vec<[u8; 2]>
31.05 MB   7.07% ├╴cooccurrences: sux::bits::bit_field_vec::BitFieldVec
31.05 MB   7.07% │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ ├╴bit_width: usize
    8  B   0.00% │ ├╴mask: usize
    8  B   0.00% │ ╰╴len: usize
8.356 MB   1.90% ├╴key_offsets: sux::bits::bit_field_vec::BitFieldVec
8.356 MB   1.90% │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ ├╴bit_width: usize
    8  B   0.00% │ ├╴mask: usize
    8  B   0.00% │ ╰╴len: usize
18.43 kB   0.00% ├╴ngram_offsets: sux::bits::bit_field_vec::BitFieldVec
18.41 kB   0.00% │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ ├╴bit_width: usize
    8  B   0.00% │ ├╴mask: usize
    8  B   0.00% │ ╰╴len: usize
100.9 MB  22.96% ├╴key_to_ngram_edges: sux::bits::bit_field_vec::BitFieldVec
100.9 MB  22.96% │ ├╴data: alloc::vec::Vec<usize>
    8  B   0.00% │ ├╴bit_width: usize
    8  B   0.00% │ ├╴mask: usize
    8  B   0.00% │ ╰╴len: usize
170.8 MB  38.86% ╰╴gram_to_key_edges: sux::bits::bit_field_vec::BitFieldVec
170.8 MB  38.86%   ├╴data: alloc::vec::Vec<usize>
    8  B   0.00%   ├╴bit_width: usize
    8  B   0.00%   ├╴mask: usize
    8  B   0.00%   ╰╴len: usize
```

Impressively, the memory requirements have been reduced by more than 50% compared to the previous run. This is a significant improvement.

## Benchmarks 2 April 2024, 11:00 PM
The third benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.

### Time required
Altough the time required to load the dataset into memory was not accurately measured as we did not do several runs, for this specific run it was `13.639367419s`. This is still an improvement, but I am rather confident that we can do better.

### Memory required
The memory requirements for the dataset were:

```text
1.010 GB 100.00% ⏺: ngrammatic::Corpus<ngrammatic::traits::arity::ArityTwo, ngrammatic::key_transformers::Lower, alloc::string::String, u16>
517.7 MB  51.25% ├╴keys_to_ngrams: std::collections::hash::map::HashMap<ngrammatic::traits::key::Normalizer<alloc::string::String, ngrammatic::key_transformers::Lower>, ngrammatic::ngrams::Ngram<ngrammatic::traits::arity::ArityTwo, u16>>
492.3 MB  48.75% ╰╴ngrams_to_keys: std::collections::hash::map::HashMap<[u8; 2], alloc::vec::Vec<&ngrammatic::traits::key::Normalizer<alloc::string::String, ngrammatic::key_transformers::Lower>>>
```

This is a further improvement compared to the previous run, as it is requires 300MBs less memory.

## Benchmarks 2 April 2024, 10:00 PM
The second benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.

#### Time required
Altough the time required to load the dataset into memory was not accurately measured as we did not do several runs, for this specific run it was `14.457731947s`. This is a significant improvement over the previous run, as it is more than twice as fast.

#### Memory required
The memory requirements for the dataset were:

```text
1.378 GB 100.00% ⏺: ngrammatic::Corpus
886.1 MB  64.28% ├╴keys_to_ngrams: std::collections::hash::map::HashMap<ngrammatic::traits::key::Normalizer<alloc::string::String, ngrammatic::key_transformer::Lower>, ngrammatic::ngrams::Ngram>
492.3 MB  35.72% ╰╴ngrams_to_keys: std::collections::hash::map::HashMap<[u8; 2], alloc::vec::Vec<&ngrammatic::traits::key::Normalizer<alloc::string::String, ngrammatic::key_transformer::Lower>>>
```

This is a significant improvement over the previous run, as it is more than 5 times less memory required.

## Benchmarks 2 April 2024, 09:00 AM
The first benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the entirety of the taxons dataset into memory.

#### Time required
Altough the time required to load the dataset into memory was not accurately measured as we did not do several runs, for this specific run it was `36.779114884s`

#### Memory required
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