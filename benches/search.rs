#![feature(test)]
extern crate test;
use ngrammatic::prelude::*;
use rayon::slice::ParallelSliceMut;
use std::fmt::Debug;
use sux::dict::rear_coded_list::{RearCodedList, RearCodedListBuilder};
use test::{black_box, Bencher};

/// Returns an iterator over the taxons in the corpus.
fn iter_taxons() -> impl Iterator<Item = String> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./benchmarks/taxons.csv.gz").unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    reader.lines().take(100_000).map(|line| line.unwrap())
}

fn build_vec() -> Vec<String> {
    iter_taxons().collect()
}

/// Returns the built RCL
fn build_rcl() -> RearCodedList {
    let mut taxons: Vec<String> = build_vec();
    taxons.par_sort_unstable();

    let mut rcl_builder = RearCodedListBuilder::new(8);
    for taxon in taxons {
        rcl_builder.push(&taxon);
    }
    rcl_builder.build()
}

/// Returns ngram par-corpus.
fn new_corpus_bitvec<NG, B>(keys: B) -> Corpus<B, NG, Lowercase<str>>
where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<ngrammatic::Lowercase<str>>,
{
    Corpus::par_from(keys)
}

/// Returns ngram webgraph-based par-corpus.
fn new_corpus_webgraph<NG, B>(keys: B) -> Corpus<B, NG, Lowercase<str>, BiWebgraph>
where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<ngrammatic::Lowercase<str>>,
{
    Corpus::try_from(new_corpus_bitvec::<NG, B>(keys)).unwrap()
}

/// Returns ngram par-corpus.
fn new_corpus_bitvec_vec<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    new_corpus_bitvec::<NG, Vec<String>>(build_vec())
}

/// Returns ngram par-corpus.
fn new_corpus_bitvec_rcl<NG>() -> Corpus<RearCodedList, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    new_corpus_bitvec::<NG, RearCodedList>(build_rcl())
}

/// Returns ngram par-corpus.
fn new_corpus_webgraph_vec<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    new_corpus_webgraph::<NG, Vec<String>>(build_vec())
}

/// Returns ngram par-corpus.
fn new_corpus_webgraph_rcl<NG>() -> Corpus<RearCodedList, NG, Lowercase<str>, BiWebgraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    new_corpus_webgraph::<NG, RearCodedList>(build_rcl())
}

/// Returns old ngram corpus.
fn old_load_corpus<NG: Ngram>() -> ngrammatic_old::Corpus {
    let mut corpus = ngrammatic_old::CorpusBuilder::new()
        .arity(NG::ARITY)
        .pad_full(ngrammatic_old::Pad::Auto)
        .finish();

    for line in iter_taxons() {
        corpus.add_text(&line);
    }

    corpus
}

fn ngram_search<NG, B, G>(
    b: &mut Bencher,
    corpus: Corpus<B, NG, Lowercase<str>, G>,
    search: fn(&Corpus<B, NG, Lowercase<str>, G>, &str, NgramSearchConfig),
) where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    G: WeightedBipartiteGraph,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<ngrammatic::Lowercase<str>>,
{
    let search_config = NgramSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        // The old approach by default returned 10 results, so
        // to better compare the two, we set the same limit here.
        .set_maximum_number_of_results(10);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            search(&corpus, "Acanthocephala", search_config);
            search(&corpus, "Doggus Lionenus", search_config);
            search(&corpus, "Felis Caninus", search_config);
        });
    });
}

fn tf_idf_search<NG, B, G>(
    b: &mut Bencher,
    corpus: Corpus<B, NG, Lowercase<str>, G>,
    search: fn(&Corpus<B, NG, Lowercase<str>, G>, &str, TFIDFSearchConfig),
) where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    G: WeightedBipartiteGraph,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<ngrammatic::Lowercase<str>>,
{
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        // The old approach by default returned 10 results, so
        // to better compare the two, we set the same limit here.
        .set_maximum_number_of_results(10);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            search(&corpus, "Acanthocephala", search_config);
            search(&corpus, "Doggus Lionenus", search_config);
            search(&corpus, "Felis Caninus", search_config);
        });
    });
}

fn ngram_seq_search<NG, B, G>(b: &mut Bencher, corpus: Corpus<B, NG, Lowercase<str>, G>)
where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    G: WeightedBipartiteGraph,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<ngrammatic::Lowercase<str>>,
{
    ngram_search(b, corpus, |corpus, key, search_config| {
        corpus.ngram_search(key, search_config);
    });
}

fn ngram_par_search<NG, B, G>(b: &mut Bencher, corpus: Corpus<B, NG, Lowercase<str>, G>)
where
    B: Keys<NG> + Send + Sync,
    NG: Ngram<G = ASCIIChar> + Debug + Send + Sync,
    G: WeightedBipartiteGraph + Send + Sync,
    <<B as Keys<NG>>::K as Key<NG, ASCIIChar>>::Ref: Send + Sync,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>:
        AsRef<ngrammatic::Lowercase<str>> + Send + Sync,
{
    ngram_search(b, corpus, |corpus, key, search_config| {
        corpus.ngram_par_search(key, search_config);
    });
}

fn tfidf_seq_search<NG, B, G>(b: &mut Bencher, corpus: Corpus<B, NG, Lowercase<str>, G>)
where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    G: WeightedBipartiteGraph,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<ngrammatic::Lowercase<str>>,
{
    tf_idf_search(b, corpus, |corpus, key, search_config| {
        corpus.tf_idf_search(key, search_config);
    });
}

