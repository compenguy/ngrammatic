# Time benchmarks
The goal of this benchmark is to accurately measure how much time is required by the `ngrammatic` library to load the taxons dataset into memory.
Since cargo bench will run the benchmarks multiple times, we will use only a subset of the rows from the dataset as at the time of writing the library is still exceedingly slow, and we do not want to wait for hours for the benchmarks to finish.

## How to run the benchmarks
To run the time benchmarks, run from the root of the repository the following command:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench
```

## Benchmarks 9 April 2024, 06:00 PM
The sixth benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. In this iteration, we run the benchmarks relative to loading the first `10_000` taxons from the dataset into memory. The novelty of this benchmark is the use of the RCL data structure for holding the strings of the dataset itself.

Compared to the previouusly used data structure, a simple vector, the improvement in memory requirements is massive. Also, it does not seem to impact too significantly the time required to perform the key operations of the library.

```text
running 136 tests
test bigram_bitvec_par_search_rcl      ... bench:  14,158,963 ns/iter (+/- 4,382,217)
test bigram_bitvec_par_search_vec      ... bench:  14,283,047 ns/iter (+/- 2,872,322)
test bigram_bitvec_seq_search_rcl      ... bench:  23,572,426 ns/iter (+/- 100,103)
test bigram_bitvec_seq_search_vec      ... bench:  23,765,254 ns/iter (+/- 85,246)
test bigram_old_seq_search             ... bench: 187,365,001 ns/iter (+/- 1,453,912)
test bigram_tfidf_bitvec_par_rcl       ... bench:  27,032,078 ns/iter (+/- 2,165,998)
test bigram_tfidf_bitvec_par_vec       ... bench:  24,042,987 ns/iter (+/- 2,238,973)
test bigram_tfidf_bitvec_seq_rcl       ... bench:  40,213,882 ns/iter (+/- 1,741,292)
test bigram_tfidf_bitvec_seq_vec       ... bench:  34,101,151 ns/iter (+/- 79,071)
test bigram_tfidf_webgraph_par_rcl     ... bench:  80,790,835 ns/iter (+/- 14,922,114)
test bigram_tfidf_webgraph_par_vec     ... bench:  78,238,592 ns/iter (+/- 13,683,586)
test bigram_tfidf_webgraph_seq_rcl     ... bench: 170,019,399 ns/iter (+/- 971,670)
test bigram_tfidf_webgraph_seq_vec     ... bench: 158,185,323 ns/iter (+/- 1,109,479)
test bigram_webgraph_par_search_rcl    ... bench:  56,825,438 ns/iter (+/- 11,179,772)
test bigram_webgraph_par_search_vec    ... bench:  54,109,527 ns/iter (+/- 12,707,143)
test bigram_webgraph_seq_search_rcl    ... bench: 124,851,353 ns/iter (+/- 1,990,438)
test bigram_webgraph_seq_search_vec    ... bench: 121,921,455 ns/iter (+/- 2,343,263)
test heptagram_bitvec_par_search_rcl   ... bench:  10,736,349 ns/iter (+/- 2,249,504)
test heptagram_bitvec_par_search_vec   ... bench:  11,307,423 ns/iter (+/- 2,524,018)
test heptagram_bitvec_seq_search_rcl   ... bench:   8,445,303 ns/iter (+/- 35,293)
test heptagram_bitvec_seq_search_vec   ... bench:   8,630,346 ns/iter (+/- 28,891)
test heptagram_old_seq_search          ... bench:  25,881,998 ns/iter (+/- 232,908)
test heptagram_tfidf_bitvec_par_rcl    ... bench:  20,418,717 ns/iter (+/- 2,115,452)
test heptagram_tfidf_bitvec_par_vec    ... bench:  28,825,448 ns/iter (+/- 4,037,268)
test heptagram_tfidf_bitvec_seq_rcl    ... bench:  19,649,438 ns/iter (+/- 54,359)
test heptagram_tfidf_bitvec_seq_vec    ... bench:  16,837,797 ns/iter (+/- 59,656)
test heptagram_tfidf_webgraph_par_rcl  ... bench:  87,730,932 ns/iter (+/- 34,172,262)
test heptagram_tfidf_webgraph_par_vec  ... bench:  74,167,475 ns/iter (+/- 29,023,645)
test heptagram_tfidf_webgraph_seq_rcl  ... bench:  78,161,557 ns/iter (+/- 450,187)
test heptagram_tfidf_webgraph_seq_vec  ... bench:  73,324,960 ns/iter (+/- 547,239)
test heptagram_webgraph_par_search_rcl ... bench:  51,301,970 ns/iter (+/- 20,526,802)
test heptagram_webgraph_par_search_vec ... bench:  50,647,931 ns/iter (+/- 16,538,200)
test heptagram_webgraph_seq_search_rcl ... bench:  54,258,315 ns/iter (+/- 1,333,734)
test heptagram_webgraph_seq_search_vec ... bench:  52,708,023 ns/iter (+/- 312,741)
test hexagram_bitvec_par_search_rcl    ... bench:  12,325,614 ns/iter (+/- 2,104,040)
test hexagram_bitvec_par_search_vec    ... bench:  12,095,099 ns/iter (+/- 2,643,998)
test hexagram_bitvec_seq_search_rcl    ... bench:   8,933,519 ns/iter (+/- 28,469)
test hexagram_bitvec_seq_search_vec    ... bench:   9,065,280 ns/iter (+/- 23,255)
test hexagram_old_seq_search           ... bench:  22,449,601 ns/iter (+/- 203,832)
test hexagram_tfidf_bitvec_par_rcl     ... bench:  20,665,191 ns/iter (+/- 8,661,905)
test hexagram_tfidf_bitvec_par_vec     ... bench:  24,392,885 ns/iter (+/- 2,454,376)
test hexagram_tfidf_bitvec_seq_rcl     ... bench:  17,497,485 ns/iter (+/- 45,755)
test hexagram_tfidf_bitvec_seq_vec     ... bench:  15,082,711 ns/iter (+/- 38,812)
test hexagram_tfidf_webgraph_par_rcl   ... bench:  82,825,894 ns/iter (+/- 22,041,872)
test hexagram_tfidf_webgraph_par_vec   ... bench:  84,492,459 ns/iter (+/- 31,916,772)
test hexagram_tfidf_webgraph_seq_rcl   ... bench:  77,316,499 ns/iter (+/- 80,025)
test hexagram_tfidf_webgraph_seq_vec   ... bench:  73,777,869 ns/iter (+/- 97,729)
test hexagram_webgraph_par_search_rcl  ... bench:  59,279,722 ns/iter (+/- 17,028,813)
test hexagram_webgraph_par_search_vec  ... bench:  56,754,418 ns/iter (+/- 14,364,301)
test hexagram_webgraph_seq_search_rcl  ... bench:  54,877,503 ns/iter (+/- 194,789)
test hexagram_webgraph_seq_search_vec  ... bench:  52,242,901 ns/iter (+/- 227,578)
test monogram_bitvec_par_search_rcl    ... bench:     164,110 ns/iter (+/- 8,359)
test monogram_bitvec_par_search_vec    ... bench:     166,880 ns/iter (+/- 10,705)
test monogram_bitvec_seq_search_rcl    ... bench:       1,496 ns/iter (+/- 28)
test monogram_bitvec_seq_search_vec    ... bench:       1,492 ns/iter (+/- 30)
test monogram_old_seq_search           ... bench: 617,888,122 ns/iter (+/- 3,353,254)
test monogram_tfidf_bitvec_par_rcl     ... bench:     164,078 ns/iter (+/- 8,445)
test monogram_tfidf_bitvec_par_vec     ... bench:     163,612 ns/iter (+/- 8,678)
test monogram_tfidf_bitvec_seq_rcl     ... bench:       1,575 ns/iter (+/- 95)
test monogram_tfidf_bitvec_seq_vec     ... bench:       1,486 ns/iter (+/- 42)
test monogram_tfidf_webgraph_par_rcl   ... bench:     160,863 ns/iter (+/- 9,815)
test monogram_tfidf_webgraph_par_vec   ... bench:     160,739 ns/iter (+/- 18,404)
test monogram_tfidf_webgraph_seq_rcl   ... bench:       3,029 ns/iter (+/- 210)
test monogram_tfidf_webgraph_seq_vec   ... bench:       2,904 ns/iter (+/- 37)
test monogram_webgraph_par_search_rcl  ... bench:     161,719 ns/iter (+/- 28,493)
test monogram_webgraph_par_search_vec  ... bench:     157,261 ns/iter (+/- 8,684)
test monogram_webgraph_seq_search_rcl  ... bench:       3,026 ns/iter (+/- 291)
test monogram_webgraph_seq_search_vec  ... bench:       2,872 ns/iter (+/- 53)
test octagram_bitvec_par_search_rcl    ... bench:   8,772,018 ns/iter (+/- 4,912,144)
test octagram_bitvec_par_search_vec    ... bench:  10,589,881 ns/iter (+/- 3,035,243)
test octagram_bitvec_seq_search_rcl    ... bench:   8,460,512 ns/iter (+/- 74,560)
test octagram_bitvec_seq_search_vec    ... bench:   8,771,734 ns/iter (+/- 94,194)
test octagram_old_seq_search           ... bench:  27,321,061 ns/iter (+/- 254,917)
test octagram_tfidf_bitvec_par_rcl     ... bench:  17,778,826 ns/iter (+/- 6,392,077)
test octagram_tfidf_bitvec_par_vec     ... bench:  19,980,522 ns/iter (+/- 6,520,545)
test octagram_tfidf_bitvec_seq_rcl     ... bench:  19,357,493 ns/iter (+/- 48,306)
test octagram_tfidf_bitvec_seq_vec     ... bench:  16,973,596 ns/iter (+/- 46,473)
test octagram_tfidf_webgraph_par_rcl   ... bench:  80,305,030 ns/iter (+/- 24,177,119)
test octagram_tfidf_webgraph_par_vec   ... bench:  77,648,040 ns/iter (+/- 30,857,219)
test octagram_tfidf_webgraph_seq_rcl   ... bench:  79,402,699 ns/iter (+/- 610,597)
test octagram_tfidf_webgraph_seq_vec   ... bench:  73,464,327 ns/iter (+/- 146,384)
test octagram_webgraph_par_search_rcl  ... bench:  55,291,116 ns/iter (+/- 21,627,121)
test octagram_webgraph_par_search_vec  ... bench:  51,256,769 ns/iter (+/- 16,806,643)
test octagram_webgraph_seq_search_rcl  ... bench:  55,277,968 ns/iter (+/- 1,299,991)
test octagram_webgraph_seq_search_vec  ... bench:  53,852,872 ns/iter (+/- 134,304)
test pentagram_bitvec_par_search_rcl   ... bench:  14,081,699 ns/iter (+/- 1,141,672)
test pentagram_bitvec_par_search_vec   ... bench:  14,345,128 ns/iter (+/- 3,746,306)
test pentagram_bitvec_seq_search_rcl   ... bench:   9,410,065 ns/iter (+/- 53,170)
test pentagram_bitvec_seq_search_vec   ... bench:   9,639,321 ns/iter (+/- 28,633)
test pentagram_old_seq_search          ... bench:  22,056,607 ns/iter (+/- 169,907)
test pentagram_tfidf_bitvec_par_rcl    ... bench:  28,571,099 ns/iter (+/- 2,566,422)
test pentagram_tfidf_bitvec_par_vec    ... bench:  25,248,974 ns/iter (+/- 3,738,167)
test pentagram_tfidf_bitvec_seq_rcl    ... bench:  18,308,269 ns/iter (+/- 49,531)
test pentagram_tfidf_bitvec_seq_vec    ... bench:  15,572,161 ns/iter (+/- 70,033)
test pentagram_tfidf_webgraph_par_rcl  ... bench:  99,301,417 ns/iter (+/- 18,097,331)
test pentagram_tfidf_webgraph_par_vec  ... bench:  99,229,827 ns/iter (+/- 10,387,806)
test pentagram_tfidf_webgraph_seq_rcl  ... bench:  77,753,268 ns/iter (+/- 90,537)
test pentagram_tfidf_webgraph_seq_vec  ... bench:  73,554,579 ns/iter (+/- 548,396)
test pentagram_webgraph_par_search_rcl ... bench:  75,750,568 ns/iter (+/- 11,088,086)
test pentagram_webgraph_par_search_vec ... bench:  70,994,717 ns/iter (+/- 8,623,301)
test pentagram_webgraph_seq_search_rcl ... bench:  54,427,636 ns/iter (+/- 1,155,450)
test pentagram_webgraph_seq_search_vec ... bench:  52,534,890 ns/iter (+/- 99,501)
test tetragram_bitvec_par_search_rcl   ... bench:  12,003,433 ns/iter (+/- 4,718,873)
test tetragram_bitvec_par_search_vec   ... bench:  13,415,564 ns/iter (+/- 4,158,818)
test tetragram_bitvec_seq_search_rcl   ... bench:  10,090,587 ns/iter (+/- 129,811)
test tetragram_bitvec_seq_search_vec   ... bench:  10,344,738 ns/iter (+/- 52,067)
test tetragram_old_seq_search          ... bench:  24,920,493 ns/iter (+/- 305,297)
test tetragram_tfidf_bitvec_par_rcl    ... bench:  24,946,904 ns/iter (+/- 5,300,785)
test tetragram_tfidf_bitvec_par_vec    ... bench:  22,519,442 ns/iter (+/- 3,133,970)
test tetragram_tfidf_bitvec_seq_rcl    ... bench:  18,379,084 ns/iter (+/- 92,276)
test tetragram_tfidf_bitvec_seq_vec    ... bench:  15,571,167 ns/iter (+/- 72,983)
test tetragram_tfidf_webgraph_par_rcl  ... bench: 104,012,794 ns/iter (+/- 13,122,154)
test tetragram_tfidf_webgraph_par_vec  ... bench:  96,789,597 ns/iter (+/- 15,322,571)
test tetragram_tfidf_webgraph_seq_rcl  ... bench:  82,299,372 ns/iter (+/- 228,436)
test tetragram_tfidf_webgraph_seq_vec  ... bench:  78,087,686 ns/iter (+/- 268,301)
test tetragram_webgraph_par_search_rcl ... bench:  74,530,232 ns/iter (+/- 10,537,080)
test tetragram_webgraph_par_search_vec ... bench:  69,175,308 ns/iter (+/- 9,681,547)
test tetragram_webgraph_seq_search_rcl ... bench:  57,230,776 ns/iter (+/- 103,174)
test tetragram_webgraph_seq_search_vec ... bench:  55,150,709 ns/iter (+/- 91,656)
test trigram_bitvec_par_search_rcl     ... bench:  15,892,127 ns/iter (+/- 2,961,082)
test trigram_bitvec_par_search_vec     ... bench:  14,619,355 ns/iter (+/- 3,775,417)
test trigram_bitvec_seq_search_rcl     ... bench:  20,725,798 ns/iter (+/- 70,959)
test trigram_bitvec_seq_search_vec     ... bench:  21,020,933 ns/iter (+/- 54,712)
test trigram_old_seq_search            ... bench:  63,907,148 ns/iter (+/- 855,000)
test trigram_tfidf_bitvec_par_rcl      ... bench:  28,114,472 ns/iter (+/- 4,780,672)
test trigram_tfidf_bitvec_par_vec      ... bench:  24,558,503 ns/iter (+/- 1,988,821)
test trigram_tfidf_bitvec_seq_rcl      ... bench:  35,802,803 ns/iter (+/- 83,510)
test trigram_tfidf_bitvec_seq_vec      ... bench:  29,405,303 ns/iter (+/- 61,802)
test trigram_tfidf_webgraph_par_rcl    ... bench: 102,373,931 ns/iter (+/- 12,068,697)
test trigram_tfidf_webgraph_par_vec    ... bench:  97,639,416 ns/iter (+/- 14,051,164)
test trigram_tfidf_webgraph_seq_rcl    ... bench: 160,022,083 ns/iter (+/- 801,983)
test trigram_tfidf_webgraph_seq_vec    ... bench: 150,471,471 ns/iter (+/- 180,682)
test trigram_webgraph_par_search_rcl   ... bench:  72,191,176 ns/iter (+/- 9,371,052)
test trigram_webgraph_par_search_vec   ... bench:  70,775,431 ns/iter (+/- 8,944,474)
test trigram_webgraph_seq_search_rcl   ... bench: 112,489,656 ns/iter (+/- 2,381,201)
test trigram_webgraph_seq_search_vec   ... bench: 109,659,835 ns/iter (+/- 251,582)

