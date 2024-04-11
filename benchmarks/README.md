# Memory benchmarks
The goal of this benchmark is to accurately measure how much memory is required by the `ngrammatic` library to load the taxons dataset into memory.
The taxons dataset contains the `2_571_000` taxons from NCBI Taxons. While compressed in gzip, it is a merely 12MBs file.

## How to run the benchmarks
To run the memory benchmarks, navigate to the `benchmarks` directory and run the following command:

```bash
RUST_LOG=info RUSTFLAGS="-C target-cpu=native" cargo run --release
```

## Benchmarks 11 April 2024, 02:00 PM
The ileventh benchmark was run on a 32-core machine (64 threads) with 256 GBs of RAM. We loaded the entirety of the taxons dataset into memory.
The novelty of this benchmark is to use the Vec data structure of ngrams for the initial indexof conversion, and only afterwards compressing it into an Elias-Fano. This has lead to a massive improvement in construction time, while not impacting the memory requirements of the built corpus.

### Arity 1
Edges: 78_571_966, Ngrams: 37

| Operation   | Time (ms)  | Memory (B)   |
|-------------|------------|--------------|
| NEW         | 3,209      | 282,604,340  |
| NEWPAR      | 2,988      | 282,604,340  |
| RCL NEWPAR  | 3,347      | 183,503,767  |
| WEBGRAPH    | 4,500      | 172,145,420  |
| RCL WEBGRAPH| 4,841      | 72,340,935   |
| OLD         | 11,819     | 5,603,963,834|

### Arity 2
Edges: 129_014_720, Ngrams: 1_437

| Operation   | Time (ms)  | Memory (B)   |
|-------------|------------|--------------|
| NEW         | 6,265      | 407,237,104  |
| NEWPAR      | 5,009      | 407,237,104  |
| RCL NEWPAR  | 5,451      | 308,136,531  |
| WEBGRAPH    | 7,965      | 204,590,616  |
| RCL WEBGRAPH| 8,282      | 103,757,115  |
| OLD         | 15,346     | 8,003,769,656|

### Arity 3
Edges: 138_978_258, Ngrams: 47_111

| Operation   | Time (ms)  | Memory (B)   |
|-------------|------------|--------------|
| NEW         | 8,303      | 469,848,532  |
| NEWPAR      | 5,945      | 469,848,532  |
| RCL NEWPAR  | 6,361      | 370,747,959  |
| WEBGRAPH    | 8,514      | 239,354,844  |
| RCL WEBGRAPH| 8,878      | 137,803,367  |
| OLD         | 16,382     | 8,583,476,604|

### Arity 4
Edges: 144_931_790, Ngrams: 47_7806

| Operation   | Time (ms)  | Memory (B)   |
|-------------|------------|--------------|
| NEW         | 11,449     | 512,214,744  |
| NEWPAR      | 7,014      | 512,214,744  |
| RCL NEWPAR  | 7,266      | 413,114,171  |
| WEBGRAPH    | 8,222      | 274,258,752  |
| RCL WEBGRAPH| 8,538      | 172,842,803  |
| OLD         | 18,036     | 9,036,530,407|

### Arity 5
Edges: 150_243_064, Ngrams: 1_982_191

| Operation   | Time (ms)  | Memory (B)   |
|-------------|------------|--------------|
| NEW         | 15,549     | 550,135,192  |
| NEWPAR      | 9,135      | 550,135,192  |
| RCL NEWPAR  | 9,691      | 451,034,619  |
| WEBGRAPH    | 10,444     | 312,453,624  |
| RCL WEBGRAPH| 10,854     | 211,403,363  |
| OLD         | 21,367     | 9,583,720,360|

### Arity 6
Edges: 155_497_150, Ngrams: 4_351_054

