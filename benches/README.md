# Time benchmarks
The goal of this benchmark is to accurately measure how much time is required by the `ngrammatic` library to load the taxons dataset into memory.
Since cargo bench will run the benchmarks multiple times, we will use only a subset of the rows from the dataset as at the time of writing the library is still exceedingly slow, and we do not want to wait for hours for the benchmarks to finish.

## How to run the benchmarks
To run the time benchmarks, run from the root of the repository the following command:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench  
```

## Benchmarks 26 July 2024, 05:00 PM

## Benchmarks 9 April 2024, 06:00 PM
The sixth benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. In this iteration, we run the benchmarks relative to loading the first `10_000` taxons from the dataset into memory. The novelty of this benchmark is the use of the RCL data structure for holding the strings of the dataset itself.

Compared to the previouusly used data structure, a simple vector, the improvement in memory requirements is massive. Also, it does not seem to impact too significantly the time required to perform the key operations of the library.

### Monogram Benchmarks

| Test                           | Time (ns/iter)            | +/-                 |
|--------------------------------|---------------------------|---------------------|
| bitvec_par_search_rcl          | 164,110                   | 8,359               |
| bitvec_par_search_vec          | 166,880                   | 10,705              |
| bitvec_seq_search_rcl          | 1,496                     | 28                  |
| bitvec_seq_search_vec          | 1,492                     | 30                  |
| old_seq_search                 | 617,888,122               | 3,353,254           |
| tfidf_bitvec_par_rcl           | 164,078                   | 8,445               |
| tfidf_bitvec_par_vec           | 163,612                   | 8,678               |
| tfidf_bitvec_seq_rcl           | 1,575                     | 95                  |
| tfidf_bitvec_seq_vec           | 1,486                     | 42                  |
| tfidf_webgraph_par_rcl         | 160,863                   | 9,815               |
| tfidf_webgraph_par_vec         | 160,739                   | 18,404              |
| tfidf_webgraph_seq_rcl         | 3,029                     | 210                 |
| tfidf_webgraph_seq_vec         | 2,904                     | 37                  |
| webgraph_par_search_rcl        | 161,719                   | 28,493              |
| webgraph_par_search_vec        | 157,261                   | 8,684               |
| webgraph_seq_search_rcl        | 3,026                     | 291                 |
| webgraph_seq_search_vec        | 2,872                     | 53                  |

### Bigram Benchmarks

| Test                             | Time (ns/iter)            | +/-                 |
|----------------------------------|---------------------------|---------------------|
| bitvec_par_search_rcl            | 14,158,963                | 4,382,217           |
| bitvec_par_search_vec            | 14,283,047                | 2,872,322           |
| bitvec_seq_search_rcl            | 23,572,426                | 100,103             |
| bitvec_seq_search_vec            | 23,765,254                | 85,246              |
| old_seq_search                   | 187,365,001               | 1,453,912           |
| tfidf_bitvec_par_rcl             | 27,032,078                | 2,165,998           |
| tfidf_bitvec_par_vec             | 24,042,987                | 2,238,973           |
| tfidf_bitvec_seq_rcl             | 40,213,882                | 1,741,292           |
| tfidf_bitvec_seq_vec             | 34,101,151                | 79,071              |
| tfidf_webgraph_par_rcl           | 80,790,835                | 14,922,114          |
| tfidf_webgraph_par_vec           | 78,238,592                | 13,683,586          |
| tfidf_webgraph_seq_rcl           | 170,019,399               | 971,670             |
| tfidf_webgraph_seq_vec           | 158,185,323               | 1,109,479           |
| webgraph_par_search_rcl          | 56,825,438                | 11,179,772          |
| webgraph_par_search_vec          | 54,109,527                | 12,707,143          |
| webgraph_seq_search_rcl          | 124,851,353               | 1,990,438           |
| webgraph_seq_search_vec          | 121,921,455               | 2,343,263           |

### Trigram Benchmarks

| Test                           | Time (ns/iter)            | +/-                 |
|--------------------------------|---------------------------|---------------------|
| bitvec_par_search_rcl          | 15,892,127                | 2,961,082           |
| bitvec_par_search_vec          | 14,619,355                | 3,775,417           |
| bitvec_seq_search_rcl          | 20,725,798                | 70,959              |
| bitvec_seq_search_vec          | 21,020,933                | 54,712              |
| old_seq_search                 | 63,907,148                | 855,000             |
| tfidf_bitvec_par_rcl           | 28,114,472                | 4,780,672           |
| tfidf_bitvec_par_vec           | 24,558,503                | 1,988,821           |
| tfidf_bitvec_seq_rcl           | 35,802,803                | 83,510              |
| tfidf_bitvec_seq_vec           | 29,405,303                | 61,802              |
| tfidf_webgraph_par_rcl         | 102,373,931               | 12,068,697          |
| tfidf_webgraph_par_vec         | 97,639,416                | 14,051,164          |
| tfidf_webgraph_seq_rcl         | 160,022,083               | 801,983             |
| tfidf_webgraph_seq_vec         | 150,471,471               | 180,682             |
| webgraph_par_search_rcl        | 72,191,176                | 9,371,052           |
| webgraph_par_search_vec        | 70,775,431                | 8,944,474           |
| webgraph_seq_search_rcl        | 112,489,656               | 2,381,201           |
| webgraph_seq_search_vec        | 109,659,835               | 251,582             |

### Tetragram Benchmarks

| Test                          | Time (ns/iter)            | +/-                 |
|-------------------------------|---------------------------|---------------------|
| bitvec_par_search_rcl         | 12,003,433                | 4,718,873           |
| bitvec_par_search_vec         | 13,415,564                | 4,158,818           |
| bitvec_seq_search_rcl         | 10,090,587                | 129,811             |
| bitvec_seq_search_vec         | 10,344,738                | 52,067              |
| old_seq_search                | 24,920,493                | 305,297             |
| tfidf_bitvec_par_rcl          | 24,946,904                | 5,300,785           |
| tfidf_bitvec_par_vec          | 22,519,442                | 3,133,970           |
| tfidf_bitvec_seq_rcl          | 18,379,084                | 92,276              |
| tfidf_bitvec_seq_vec          | 15,571,167                | 72,983              |
| tfidf_webgraph_par_rcl        | 104,012,794               | 13,122,154          |
| tfidf_webgraph_par_vec        | 96,789,597                | 15,322,571          |
| tfidf_webgraph_seq_rcl        | 82,299,372                | 228,436             |
| tfidf_webgraph_seq_vec        | 78,087,686                | 268,301             |
| webgraph_par_search_rcl       | 74,530,232                | 10,537,080          |
| webgraph_par_search_vec       | 69,175,308                | 9,681,547           |
| webgraph_seq_search_rcl       | 57,230,776                | 103,174             |
| webgraph_seq_search_vec       | 55,150,709                | 91,656              |

### Pentagram Benchmarks

| Test                          | Time (ns/iter)            | +/-                 |
|-------------------------------|---------------------------|---------------------|
| bitvec_par_search_rcl         | 14,081,699                | 1,141,672           |
| bitvec_par_search_vec         | 14,345,128                | 3,746,306           |
| bitvec_seq_search_rcl         | 9,410,065                 | 53,170              |
| bitvec_seq_search_vec         | 9,639,321                 | 28,633              |
| old_seq_search                | 22,056,607                | 169,907             |
| tfidf_bitvec_par_rcl          | 28,571,099                | 2,566,422           |
| tfidf_bitvec_par_vec          | 25,248,974                | 3,738,167           |
| tfidf_bitvec_seq_rcl          | 18,308,269                | 49,531              |
| tfidf_bitvec_seq_vec          | 15,572,161                | 70,033              |
| tfidf_webgraph_par_rcl        | 99,301,417                | 18,097,331          |
| tfidf_webgraph_par_vec        | 99,229,827                | 10,387,806          |
| tfidf_webgraph_seq_rcl        | 77,753,268                | 90,537              |
| tfidf_webgraph_seq_vec        | 73,554,579                | 548,396             |
| webgraph_par_search_rcl       | 75,750,568                | 11,088,086          |
| webgraph_par_search_vec       | 70,994,717                | 8,623,301           |
| webgraph_seq_search_rcl       | 54,427,636                | 1,155,450           |
| webgraph_seq_search_vec       | 52,534,890                | 99,501              |


### Hexagram Benchmarks

| Test                           | Time (ns/iter)            | +/-                 |
|--------------------------------|---------------------------|---------------------|
| bitvec_par_search_rcl          | 12,325,614                | 2,104,040           |
| bitvec_par_search_vec          | 12,095,099                | 2,643,998           |
| bitvec_seq_search_rcl          | 8,933,519                 | 28,469              |
| bitvec_seq_search_vec          | 9,065,280                 | 23,255              |
| old_seq_search                 | 22,449,601                | 203,832             |
| tfidf_bitvec_par_rcl           | 20,665,191                | 8,661,905           |
| tfidf_bitvec_par_vec           | 24,392,885                | 2,454,376           |
| tfidf_bitvec_seq_rcl           | 17,497,485                | 45,755              |
| tfidf_bitvec_seq_vec           | 15,082,711                | 38,812              |
| tfidf_webgraph_par_rcl         | 82,825,894                | 22,041,872          |
| tfidf_webgraph_par_vec         | 84,492,459                | 31,916,772          |
| tfidf_webgraph_seq_rcl         | 77,316,499                | 80,025              |
| tfidf_webgraph_seq_vec         | 73,777,869                | 97,729              |
| webgraph_par_search_rcl        | 59,279,722                | 17,028,813          |
| webgraph_par_search_vec        | 56,754,418                | 14,364,301          |
| webgraph_seq_search_rcl        | 54,877,503                | 194,789             |
| webgraph_seq_search_vec        | 52,242,901                | 227,578             |

### Heptagram Benchmarks

| Test                          | Time (ns/iter)            | +/-                 |
|-------------------------------|---------------------------|---------------------|
| bitvec_par_search_rcl         | 10,736,349                | 2,249,504           |
| bitvec_par_search_vec         | 11,307,423                | 2,524,018           |
| bitvec_seq_search_rcl         | 8,445,303                 | 35,293              |
| bitvec_seq_search_vec         | 8,630,346                 | 28,891              |
| old_seq_search                | 25,881,998                | 232,908             |
| tfidf_bitvec_par_rcl          | 20,418,717                | 2,115,452           |
| tfidf_bitvec_par_vec          | 28,825,448                | 4,037,268           |
| tfidf_bitvec_seq_rcl          | 19,649,438                | 54,359              |
| tfidf_bitvec_seq_vec          | 16,837,797                | 59,656              |
| tfidf_webgraph_par_rcl        | 87,730,932                | 34,172,262          |
| tfidf_webgraph_par_vec        | 74,167,475                | 29,023,645          |
| tfidf_webgraph_seq_rcl        | 78,161,557                | 450,187             |
| tfidf_webgraph_seq_vec        | 73,324,960                | 547,239             |
| webgraph_par_search_rcl       | 51,301,970                | 20,526,802          |
| webgraph_par_search_vec       | 50,647,931                | 16,538,200          |
| webgraph_seq_search_rcl       | 54,258,315                | 1,333,734           |
| webgraph_seq_search_vec       | 52,708,023                | 312,741             |

### Octagram Benchmarks

| Test                           | Time (ns/iter)            | +/-                 |
|--------------------------------|---------------------------|---------------------|
| bitvec_par_search_rcl          | 8,772,018                 | 4,912,144           |
| bitvec_par_search_vec          | 10,589,881                | 3,035,243           |
| bitvec_seq_search_rcl          | 8,460,512                 | 74,560              |
| bitvec_seq_search_vec          | 8,771,734                 | 94,194              |
| old_seq_search                 | 27,321,061                | 254,917             |
| tfidf_bitvec_par_rcl           | 17,778,826                | 6,392,077           |
| tfidf_bitvec_par_vec           | 19,980,522                | 6,520,545           |
| tfidf_bitvec_seq_rcl           | 19,357,493                | 48,306              |
| tfidf_bitvec_seq_vec           | 16,973,596                | 46,473              |
| tfidf_webgraph_par_rcl         | 80,305,030                | 24,177,119          |
| tfidf_webgraph_par_vec         | 77,648,040                | 30,857,219          |
| tfidf_webgraph_seq_rcl         | 79,402,699                | 610,597             |
| tfidf_webgraph_seq_vec         | 73,464,327                | 146,384             |
| webgraph_par_search_rcl        | 55,291,116                | 21,627,121          |
| webgraph_par_search_vec        | 51,256,769                | 16,806,643          |
| webgraph_seq_search_rcl        | 55,277,968                | 1,299,991           |
| webgraph_seq_search_vec        | 53,852,872                | 134,304             |


## Benchmarks 9 April 2024, 01:00 PM
The fifth benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. In this iteration, we run the benchmarks relative to loading the first `10_000` taxons from the dataset into memory. The novelty of this benchmark is the use of the webgraph data structure as one of the possible underlying graph representations, which requires a significantly smaller memory footprint than the default graph representation. 

### Building the corpus

| Test                             | Time (ns/iter)         | Standard Deviation (ns/iter) |
|----------------------------------|------------------------|-------------------------------|
| bigram_new                       | 30,063,497             | 192,926                    |
| bigram_new_webgraph              | 49,252,438             | 1,152,559                  |
| bigram_old                       | 79,633,044             | 2,555,930                  |
| bigram_par_new                   | 25,583,916             | 757,674                    |
| bigram_par_new_webgraph          | 42,800,831             | 1,595,224                  |
| heptagram_new                    | 296,716,962            | 886,155                    |
| heptagram_new_webgraph           | 322,676,915            | 3,706,157                  |
| heptagram_old                    | 119,413,976            | 6,942,323                  |
| heptagram_par_new                | 57,760,362             | 3,907,160                  |
| heptagram_par_new_webgraph       | 75,677,778             | 2,540,270                  |
| hexagram_new                     | 256,702,718            | 754,501                    |
| hexagram_new_webgraph            | 280,953,094            | 1,999,904                  |
| hexagram_old                     | 107,797,245            | 5,356,921                  |
| hexagram_par_new                 | 55,051,893             | 1,797,274                  |
| hexagram_par_new_webgraph        | 71,713,398             | 2,758,198                  |
| monogram_new                     | 12,736,422             | 108,683                    |
| monogram_new_webgraph            | 32,293,678             | 1,039,797                  |
| monogram_old                     | 46,012,249             | 611,606                    |
| monogram_par_new                 | 13,943,383             | 872,744                    |
| monogram_par_new_webgraph        | 32,763,070             | 1,146,276                  |
| octagram_new                     | 417,944,773            | 1,408,906                  |
| octagram_new_webgraph            | 444,898,880            | 7,310,793                  |
| octagram_old                     | 127,136,529            | 2,154,483                  |
| octagram_par_new                 | 66,019,797             | 3,082,655                  |
| octagram_par_new_webgraph        | 86,585,359             | 3,218,811                  |
| pentagram_new                    | 221,473,322            | 1,298,914                  |
| pentagram_new_webgraph           | 245,089,696            | 4,771,393                  |
| pentagram_old                    | 106,523,748            | 4,130,615                  |
| pentagram_par_new                | 49,113,952             | 1,429,054                  |
| pentagram_par_new_webgraph       | 64,072,344             | 1,894,184                  |
| tetragram_new                    | 131,636,869            | 147,802                    |
| tetragram_new_webgraph           | 153,685,377            | 2,089,815                  |
| tetragram_old                    | 96,213,548             | 3,512,520                  |
| tetragram_par_new                | 39,722,521             | 897,996                    |
| tetragram_par_new_webgraph       | 56,044,704             | 1,631,917                  |
| trigram_new                      | 72,245,656             | 129,528                    |
| trigram_new_webgraph             | 92,708,174             | 1,656,958                  |
| trigram_old                      | 90,953,416             | 2,411,579                  |
| trigram_par_new                  | 33,195,119             | 447,306                    |
| trigram_par_new_webgraph         | 48,771,236             | 1,139,261                  |


The following is the benchmark over the search operations, including both the ngram search and the tfidf search, across the first `100_000` taxons from the dataset. We limit the search to the first `100_000` taxons to avoid running the benchmarks for hours. We observe that the parallel implementation, on the considered benchmark, is often slower than the sequential one as the query strings are rather small and the overhead of the parallelism is not compensated by the parallel execution.

| Test                                          | Time (ns/iter)          | Standard Deviation (ns/iter) |
|-----------------------------------------------|-------------------------|------------------------------|
| bigram_ngram_search_new                       | 16,733,818              | ± 84,531                     |
| bigram_ngram_search_new_webgraph              | 85,000,471              | ± 329,925                    |
| bigram_ngram_search_old                       | 191,041,933             | ± 1,596,474                  |
| bigram_ngram_search_par_new                   | 10,505,062              | ± 1,384,319                  |
| bigram_ngram_search_par_new_webgraph          | 43,883,661              | ± 13,639,106                 |
| bigram_tfidf_search_new                       | 25,801,844              | ± 1,197,147                  |
| bigram_tfidf_search_new_webgraph              | 116,396,838             | ± 134,291                    |
| bigram_tfidf_search_par_new                   | 16,068,499              | ± 1,986,958                  |
| bigram_tfidf_search_par_new_webgraph          | 58,148,526              | ± 11,688,490                 |
| heptagram_ngram_search_new                    | 5,678,363               | ± 145,799                    |
| heptagram_ngram_search_new_webgraph           | 36,320,955              | ± 346,692                    |
| heptagram_ngram_search_old                    | 26,031,489              | ± 254,458                    |
| heptagram_ngram_search_par_new                | 7,523,810               | ± 1,523,139                  |
| heptagram_ngram_search_par_new_webgraph       | 45,321,112              | ± 14,109,003                 |
| heptagram_tf_idf_search_new                   | 11,513,398              | ± 701,384                    |
| heptagram_tf_idf_search_new_webgraph          | 50,987,997              | ± 329,138                    |
| heptagram_tf_idf_search_par_new               | 19,044,546              | ± 1,590,498                  |
| heptagram_tf_idf_search_par_new_webgraph      | 53,514,306              | ± 30,448,600                 |
| hexagram_ngram_search_new                     | 6,038,707               | ± 157,491                    |
| hexagram_ngram_search_new_webgraph            | 36,282,229              | ± 110,640                    |
| hexagram_ngram_search_old                     | 22,900,736              | ± 244,797                    |
| hexagram_ngram_search_par_new                 | 9,027,312               | ± 1,136,602                  |
| hexagram_ngram_search_par_new_webgraph        | 43,994,350              | ± 13,848,646                 |
| hexagram_tf_idf_search_new                    | 10,210,663              | ± 48,730                     |
| hexagram_tf_idf_search_new_webgraph           | 50,491,341              | ± 124,927                    |
| hexagram_tf_idf_search_par_new                | 16,174,063              | ± 1,111,776                  |
| hexagram_tf_idf_search_par_new_webgraph       | 61,756,269              | ± 16,559,935                 |
| monogram_ngram_search_new                     | 973                     | ± 16                         |
| monogram_ngram_search_new_webgraph            | 2,486                   | ± 21                         |
| monogram_ngram_search_old                     | 691,227,479             | ± 10,317,495                 |
| monogram_ngram_search_par_new                 | 108,762                 | ± 5,751                      |
| monogram_ngram_search_par_new_webgraph        | 110,843                 | ± 6,612                      |
| monogram_tfidf_search_new                     | 945                     | ± 16                         |
| monogram_tfidf_search_new_webgraph            | 2,484                   | ± 16                         |
| monogram_tfidf_search_par_new                 | 105,000                 | ± 6,553                      |
| monogram_tfidf_search_par_new_webgraph        | 108,017                 | ± 6,292                      |
| octagram_ngram_search_new                     | 5,774,531               | ± 27,291                     |
| octagram_ngram_search_new_webgraph            | 36,767,845              | ± 127,595                    |
| octagram_ngram_search_old                     | 31,533,250              | ± 320,307                    |
| octagram_ngram_search_par_new                 | 8,198,898               | ± 1,318,197                  |
| octagram_ngram_search_par_new_webgraph        | 42,032,366              | ± 14,594,741                 |
| octagram_tf_idf_search_new                    | 11,578,159              | ± 32,703                     |
| octagram_tf_idf_search_new_webgraph           | 51,601,231              | ± 127,108                    |
| octagram_tf_idf_search_par_new                | 12,592,889              | ± 2,627,268                  |
| octagram_tf_idf_search_par_new_webgraph       | 51,081,519              | ± 23,510,917                 |
| pentagram_ngram_search_new                    | 6,404,633               | ± 25,578                     |
| pentagram_ngram_search_new_webgraph           | 36,406,376              | ± 82,059                     |
| pentagram_ngram_search_old                    | 22,110,618              | ± 164,017                    |
| pentagram_ngram_search_par_new                | 9,612,528               | ± 1,064,053                  |
| pentagram_ngram_search_par_new_webgraph       | 50,572,814              | ± 9,078,281                  |
| pentagram_tf_idf_search_new                   | 10,613,421              | ± 675,242                    |
| pentagram_tf_idf_search_new_webgraph          | 50,604,519              | ± 83,511                     |
| pentagram_tf_idf_search_par_new               | 16,342,316              | ± 1,273,526                  |
| pentagram_tf_idf_search_par_new_webgraph      | 67,601,365              | ± 5,575,331                  |
| tetragram_ngram_search_new                    | 6,892,730               | ± 86,218                     |
| tetragram_ngram_search_new_webgraph           | 38,824,290              | ± 635,002                    |
| tetragram_ngram_search_old                    | 24,787,158              | ± 674,009                    |
| tetragram_ngram_search_par_new                | 9,756,342               | ± 1,392,526                  |
| tetragram_ngram_search_par_new_webgraph       | 49,722,566              | ± 8,288,464                  |
| tetragram_tf_idf_search_new                   | 10,496,280              | ± 73,933                     |
| tetragram_tf_idf_search_new_webgraph          | 54,398,647              | ± 293,948                    |
| tetragram_tf_idf_search_par_new               | 14,993,612              | ± 1,203,980                  |
| tetragram_tf_idf_search_par_new_webgraph      | 65,550,633              | ± 9,234,801                  |
| trigram_ngram_search_new                      | 14,195,566              | ± 92,455                     |
| trigram_ngram_search_new_webgraph             | 77,354,182              | ± 240,438                    |
| trigram_ngram_search_old                      | 67,413,608              | ± 12,128,538                 |
| trigram_ngram_search_par_new                  | 11,202,064              | ± 2,082,499                  |
| trigram_ngram_search_par_new_webgraph         | 54,546,009              | ± 5,139,539                  |
| trigram_tf_idf_search_new                     | 20,400,317              | ± 1,243,061                  |
| trigram_tf_idf_search_new_webgraph            | 107,031,641             | ± 1,650,785                  |
| trigram_tf_idf_search_par_new                 | 16,566,241              | ± 2,359,137                  |
| trigram_tf_idf_search_par_new_webgraph        | 70,078,550              | ± 10,145,876                 |

## Benchmarks 9 April 2024, 10:00 AM
The fourth benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. In this iteration, we run the benchmarks relative to loading the first `10_000` taxons from the dataset into memory. 


| Test                      | Time (ns/iter)          | Standard Deviation (ns/iter) |
|---------------------------|-------------------------|------------------------------|
| bigram_new                | 30,027,421              | ± 169,793                    |
| bigram_old                | 75,731,669              | ± 1,092,273                  |
| bigram_par_new            | 25,305,966              | ± 2,203,643                  |
| heptagram_new             | 262,093,324             | ± 2,863,954                  |
| heptagram_old             | 121,937,447             | ± 2,090,908                  |
| heptagram_par_new         | 56,524,373              | ± 1,672,922                  |
| hexagram_new              | 221,587,860             | ± 1,865,118                  |
| hexagram_old              | 106,219,509             | ± 4,934,105                  |
| hexagram_par_new          | 52,091,040              | ± 1,349,573                  |
| monogram_new              | 12,594,027              | ± 62,096                     |
| monogram_old              | 45,972,180              | ± 322,730                    |
| monogram_par_new          | 13,947,290              | ± 915,101                    |
| octagram_new              | 353,140,027             | ± 1,275,524                  |
| octagram_old              | 125,616,055             | ± 4,814,695                  |
| octagram_par_new          | 63,328,865              | ± 3,103,435                  |
| pentagram_new             | 190,646,762             | ± 429,277                    |
| pentagram_old             | 103,076,069             | ± 982,063                    |
| pentagram_par_new         | 46,404,847              | ± 2,203,068                  |
| tetragram_new             | 114,192,736             | ± 544,679                    |
| tetragram_old             | 95,666,390              | ± 995,738                    |
| tetragram_par_new         | 37,794,160              | ± 1,190,879                  |
| trigram_new               | 67,298,381              | ± 121,783                    |
| trigram_old               | 90,324,434              | ± 745,276                    |
| trigram_par_new           | 32,736,639              | ± 405,984                    |

The following is the benchmark over the search operations, including both the ngram search and the tfidf search, across the first `100_000` taxons from the dataset. We limit the search to the first `100_000` taxons to avoid running the benchmarks for hours. We observe that the parallel implementation, on the considered benchmark, is often slower than the sequential one as the query strings are rather small and the overhead of the parallelism is not compensated by the parallel execution.

| Test                               | Time (ns/iter)      | Standard Deviation (ns/iter)  |
|------------------------------------|---------------------|-------------------------------|
| bigram_ngram_search_new            | 16,782,380          | ± 124,243                     |
| bigram_ngram_search_old            | 194,000,614         | ± 1,012,790                   |
| bigram_ngram_search_par_new        | 9,617,886           | ± 1,278,360                   |
| bigram_tfidf_search_new            | 24,929,751          | ± 61,441                      |
| bigram_tfidf_search_par_new        | 15,741,710          | ± 3,592,719                   |
| heptagram_ngram_search_new         | 5,717,789           | ± 11,082                      |
| heptagram_ngram_search_old         | 26,104,106          | ± 227,269                     |
| heptagram_ngram_search_par_new     | 7,497,285           | ± 1,486,217                   |
| heptagram_tf_idf_search_new        | 10,277,919          | ± 16,598                      |
| heptagram_tf_idf_search_par_new    | 17,046,919          | ± 1,162,515                   |
| hexagram_ngram_search_new          | 6,071,483           | ± 8,423                       |
| hexagram_ngram_search_old          | 22,593,025          | ± 184,389                     |
| hexagram_ngram_search_par_new      | 7,977,281           | ± 1,898,493                   |
| hexagram_tf_idf_search_new         | 9,783,687           | ± 45,868                      |
| hexagram_tf_idf_search_par_new     | 15,721,203          | ± 1,565,247                   |
| monogram_ngram_search_new          | 1,003               | ± 29                          |
| monogram_ngram_search_old          | 627,435,969         | ± 83,761,877                  |
| monogram_ngram_search_par_new      | 111,861             | ± 6,024                       |
| monogram_tfidf_search_new          | 1,053               | ± 27                          |
| monogram_tfidf_search_par_new      | 113,906             | ± 7,402                       |
| octagram_ngram_search_new          | 5,828,795           | ± 9,595                       |
| octagram_ngram_search_old          | 29,804,453          | ± 161,806                     |
| octagram_ngram_search_par_new      | 7,639,283           | ± 2,578,534                   |
| octagram_tf_idf_search_new         | 10,364,579          | ± 12,066                      |
| octagram_tf_idf_search_par_new     | 11,082,811          | ± 5,716,547                   |
| pentagram_ngram_search_new         | 6,443,693           | ± 27,866                      |
| pentagram_ngram_search_old         | 22,054,218          | ± 178,649                     |
| pentagram_ngram_search_par_new     | 9,433,097           | ± 1,108,511                   |
| pentagram_tf_idf_search_new        | 10,158,476          | ± 13,531                      |
| pentagram_tf_idf_search_par_new    | 15,642,654          | ± 1,590,939                   |
| tetragram_ngram_search_new         | 6,954,196           | ± 13,741                      |
| tetragram_ngram_search_old         | 25,243,201          | ± 234,073                     |
| tetragram_ngram_search_par_new     | 9,514,594           | ± 607,614                     |
| tetragram_tf_idf_search_new        | 10,387,104          | ± 39,213                      |
| tetragram_tf_idf_search_par_new    | 14,894,122          | ± 1,445,305                   |
| trigram_ngram_search_new           | 14,262,156          | ± 124,193                     |
| trigram_ngram_search_old           | 65,163,273          | ± 6,750,943                   |
| trigram_ngram_search_par_new       | 10,972,258          | ± 1,413,506                   |
| trigram_tf_idf_search_new          | 20,467,847          | ± 103,989                     |
| trigram_tf_idf_search_par_new      | 15,509,297          | ± 4,235,310                   |

## Benchmarks 7 April 2024, 09:00 PM
The third benchmark was run on a 32-core machine with 64 threads and with 256 GBs of RAM. Overall, this machine is significantly more powerful than the previous ones, so avoid comparing these results with the previous ones. Solely compare `*_new`, which is the new implementation, and the `*_par_new`, which is the new concurrent implementation, with `*_old`, which is the old implementation as available on crate (version `0.4.0`), both runned on the same machine.

First, we run the benchmarks relative to loading the first `10_000` taxons from the dataset into memory.

```bash
RUST_LOG=info RUSTFLAGS="-C target-cpu=native" cargo bench --bench build_corpus
```

| Test                 | Time (ns/iter)      | Standard Deviation (ns/iter)  |
|----------------------|---------------------|-------------------------------|
| bigram_new           | 30,896,888          | ± 398,108                     |
| bigram_old           | 77,941,909          | ± 3,505,171                   |
| bigram_par_new       | 26,432,127          | ± 1,946,359                   |
| heptagram_new        | 272,009,743         | ± 607,368                     |
| heptagram_old        | 118,533,641         | ± 1,728,095                   |
| heptagram_par_new    | 56,744,864          | ± 1,476,456                   |
| hexagram_new         | 236,410,291         | ± 627,052                     |
| hexagram_old         | 111,394,763         | ± 1,421,418                   |
| hexagram_par_new     | 52,687,949          | ± 1,497,216                   |
| monogram_new         | 12,722,442          | ± 55,331                      |
| monogram_old         | 46,011,731          | ± 607,534                     |
| monogram_par_new     | 13,875,319          | ± 1,238,036                   |
| octagram_new         | 366,530,006         | ± 740,843                     |
| octagram_old         | 130,562,743         | ± 1,229,807                   |
| octagram_par_new     | 64,729,705          | ± 3,015,636                   |
| pentagram_new        | 197,060,504         | ± 364,804                     |
| pentagram_old        | 103,242,243         | ± 2,812,735                   |
| pentagram_par_new    | 47,142,296          | ± 1,742,570                   |
| tetragram_new        | 119,104,440         | ± 804,896                     |
| tetragram_old        | 98,156,298          | ± 3,034,541                   |
| tetragram_par_new    | 38,689,265          | ± 1,123,753                   |
| trigram_new          | 68,718,691          | ± 928,889                     |
| trigram_old          | 92,043,546          | ± 2,601,382                   |
| trigram_par_new      | 33,355,542          | ± 776,818                     |

Secondarily, we run the benchmarks relative to searching taxons across the first `100_000` taxons from the dataset.

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench --bench search
```