test result: ok. 0 passed; 0 failed; 0 ignored; 136 measured; 0 filtered out; finished in 2132.41s
```

## Benchmarks 9 April 2024, 01:00 PM
The fifth benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. In this iteration, we run the benchmarks relative to loading the first `10_000` taxons from the dataset into memory. The novelty of this benchmark is the use of the webgraph data structure as one of the possible underlying graph representations, which requires a significantly smaller memory footprint than the default graph representation. 

```text
running 40 tests
test build_corpus_bigram_new                 ... bench:  30,063,497 ns/iter (+/- 192,926)
test build_corpus_bigram_new_webgraph        ... bench:  49,252,438 ns/iter (+/- 1,152,559)
test build_corpus_bigram_old                 ... bench:  79,633,044 ns/iter (+/- 2,555,930)
test build_corpus_bigram_par_new             ... bench:  25,583,916 ns/iter (+/- 757,674)
test build_corpus_bigram_par_new_webgraph    ... bench:  42,800,831 ns/iter (+/- 1,595,224)
test build_corpus_heptagram_new              ... bench: 296,716,962 ns/iter (+/- 886,155)
test build_corpus_heptagram_new_webgraph     ... bench: 322,676,915 ns/iter (+/- 3,706,157)
test build_corpus_heptagram_old              ... bench: 119,413,976 ns/iter (+/- 6,942,323)
test build_corpus_heptagram_par_new          ... bench:  57,760,362 ns/iter (+/- 3,907,160)
test build_corpus_heptagram_par_new_webgraph ... bench:  75,677,778 ns/iter (+/- 2,540,270)
test build_corpus_hexagram_new               ... bench: 256,702,718 ns/iter (+/- 754,501)
test build_corpus_hexagram_new_webgraph      ... bench: 280,953,094 ns/iter (+/- 1,999,904)
test build_corpus_hexagram_old               ... bench: 107,797,245 ns/iter (+/- 5,356,921)
test build_corpus_hexagram_par_new           ... bench:  55,051,893 ns/iter (+/- 1,797,274)
test build_corpus_hexagram_par_new_webgraph  ... bench:  71,713,398 ns/iter (+/- 2,758,198)
test build_corpus_monogram_new               ... bench:  12,736,422 ns/iter (+/- 108,683)
test build_corpus_monogram_new_webgraph      ... bench:  32,293,678 ns/iter (+/- 1,039,797)
test build_corpus_monogram_old               ... bench:  46,012,249 ns/iter (+/- 611,606)
test build_corpus_monogram_par_new           ... bench:  13,943,383 ns/iter (+/- 872,744)
test build_corpus_monogram_par_new_webgraph  ... bench:  32,763,070 ns/iter (+/- 1,146,276)
test build_corpus_octagram_new               ... bench: 417,944,773 ns/iter (+/- 1,408,906)
test build_corpus_octagram_new_webgraph      ... bench: 444,898,880 ns/iter (+/- 7,310,793)
test build_corpus_octagram_old               ... bench: 127,136,529 ns/iter (+/- 2,154,483)
test build_corpus_octagram_par_new           ... bench:  66,019,797 ns/iter (+/- 3,082,655)
test build_corpus_octagram_par_new_webgraph  ... bench:  86,585,359 ns/iter (+/- 3,218,811)
test build_corpus_pentagram_new              ... bench: 221,473,322 ns/iter (+/- 1,298,914)
test build_corpus_pentagram_new_webgraph     ... bench: 245,089,696 ns/iter (+/- 4,771,393)
test build_corpus_pentagram_old              ... bench: 106,523,748 ns/iter (+/- 4,130,615)
test build_corpus_pentagram_par_new          ... bench:  49,113,952 ns/iter (+/- 1,429,054)
test build_corpus_pentagram_par_new_webgraph ... bench:  64,072,344 ns/iter (+/- 1,894,184)
test build_corpus_tetragram_new              ... bench: 131,636,869 ns/iter (+/- 147,802)
test build_corpus_tetragram_new_webgraph     ... bench: 153,685,377 ns/iter (+/- 2,089,815)
test build_corpus_tetragram_old              ... bench:  96,213,548 ns/iter (+/- 3,512,520)
test build_corpus_tetragram_par_new          ... bench:  39,722,521 ns/iter (+/- 897,996)
test build_corpus_tetragram_par_new_webgraph ... bench:  56,044,704 ns/iter (+/- 1,631,917)
test build_corpus_trigram_new                ... bench:  72,245,656 ns/iter (+/- 129,528)
test build_corpus_trigram_new_webgraph       ... bench:  92,708,174 ns/iter (+/- 1,656,958)
test build_corpus_trigram_old                ... bench:  90,953,416 ns/iter (+/- 2,411,579)
test build_corpus_trigram_par_new            ... bench:  33,195,119 ns/iter (+/- 447,306)
test build_corpus_trigram_par_new_webgraph   ... bench:  48,771,236 ns/iter (+/- 1,139,261)