| Operation   | Time (ms)  | Memory (B)   |
|-------------|------------|--------------|
| NEW         | 20,219     | 595,148,528  |
| NEWPAR      | 11,230     | 595,148,528  |
| RCL NEWPAR  | 11,962     | 496,047,955  |
| WEBGRAPH    | 12,744     | 355,163,944  |
| RCL WEBGRAPH| 13,219     | 254,433,459  |
| OLD         | 23,004     | 10,211,711,214|

### Arity 7
Edges: 160_731_872, Ngrams: 6_995_796

| Operation   | Time (ms)  | Memory (B)   |
|-------------|------------|--------------|
| NEW         | 27,834     | 626,829,580  |
| NEWPAR      | 14,441     | 626,829,580  |
| RCL NEWPAR  | 15,143     | 527,729,007  |
| WEBGRAPH    | 16,119     | 402,533,060  |
| RCL WEBGRAPH| 16,592     | 301,929,695  |
| OLD         | 26,075     | 11,052,721,209|

### Arity 8
Edges: 165_946_588, Ngrams: 9_979_870

| Operation   | Time (ms)  | Memory (B)   |
|-------------|------------|--------------|
| NEW         | 27,796     | 675,743,136  |
| NEWPAR      | 15,483     | 675,743,136  |
| RCL NEWPAR  | 16,644     | 576,642,563  |
| WEBGRAPH    | 17,418     | 458,193,928  |
| RCL WEBGRAPH| 18,063     | 357,589,579  |
| OLD         | 27,606     | 11,496,992,467|

## Benchmarks 9 April 2024, 06:00 PM
The tenth benchmark was run on a 32-core machine (64 threads) with 256 GBs of RAM. We loaded the entirety of the taxons dataset into memory.
The novelty of this benchmark is the use of a RCL data structure to store the strings associated with the dataset. The savings in memory requirements are significant.

| Test                    | Arity | Time (ms) | Memory (B)    |
|-------------------------|-------|-----------|---------------|
| NEWPAR                  | 1     | 2,977     | 282,604,340   |
| RCL NEWPAR              | 1     | 3,349     | 183,503,767   |
| WEBGRAPH                | 1     | 4,517     | 172,145,420   |
| RCL WEBGRAPH            | 1     | 4,803     | 72,340,935    |
| OLD                     | 1     | 11,650    | 5,603,963,834 |
| NEWPAR                  | 2     | 5,011     | 407,237,104   |
| RCL NEWPAR              | 2     | 5,423     | 308,136,531   |
| WEBGRAPH                | 2     | 7,827     | 204,590,616   |
| RCL WEBGRAPH            | 2     | 8,135     | 103,757,115   |
| OLD                     | 2     | 15,450    | 8,003,769,656 |
| NEWPAR                  | 3     | 6,821     | 469,848,532   |
| RCL NEWPAR              | 3     | 7,155     | 370,747,959   |
| WEBGRAPH                | 3     | 9,358     | 239,354,844   |
| RCL WEBGRAPH            | 3     | 9,554     | 137,803,367   |
| OLD                     | 3     | 16,051    | 8,583,476,604 |
| NEWPAR                  | 4     | 14,487    | 512,214,744   |
| RCL NEWPAR              | 4     | 14,836    | 413,114,171   |
| WEBGRAPH                | 4     | 15,645    | 274,258,752   |
| RCL WEBGRAPH            | 4     | 16,061    | 172,842,803   |
| OLD                     | 4     | 18,326    | 9,036,530,407 |
| NEWPAR                  | 5     | 40,051    | 550,135,192   |
| RCL NEWPAR              | 5     | 40,956    | 451,034,619   |
| WEBGRAPH                | 5     | 41,663    | 312,453,624   |
| RCL WEBGRAPH            | 5     | 42,173    | 211,403,363   |
| OLD                     | 5     | 20,883    | 9,583,720,360 |
| NEWPAR                  | 6     | 118,058   | 595,148,528   |
| RCL NEWPAR              | 6     | 120,697   | 496,047,955   |
| WEBGRAPH                | 6     | 119,848   | 355,163,944   |
| RCL WEBGRAPH            | 6     | 122,529   | 254,433,459   |
| OLD                     | 6     | 22,219    | 10,211,711,214|
| NEWPAR                  | 7     | 145,731   | 626,829,580   |
| RCL NEWPAR              | 7     | 150,171   | 527,729,007   |
| WEBGRAPH                | 7     | 147,696   | 402,533,060   |
| RCL WEBGRAPH            | 7     | 152,126   | 301,929,695   |
| OLD                     | 7     | 27,087    | 11,052,721,209|
| NEWPAR                  | 8     | 260,231   | 675,743,136   |
| RCL NEWPAR              | 8     | 266,866   | 576,642,563   |
| WEBGRAPH                | 8     | 262,128   | 458,193,928   |
| RCL WEBGRAPH            | 8     | 265,629   | 357,589,579   |
| OLD                     | 8     | 26,739    | 11,496,992,467|

