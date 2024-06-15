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

/// Returns an iterator over the taxons in the corpus.
fn iter_taxons() -> impl Iterator<Item = String> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./taxons.csv.gz").unwrap();
    let reader = BufReader::new(GzDecoder::new(file));

    let loading_bar = indicatif::ProgressBar::new(2_571_000);

    reader
        .lines()
        .progress_with(loading_bar)
        .map(|line| line.unwrap())
}

/// Returns the built RCL
fn build_rcl() -> RearCodedList {
    let mut taxons: Vec<String> = iter_taxons().collect();
    taxons.par_sort_unstable();

    let mut rcl_builder = RearCodedListBuilder::new(8);
    for taxon in taxons {
        rcl_builder.push(&taxon);
    }
    rcl_builder.build()
}

/// Returns the built trie
fn build_trie() -> Trie<u8> {
    Trie::from_iter(iter_taxons())
}

fn load_corpus_new<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar>,
{
    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::from(taxons);

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "NEW - Arity: {}, Time (ms): {}, memory (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored()
    );

    corpus
}

fn load_corpus_par_new<NG>()
where
    NG: Ngram<G = ASCIIChar>,
{
    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "NEWPAR - Arity: {}, Time (ms): {}, memory (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored(),
    );
}

fn load_corpus_rcl_par_new<NG>()
where
    NG: Ngram<G = ASCIIChar>,
{
    let start_time = std::time::Instant::now();
    let corpus: Corpus<_, NG, Lowercase<str>> = Corpus::par_from(build_rcl());

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "RCL NEWPAR - Arity: {}, Time (ms): {}, memory (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored(),
    );
}

fn load_corpus_trie_par_new<NG>() -> Corpus<Trie<u8>, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar>,
{
    let start_time = std::time::Instant::now();
    let corpus: Corpus<Trie<u8>, NG, Lowercase<str>> = Corpus::par_from(build_trie());

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "TRIE NEWPAR - Arity: {}, Time (ms): {}, memory (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored(),
    );

    corpus
        .mem_dbg(DbgFlags::default() | DbgFlags::CAPACITY | DbgFlags::HUMANIZE)
        .unwrap();

    corpus
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

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "OLD - Arity: {}, Time (ms): {}, memory (B): {}",
        arity,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored()
    );

    corpus
}

/// We allow dead code here because the version of the
/// webgraph crate that is necessary for this benchmark
/// is currently in nightly.
#[allow(dead_code)]
fn load_corpus_webgraph<NG>()
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    let corpus_webgraph: Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "WEBGRAPH - Arity: {}, Time (ms): {}, memory (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus_webgraph
            .mem_size(SizeFlags::default() | SizeFlags::FOLLOW_REFS)
            .underscored(),
    );
}

/// We allow dead code here because the version of the
/// webgraph crate that is necessary for this benchmark
/// is currently in nightly.
#[allow(dead_code)]
fn load_corpus_rcl_webgraph<NG>()
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let start_time = std::time::Instant::now();

    let corpus: Corpus<_, NG, Lowercase<str>> = Corpus::par_from(build_rcl());

    let corpus_webgraph: Corpus<_, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "RCL WEBGRAPH - Arity: {}, Time (ms): {}, memory (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus_webgraph
            .mem_size(SizeFlags::default() | SizeFlags::FOLLOW_REFS)
            .underscored()
    );
}

fn experiment<NG>()
where
    NG: Ngram<G = ASCIIChar>,
{
    let corpus = load_corpus_new::<NG>();
    log::error!(
        "Edges: {}, Ngrams: {}",
        corpus.graph().number_of_edges() * 2,
        corpus.number_of_ngrams()
    );
    load_corpus_par_new::<NG>();
    load_corpus_rcl_par_new::<NG>();
    log::warn!("The webgraph benchmarks are skipped because the necessary version of the webgraph crate is not available.");
    // load_corpus_webgraph::<NG>();
    // load_corpus_rcl_webgraph::<NG>();
    load_corpus_trie_par_new::<NG>();
    load_corpus_old(NG::ARITY);
}

fn main() {
    env_logger::builder().try_init().unwrap();
    // experiment::<UniGram<ASCIIChar>>();
    // experiment::<BiGram<ASCIIChar>>();
    experiment::<TriGram<ASCIIChar>>();
    // experiment::<TetraGram<ASCIIChar>>();
    // experiment::<PentaGram<ASCIIChar>>();
    // experiment::<HexaGram<ASCIIChar>>();
    // experiment::<HeptaGram<ASCIIChar>>();
    // experiment::<OctaGram<ASCIIChar>>();
}
