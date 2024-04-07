#![feature(test)]
extern crate test;
use indicatif::ProgressIterator;
use ngrammatic::*;
use test::{black_box, Bencher};

/// Returns an iterator over the taxons in the corpus.
fn iter_taxons() -> impl Iterator<Item = String> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./benchmarks/taxons.csv.gz").unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    reader.lines().map(|line| line.unwrap())
}

fn load_corpus() -> Corpus<'static, ArityTwo, Lower, String, usize> {
    let mut corpus = CorpusBuilder::<ArityTwo>::default()
        // .pad_full(Pad::Auto)
        .lower()
        .finish();

    let number_of_taxons = 5_000;

    let loading_bar = indicatif::ProgressBar::new(number_of_taxons as u64);

    let progress_style = indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-");

    loading_bar.set_style(progress_style);

    for taxon in iter_taxons()
        .progress_with(loading_bar)
        .take(number_of_taxons)
    {
        corpus.push(taxon)
    }

    corpus
}

#[bench]
fn build_corpus_2(b: &mut Bencher) {
    // We load it first once outside the benchmark
    // to avoid the noise related to not having the
    // textual file loaded in memory.
    let _ = load_corpus();

    b.iter(|| {
        // Then we measure the time it takes to recreate
        // the corpus from scratch several times.
        black_box({
            let _ = load_corpus();
        });
    });
}