## Benchmarks 9 April 2024, 04:00 PM
The ninth benchmark was run on a 32-core machine (64 threads) with 256 GBs of RAM. We loaded the entirety of the taxons dataset into memory.
The novelty of this benchmark is the introduction of the Webgraph datastructure to store the graph itself. **At this time the MemSize trait is not available in the published version of webgraph, so this is solely obtained by using a nightly version - it should be available in the public version soon.**

There is a significant reduction in memory requirements for the version which uses webgraph. 

| Test    | Arity | Time (ms) | Memory (B)    | Memory Graph (B) |
|---------|-------|-----------|---------------|------------------|
| NEWPAR  | 1     | 3,005     | 282,604,340   | 154,219,212      |
| WEBGRAPH| 1     | 4,613     | 172,145,376   | 43,760,288       |
| OLD     | 1     | 11,757    | 5,603,963,834| -                |
| NEWPAR  | 2     | 5,053     | 407,237,104   | 278,850,808      |
| WEBGRAPH| 2     | 7,936     | 204,590,560   | 76,204,304       |
| OLD     | 2     | 15,033    | 8,003,769,656| -                |
| NEWPAR  | 3     | 6,733     | 469,848,532   | 341,406,636      |
| WEBGRAPH| 3     | 9,328     | 239,354,272   | 110,912,416      |
| OLD     | 3     | 16,011    | 8,583,476,604| -                |
| NEWPAR  | 4     | 14,349    | 512,214,744   | 382,971,848      |
| WEBGRAPH| 4     | 15,666    | 274,258,696   | 145,015,840      |
| OLD     | 4     | 17,557    | 9,036,530,407| -                |
| NEWPAR  | 5     | 40,938    | 550,135,192   | 416,714,136      |
| WEBGRAPH| 5     | 41,779    | 312,453,544   | 179,032,528      |
| OLD     | 5     | 19,575    | 9,583,720,360| -                |
| NEWPAR  | 6     | 117,498   | 595,148,528   | 451,993,064      |
| WEBGRAPH| 6     | 119,641   | 355,163,860   | 212,008,444      |
| OLD     | 6     | 22,782    | 10,211,711,214| -                |
| NEWPAR  | 7     | 145,084   | 626,829,580   | 468,310,228      |
| WEBGRAPH| 7     | 147,303   | 402,532,964   | 244,013,660      |
| OLD     | 7     | 27,476    | 11,052,721,209| -                |
| NEWPAR  | 8     | 258,349   | 675,743,136   | 495,062,184      |
| WEBGRAPH| 8     | 260,305   | 458,193,816   | 277,512,928      |
| OLD     | 8     | 26,892    | 11,496,992,467| -                |

## Benchmarks 9 April 2024, 09:00 AM
The eighth benchmark was run on a 32-core machine (64 threads) with 256 GBs of RAM. We loaded the entirety of the taxons dataset into memory.
The novelty of this benchmark is the introduction of a new datastructure for the weights, which is now similar to how a Webgraph is stored.