test result: ok. 0 passed; 0 failed; 0 ignored; 40 measured; 0 filtered out; finished in 1407.16s
```

The following is the benchmark over the search operations, including both the ngram search and the tfidf search, across the first `100_000` taxons from the dataset. We limit the search to the first `100_000` taxons to avoid running the benchmarks for hours. We observe that the parallel implementation, on the considered benchmark, is often slower than the sequential one as the query strings are rather small and the overhead of the parallelism is not compensated by the parallel execution.

```text
running 72 tests
test bigram_ngram_search_new                  ... bench:  16,733,818 ns/iter (+/- 84,531)
test bigram_ngram_search_new_webgraph         ... bench:  85,000,471 ns/iter (+/- 329,925)
test bigram_ngram_search_old                  ... bench: 191,041,933 ns/iter (+/- 1,596,474)
test bigram_ngram_search_par_new              ... bench:  10,505,062 ns/iter (+/- 1,384,319)
test bigram_ngram_search_par_new_webgraph     ... bench:  43,883,661 ns/iter (+/- 13,639,106)
test bigram_tfidf_search_new                  ... bench:  25,801,844 ns/iter (+/- 1,197,147)
test bigram_tfidf_search_new_webgraph         ... bench: 116,396,838 ns/iter (+/- 134,291)
test bigram_tfidf_search_par_new              ... bench:  16,068,499 ns/iter (+/- 1,986,958)
test bigram_tfidf_search_par_new_webgraph     ... bench:  58,148,526 ns/iter (+/- 11,688,490)
test heptagram_ngram_search_new               ... bench:   5,678,363 ns/iter (+/- 145,799)
test heptagram_ngram_search_new_webgraph      ... bench:  36,320,955 ns/iter (+/- 346,692)
test heptagram_ngram_search_old               ... bench:  26,031,489 ns/iter (+/- 254,458)
test heptagram_ngram_search_par_new           ... bench:   7,523,810 ns/iter (+/- 1,523,139)
test heptagram_ngram_search_par_new_webgraph  ... bench:  45,321,112 ns/iter (+/- 14,109,003)
test heptagram_tf_idf_search_new              ... bench:  11,513,398 ns/iter (+/- 701,384)
test heptagram_tf_idf_search_new_webgraph     ... bench:  50,987,997 ns/iter (+/- 329,138)
test heptagram_tf_idf_search_par_new          ... bench:  19,044,546 ns/iter (+/- 1,590,498)
test heptagram_tf_idf_search_par_new_webgraph ... bench:  53,514,306 ns/iter (+/- 30,448,600)
test hexagram_ngram_search_new                ... bench:   6,038,707 ns/iter (+/- 157,491)
test hexagram_ngram_search_new_webgraph       ... bench:  36,282,229 ns/iter (+/- 110,640)
test hexagram_ngram_search_old                ... bench:  22,900,736 ns/iter (+/- 244,797)
test hexagram_ngram_search_par_new            ... bench:   9,027,312 ns/iter (+/- 1,136,602)
test hexagram_ngram_search_par_new_webgraph   ... bench:  43,994,350 ns/iter (+/- 13,848,646)
test hexagram_tf_idf_search_new               ... bench:  10,210,663 ns/iter (+/- 48,730)
test hexagram_tf_idf_search_new_webgraph      ... bench:  50,491,341 ns/iter (+/- 124,927)
test hexagram_tf_idf_search_par_new           ... bench:  16,174,063 ns/iter (+/- 1,111,776)
test hexagram_tf_idf_search_par_new_webgraph  ... bench:  61,756,269 ns/iter (+/- 16,559,935)
test monogram_ngram_search_new                ... bench:         973 ns/iter (+/- 16)
test monogram_ngram_search_new_webgraph       ... bench:       2,486 ns/iter (+/- 21)
test monogram_ngram_search_old                ... bench: 691,227,479 ns/iter (+/- 10,317,495)
test monogram_ngram_search_par_new            ... bench:     108,762 ns/iter (+/- 5,751)
test monogram_ngram_search_par_new_webgraph   ... bench:     110,843 ns/iter (+/- 6,612)
test monogram_tfidf_search_new                ... bench:         945 ns/iter (+/- 16)
test monogram_tfidf_search_new_webgraph       ... bench:       2,484 ns/iter (+/- 16)
test monogram_tfidf_search_par_new            ... bench:     105,000 ns/iter (+/- 6,553)
test monogram_tfidf_search_par_new_webgraph   ... bench:     108,017 ns/iter (+/- 6,292)
test octagram_ngram_search_new                ... bench:   5,774,531 ns/iter (+/- 27,291)
test octagram_ngram_search_new_webgraph       ... bench:  36,767,845 ns/iter (+/- 127,595)
test octagram_ngram_search_old                ... bench:  31,533,250 ns/iter (+/- 320,307)
test octagram_ngram_search_par_new            ... bench:   8,198,898 ns/iter (+/- 1,318,197)
test octagram_ngram_search_par_new_webgraph   ... bench:  42,032,366 ns/iter (+/- 14,594,741)
test octagram_tf_idf_search_new               ... bench:  11,578,159 ns/iter (+/- 32,703)
test octagram_tf_idf_search_new_webgraph      ... bench:  51,601,231 ns/iter (+/- 127,108)
test octagram_tf_idf_search_par_new           ... bench:  12,592,889 ns/iter (+/- 2,627,268)
test octagram_tf_idf_search_par_new_webgraph  ... bench:  51,081,519 ns/iter (+/- 23,510,917)
test pentagram_ngram_search_new               ... bench:   6,404,633 ns/iter (+/- 25,578)
test pentagram_ngram_search_new_webgraph      ... bench:  36,406,376 ns/iter (+/- 82,059)
test pentagram_ngram_search_old               ... bench:  22,110,618 ns/iter (+/- 164,017)
test pentagram_ngram_search_par_new           ... bench:   9,612,528 ns/iter (+/- 1,064,053)
test pentagram_ngram_search_par_new_webgraph  ... bench:  50,572,814 ns/iter (+/- 9,078,281)
test pentagram_tf_idf_search_new              ... bench:  10,613,421 ns/iter (+/- 675,242)
test pentagram_tf_idf_search_new_webgraph     ... bench:  50,604,519 ns/iter (+/- 83,511)
test pentagram_tf_idf_search_par_new          ... bench:  16,342,316 ns/iter (+/- 1,273,526)
test pentagram_tf_idf_search_par_new_webgraph ... bench:  67,601,365 ns/iter (+/- 5,575,331)
test tetragram_ngram_search_new               ... bench:   6,892,730 ns/iter (+/- 86,218)
test tetragram_ngram_search_new_webgraph      ... bench:  38,824,290 ns/iter (+/- 635,002)
test tetragram_ngram_search_old               ... bench:  24,787,158 ns/iter (+/- 674,009)
test tetragram_ngram_search_par_new           ... bench:   9,756,342 ns/iter (+/- 1,392,526)
test tetragram_ngram_search_par_new_webgraph  ... bench:  49,722,566 ns/iter (+/- 8,288,464)
test tetragram_tf_idf_search_new              ... bench:  10,496,280 ns/iter (+/- 73,933)
test tetragram_tf_idf_search_new_webgraph     ... bench:  54,398,647 ns/iter (+/- 293,948)
test tetragram_tf_idf_search_par_new          ... bench:  14,993,612 ns/iter (+/- 1,203,980)
test tetragram_tf_idf_search_par_new_webgraph ... bench:  65,550,633 ns/iter (+/- 9,234,801)
test trigram_ngram_search_new                 ... bench:  14,195,566 ns/iter (+/- 92,455)
test trigram_ngram_search_new_webgraph        ... bench:  77,354,182 ns/iter (+/- 240,438)
test trigram_ngram_search_old                 ... bench:  67,413,608 ns/iter (+/- 12,128,538)
test trigram_ngram_search_par_new             ... bench:  11,202,064 ns/iter (+/- 2,082,499)
test trigram_ngram_search_par_new_webgraph    ... bench:  54,546,009 ns/iter (+/- 5,139,539)
test trigram_tf_idf_search_new                ... bench:  20,400,317 ns/iter (+/- 1,243,061)
test trigram_tf_idf_search_new_webgraph       ... bench: 107,031,641 ns/iter (+/- 1,650,785)
test trigram_tf_idf_search_par_new            ... bench:  16,566,241 ns/iter (+/- 2,359,137)
test trigram_tf_idf_search_par_new_webgraph   ... bench:  70,078,550 ns/iter (+/- 10,145,876)

