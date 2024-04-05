//! Submodule providing the Padder trait and relative structs.
//!
//! # Implementative details
//! The goal of the Padder trait and structs is to provide a way to pad iterators
//! of paddable grams, i.e. the types that implement the trait Paddable.

use crate::{Gram, Paddable, PaddableNgram};
use std::iter::Chain;

/// Type alias for the padding both iterator.
pub type BothPadding<NG, S> = Chain<
    Chain<<<NG as PaddableNgram>::Pad as IntoIterator>::IntoIter, S>,
    <<NG as PaddableNgram>::Pad as IntoIterator>::IntoIter,
>;

/// Trait defining a padder.
pub trait IntoPadder: Iterator + Sized
where
    <Self as Iterator>::Item: Paddable + Gram,
{
    /// Adds padding to the left (beginning) of the iterator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = vec![b'a', b'b', b'c'].into_iter();
    /// let padded_left = iter.left_padding::<BiGram<u8>>();
    /// let padded: Vec<_> = padded_left.collect();
    /// assert_eq!(padded, vec![b' ', b'a', b'b', b'c']);
    /// ```
    fn left_padding<NG>(self) -> Chain<<<NG as PaddableNgram>::Pad as IntoIterator>::IntoIter, Self>
    where
        NG: PaddableNgram<G = Self::Item>,
    {
        NG::PADDING.into_iter().chain(self)
    }

    /// Adds padding to the right (end) of the iterator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = vec![b'a', b'b', b'c'].into_iter();
    /// let padded_right = iter.right_padding::<BiGram<u8>>();
    /// let padded: Vec<_> = padded_right.collect();
    /// assert_eq!(padded, vec![b'a', b'b', b'c', b' ']);
    /// ```
    ///
    fn right_padding<NG>(
        self,
    ) -> Chain<Self, <<NG as PaddableNgram>::Pad as IntoIterator>::IntoIter>
    where
        NG: PaddableNgram<G = Self::Item>,
    {
        self.chain(NG::PADDING)
    }

    /// Adds padding to both sides of the iterator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = vec![b'a', b'b', b'c'].into_iter();
    /// let padded_both = iter.both_padding::<BiGram<u8>>();
    /// let padded: Vec<_> = padded_both.collect();
    /// assert_eq!(padded, vec![b' ', b'a', b'b', b'c', b' ']);
    /// ```
    fn both_padding<NG>(self) -> BothPadding<NG, Self>
    where
        NG: PaddableNgram<G = Self::Item>,
    {
        NG::PADDING.into_iter().chain(self).chain(NG::PADDING)
    }
}

impl<I> IntoPadder for I
where
    I: Iterator,
    <I as Iterator>::Item: Paddable + Gram,
{
}