fn tfidf_par_search<NG, B, G>(b: &mut Bencher, corpus: Corpus<B, NG, Lowercase<str>, G>)
where
    B: Keys<NG> + Send + Sync,
    NG: Ngram<G = ASCIIChar> + Debug + Send + Sync,
    G: WeightedBipartiteGraph + Send + Sync,
    <<B as Keys<NG>>::K as Key<NG, ASCIIChar>>::Ref: Send + Sync,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>:
        AsRef<ngrammatic::Lowercase<str>> + Send + Sync,
{
    tf_idf_search(b, corpus, |corpus, key, search_config| {
        corpus.tf_idf_par_search(key, search_config);
    });
}

fn ngram_webgraph_seq_search_vec<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    ngram_seq_search(b, new_corpus_webgraph_vec::<NG>());
}

fn tfidf_webgraph_seq_vec<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    tfidf_seq_search(b, new_corpus_webgraph_vec::<NG>());
}

fn ngram_webgraph_par_search_vec<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    ngram_par_search(b, new_corpus_webgraph_vec::<NG>());
}

fn tfidf_webgraph_par_vec<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    tfidf_par_search(b, new_corpus_webgraph_vec::<NG>());
}

fn ngram_bitvec_seq_search_vec<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    ngram_seq_search(b, new_corpus_bitvec_vec::<NG>());
}

fn tfidf_bitvec_seq_vec<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    tfidf_seq_search(b, new_corpus_bitvec_vec::<NG>());
}

fn ngram_bitvec_par_search_vec<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    ngram_par_search(b, new_corpus_bitvec_vec::<NG>());
}

fn tfidf_bitvec_par_vec<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    tfidf_par_search(b, new_corpus_bitvec_vec::<NG>());
}

fn ngram_webgraph_seq_search_rcl<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    ngram_seq_search(b, new_corpus_webgraph_rcl::<NG>());
}

fn tfidf_webgraph_seq_rcl<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    tfidf_seq_search(b, new_corpus_webgraph_rcl::<NG>());
}

fn ngram_webgraph_par_search_rcl<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    ngram_par_search(b, new_corpus_webgraph_rcl::<NG>());
}

fn tfidf_webgraph_par_rcl<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    tfidf_par_search(b, new_corpus_webgraph_rcl::<NG>());
}

fn ngram_bitvec_seq_search_rcl<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    ngram_seq_search(b, new_corpus_bitvec_rcl::<NG>());
}

fn tfidf_bitvec_seq_rcl<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    tfidf_seq_search(b, new_corpus_bitvec_rcl::<NG>());
}

fn ngram_bitvec_par_search_rcl<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    ngram_par_search(b, new_corpus_bitvec_rcl::<NG>());
}

fn tfidf_bitvec_par_rcl<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    tfidf_par_search(b, new_corpus_bitvec_rcl::<NG>());
}

fn ngram_old_seq_search<NG>(b: &mut Bencher)
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let corpus = old_load_corpus::<NG>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

/// Macro to generate the benchmarks for the different ngram types.
///
/// The macro receives the name of the ngram type and the struct
/// that implements the ngram trait and prints out several benches
/// at once.

macro_rules! make_bench {
    ($gram:ident,  $ngram_type:ty) => {
        paste::item! {
            #[bench]
            fn [< $gram _webgraph_seq_search_vec >] (b: &mut Bencher) {
                ngram_webgraph_seq_search_vec::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _webgraph_par_search_vec >] (b: &mut Bencher) {
                ngram_webgraph_par_search_vec::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _tfidf_webgraph_seq_vec >] (b: &mut Bencher) {
                tfidf_webgraph_seq_vec::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _tfidf_webgraph_par_vec >] (b: &mut Bencher) {
                tfidf_webgraph_par_vec::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _webgraph_seq_search_rcl >] (b: &mut Bencher) {
                ngram_webgraph_seq_search_rcl::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _webgraph_par_search_rcl >] (b: &mut Bencher) {
                ngram_webgraph_par_search_rcl::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _tfidf_webgraph_seq_rcl >] (b: &mut Bencher) {
                tfidf_webgraph_seq_rcl::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _tfidf_webgraph_par_rcl >] (b: &mut Bencher) {
                tfidf_webgraph_par_rcl::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _bitvec_seq_search_vec >] (b: &mut Bencher) {
                ngram_bitvec_seq_search_vec::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _bitvec_par_search_vec >] (b: &mut Bencher) {
                ngram_bitvec_par_search_vec::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _tfidf_bitvec_seq_vec >] (b: &mut Bencher) {
                tfidf_bitvec_seq_vec::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _tfidf_bitvec_par_vec >] (b: &mut Bencher) {
                tfidf_bitvec_par_vec::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _bitvec_seq_search_rcl >] (b: &mut Bencher) {
                ngram_bitvec_seq_search_rcl::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _bitvec_par_search_rcl >] (b: &mut Bencher) {
                ngram_bitvec_par_search_rcl::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _tfidf_bitvec_seq_rcl >] (b: &mut Bencher) {
                tfidf_bitvec_seq_rcl::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _tfidf_bitvec_par_rcl >] (b: &mut Bencher) {
                tfidf_bitvec_par_rcl::<$ngram_type>(b);
            }

            #[bench]
            fn [< $gram _old_seq_search >] (b: &mut Bencher) {
                ngram_old_seq_search::<$ngram_type>(b);
            }
        }
    };
}

make_bench!(monogram, MonoGram<ASCIIChar>);
make_bench!(bigram, BiGram<ASCIIChar>);
make_bench!(trigram, TriGram<ASCIIChar>);
make_bench!(tetragram, TetraGram<ASCIIChar>);
make_bench!(pentagram, PentaGram<ASCIIChar>);
make_bench!(hexagram, HexaGram<ASCIIChar>);
make_bench!(heptagram, HeptaGram<ASCIIChar>);
make_bench!(octagram, OctaGram<ASCIIChar>);
