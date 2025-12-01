use criterion::{criterion_group, criterion_main, Criterion};

use ngrammatic;

// these benchmarks were taken from https://github.com/bluecatengineering/fast_radix_trie/
// which was taken from https://github.com/cloudflare/trie-hard/
// which was taken from https://github.com/michaelsproul/rust_radix_trie/

const OW_1984: &str = include_str!("../data/1984.txt");
const RANDOM: &str = include_str!("../data/random.txt");
const TOP_MILLION: &str = include_str!("../data/top-domains.txt");

fn get_novel() -> Vec<&'static str> {
    OW_1984.split(|c: char| c.is_whitespace()).collect()
}

fn get_random() -> Vec<&'static str> {
    RANDOM.split(|c: char| c.is_whitespace()).collect()
}

fn get_domains() -> Vec<&'static str> {
    TOP_MILLION.split(|c: char| c.is_whitespace()).collect()
}

fn build_corpus<It>(words: It) -> ngrammatic::Corpus
where
    It: IntoIterator,
    It::Item: AsRef<str>,
{
    ngrammatic::CorpusBuilder::new()
        .arity(2)
        .pad_full(ngrammatic::Pad::Auto)
        .fill(words)
        .finish()
}

fn build_corpus_insensitive<It>(words: It) -> ngrammatic::Corpus
where
    It: IntoIterator,
    It::Item: AsRef<str>,
{
    ngrammatic::CorpusBuilder::new()
        .arity(2)
        .key_trans(Box::new(|x| x.to_lowercase()))
        .pad_full(ngrammatic::Pad::Auto)
        .fill(words)
        .finish()
}

fn build_corpus_par<It>(words: It) -> ngrammatic::Corpus
where
    It: IntoIterator + rayon::iter::IntoParallelIterator,
    String: From<<It as rayon::iter::IntoParallelIterator>::Item>,
{
    ngrammatic::CorpusBuilder::new()
        .arity(2)
        .pad_full(ngrammatic::Pad::Auto)
        .fill_par(words)
        .finish()
}

fn build_corpus_insensitive_par<It>(words: It) -> ngrammatic::Corpus
where
    It: IntoIterator + rayon::iter::IntoParallelIterator,
    String: From<<It as rayon::iter::IntoParallelIterator>::Item>,
{
    ngrammatic::CorpusBuilder::new()
        .arity(2)
        .key_trans(Box::new(|x| x.to_lowercase()))
        .pad_full(ngrammatic::Pad::Auto)
        .fill_par(words)
        .finish()
}

fn bench_corpus_novel(c: &mut Criterion) {
    let mut group = c.benchmark_group("novel corpus creation");

    let words = get_novel();
    group.bench_function("novel parallel insertion case sensitive", |b| {
        b.iter(|| build_corpus_par(std::hint::black_box(words.clone())));
    });

    group.bench_function("novel parallel insertion case insensitive", |b| {
        b.iter(|| build_corpus_insensitive_par(std::hint::black_box(words.clone())));
    });

    group.bench_function("novel serial insertion case sensitive", |b| {
        b.iter(|| build_corpus(std::hint::black_box(words.clone())));
    });

    group.bench_function("novel serial insertion case insensitive", |b| {
        b.iter(|| build_corpus_insensitive(std::hint::black_box(words.clone())));
    });

    group.finish();
}

fn bench_corpus_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("random corpus creation");

    let words = get_random();
    group.bench_function("random text parallel insertion case sensitive", |b| {
        b.iter(|| build_corpus_par(std::hint::black_box(words.clone())));
    });

    group.bench_function("random text parallel insertion case insensitive", |b| {
        b.iter(|| build_corpus_insensitive_par(std::hint::black_box(words.clone())));
    });

    group.bench_function("random text serial insertion case sensitive", |b| {
        b.iter(|| build_corpus(std::hint::black_box(words.clone())));
    });

    group.bench_function("random text serial insertion case insensitive", |b| {
        b.iter(|| build_corpus_insensitive(std::hint::black_box(words.clone())));
    });

    group.finish();
}

