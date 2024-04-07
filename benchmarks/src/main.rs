//! # Benchmarks
//!
//! This crate contains memory benchmarks for the `ngrammatic` crate.
//! For the time-related benchmarks, please refer to the benches directory.
//!
//! The memory benchmarks compare different support data-structures that can be used to store the n-grams.
//! As corpus we use the `../taxons.csv.gz` file, which contains a single column with the scientific names
//! of the taxons as provided by NCBI Taxonomy.
use core::fmt::Debug;
use indicatif::ProgressIterator;
use mem_dbg::*;
use ngrammatic::prelude::*;
use rayon::prelude::*;

/// Returns an iterator over the taxons in the corpus.
fn iter_taxons() -> impl Iterator<Item = String> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./taxons.csv.gz").unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    reader.lines().map(|line| line.unwrap())
}

/// Returns bigram corpus.
fn load_corpus<NG>()
where
    NG: PaddableNgram<G = ASCIIChar>,
{
    // let number_of_taxons = 2_571_000;
    let number_of_taxons = 1_00;

    let loading_bar = indicatif::ProgressBar::new(number_of_taxons as u64);

    let progress_style = indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-");

    loading_bar.set_style(progress_style);

    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons()
        .take(number_of_taxons)
        .progress_with(loading_bar)
        .collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::from(taxons);

    let end_time = std::time::Instant::now();
    let duration = end_time - start_time;

    println!("Time taken to load corpus: {:?}", duration);

    corpus
        .mem_dbg(DbgFlags::HUMANIZE | DbgFlags::PERCENTAGE | DbgFlags::TYPE_NAME)
        .unwrap();
}

/// Returns bigram corpus.
fn load_corpus_webgraph<NG>()
where
    NG: PaddableNgram<G = ASCIIChar> + Debug,
{
    // let number_of_taxons = 2_571_000;
    let number_of_taxons = 2_571_000;

    let loading_bar = indicatif::ProgressBar::new(number_of_taxons as u64);

    let progress_style = indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-");

    loading_bar.set_style(progress_style);

    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons()
        .take(number_of_taxons)
        .progress_with(loading_bar)
        .collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    let end_time = std::time::Instant::now();
    let duration = end_time - start_time;

    log::info!("Time taken to load corpus: {:?}", duration);

    log::info!("\n{}", corpus.report());

    log::info!("The 5 most frequent ngrams are:");
    let top_k_ngram = corpus.top_k_ngrams(5);
    top_k_ngram
        .iter()
        .for_each(|(degree, ngram)| log::info!("{}: {:?}", degree.underscored(), ngram));

    log::info!("The following are 10 keys associated to the most frequent ngram:");
    let top_k_ngram = top_k_ngram[0].1.clone();
    for key in corpus.keys_from_ngram(top_k_ngram).unwrap().take(10) {
        log::info!("{}", key);
    }

    corpus
        .mem_dbg(DbgFlags::HUMANIZE | DbgFlags::PERCENTAGE | DbgFlags::TYPE_NAME)
        .unwrap();

    // We write the node degrees to a CSV file.
    let mut writer = csv::Writer::from_path("node_degrees.csv").unwrap();
    writer.write_record(&["degree"]).unwrap();
    corpus
        .graph()
        .degrees()
        .for_each(|degree| writer.write_record(&[degree.to_string()]).unwrap());

    // We close the writer.
    writer.flush().unwrap();

    // We write the cooccurrence weigts to a CSV file.
    let mut writer = csv::Writer::from_path("cooccurrence_weights.csv").unwrap();
    writer.write_record(&["weight"]).unwrap();
    corpus
        .cooccurrences()
        .for_each(|weight| writer.write_record(&[weight.to_string()]).unwrap());

    // We close the writer.
    writer.flush().unwrap();

    // log::info!("Creating webgraph from corpus");

    // let corpus_webgraph: Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph> = Corpus::from(corpus);

    // log::info!("Created webgraph from corpus");
}

/// Returns bigram corpus.
fn bigram_corpus() {
    load_corpus::<BiGram<ASCIIChar>>()
}

/// Returns trigram corpus.
fn trigram_corpus() {
    load_corpus::<TriGram<ASCIIChar>>()
}

fn main() {
    env_logger::builder().try_init().unwrap();
    load_corpus_webgraph::<TriGram<ASCIIChar>>();
    // trigram_corpus();
}