| Test                                    | Time (ns/iter)     | Standard Deviation (ns/iter)  |
|-----------------------------------------|---------------------|--------------------------------|
| ngram_search_corpus_bigram_new         | 7,497,412           | ± 68,003                       |
| ngram_search_corpus_bigram_old         | 99,955,280          | ± 8,075,258                    |
| ngram_search_corpus_bigram_par_new     | 4,977,195           | ± 815,739                      |
| ngram_search_corpus_heptagram_new      | 2,873,109           | ± 13,342                       |
| ngram_search_corpus_heptagram_old      | 16,461,192          | ± 165,196                      |
| ngram_search_corpus_heptagram_par_new  | 1,643,750           | ± 151,236                      |
| ngram_search_corpus_hexagram_new       | 3,164,364           | ± 15,709                       |
| ngram_search_corpus_hexagram_old       | 14,526,763          | ± 176,189                      |
| ngram_search_corpus_hexagram_par_new   | 3,463,607           | ± 871,659                      |
| ngram_search_corpus_monogram_new       | 477                 | ± 8                            |
| ngram_search_corpus_monogram_old       | 326,974,147         | ± 3,933,313                    |
| ngram_search_corpus_monogram_par_new   | 53,081              | ± 4,474                        |
| ngram_search_corpus_octagram_new       | 2,925,086           | ± 12,178                       |
| ngram_search_corpus_octagram_old       | 19,291,253          | ± 112,943                      |
| ngram_search_corpus_octagram_par_new   | 3,344,486           | ± 560,629                      |
| ngram_search_corpus_pentagram_new      | 3,437,322           | ± 40,099                       |
| ngram_search_corpus_pentagram_old      | 13,925,100          | ± 138,846                      |
| ngram_search_corpus_pentagram_par_new  | 4,533,541           | ± 417,050                      |
| ngram_search_corpus_tetragram_new      | 3,742,860           | ± 19,222                       |
| ngram_search_corpus_tetragram_old      | 16,754,572          | ± 686,706                      |
| ngram_search_corpus_tetragram_par_new  | 4,401,330           | ± 515,740                      |
| ngram_search_corpus_trigram_new        | 5,435,892           | ± 65,594                       |
| ngram_search_corpus_trigram_old        | 32,010,687          | ± 814,910                      |
| ngram_search_corpus_trigram_par_new    | 3,717,600           | ± 574,098                      |

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