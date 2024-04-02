//! # Benchmarks
//!
//! This crate contains memory benchmarks for the `ngrammatic` crate.
//! For the time-related benchmarks, please refer to the benches directory.
//!
//! The memory benchmarks compare different support data-structures that can be used to store the n-grams.
//! As corpus we use the `../taxons.csv.gz` file, which contains a single column with the scientific names
//! of the taxons as provided by NCBI Taxonomy.
use indicatif::ProgressIterator;
use mem_dbg::*;
use ngrammatic::{CorpusBuilder, Pad};

/// Returns an iterator over the taxons in the corpus.
fn iter_taxons() -> impl Iterator<Item = String> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./taxons.csv.gz").unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    reader.lines().map(|line| line.unwrap())
}

/// Returns human readable size.
/// 
/// 

fn main() {
    let mut corpus = CorpusBuilder::<2>::default()
        .pad_full(Pad::Auto)
        .case_insensitive()
        .finish();

    let number_of_taxons = 2_571_000;

    let loading_bar = indicatif::ProgressBar::new(number_of_taxons as u64);

    let progress_style  = indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").unwrap()
        .progress_chars("#>-");

    loading_bar.set_style(progress_style);

    let start_time = std::time::Instant::now();
    for taxon in iter_taxons().progress_with(loading_bar) {
        corpus.add_text(&taxon)
    }
    let end_time = std::time::Instant::now();
    let duration = end_time - start_time;
    
    println!("Time taken to load corpus: {:?}", duration);

    corpus.mem_dbg(DbgFlags::HUMANIZE | DbgFlags::PERCENTAGE | DbgFlags::TYPE_NAME | DbgFlags::FOLLOW_REFS).unwrap();
}
