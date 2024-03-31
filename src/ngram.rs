use std::collections::HashMap;
use std::f32;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Pad, SearchResult};

/// Stores a "word", with all its n-grams. The "arity" member determines the
/// value of "n" used in generating the n-grams.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ngram {
    /// The "symbol size" for the ngrams
    pub arity: usize,
    /// The text for which ngrams were generated
    pub text: String,
    /// The text for which ngrams were generated, with the padding
    /// used for generating the ngrams
    pub text_padded: String,
    /// A collection of all generated ngrams for the text, with a
    /// count of how many times that ngram appears in the text
    // TODO: this should benefit from const generics, as a [u8; arity]
    // significantly reducing overhead
    // to a size of `arity` bytes, which is typically pretty small
    pub grams: HashMap<String, usize>,
}

impl PartialEq for Ngram {
    fn eq(&self, other: &Self) -> bool {
        self.text_padded == other.text_padded && self.arity == other.arity
    }
}
impl Eq for Ngram {}

impl Hash for Ngram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text_padded.hash(state);
        self.arity.hash(state);
    }
}

// TODO: When rust adds const generics
// (see https://github.com/rust-lang/rust/issues/44580)
// switch Ngram's "arity" member to be a const generic
// on Ngram, and implement From(String) so that we can
// do things like Ngram::<3>::From(text) to construct
// new ngrams

impl Ngram {
    /// Static method to calculate `Ngram` similarity based on samegram count,
    /// allgram count, and a `warp` factor.
    pub(crate) fn similarity(samegram_count: usize, allgram_count: usize, warp: f32) -> f32 {
        if allgram_count == 0 {
            return 0.0;
        }
        let warp = warp.clamp(1.0, 3.0);
        let samegrams = samegram_count as f32;
        let allgrams = allgram_count as f32;
        if (warp - 1.0).abs() < 0.0000000001 {
            samegrams / allgrams
        } else {
            let diffgrams = allgrams - samegrams;
            (allgrams.powf(warp) - diffgrams.powf(warp)) / (allgrams.powf(warp))
        }
    }

    /// Calculate the similarity of this `Ngram` and an `other`, for a given `warp`
    /// factor (clamped to the range 1.0 to 3.0).
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").finish();
    /// let b = NgramBuilder::new("tomacco").finish();
    /// println!("Similarity factor for {} and {}: {:.0}%", a.text, b.text, a.similarity_to(&b, 2.0) *
    /// 100.0);
    /// # }
    /// ```
    pub fn similarity_to(&self, other: &Ngram, warp: f32) -> f32 {
        let warp = warp.clamp(1.0, 3.0);
        let samegram_count = self.count_samegrams(other);
        let allgram_count = self.count_allgrams(other);
        Ngram::similarity(samegram_count, allgram_count, warp)
    }

    /// Determines if this `Ngram` matches a given `other` `Ngram`, for a given
    /// `threshold` of certainty. This is equivalent to `matches_with_warp` and a warp
    /// of 2.0.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").finish();
    /// let b = NgramBuilder::new("tomacco").finish();
    /// if let Some(word_match) = a.matches(&b, 0.40) {
    ///     println!("{} matches {} with {:.0}% certainty", a.text, b.text, word_match.similarity *
    ///     100.0);
    /// } else {
    ///     println!("{} doesn't look anything like {}.", a.text, b.text);
    /// }
    /// # }
    /// ```
    pub fn matches(&self, other: &Ngram, threshold: f32) -> Option<SearchResult> {
        self.matches_with_warp(other, 2.0, threshold)
    }

    /// Determines if this `Ngram` matches a given `other` `Ngram`, with the specified warp
    /// (clamped to the range 1.0 to 3.0), and for a given `threshold` of certainty.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").finish();
    /// let b = NgramBuilder::new("tomacco").finish();
    /// if let Some(word_match) = a.matches_with_warp(&b, 2.0, 0.40) {
    ///     println!("{} matches {} with {:.0}% certainty", a.text, b.text, word_match.similarity *
    ///     100.0);
    /// } else {
    ///     println!("{} doesn't look anything like {}.", a.text, b.text);
    /// }
    /// # }
    /// ```
    pub fn matches_with_warp(
        &self,
        other: &Ngram,
        warp: f32,
        threshold: f32,
    ) -> Option<SearchResult> {
        let similarity = self.similarity_to(other, warp);
        if similarity >= threshold {
            Some(SearchResult::new(other.text.clone(), similarity))
        } else {
            None
        }
    }

