//! Submodule providing an iterator to convert an iterator of grams to an iterator of n-grams.

use crate::{Gram, Ngram};

/// Struct implementing an iterator to convert an iterator
/// of grams to an iterator of n-grams.
pub struct IterNgrams<I, NG>
where
    I: Iterator<Item = <NG as Ngram>::G>,
    NG: Ngram,
{
    iter: I,
    ngram: NG,
}

impl<I, NG> From<I> for IterNgrams<I, NG>
where
    I: Iterator<Item = <NG as Ngram>::G>,
    NG: Ngram,
{
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
    /// Converts an iterator of grams to an iterator of n-grams.
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