We observe, in average a reduction of memory requirements of about `10MBs` x arity. Also, the time requirements are reduced, expecially for larger arities.

| Test    | Arity | Time (ms) | Memory (B)    |
|---------|-------|-----------|---------------|
| NEW     | 1     | 3,256     | 282,604,340   |
| NEWPAR  | 1     | 2,958     | 282,604,340   |
| OLD     | 1     | 11,624    | 5,603,963,834 |
| NEW     | 2     | 6,989     | 407,237,104   |
| NEWPAR  | 2     | 5,098     | 407,237,104   |
| OLD     | 2     | 15,231    | 8,003,769,656 |
| NEW     | 3     | 32,827    | 469,848,532   |
| NEWPAR  | 3     | 6,937     | 469,848,532   |
| OLD     | 3     | 16,480    | 8,583,476,604 |
| NEW     | 4     | 229,491   | 512,214,744   |
| NEWPAR  | 4     | 14,390    | 512,214,744   |
| OLD     | 4     | 17,782    | 9,036,530,407 |
| NEW     | 5     | 910,467   | 550,135,192   |
| NEWPAR  | 5     | 40,371    | 550,135,192   |
| OLD     | 5     | 19,549    | 9,583,720,360 |
| NEW     | 6     | 2,953,288 | 595,148,528   |
| NEWPAR  | 6     | 118,846   | 595,148,528   |
| OLD     | 6     | 20,655    | 10,211,711,214|
| NEW     | 7     | 3,650,896 | 626,829,580   |
| NEWPAR  | 7     | 147,647   | 626,829,580   |
| OLD     | 7     | 23,734    | 11,052,721,209|
| NEW     | 8     | 6,733,734 | 675,743,136   |
| NEWPAR  | 8     | 256,439   | 675,743,136   |
| OLD     | 8     | 26,134    | 11,496,992,467|

## Benchmarks 8 April 2024, 08:00 AM
The seventh benchmark was run on a 32-core machine (64 threads) with 256 GBs of RAM. We loaded the entirety of the taxons dataset into memory.
In this benchmark, we are comparing the time and memory required to load the dataset into memory using the old and new implementations of the `Corpus` struct, with arities from 1 to 6.

While the new edition is for arities 1 and 2 faster than the old one, for larger arities it becomes increasingly slower. Still, for all arities, the new edition is using significantly less memory than the old one. This is a significant improvement, as it allows us to scale to much larger dictionaries.

In the new edition we also provide a parallel version, which has the same memory requirements as the non-parallel version, but is significantly faster.

| Test    | Arity | Time (ms) | Memory (B)    |
|---------|-------|-----------|---------------|
| NEW     | 1     | 3,201     | 292,440,192   |
| NEWPAR  | 1     | 2,862     | 292,440,192   |
| OLD     | 1     | 11,870    | 5,603,963,834 |
| NEW     | 2     | 7,113     | 428,947,776   |
| NEWPAR  | 2     | 5,173     | 428,947,776   |
| OLD     | 2     | 15,583    | 8,003,769,656 |
| NEW     | 3     | 39,766    | 486,899,488   |
| NEWPAR  | 3     | 7,314     | 486,899,488   |
| OLD     | 3     | 16,554    | 8,583,476,604 |
| NEW     | 4     | 315,398   | 530,646,488   |
| NEWPAR  | 4     | 17,582    | 530,646,488   |
| OLD     | 4     | 18,561    | 9,036,530,407 |
| NEW     | 5     | 1,194,200 | 569,522,048   |
| NEWPAR  | 5     | 52,986    | 569,522,048   |
| OLD     | 5     | 20,336    | 9,583,720,360 |
| NEW     | 6     | 3,893,922 | 615,458,920   |
| NEWPAR  | 6     | 163,489   | 615,458,920   |
| OLD     | 6     | 22,206    | 10,211,711,214|

## Benchmarks 5 April 2024, 08:00 PM
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