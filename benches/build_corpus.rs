#![feature(test)]
extern crate test;
use std::fmt::Debug;

use ngrammatic::prelude::*;
use rayon::slice::ParallelSliceMut;
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

fn vec_rcl_load_corpus<NG>() -> Corpus<RearCodedList, NG, Lowercase<str>, WeightedVecBipartiteGraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let corpus: Corpus<RearCodedList, NG, Lowercase<str>, WeightedVecBipartiteGraph> =
        Corpus::from(build_rcl());

    corpus
}

fn bitvec_rcl_load_corpus<NG>(
) -> Corpus<RearCodedList, NG, Lowercase<str>, WeightedBitFieldBipartiteGraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let corpus: Corpus<RearCodedList, NG, Lowercase<str>, WeightedBitFieldBipartiteGraph> =
        Corpus::from(build_rcl());

    corpus
}

fn bitvec_rcl_par_load_corpus<NG>() -> Corpus<RearCodedList, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let corpus: Corpus<RearCodedList, NG, Lowercase<str>> = Corpus::par_from(build_rcl());

    corpus
}

fn webgraph_rcl_load_corpus<NG>() -> Corpus<RearCodedList, NG, Lowercase<str>, BiWebgraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let corpus: Corpus<RearCodedList, NG, Lowercase<str>, WeightedBitFieldBipartiteGraph> =
        Corpus::from(build_rcl());
    let corpus_webgraph: Corpus<RearCodedList, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    corpus_webgraph
}

fn webgraph_rcl_par_load_corpus<NG>() -> Corpus<RearCodedList, NG, Lowercase<str>, BiWebgraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let corpus: Corpus<RearCodedList, NG, Lowercase<str>, WeightedBitFieldBipartiteGraph> =
        Corpus::par_from(build_rcl());
    let corpus_webgraph: Corpus<RearCodedList, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    corpus_webgraph
}

fn vec_load_corpus<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>, WeightedVecBipartiteGraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>, WeightedVecBipartiteGraph> =
        Corpus::from(taxons);

    corpus
}

fn bitvec_load_corpus<NG>(
) -> Corpus<Vec<String>, NG, Lowercase<str>, WeightedBitFieldBipartiteGraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>, WeightedBitFieldBipartiteGraph> =
        Corpus::from(taxons);

    corpus
}

fn bitvec_par_load_corpus<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    corpus
}

fn webgraph_load_corpus<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>, WeightedBitFieldBipartiteGraph> =
        Corpus::from(taxons);
    let corpus_webgraph: Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    corpus_webgraph
}

fn webgraph_par_load_corpus<NG>() -> Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph>
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>, WeightedBitFieldBipartiteGraph> =
        Corpus::par_from(taxons);
    let corpus_webgraph: Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    corpus_webgraph
}

fn hashmap_load_corpus<NG>() -> ngrammatic_old::Corpus
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    let mut corpus = ngrammatic_old::CorpusBuilder::new()
        .arity(NG::ARITY)
        .pad_full(ngrammatic_old::Pad::Auto)
        .finish();

    for line in iter_taxons() {
        corpus.add_text(&line);
    }

    corpus
}

macro_rules! make_bench {
    ($gram:ident, $structure:ident, $ngram_type:ty) => {
        paste::item! {
            #[bench]
            fn [< $gram _ $structure _load_corpus >] (b: &mut Bencher) {
                // We load it first once outside the benchmark
                // to avoid the noise related to not having the
                // textual file loaded in memory.
                let _ = [< $structure _load_corpus >]::<$ngram_type>();

                b.iter(|| {
                    // Then we measure the time it takes to recreate
                    // the corpus from scratch several times.
                    black_box({
                        let _ = [< $structure _load_corpus >]::<$ngram_type>();
                    });
                });
            }
        }
    };
}

macro_rules! make_multi_bench {
    ($structure:ident) => {
        make_bench!(monogram, $structure, UniGram<ASCIIChar>);
        make_bench!(bigram, $structure, BiGram<ASCIIChar>);
        make_bench!(trigram, $structure, TriGram<ASCIIChar>);
        make_bench!(tetragram, $structure, TetraGram<ASCIIChar>);
        make_bench!(pentagram, $structure, PentaGram<ASCIIChar>);
        make_bench!(hexagram, $structure, HexaGram<ASCIIChar>);
        make_bench!(heptagram, $structure, HeptaGram<ASCIIChar>);
        make_bench!(octagram, $structure, OctaGram<ASCIIChar>);
    };
}

make_multi_bench!(vec);
make_multi_bench!(bitvec);
make_multi_bench!(webgraph);
make_multi_bench!(hashmap);
make_multi_bench!(vec_rcl);
make_multi_bench!(bitvec_rcl);
make_multi_bench!(webgraph_rcl);
make_multi_bench!(bitvec_rcl_par);
make_multi_bench!(webgraph_rcl_par);
make_multi_bench!(bitvec_par);
make_multi_bench!(webgraph_par);
