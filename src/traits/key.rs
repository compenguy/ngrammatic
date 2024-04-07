//! Trait defining a key and its hasher.

use crate::traits::ascii_char::ToASCIICharIterator;
use crate::traits::iter_ngrams::IntoNgrams;
use crate::{
    ASCIIChar, ASCIICharIterator, Alphanumeric, BothPadding, CharLike, CharNormalizer, Gram,
    IntoPadder, Lowercase, Ngram, SpaceNormalizer, Trim, TrimNull,
};
use fxhash::FxBuildHasher;
use std::collections::HashMap;

/// Trait defining a key.
pub trait Key<NG: Ngram<G = G>, G: Gram>: AsRef<<Self as Key<NG, G>>::Ref> {
    /// The type of the grams iterator.
    type Grams<'a>: Iterator<Item = G>
    where
        Self: 'a;

    /// Default reference type when no more specific type is
    /// specified in the corpus.
    type Ref: ?Sized + std::fmt::Debug;

    /// Returns an iterator over the grams of the key.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to get the grams of a key
    /// represented by a string, composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let grams: Vec<u8> = <&str as Key<BiGram<u8>, u8>>::grams(&key).collect();
    /// assert_eq!(grams, vec![b'\0', b'a', b'b', b'c', b'\0',]);
    /// ```
    ///
    /// The following example demonstrates how to get the grams of a key
    /// represented by a string, composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let grams: Vec<char> = <&str as Key<BiGram<char>, char>>::grams(&key).collect();
    /// assert_eq!(grams, vec!['\0', 'a', 'b', 'c', '\0']);
    /// ```
    ///
    /// The following example demonstrates how to get the grams of a key
    /// represented by a string, composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let grams: Vec<ASCIIChar> = <&str as Key<BiGram<ASCIIChar>, ASCIIChar>>::grams(&key).collect();
    /// assert_eq!(
    ///     grams,
    ///     vec![
    ///         ASCIIChar::from(b'\0'),
    ///         ASCIIChar::from(b'a'),
    ///         ASCIIChar::from(b'b'),
    ///         ASCIIChar::from(b'c'),
    ///         ASCIIChar::from(b'\0')
    ///     ]
    /// );
    /// ```
    fn grams(&self) -> Self::Grams<'_>;

    /// Returns the counts of the ngrams.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to get the counts of the ngrams
    /// of a key represented by a string, composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let counts = <&str as Key<BiGram<u8>, u8>>::counts(&key);
    /// assert_eq!(counts.get(&[b'\0', b'a']), Some(&1));
    /// assert_eq!(counts.get(&[b'a', b'b']), Some(&1));
    /// assert_eq!(counts.get(&[b'b', b'c']), Some(&1));
    /// assert_eq!(counts.get(&[b'b', b'Z']), None);
    /// assert_eq!(counts.get(&[b'c', b'\0']), Some(&1));
    /// assert_eq!(counts.get(&[b'Z', b'\0']), None);
    /// assert_eq!(counts.len(), 4);
    /// ```
    ///
    /// The following example demonstrates how to get the counts of the ngrams
    /// of a key represented by a string, composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let counts = <&str as Key<BiGram<char>, char>>::counts(&key);
    /// assert_eq!(counts.get(&['\0', 'a']), Some(&1));
    /// assert_eq!(counts.get(&['a', 'b']), Some(&1));
    /// assert_eq!(counts.get(&['b', 'c']), Some(&1));
    /// assert_eq!(counts.get(&['b', 'Z']), None);
    /// assert_eq!(counts.get(&['c', '\0']), Some(&1));
    /// assert_eq!(counts.get(&['Z', '\0']), None);
    /// assert_eq!(counts.len(), 4);
    /// ```
    ///
    /// The following example demonstrates how to get the counts of the ngrams
    /// of a more human-readable example such as the word "Cat":
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "Cat";
    /// let counts = <&str as Key<BiGram<char>, char>>::counts(&key);
    /// assert_eq!(counts.get(&['\0', 'C']), Some(&1));
    /// assert_eq!(counts.get(&['C', 'a']), Some(&1));
    /// assert_eq!(counts.get(&['a', 't']), Some(&1));
    /// assert_eq!(counts.get(&['a', 'T']), None);
    /// assert_eq!(counts.get(&['t', '\0']), Some(&1));
    /// assert_eq!(counts.get(&['T', '\0']), None);
    /// assert_eq!(counts.len(), 4);
    /// ```
    ///
    /// The following example demonstrates how to get the counts of the ngrams
    /// of a key represented by a string, composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let counts = <&str as Key<BiGram<ASCIIChar>, ASCIIChar>>::counts(&key);
    /// assert_eq!(
    ///     counts.get(&[ASCIIChar::from(b'\0'), ASCIIChar::from(b'a')]),
    ///     Some(&1)
    /// );
    /// assert_eq!(
    ///     counts.get(&[ASCIIChar::from(b'a'), ASCIIChar::from(b'b')]),
    ///     Some(&1)
    /// );
    /// assert_eq!(
    ///     counts.get(&[ASCIIChar::from(b'b'), ASCIIChar::from(b'c')]),
    ///     Some(&1)
    /// );
    /// assert_eq!(
    ///     counts.get(&[ASCIIChar::from(b'b'), ASCIIChar::from(b'Z')]),
    ///     None
    /// );
    /// assert_eq!(
    ///     counts.get(&[ASCIIChar::from(b'c'), ASCIIChar::from(b'\0')]),
    ///     Some(&1)
    /// );
    /// assert_eq!(
    ///     counts.get(&[ASCIIChar::from(b'Z'), ASCIIChar::from(b'\0')]),
    ///     None
    /// );
    /// assert_eq!(counts.len(), 4);
    /// ```
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
    NG: Ngram<G = char>,
{
    type Grams<'a> =
        BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<std::str::Chars<'a>>>>>>;

    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, char> for str
where
    NG: Ngram<G = char>,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<std::str::Chars<'a>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, u8> for str
where
    NG: Ngram<G = u8>,
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
    NG: Ngram<G = ASCIIChar>,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<ASCIICharIterator<std::str::Chars<'a>>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .ascii()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, char> for &str
where
    NG: Ngram<G = char>,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<std::str::Chars<'a>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, u8> for String
where
    NG: Ngram<G = u8>,
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
    NG: Ngram<G = u8>,
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
    NG: Ngram<G = ASCIIChar>,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<ASCIICharIterator<std::str::Chars<'a>>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .ascii()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, ASCIIChar> for &str
where
    NG: Ngram<G = ASCIIChar>,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<ASCIICharIterator<std::str::Chars<'a>>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .ascii()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
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

impl<W, NG> Key<NG, NG::G> for Alphanumeric<W>
where
    NG: Ngram,
    W: Key<NG, NG::G> + ?Sized,
    NG::G: CharLike,
    Self: AsRef<<W as Key<NG, <NG as Ngram>::G>>::Ref>,
{
    type Grams<'a> = Alphanumeric<W::Grams<'a>> where Self: 'a;
    type Ref = W::Ref;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.inner().grams().alphanumeric()
    }
}
