//! Trait defining a key and its hasher.

use std::collections::HashMap;

use crate::traits::ascii_char::ToASCIICharIterator;
use crate::traits::iter_ngrams::IntoNgrams;
use crate::{ASCIIChar, ASCIICharIterator, Gram, Ngram};

/// Trait defining a key.
pub trait Key<G: Gram>: Clone + PartialEq + Eq {
    /// The type of the grams iterator.
    type Grams<'a>: Iterator<Item = G>
    where
        Self: 'a;

    /// Returns an iterator over the grams of the key.
    fn grams(&self) -> Self::Grams<'_>;

    /// Returns the counts of the ngrams.
    fn counts<NG: Ngram<G = G>>(&self) -> HashMap<NG, usize> {
        let mut ngram_counts: HashMap<NG, usize> = HashMap::new();

        // We populate it with the ngrams of the key.
        for ngram in self.grams().ngrams::<NG>() {
            ngram_counts
                .entry(ngram)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        ngram_counts
    }
}

impl Key<char> for String {
    type Grams<'a> = std::str::Chars<'a>;

    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
    }
}

impl Key<char> for &str {
    type Grams<'a> = std::str::Chars<'a> where Self: 'a;

    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
    }
}

impl Key<u8> for String {
    type Grams<'a> = std::str::Bytes<'a>;

    fn grams(&self) -> Self::Grams<'_> {
        self.bytes()
    }
}

impl Key<u8> for &str {
    type Grams<'a> = std::str::Bytes<'a> where Self: 'a;

    fn grams(&self) -> Self::Grams<'_> {
        self.bytes()
    }
}

impl Key<ASCIIChar> for String {
    type Grams<'a> = ASCIICharIterator<std::str::Chars<'a>> where Self: 'a;

    fn grams(&self) -> Self::Grams<'_> {
        self.chars().ascii()
    }
}

impl Key<ASCIIChar> for &str {
    type Grams<'a> = ASCIICharIterator<std::str::Chars<'a>> where Self: 'a;

    fn grams(&self) -> Self::Grams<'_> {
        self.chars().ascii()
    }
}
