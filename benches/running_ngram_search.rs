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

fn new_load_corpus<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::from(taxons);

    corpus
}

fn new_par_load_corpus<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    corpus
}

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
fn ngram_search_corpus_monogram_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<MonoGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_monogram_par_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<MonoGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_monogram_old(b: &mut Bencher) {
    let corpus = old_load_corpus(1);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
        });
    });
}

#[bench]
fn ngram_search_corpus_bigram_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<BiGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_bigram_par_new(b: &mut Bencher) {
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
        });
    });
}

#[bench]
fn ngram_search_corpus_bigram_old(b: &mut Bencher) {
    let corpus = old_load_corpus(2);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
        });
    });
}

#[bench]
fn ngram_search_corpus_trigram_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<TriGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_trigram_par_new(b: &mut Bencher) {
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
            let _ = corpus.ngram_par_search("Felis Caninus", search_config);
        });
    });
}

#[bench]
fn ngram_search_corpus_trigram_old(b: &mut Bencher) {
    let corpus = old_load_corpus(3);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
        });
    });
}

#[bench]
fn ngram_search_corpus_tetragram_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<TetraGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_tetragram_par_new(b: &mut Bencher) {
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
        });
    });
}

#[bench]
fn ngram_search_corpus_tetragram_old(b: &mut Bencher) {
    let corpus = old_load_corpus(4);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
        });
    });
}

#[bench]
fn ngram_search_corpus_pentagram_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<PentaGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_pentagram_par_new(b: &mut Bencher) {
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
        });
    });
}

#[bench]
fn ngram_search_corpus_pentagram_old(b: &mut Bencher) {
    let corpus = old_load_corpus(5);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
        });
    });
}

#[bench]
fn ngram_search_corpus_hexagram_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<HexaGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_hexagram_par_new(b: &mut Bencher) {
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
        });
    });
}

#[bench]
fn ngram_search_corpus_hexagram_old(b: &mut Bencher) {
    let corpus = old_load_corpus(6);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
        });
    });
}

#[bench]
fn ngram_search_corpus_heptagram_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<HeptaGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_heptagram_par_new(b: &mut Bencher) {
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
        });
    });
}

#[bench]
fn ngram_search_corpus_heptagram_old(b: &mut Bencher) {
    let corpus = old_load_corpus(7);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
        });
    });
}

#[bench]
fn ngram_search_corpus_octagram_new(b: &mut Bencher) {
    let corpus = new_load_corpus::<OctaGram<ASCIIChar>>();
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
        });
    });
}

#[bench]
fn ngram_search_corpus_octagram_par_new(b: &mut Bencher) {
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
        });
    });
}

#[bench]
fn ngram_search_corpus_octagram_old(b: &mut Bencher) {
    let corpus = old_load_corpus(8);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = corpus.search("Felis Caninus", 0.6);
        });
    });
}