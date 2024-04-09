#![feature(test)]
extern crate test;
use std::fmt::Debug;

use ngrammatic::prelude::*;
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

/// Returns ngram par-corpus.
fn new_par_load_corpus<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    corpus
}

/// Returns ngram webgraph-based par-corpus.
fn new_par_load_corpus_webgraph<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);
    let corpus_webgraph: Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    corpus_webgraph
}

/// Returns old ngram corpus.
fn old_load_corpus(arity: usize) -> ngrammatic_old::Corpus {
    let mut corpus = ngrammatic_old::CorpusBuilder::new()
        .arity(arity)
        .pad_full(ngrammatic_old::Pad::Auto)
        .finish();

    for line in iter_taxons() {
        corpus.add_text(&line);
    }

    corpus
}

#[bench]
fn monogram_ngram_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<MonoGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn monogram_tfidf_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<MonoGram<ASCIIChar>>();
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
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn monogram_ngram_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<MonoGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn monogram_tfidf_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<MonoGram<ASCIIChar>>();
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
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn monogram_ngram_search_old(b: &mut Bencher) {
    let corpus = old_load_corpus(1);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

#[bench]
fn bigram_ngram_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<BiGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn bigram_tfidf_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<BiGram<ASCIIChar>>();
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
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn bigram_ngram_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<BiGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn bigram_tfidf_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<BiGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn bigram_ngram_search_old(b: &mut Bencher) {
    let corpus = old_load_corpus(2);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

#[bench]
fn trigram_ngram_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<TriGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn trigram_tf_idf_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<TriGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn trigram_ngram_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<TriGram<ASCIIChar>>();
    let search_config = NgramSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn trigram_tf_idf_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<TriGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn trigram_ngram_search_old(b: &mut Bencher) {
    let corpus = old_load_corpus(3);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

#[bench]
fn tetragram_ngram_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<TetraGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn tetragram_tf_idf_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<TetraGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn tetragram_ngram_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<TetraGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn tetragram_tf_idf_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<TetraGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn tetragram_ngram_search_old(b: &mut Bencher) {
    let corpus = old_load_corpus(4);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

#[bench]
fn pentagram_ngram_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<PentaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn pentagram_tf_idf_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<PentaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn pentagram_ngram_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<PentaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn pentagram_tf_idf_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<PentaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn pentagram_ngram_search_old(b: &mut Bencher) {
    let corpus = old_load_corpus(5);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

#[bench]
fn hexagram_ngram_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<HexaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn hexagram_tf_idf_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<HexaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn hexagram_ngram_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<HexaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn hexagram_tf_idf_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<HexaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn hexagram_ngram_search_old(b: &mut Bencher) {
    let corpus = old_load_corpus(6);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

#[bench]
fn heptagram_ngram_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<HeptaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn heptagram_tf_idf_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<HeptaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn heptagram_ngram_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<HeptaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn heptagram_tf_idf_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<HeptaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn heptagram_ngram_search_old(b: &mut Bencher) {
    let corpus = old_load_corpus(7);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

#[bench]
fn octagram_ngram_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<OctaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn octagram_tf_idf_search_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<OctaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn octagram_ngram_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<OctaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn octagram_tf_idf_search_par_new(b: &mut Bencher) {
    let corpus = new_par_load_corpus::<OctaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn octagram_ngram_search_old(b: &mut Bencher) {
    let corpus = old_load_corpus(8);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
            let _ = corpus.search("Doggus Lionenus", 0.6);
        });
    });
}

#[bench]
fn monogram_ngram_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<MonoGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn monogram_tfidf_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<MonoGram<ASCIIChar>>();
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
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn monogram_ngram_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<MonoGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn monogram_tfidf_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<MonoGram<ASCIIChar>>();
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
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn bigram_ngram_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<BiGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn bigram_tfidf_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<BiGram<ASCIIChar>>();
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
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn bigram_ngram_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<BiGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn bigram_tfidf_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<BiGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn trigram_ngram_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<TriGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn trigram_tf_idf_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<TriGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn trigram_ngram_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<TriGram<ASCIIChar>>();
    let search_config = NgramSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn trigram_tf_idf_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<TriGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn tetragram_ngram_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<TetraGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn tetragram_tf_idf_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<TetraGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn tetragram_ngram_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<TetraGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn tetragram_tf_idf_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<TetraGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn pentagram_ngram_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<PentaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn pentagram_tf_idf_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<PentaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn pentagram_ngram_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<PentaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn pentagram_tf_idf_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<PentaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn hexagram_ngram_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<HexaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn hexagram_tf_idf_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<HexaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn hexagram_ngram_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<HexaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn hexagram_tf_idf_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<HexaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn heptagram_ngram_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<HeptaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn heptagram_tf_idf_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<HeptaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn heptagram_ngram_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<HeptaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn heptagram_tf_idf_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<HeptaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn octagram_ngram_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<OctaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_search("Felis Caninus", search_config);
            let _ = corpus.ngram_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn octagram_tf_idf_search_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<OctaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn octagram_ngram_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<OctaGram<ASCIIChar>>();
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
            let _ = corpus.ngram_par_search("Doggus Lionenus", search_config);
        });
    });
}

#[bench]
fn octagram_tf_idf_search_par_new_webgraph(b: &mut Bencher) {
    let corpus = new_par_load_corpus_webgraph::<OctaGram<ASCIIChar>>();
    let search_config = TFIDFSearchConfig::default()
        .set_minimum_similarity_score(0.6)
        .unwrap()
        .set_maximum_number_of_results(10);

    b.iter(|| {
        black_box({
            let _ = corpus.tf_idf_par_search("Felis Caninus", search_config);
            let _ = corpus.tf_idf_par_search("Doggus Lionenus", search_config);
        });
    });
}