    /// Returns the count of symmetrically differing grams between this
    /// `Ngram` and the `other` `Ngram`.
    #[allow(dead_code)]
    pub(crate) fn count_diffgrams(&self, other: &Ngram) -> usize {
        self.count_allgrams(other) - self.count_samegrams(other)
    }

    /// Returns the total number of unique grams between this
    /// `Ngram` and the `other` `Ngram`.
    pub(crate) fn count_allgrams(&self, other: &Ngram) -> usize {
        // This is a shortcut that counts all grams between both ngrams
        // Then subtracts out one instance of the grams that are in common
        let self_length = self.text_padded.chars().count();
        let other_length = other.text_padded.chars().count();
        if self_length < self.arity || other_length < self.arity {
            0 // if either ngram is too small, they can't share a common gram
        } else {
            self_length + other_length - (2 * self.arity) + 2 - self.count_samegrams(other)
        }
    }

    /// Returns a count of grams that are common between this
    /// `Ngram` and the `other` `Ngram`.
    pub(crate) fn count_samegrams(&self, other: &Ngram) -> usize {
        let mut sames: usize = 0;
        for key in self.grams.keys() {
            let selfcount = self.count_gram(key.as_ref());
            let othercount = other.count_gram(key.as_ref());
            sames += selfcount.min(othercount);
        }
        sames
    }

    /// Return the number of times a particular `gram` appears in the `Ngram`
    /// text.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").arity(2).finish();
    /// println!("Number of times the 'to' bigram appears in {}: {}", a.text, a.count_gram("to"));
    /// # }
    /// ```
    pub fn count_gram(&self, gram: &str) -> usize {
        match self.grams.get(gram) {
            Some(count) => *count,
            None => 0,
        }
    }

    /// Return the total number of grams generated for the `Ngram` text.
    pub fn count_grams(&self) -> usize {
        self.grams.values().sum()
    }

    /// If the set of grams is empty.
    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.count_grams() == 0
    }

    /// If the set of grams contains the specified `gram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").arity(2).finish();
    /// if a.contains("to") {
    ///     println!("{} contains the bigram 'to'!", a.text);
    /// }
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn contains(&self, gram: &str) -> bool {
        self.count_gram(gram) > 0
    }

    /// Private method that initializes an `Ngram` by calculating all of its
    /// grams.
    fn init(&mut self) {
        if self.arity > self.text_padded.len() {
            return;
        }
        let chars_padded: Vec<char> = self.text_padded.chars().collect();
        let mut tmp = String::with_capacity(self.arity);
        for window in chars_padded.windows(self.arity) {
            tmp.clear();
            tmp.extend(window.iter());
            let count = self.grams.entry(tmp.clone()).or_insert(0);
            *count += 1;
        }
    }
}

/// Build an `Ngram`, one setting at a time.
// We provide a builder for ngrams to ensure initialization operations are
// performed in the correct order, without requiring an extensive parameter list
// to a constructor method, and allowing default values by omission.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NgramBuilder {
    arity: usize,
    pad_left: Pad,
    pad_right: Pad,
    text: String,
}

impl NgramBuilder {
    /// Initialize a new instance of an `NgramBuilder`, with a default `arity`
    /// of 2, padding set to `Auto`, for the given `text`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").arity(2).finish();
    /// if a.contains("to") {
    ///     println!("{} contains the bigram 'to'!", a.text);
    /// }
    /// # }
    /// ```
    pub fn new(text: &str) -> Self {
        NgramBuilder {
            arity: 2,
            pad_left: Pad::Auto,
            pad_right: Pad::Auto,
            text: text.to_string(),
        }
    }

    /// Set the left padding to build into the `Ngram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # use ngrammatic::Pad;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").arity(2).pad_left(Pad::Pad(" ".to_string())).finish();
    /// if a.contains(" t") {
    ///     println!("{}, when padded, contains the bigram ' t'!", a.text);
    /// }
    /// # }
    /// ```
    pub fn pad_left(mut self, pad_left: Pad) -> Self {
        self.pad_left = pad_left;
        self
    }

    /// Set the right padding to build into the `Ngram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # use ngrammatic::Pad;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").arity(2).pad_right(Pad::Pad(" ".to_string())).finish();
    /// if a.contains("o ") {
    ///     println!("{}, when padded, contains the bigram 'o '!", a.text);
    /// }
    /// # }
    /// ```
    pub fn pad_right(mut self, pad_right: Pad) -> Self {
        self.pad_right = pad_right;
        self
    }