test result: ok. 0 passed; 0 failed; 0 ignored; 72 measured; 0 filtered out; finished in 1005.11s
```

## Benchmarks 9 April 2024, 10:00 AM
The fourth benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. In this iteration, we run the benchmarks relative to loading the first `10_000` taxons from the dataset into memory. 

```text
running 24 tests
test build_corpus_bigram_new        ... bench:  30,027,421 ns/iter (+/- 169,793)
test build_corpus_bigram_old        ... bench:  75,731,669 ns/iter (+/- 1,092,273)
test build_corpus_bigram_par_new    ... bench:  25,305,966 ns/iter (+/- 2,203,643)
test build_corpus_heptagram_new     ... bench: 262,093,324 ns/iter (+/- 2,863,954)
test build_corpus_heptagram_old     ... bench: 121,937,447 ns/iter (+/- 2,090,908)
test build_corpus_heptagram_par_new ... bench:  56,524,373 ns/iter (+/- 1,672,922)
test build_corpus_hexagram_new      ... bench: 221,587,860 ns/iter (+/- 1,865,118)
test build_corpus_hexagram_old      ... bench: 106,219,509 ns/iter (+/- 4,934,105)
test build_corpus_hexagram_par_new  ... bench:  52,091,040 ns/iter (+/- 1,349,573)
test build_corpus_monogram_new      ... bench:  12,594,027 ns/iter (+/- 62,096)
test build_corpus_monogram_old      ... bench:  45,972,180 ns/iter (+/- 322,730)
test build_corpus_monogram_par_new  ... bench:  13,947,290 ns/iter (+/- 915,101)
test build_corpus_octagram_new      ... bench: 353,140,027 ns/iter (+/- 1,275,524)
test build_corpus_octagram_old      ... bench: 125,616,055 ns/iter (+/- 4,814,695)
test build_corpus_octagram_par_new  ... bench:  63,328,865 ns/iter (+/- 3,103,435)
test build_corpus_pentagram_new     ... bench: 190,646,762 ns/iter (+/- 429,277)
test build_corpus_pentagram_old     ... bench: 103,076,069 ns/iter (+/- 982,063)
test build_corpus_pentagram_par_new ... bench:  46,404,847 ns/iter (+/- 2,203,068)
test build_corpus_tetragram_new     ... bench: 114,192,736 ns/iter (+/- 544,679)
test build_corpus_tetragram_old     ... bench:  95,666,390 ns/iter (+/- 995,738)
test build_corpus_tetragram_par_new ... bench:  37,794,160 ns/iter (+/- 1,190,879)
test build_corpus_trigram_new       ... bench:  67,298,381 ns/iter (+/- 121,783)
test build_corpus_trigram_old       ... bench:  90,324,434 ns/iter (+/- 745,276)
test build_corpus_trigram_par_new   ... bench:  32,736,639 ns/iter (+/- 405,984)
```

The following is the benchmark over the search operations, including both the ngram search and the tfidf search, across the first `100_000` taxons from the dataset. We limit the search to the first `100_000` taxons to avoid running the benchmarks for hours. We observe that the parallel implementation, on the considered benchmark, is often slower than the sequential one as the query strings are rather small and the overhead of the parallelism is not compensated by the parallel execution.

```text
running 40 tests
test bigram_ngram_search_new         ... bench:  16,782,380 ns/iter (+/- 124,243)
test bigram_ngram_search_old         ... bench: 194,000,614 ns/iter (+/- 1,012,790)
test bigram_ngram_search_par_new     ... bench:   9,617,886 ns/iter (+/- 1,278,360)
test bigram_tfidf_search_new         ... bench:  24,929,751 ns/iter (+/- 61,441)
test bigram_tfidf_search_par_new     ... bench:  15,741,710 ns/iter (+/- 3,592,719)
test heptagram_ngram_search_new      ... bench:   5,717,789 ns/iter (+/- 11,082)
test heptagram_ngram_search_old      ... bench:  26,104,106 ns/iter (+/- 227,269)
test heptagram_ngram_search_par_new  ... bench:   7,497,285 ns/iter (+/- 1,486,217)
test heptagram_tf_idf_search_new     ... bench:  10,277,919 ns/iter (+/- 16,598)
test heptagram_tf_idf_search_par_new ... bench:  17,046,919 ns/iter (+/- 1,162,515)
test hexagram_ngram_search_new       ... bench:   6,071,483 ns/iter (+/- 8,423)
test hexagram_ngram_search_old       ... bench:  22,593,025 ns/iter (+/- 184,389)
test hexagram_ngram_search_par_new   ... bench:   7,977,281 ns/iter (+/- 1,898,493)
test hexagram_tf_idf_search_new      ... bench:   9,783,687 ns/iter (+/- 45,868)
test hexagram_tf_idf_search_par_new  ... bench:  15,721,203 ns/iter (+/- 1,565,247)
test monogram_ngram_search_new       ... bench:       1,003 ns/iter (+/- 29)
test monogram_ngram_search_old       ... bench: 627,435,969 ns/iter (+/- 83,761,877)
test monogram_ngram_search_par_new   ... bench:     111,861 ns/iter (+/- 6,024)
test monogram_tfidf_search_new       ... bench:       1,053 ns/iter (+/- 27)
test monogram_tfidf_search_par_new   ... bench:     113,906 ns/iter (+/- 7,402)
test octagram_ngram_search_new       ... bench:   5,828,795 ns/iter (+/- 9,595)
test octagram_ngram_search_old       ... bench:  29,804,453 ns/iter (+/- 161,806)
test octagram_ngram_search_par_new   ... bench:   7,639,283 ns/iter (+/- 2,578,534)
test octagram_tf_idf_search_new      ... bench:  10,364,579 ns/iter (+/- 12,066)
test octagram_tf_idf_search_par_new  ... bench:  11,082,811 ns/iter (+/- 5,716,547)
test pentagram_ngram_search_new      ... bench:   6,443,693 ns/iter (+/- 27,866)
test pentagram_ngram_search_old      ... bench:  22,054,218 ns/iter (+/- 178,649)
test pentagram_ngram_search_par_new  ... bench:   9,433,097 ns/iter (+/- 1,108,511)
test pentagram_tf_idf_search_new     ... bench:  10,158,476 ns/iter (+/- 13,531)
test pentagram_tf_idf_search_par_new ... bench:  15,642,654 ns/iter (+/- 1,590,939)
test tetragram_ngram_search_new      ... bench:   6,954,196 ns/iter (+/- 13,741)
test tetragram_ngram_search_old      ... bench:  25,243,201 ns/iter (+/- 234,073)
test tetragram_ngram_search_par_new  ... bench:   9,514,594 ns/iter (+/- 607,614)
test tetragram_tf_idf_search_new     ... bench:  10,387,104 ns/iter (+/- 39,213)
test tetragram_tf_idf_search_par_new ... bench:  14,894,122 ns/iter (+/- 1,445,305)
test trigram_ngram_search_new        ... bench:  14,262,156 ns/iter (+/- 124,193)
test trigram_ngram_search_old        ... bench:  65,163,273 ns/iter (+/- 6,750,943)
test trigram_ngram_search_par_new    ... bench:  10,972,258 ns/iter (+/- 1,413,506)
test trigram_tf_idf_search_new       ... bench:  20,467,847 ns/iter (+/- 103,989)
test trigram_tf_idf_search_par_new   ... bench:  15,509,297 ns/iter (+/- 4,235,310)

test result: ok. 0 passed; 0 failed; 0 ignored; 40 measured; 0 filtered out; finished in 573.27s
```

## Benchmarks 7 April 2024, 09:00 PM
The third benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. Overall, this machine is significantly more powerful than the previous ones, so avoid comparing these results with the previous ones. Solely compare `*_new`, which is the new implementation, and the `*_par_new`, which is the new concurrent implementation, with `*_old`, which is the old implementation as available on crate (version `0.4.0`), both runned on the same machine.

First, we run the benchmarks relative to loading the first `10_000` taxons from the dataset into memory.

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
RUSTFLAGS="-C target-cpu=native" cargo bench --bench search
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