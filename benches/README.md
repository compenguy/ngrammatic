# Time benchmarks
The goal of this benchmark is to accurately measure how much time is required by the `ngrammatic` library to load the taxons dataset into memory.
Since cargo bench will run the benchmarks multiple times, we will use only a subset of the rows from the dataset as at the time of writing the library is still exceedingly slow, and we do not want to wait for hours for the benchmarks to finish.

## How to run the benchmarks
To run the time benchmarks, run from the root of the repository the following command:

```bash
RUST_LOG=info RUSTFLAGS="-C target-cpu=native" cargo bench
```

## Benchmarks 7 April 2024, 09:00 PM
The third benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. Overall, this machine is significantly more powerful than the previous ones, so avoid comparing these results with the previous ones. Solely compare `*_new`, which is the new implementation, and the `*_par_new`, which is the new concurrent implementation, with `*_old`, which is the old implementation as available on crate (version `0.4.0`), both runned on the same machine.

First, we run the benchmarks relative to loading the first `5_000` taxons from the dataset into memory.

```bash
RUST_LOG=info RUSTFLAGS="-C target-cpu=native" cargo bench --bench build_corpus
```

```text
running 24 tests
test build_corpus_bigram_new        ... bench:  30,896,888 ns/iter (+/- 398,108)
test build_corpus_bigram_old        ... bench:  77,941,909 ns/iter (+/- 3,505,171)
test build_corpus_bigram_par_new    ... bench:  26,432,127 ns/iter (+/- 1,946,359)
test build_corpus_heptagram_new     ... bench: 272,009,743 ns/iter (+/- 607,368)
test build_corpus_heptagram_old     ... bench: 118,533,641 ns/iter (+/- 1,728,095)
test build_corpus_heptagram_par_new ... bench:  56,744,864 ns/iter (+/- 1,476,456)
test build_corpus_hexagram_new      ... bench: 236,410,291 ns/iter (+/- 627,052)
test build_corpus_hexagram_old      ... bench: 111,394,763 ns/iter (+/- 1,421,418)
test build_corpus_hexagram_par_new  ... bench:  52,687,949 ns/iter (+/- 1,497,216)
test build_corpus_monogram_new      ... bench:  12,722,442 ns/iter (+/- 55,331)
test build_corpus_monogram_old      ... bench:  46,011,731 ns/iter (+/- 607,534)
test build_corpus_monogram_par_new  ... bench:  13,875,319 ns/iter (+/- 1,238,036)
test build_corpus_octagram_new      ... bench: 366,530,006 ns/iter (+/- 740,843)
test build_corpus_octagram_old      ... bench: 130,562,743 ns/iter (+/- 1,229,807)
test build_corpus_octagram_par_new  ... bench:  64,729,705 ns/iter (+/- 3,015,636)
test build_corpus_pentagram_new     ... bench: 197,060,504 ns/iter (+/- 364,804)
test build_corpus_pentagram_old     ... bench: 103,242,243 ns/iter (+/- 2,812,735)
test build_corpus_pentagram_par_new ... bench:  47,142,296 ns/iter (+/- 1,742,570)
test build_corpus_tetragram_new     ... bench: 119,104,440 ns/iter (+/- 804,896)
test build_corpus_tetragram_old     ... bench:  98,156,298 ns/iter (+/- 3,034,541)
test build_corpus_tetragram_par_new ... bench:  38,689,265 ns/iter (+/- 1,123,753)
test build_corpus_trigram_new       ... bench:  68,718,691 ns/iter (+/- 928,889)
test build_corpus_trigram_old       ... bench:  92,043,546 ns/iter (+/- 2,601,382)
test build_corpus_trigram_par_new   ... bench:  33,355,542 ns/iter (+/- 776,818)

test result: ok. 0 passed; 0 failed; 0 ignored; 24 measured; 0 filtered out; finished in 729.70s
```

Secondarily, we run the benchmarks relative to searching taxons across the first `100_000` taxons from the dataset.

```bash
RUST_LOG=info RUSTFLAGS="-C target-cpu=native" cargo bench --bench running_ngram_search
```

```text
running 24 tests
test ngram_search_corpus_bigram_new        ... bench:   7,497,412 ns/iter (+/- 68,003)
test ngram_search_corpus_bigram_old        ... bench:  99,955,280 ns/iter (+/- 8,075,258)
test ngram_search_corpus_bigram_par_new    ... bench:   4,977,195 ns/iter (+/- 815,739)
test ngram_search_corpus_heptagram_new     ... bench:   2,873,109 ns/iter (+/- 13,342)
test ngram_search_corpus_heptagram_old     ... bench:  16,461,192 ns/iter (+/- 165,196)
test ngram_search_corpus_heptagram_par_new ... bench:   1,643,750 ns/iter (+/- 151,236)
test ngram_search_corpus_hexagram_new      ... bench:   3,164,364 ns/iter (+/- 15,709)
test ngram_search_corpus_hexagram_old      ... bench:  14,526,763 ns/iter (+/- 176,189)
test ngram_search_corpus_hexagram_par_new  ... bench:   3,463,607 ns/iter (+/- 871,659)
test ngram_search_corpus_monogram_new      ... bench:         477 ns/iter (+/- 8)
test ngram_search_corpus_monogram_old      ... bench: 326,974,147 ns/iter (+/- 3,933,313)
test ngram_search_corpus_monogram_par_new  ... bench:      53,081 ns/iter (+/- 4,474)
test ngram_search_corpus_octagram_new      ... bench:   2,925,086 ns/iter (+/- 12,178)
test ngram_search_corpus_octagram_old      ... bench:  19,291,253 ns/iter (+/- 112,943)
test ngram_search_corpus_octagram_par_new  ... bench:   3,344,486 ns/iter (+/- 560,629)
test ngram_search_corpus_pentagram_new     ... bench:   3,437,322 ns/iter (+/- 40,099)
test ngram_search_corpus_pentagram_old     ... bench:  13,925,100 ns/iter (+/- 138,846)
test ngram_search_corpus_pentagram_par_new ... bench:   4,533,541 ns/iter (+/- 417,050)
test ngram_search_corpus_tetragram_new     ... bench:   3,742,860 ns/iter (+/- 19,222)
test ngram_search_corpus_tetragram_old     ... bench:  16,754,572 ns/iter (+/- 686,706)
test ngram_search_corpus_tetragram_par_new ... bench:   4,401,330 ns/iter (+/- 515,740)
test ngram_search_corpus_trigram_new       ... bench:   5,435,892 ns/iter (+/- 65,594)
test ngram_search_corpus_trigram_old       ... bench:  32,010,687 ns/iter (+/- 814,910)
test ngram_search_corpus_trigram_par_new   ... bench:   3,717,600 ns/iter (+/- 574,098)

test result: ok. 0 passed; 0 failed; 0 ignored; 24 measured; 0 filtered out; finished in 289.84s
```

## Benchmarks 2 April 2024, 10:00 PM
The second benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the first `5_000` taxons from the dataset into memory.

```text
running 1 test
test build_corpus_2 ... bench:  34,727,715 ns/iter (+/- 14,040,255)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 10.89s
```

The improvement is significant, and the library is now 3.38 times faster than before.

## Benchmarks 2 April 2024, 09:00 AM
The first benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the first `5_000` taxons from the dataset into memory.

```text
running 1 test
test build_corpus_2 ... bench: 117,174,622 ns/iter (+/- 21,895,564)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 35.62s
```