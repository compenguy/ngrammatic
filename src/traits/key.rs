//! Trait defining a key and its hasher.

use fxhash::FxBuildHasher;
use std::collections::HashMap;
use crate::traits::ascii_char::ToASCIICharIterator;
use crate::traits::iter_ngrams::IntoNgrams;
use crate::{
    ASCIIChar, ASCIICharIterator, BothPadding, CharLike, CharNormalizer, Gram, IntoPadder,
    Lowercase, Ngram, PaddableNgram,
};

/// Trait defining a key.
pub trait Key<NG: Ngram<G = G>, G: Gram>: AsRef<<Self as Key<NG, G>>::Ref> {
    /// The type of the grams iterator.
    type Grams<'a>: Iterator<Item = G>
    where
        Self: 'a;

    /// Default reference type when no more specific type is
    /// specified in the corpus.
    type Ref: ?Sized;

    /// Returns an iterator over the grams of the key.
    fn grams(&self) -> Self::Grams<'_>;

    /// Returns the counts of the ngrams.
    fn counts(&self) -> HashMap<NG, usize, FxBuildHasher> {
        let mut ngram_counts: HashMap<NG, usize, FxBuildHasher> =
            HashMap::with_hasher(FxBuildHasher::default());

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

impl<NG> Key<NG, char> for String
where
    NG: Ngram<G = char> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Chars<'a>>;

    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars().trim().trim_null().both_padding::<NG>()
    }
}

impl<NG> Key<NG, char> for str
where
    NG: Ngram<G = char> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Chars<'a>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars().trim().trim_null().both_padding::<NG>()
    }
}

impl<NG> Key<NG, u8> for str
where
    NG: Ngram<G = u8> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Bytes<'a>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.bytes().both_padding::<NG>()
    }
}

impl<NG> Key<NG, ASCIIChar> for str
where
    NG: Ngram<G = ASCIIChar> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, ASCIICharIterator<std::str::Chars<'a>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars().ascii().trim().trim_null().both_padding::<NG>()
    }
}

impl<NG> Key<NG, char> for &str
where
    NG: Ngram<G = char> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Chars<'a>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars().trim().trim_null().both_padding::<NG>()
    }
}

impl<NG> Key<NG, u8> for String
where
    NG: Ngram<G = u8> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Bytes<'a>>;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.bytes().both_padding::<NG>()
    }
}

impl<NG> Key<NG, u8> for &str
where
    NG: Ngram<G = u8> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Bytes<'a>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.bytes().both_padding::<NG>()
    }
}

impl<NG> Key<NG, ASCIIChar> for String
where
    NG: Ngram<G = ASCIIChar> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, ASCIICharIterator<std::str::Chars<'a>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars().ascii().trim().trim_null().both_padding::<NG>()
    }
}

impl<NG> Key<NG, ASCIIChar> for &str
where
    NG: Ngram<G = ASCIIChar> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, ASCIICharIterator<std::str::Chars<'a>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars().ascii().trim().trim_null().both_padding::<NG>()
    }
}

impl<W, NG> Key<NG, NG::G> for Lowercase<W>
where
    NG: Ngram,
    W: Key<NG, NG::G> + ?Sized,
    NG::G: CharLike,
    Self: AsRef<<W as Key<NG, <NG as Ngram>::G>>::Ref>,
{
    type Grams<'a> = Lowercase<W::Grams<'a>> where Self: 'a;
    type Ref = W::Ref;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.inner().grams().lower()
    }
}
