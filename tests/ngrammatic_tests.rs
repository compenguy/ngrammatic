//! Test suite for the ngrammatic crate.
use ngrammatic::traits::*;
use ngrammatic::CorpusBuilder;

fn float_approx_eq(a: f32, b: f32, epsilon: Option<f32>) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();
    let epsilon = epsilon.unwrap_or(f32::EPSILON);

    if a == b {
        // infinity/NaN/exactly equal
        true
    } else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
        // one or both is very close to zero, or they're very close to each other
        diff < (epsilon * f32::MIN_POSITIVE)
    } else {
        // relative error
        (diff / f32::min(abs_a + abs_b, f32::MAX)) < epsilon
    }
}


#[test]
fn short_string_nopad() {
    let ngram = NgramBuilder::<ArityTwo>::new("ab")
        .finish();
    assert!(ngram.contains("ab"));
}

#[test]
fn short_string_autopad() {
    let ngram = NgramBuilder::<ArityTwo>::new("ab").finish();
    assert!(ngram.contains(" a"));
    assert!(ngram.contains("ab"));
    assert!(ngram.contains("b "));
}

#[test]
fn short_string_strpad() {
    let ngram = NgramBuilder::<ArityTwo>::new("ab")
        .pad_left(Pad::Pad("--".to_string()))
        .pad_right(Pad::Pad("++".to_string()))
        .finish();
    assert!(ngram.contains("--"));
    assert!(ngram.contains("-a"));
    assert!(ngram.contains("ab"));
    assert!(ngram.contains("b+"));
    assert!(ngram.contains("++"));
}

#[test]
fn similarity_identical() {
    let ngram0 = NgramBuilder::<ArityTwo>::new("ab").finish();
    let ngram1 = NgramBuilder::<ArityTwo>::new("ab").finish();
    assert!(float_approx_eq(
        ngram0.similarity_to(&ngram1, 3.0),
        1.0,
        None,
    ));
}

#[test]
fn similarity_completelydifferent() {
    let ngram0 = NgramBuilder::<ArityTwo>::new("ab").finish();
    let ngram1 = NgramBuilder::<ArityTwo>::new("cd").finish();
    assert!(float_approx_eq(
        ngram0.similarity_to(&ngram1, 3.0),
        0.0,
        None,
    ));
}

#[test]
fn corpus_add_text_before_setting_arity() {
    let corpus = CorpusBuilder::<ArityTwo>::default()
        .fill(vec!["ab", "ba"])
        .finish();
    println!("{:?}", corpus);
}

#[test]
fn corpus_set_padding_after_adding_text() {
    let corpus = CorpusBuilder::<ArityTwo>::default()
        .fill(vec!["ab", "ba"])
        .pad_full(Pad::None)
        .finish();
    println!("{:?}", corpus);
}

#[test]
fn corpus_add_multiple() {
    let corpus = CorpusBuilder::<ArityTwo>::default()
        .pad_full(Pad::Auto)
        .fill(vec!["ab", "ba"])
        .finish();
    assert_eq!(corpus.is_empty(), false);
    assert_eq!(corpus.key("ab"), Some("ab".to_string()));
    assert_eq!(corpus.key("ba"), Some("ba".to_string()));
    assert_eq!(corpus.key("zabba"), None);
}

#[test]
fn corpus_search() {
    let corpus = CorpusBuilder::<ArityOne>::default()
        .pad_full(Pad::None)
        .fill(vec!["ab", "ba", "cd"])
        .finish();
    assert_eq!(corpus.search("ce", 0.3, 10).len(), 1);
    assert_eq!(corpus.search("ec", 0.3, 10).len(), 1);
    assert_eq!(corpus.search("b", 0.5, 10).len(), 2);
}

#[test]
fn corpus_case_insensitive_corpus_search() {
    let corpus = CorpusBuilder::<ArityOne>::default()
        .pad_full(Pad::None)
        .fill(vec!["Ab", "Ba", "Cd"])
        .case_insensitive()
        .finish();
    assert_eq!(corpus.search("ce", 0.3, 10).len(), 1);
    assert_eq!(corpus.search("ec", 0.3, 10).len(), 1);
    assert_eq!(corpus.search("b", 0.5, 10).len(), 2);
}

#[test]
fn corpus_case_insensitive_corpus_search_terms() {
    let corpus = CorpusBuilder::<ArityOne>::default()
        .pad_full(Pad::None)
        .fill(vec!["Ab", "Ba", "Cd"])
        .case_insensitive()
        .finish();
    assert_eq!(corpus.search("cE", 0.3, 10).len(), 1);
    assert_eq!(corpus.search("eC", 0.3, 10).len(), 1);
    assert_eq!(corpus.search("b", 0.5, 10).len(), 2);
}

#[test]
fn corpus_search_emoji() {
    let corpus = CorpusBuilder::<ArityOne>::default()
        .pad_full(Pad::None)
        .fill(vec!["\u{1f60f}\u{1f346}", "ba", "cd"])
        .finish();

    assert_eq!(corpus.search("ac", 0.3, 10).len(), 2);
    assert_eq!(corpus.search("\u{1f346}d", 0.3, 10).len(), 2);
}

#[test]
fn corpus_search_small_word() {
    let corpus = CorpusBuilder::<ArityFive>::default()
        .pad_full(Pad::Pad(" ".to_string()))
        .fill(vec!["ab"])
        .case_insensitive()
        .finish();
    assert!(corpus.search("a", 0., 10).is_empty());
}

#[test]
fn corpus_search_empty_string() {
    let corpus = CorpusBuilder::<ArityThree>::default()
        .pad_full(Pad::Pad(" ".to_string()))
        .fill(vec!["a"])
        .case_insensitive()
        .finish();
    assert!(corpus.search("", 0., 10).is_empty());
}