    /// Set both the left and right padding to build into the `Ngram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # use ngrammatic::Pad;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").arity(2).pad_full(Pad::Pad(" ".to_string())).finish();
    /// if a.contains(" t") {
    ///     println!("{}, when padded, contains the bigram ' t'!", a.text);
    /// }
    /// if a.contains("o ") {
    ///     println!("{}, when padded, contains the bigram 'o '!", a.text);
    /// }
    /// # }
    /// ```
    pub fn pad_full(mut self, pad: Pad) -> Self {
        self.pad_left = pad.clone();
        self.pad_right = pad;
        self
    }

    /// Set `arity` (the _n_ in _ngram_) to use for the resulting `Ngram`.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").arity(3).finish();
    /// if a.contains("tom") {
    ///     println!("{} contains the trigram 'tom'!", a.text);
    /// }
    /// # }
    /// ```
    pub fn arity(mut self, arity: usize) -> Self {
        self.arity = arity.max(1);
        self
    }

    /// Yield an `Ngram` instance with all the properties set with this builder.
    /// ```rust
    /// # use ngrammatic::NgramBuilder;
    /// # fn main() {
    /// let a = NgramBuilder::new("tomato").arity(3).finish();
    /// if a.contains("tom") {
    ///     println!("{} contains the trigram 'tom'!", a.text);
    /// }
    /// # }
    /// ```
    pub fn finish(self) -> Ngram {
        let mut ngram = Ngram {
            arity: self.arity,
            text: self.text.clone(),
            text_padded: Pad::pad_text(&self.text, self.pad_left, self.pad_right, self.arity - 1),
            grams: HashMap::new(),
        };
        ngram.init();
        ngram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn arity_clamp_empty_string_nopad() {
        let ngram = NgramBuilder::new("").arity(1).pad_full(Pad::None).finish();
        assert!(ngram.is_empty());
    }

    #[test]
    fn arity_clamp_empty_string_padded() {
        let ngram = NgramBuilder::new("")
            .arity(2)
            .pad_left(Pad::Pad("--".to_string()))
            .pad_right(Pad::Pad("++".to_string()))
            .finish();
        assert!(ngram.contains("--"));
        assert!(ngram.contains("-+"));
        assert!(ngram.contains("++"));
    }

    #[test]
    fn empty_string_nopad() {
        let ngram = NgramBuilder::new("").arity(2).pad_full(Pad::None).finish();
        assert!(ngram.is_empty());
    }

    #[test]
    fn empty_string_autopad() {
        let ngram = NgramBuilder::new("").arity(2).finish();
        assert!(ngram.contains("  "));
    }

    #[test]
    fn empty_string_strpad() {
        let ngram = NgramBuilder::new("")
            .arity(2)
            .pad_left(Pad::Pad("--".to_string()))
            .pad_right(Pad::Pad("++".to_string()))
            .finish();
        assert!(ngram.contains("--"));
        assert!(ngram.contains("-+"));
        assert!(ngram.contains("++"));
    }

    #[test]
    fn short_string_nopad() {
        let ngram = NgramBuilder::new("ab")
            .arity(2)
            .pad_full(Pad::None)
            .finish();
        assert!(ngram.contains("ab"));
    }

    #[test]
    fn short_string_autopad() {
        let ngram = NgramBuilder::new("ab").arity(2).finish();
        assert!(ngram.contains(" a"));
        assert!(ngram.contains("ab"));
        assert!(ngram.contains("b "));
    }

    #[test]
    fn short_string_strpad() {
        let ngram = NgramBuilder::new("ab")
            .arity(2)
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
    fn ngram_similarity_raw() {
        assert!(float_approx_eq(Ngram::similarity(5, 10, 1.0), 0.5, None));
        assert!(float_approx_eq(Ngram::similarity(5, 10, 2.0), 0.75, None));
        assert!(float_approx_eq(Ngram::similarity(5, 10, 3.0), 0.875, None));
        assert!(float_approx_eq(Ngram::similarity(2, 4, 2.0), 0.75, None));
        assert!(float_approx_eq(Ngram::similarity(3, 4, 1.0), 0.75, None));
    }

    #[test]
    fn similarity_identical() {
        let ngram0 = NgramBuilder::new("ab").arity(2).finish();
        let ngram1 = NgramBuilder::new("ab").arity(2).finish();
        assert!(float_approx_eq(
            ngram0.similarity_to(&ngram1, 3.0),
            1.0,
            None,
        ));
    }

    #[test]
    fn similarity_completelydifferent() {
        let ngram0 = NgramBuilder::new("ab").arity(2).finish();
        let ngram1 = NgramBuilder::new("cd").arity(2).finish();
        assert!(float_approx_eq(
            ngram0.similarity_to(&ngram1, 3.0),
            0.0,
            None,
        ));
    }
}
