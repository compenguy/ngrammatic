# Time benchmarks
The goal of this benchmark is to accurately measure how much time is required by the `ngrammatic` library to load the taxons dataset into memory.
Since cargo bench will run the benchmarks multiple times, we will use only a subset of the rows from the dataset as at the time of writing the library is still exceedingly slow, and we do not want to wait for hours for the benchmarks to finish.

## How to run the benchmarks
To run the time benchmarks, run from the root of the repository the following command:

```bash
cargo bench
```

## Benchmarks 2 April 2024, 09:00 AM
The first benchmark was run on a 6-core machine with 32 GBs of RAM. We loaded the first `5_000` taxons from the dataset into memory.

```text
running 1 test
test build_corpus_2 ... bench: 117,174,622 ns/iter (+/- 21,895,564)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 35.62s
```