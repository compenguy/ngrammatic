//! # Benchmarks
//!
//! This crate contains memory benchmarks for the `ngrammatic` crate.
//! For the time-related benchmarks, please refer to the benches directory.
//!
//! The memory benchmarks compare different support data-structures that can be used to store the n-grams.
//! As corpus we use the `./taxons.csv.gz` file, which contains a single column with the scientific names
//! of the taxons as provided by NCBI Taxonomy.
use core::fmt::Debug;
use indicatif::ProgressIterator;
use mem_dbg::*;
use ngrammatic::prelude::*;
use rayon::prelude::*;
use std::io::Write;


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
fn load_corpus_new<NG>()
where
    NG: Ngram<G = ASCIIChar>,
{
    let number_of_taxons = 2_571_000;
    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::from(taxons);

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    log::info!(
        "NEW - Arity: {}, Time: {:?}, memory: {:?}",
        NG::ARITY,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored()
    );
}

/// Returns bigram corpus.
fn load_corpus_par_new<NG>()
where
    NG: Ngram<G = ASCIIChar>,
{
    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    log::info!(
        "NEWPAR - Arity: {}, Time: {:?}, memory: {:?}",
        NG::ARITY,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored()
    );
}

fn load_corpus_old(arity: usize) -> ngrammatic_old::Corpus {
    let start_time = std::time::Instant::now();
    let mut corpus: ngrammatic_old::Corpus = ngrammatic_old::CorpusBuilder::new()
        .arity(arity)
        .pad_full(ngrammatic_old::Pad::Auto)
        .finish();

    for line in iter_taxons() {
        corpus.add_text(&line);
    }

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    log::info!(
        "OLD - Arity: {}, Time: {:?}, memory: {:?}",
        arity,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored()
    );

    corpus
}

/// Returns bigram corpus.
fn load_corpus_webgraph<NG>()
where
    NG: Ngram<G = ASCIIChar> + Debug,
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
    let duration: usize = (end_time - start_time).as_millis() as usize;

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

    log::info!("Creating webgraph from corpus");

    let corpus_webgraph: Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph> = Corpus::from(corpus);

    log::info!("Created webgraph from corpus");
}

/// Returns bigram corpus.
fn bigram_corpus() {
    load_corpus_new::<BiGram<ASCIIChar>>()
}

/// Returns trigram corpus.
fn trigram_corpus() {
    load_corpus_new::<TriGram<ASCIIChar>>()
}

fn main() {
    env_logger::builder().try_init().unwrap();
    load_corpus_new::<MonoGram<ASCIIChar>>();
    load_corpus_par_new::<MonoGram<ASCIIChar>>();
    load_corpus_old(1);
    load_corpus_new::<BiGram<ASCIIChar>>();
    load_corpus_par_new::<BiGram<ASCIIChar>>();
    load_corpus_old(2);
    load_corpus_new::<TriGram<ASCIIChar>>();
    load_corpus_par_new::<TriGram<ASCIIChar>>();
    load_corpus_old(3);
    load_corpus_new::<TetraGram<ASCIIChar>>();
    load_corpus_par_new::<TetraGram<ASCIIChar>>();
    load_corpus_old(4);
    load_corpus_new::<PentaGram<ASCIIChar>>();
    load_corpus_par_new::<PentaGram<ASCIIChar>>();
    load_corpus_old(5);
    load_corpus_new::<HexaGram<ASCIIChar>>();
    load_corpus_par_new::<HexaGram<ASCIIChar>>();
    load_corpus_old(6);
    load_corpus_new::<HeptaGram<ASCIIChar>>();
    load_corpus_par_new::<HeptaGram<ASCIIChar>>();
    load_corpus_old(7);
    load_corpus_new::<OctaGram<ASCIIChar>>();
    load_corpus_par_new::<OctaGram<ASCIIChar>>();
    load_corpus_old(8);
}
