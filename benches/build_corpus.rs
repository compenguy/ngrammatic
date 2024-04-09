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
    let number_of_taxons = 10_000;

    let file = File::open("./benchmarks/taxons.csv.gz").unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    reader
        .lines()
        .take(number_of_taxons)
        .map(|line| line.unwrap())
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

fn new_load_corpus_webgraph<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::from(taxons);
    let corpus_webgraph: Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    corpus_webgraph
}

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
fn build_corpus_monogram_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus::<MonoGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus::<MonoGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_monogram_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus_webgraph::<MonoGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus_webgraph::<MonoGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_monogram_par_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus::<MonoGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus::<MonoGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_monogram_par_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus_webgraph::<MonoGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus_webgraph::<MonoGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_monogram_old(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = old_load_corpus(1);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = old_load_corpus(1);
        });
    });
}

#[bench]
fn build_corpus_bigram_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus::<BiGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus::<BiGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_bigram_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus_webgraph::<BiGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus_webgraph::<BiGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_bigram_par_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus::<BiGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus::<BiGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_bigram_par_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus_webgraph::<BiGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus_webgraph::<BiGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_bigram_old(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = old_load_corpus(2);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = old_load_corpus(2);
        });
    });
}

#[bench]
fn build_corpus_trigram_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus::<TriGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus::<TriGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_trigram_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus_webgraph::<TriGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus_webgraph::<TriGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_trigram_par_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus::<TriGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus::<TriGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_trigram_par_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus_webgraph::<TriGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus_webgraph::<TriGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_trigram_old(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = old_load_corpus(3);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = old_load_corpus(3);
        });
    });
}

#[bench]
fn build_corpus_tetragram_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus::<TetraGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus::<TetraGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_tetragram_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus_webgraph::<TetraGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus_webgraph::<TetraGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_tetragram_par_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus::<TetraGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus::<TetraGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_tetragram_par_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus_webgraph::<TetraGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus_webgraph::<TetraGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_tetragram_old(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = old_load_corpus(4);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = old_load_corpus(4);
        });
    });
}

#[bench]
fn build_corpus_pentagram_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus::<PentaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus::<PentaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_pentagram_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus_webgraph::<PentaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus_webgraph::<PentaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_pentagram_par_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus::<PentaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus::<PentaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_pentagram_par_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus_webgraph::<PentaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus_webgraph::<PentaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_pentagram_old(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = old_load_corpus(5);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = old_load_corpus(5);
        });
    });
}

#[bench]
fn build_corpus_hexagram_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus::<HexaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus::<HexaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_hexagram_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus_webgraph::<HexaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus_webgraph::<HexaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_hexagram_par_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus::<HexaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus::<HexaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_hexagram_par_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus_webgraph::<HexaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus_webgraph::<HexaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_hexagram_old(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = old_load_corpus(6);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = old_load_corpus(6);
        });
    });
}

#[bench]
fn build_corpus_heptagram_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus::<HeptaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus::<HeptaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_heptagram_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus_webgraph::<HeptaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus_webgraph::<HeptaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_heptagram_par_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus::<HeptaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus::<HeptaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_heptagram_par_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus_webgraph::<HeptaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus_webgraph::<HeptaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_heptagram_old(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = old_load_corpus(7);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = old_load_corpus(7);
        });
    });
}

#[bench]
fn build_corpus_octagram_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus::<OctaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus::<OctaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_octagram_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_load_corpus_webgraph::<OctaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_load_corpus_webgraph::<OctaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_octagram_par_new(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus::<OctaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus::<OctaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_octagram_par_new_webgraph(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = new_par_load_corpus_webgraph::<OctaGram<ASCIIChar>>();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = new_par_load_corpus_webgraph::<OctaGram<ASCIIChar>>();
        });
    });
}

#[bench]
fn build_corpus_octagram_old(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = old_load_corpus(8);

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = old_load_corpus(8);
        });
    });
}
