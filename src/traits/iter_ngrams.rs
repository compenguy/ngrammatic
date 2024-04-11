//! Submodule providing an iterator to convert an iterator of grams to an iterator of n-grams.

use crate::{Gram, Ngram};

/// Struct implementing an iterator to convert an iterator
/// of grams to an iterator of n-grams.
pub struct IterNgrams<I, NG>
where
    I: Iterator<Item = <NG as Ngram>::G>,
    NG: Ngram,
{
    /// Iterator of grams.
    iter: I,
    /// The n-gram currently being built.
    ngram: NG,
}

impl<I, NG> From<I> for IterNgrams<I, NG>
where
    I: Iterator<Item = <NG as Ngram>::G>,
    NG: Ngram,
{
    #[inline(always)]
    fn from(mut iter: I) -> Self {
        let mut ngram: NG = Default::default();
        // We populate the first ARITY - 1 values
        // with the first ARITY - 1 grams.
        for i in 0..(NG::ARITY - 1) {
            // We save the values shifted of one position,
            // leaving the first one to the default value.
            // This is done as the values at each iteration
            // are shifted by one position to the left and
            // the last value is replaced by the next value.
            ngram[i + 1] = iter.next().unwrap();
        }

        IterNgrams { iter, ngram }
    }
}

impl<I, NG> Iterator for IterNgrams<I, NG>
where
    I: Iterator<Item = <NG as Ngram>::G>,
    NG: Ngram,
{
    type Item = NG;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|gram| {
            // We shift the values by one position to the left
            // and replace the last value with the next one.
            self.ngram.rotate_left();
            // We replace the last value with the next one.
            self.ngram[NG::ARITY - 1] = gram;
            // We return the ngram, which can be copied.
            self.ngram
        })
    }
}

/// Trait defining an iterator to convert an
/// iterator of grams to an iterator of n-grams.
pub trait IntoNgrams: Iterator
where
    <Self as Iterator>::Item: Gram,
{
    #[inline(always)]
    /// Converts an iterator of grams to an iterator of n-grams.
    ///
    /// # Examples
    ///
    /// An example for when using an iterator of `u8` bigrams:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = vec![b'a', b'b', b'c'].into_iter();
    /// let ngrams: Vec<_> = iter.ngrams::<BiGram<u8>>().collect();
    /// assert_eq!(
    ///     ngrams,
    ///     vec![BiGram::from([b'a', b'b']), BiGram::from([b'b', b'c'])]
    /// );
    /// ```
    ///
    /// An example for when using an iterator of `u8` trigrams:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = vec![b'a', b'b', b'c', b'd'].into_iter();
    /// let ngrams: Vec<_> = iter.ngrams::<TriGram<u8>>().collect();
    /// assert_eq!(
    ///     ngrams,
    ///     vec![
    ///         TriGram::from([b'a', b'b', b'c']),
    ///         TriGram::from([b'b', b'c', b'd'])
    ///     ]
    /// );
    /// ```
    ///
    /// An example for when using an iterator of `char` bigrams:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "abc".chars();
    /// let ngrams: Vec<_> = iter.ngrams::<BiGram<char>>().collect();
    /// assert_eq!(
    ///     ngrams,
    ///     vec![BiGram::from(['a', 'b']), BiGram::from(['b', 'c'])]
    /// );
    /// ```
    ///
    /// An example for when using an iterator of `char` trigrams:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "abcd".chars();
    /// let ngrams: Vec<_> = iter.ngrams::<TriGram<char>>().collect();
    /// assert_eq!(
    ///     ngrams,
    ///     vec![
    ///         TriGram::from(['a', 'b', 'c']),
    ///         TriGram::from(['b', 'c', 'd'])
    ///     ]
    /// );
    /// ```
    ///
    /// An example for when using an iterator of `ASCIIChar` bigrams:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "ab∂Ωc".chars().ascii();
    /// let ngrams: Vec<_> = iter.ngrams::<BiGram<ASCIIChar>>().collect();
    /// assert_eq!(
    ///     ngrams,
    ///     vec![
    ///         BiGram::from([ASCIIChar::from(b'a'), ASCIIChar::from(b'b')]),
    ///         BiGram::from([ASCIIChar::from(b'b'), ASCIIChar::from(b'c')])
    ///     ]
    /// );
    /// ```
    ///
    /// An example for when using an iterator of `ASCIIChar` trigrams:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "ab∂Ωcd".chars().ascii();
    /// let ngrams: Vec<_> = iter.ngrams::<TriGram<ASCIIChar>>().collect();
    /// assert_eq!(
    ///     ngrams,
    ///     vec![
    ///         TriGram::from([
    ///             ASCIIChar::from(b'a'),
    ///             ASCIIChar::from(b'b'),
    ///             ASCIIChar::from(b'c')
    ///         ]),
    ///         TriGram::from([
    ///             ASCIIChar::from(b'b'),
    ///             ASCIIChar::from(b'c'),
    ///             ASCIIChar::from(b'd')
    ///         ])
    ///     ]
    /// );
    /// ```
    fn ngrams<NG>(self) -> IterNgrams<Self, NG>
    where
        NG: Ngram<G = <Self as Iterator>::Item>,
        Self: Sized,
    {
        IterNgrams::from(self)
    }
}

impl<I> IntoNgrams for I
where
    I: Iterator,
    <I as std::iter::Iterator>::Item: Gram,
{
}