fn bench_corpus_domainnames(c: &mut Criterion) {
    let mut group = c.benchmark_group("domain names corpus creation");

    let words = get_random();
    group.bench_function("domain names parallel insertion case sensitive", |b| {
        b.iter(|| build_corpus_par(std::hint::black_box(words.clone())));
    });

    group.bench_function("domain names parallel insertion case insensitive", |b| {
        b.iter(|| build_corpus_insensitive_par(std::hint::black_box(words.clone())));
    });

    group.bench_function("domain names serial insertion case sensitive", |b| {
        b.iter(|| build_corpus(std::hint::black_box(words.clone())));
    });

    group.bench_function("domain names serial insertion case insensitive", |b| {
        b.iter(|| build_corpus_insensitive(std::hint::black_box(words.clone())));
    });

    group.finish();
}

criterion_group!(
    corpus_benches,
    bench_corpus_novel,
    bench_corpus_random,
    bench_corpus_domainnames,
);

fn bench_get_novel(c: &mut Criterion) {
    let mut group = c.benchmark_group("novel corpus search comparison");
    let words = get_novel();
    let corpus = build_corpus(words.into_iter());

    group.bench_function("novel parallel search no match", |b| {
        b.iter(|| {
            corpus.search_par("ToMaTo", 0.90);
            corpus.search_par("pOtAtO", 0.90);
            corpus.search_par("CaRRoT", 0.90);
        });
    });

    group.bench_function("novel parallel search match", |b| {
        b.iter(|| {
            corpus.search_par("cabbage", 0.90);
            corpus.search_par("potato", 0.90);
            corpus.search_par("carrot", 0.90);
        });
    });

    group.bench_function("novel serial search no match", |b| {
        b.iter(|| {
            corpus.search("ToMaTo", 0.90);
            corpus.search("pOtAtO", 0.90);
            corpus.search("CaRRoT", 0.90);
        });
    });

    group.bench_function("novel serial search match", |b| {
        b.iter(|| {
            corpus.search("cabbage", 0.90);
            corpus.search("potato", 0.90);
            corpus.search("carrot", 0.90);
        });
    });

    group.finish();
}

fn bench_get_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("random text corpus search comparison");
    let words = get_random();
    let corpus = build_corpus(words.into_iter());

    group.bench_function("random text parallel search no match", |b| {
        b.iter(|| {
            corpus.search_par("ToMaTo", 0.90);
            corpus.search_par("pOtAtO", 0.90);
            corpus.search_par("CaRRoT", 0.90);
        });
    });

    group.bench_function("random text parallel search match", |b| {
        b.iter(|| {
            corpus.search_par("qsratf2ar", 0.90);
            corpus.search_par("fimm5mm2f", 0.90);
            corpus.search_par("d85rrz3l3a", 0.90);
        });
    });

    group.bench_function("random text serial search no match", |b| {
        b.iter(|| {
            corpus.search("ToMaTo", 0.90);
            corpus.search("pOtAtO", 0.90);
            corpus.search("CaRRoT", 0.90);
        });
    });

    group.bench_function("random text serial search match", |b| {
        b.iter(|| {
            corpus.search("qsratf2ar", 0.90);
            corpus.search("fimm5mm2f", 0.90);
            corpus.search("d85rrz3l3a", 0.90);
        });
    });

    group.finish();
}

fn bench_get_domainnames(c: &mut Criterion) {
    let mut group = c.benchmark_group("domain names corpus search comparison");
    let words = get_domains();
    let corpus = build_corpus(words.into_iter());

    group.bench_function("domain names parallel search no match", |b| {
        b.iter(|| {
            corpus.search_par("ToMaTo", 0.90);
            corpus.search_par("pOtAtO", 0.90);
            corpus.search_par("CaRRoT", 0.90);
        });
    });

    group.bench_function("domain names parallel search match", |b| {
        b.iter(|| {
            corpus.search_par("steampowered.com", 0.90);
            corpus.search_par("steamgifts.com", 0.90);
            corpus.search_par("steamcommunity.com", 0.90);
        });
    });

    group.bench_function("domain names serial search no match", |b| {
        b.iter(|| {
            corpus.search("ToMaTo", 0.90);
            corpus.search("pOtAtO", 0.90);
            corpus.search("CaRRoT", 0.90);
        });
    });

    group.bench_function("domain names serial search match", |b| {
        b.iter(|| {
            corpus.search("steampowered.com", 0.90);
            corpus.search("steamgifts.com", 0.90);
            corpus.search("steamcommunity.com", 0.90);
        });
    });

    group.finish();
}

criterion_group!(
    search_benches,
    bench_get_novel,
    bench_get_random,
    bench_get_domainnames,
);

criterion_main!(corpus_benches, search_benches);
