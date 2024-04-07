//! Submodule providing the Padder trait and relative structs.
//!
//! # Implementative details
//! The goal of the Padder trait and structs is to provide a way to pad iterators
//! of paddable grams, i.e. the types that implement the trait Paddable.

use crate::{Gram, Ngram, Paddable};
use std::iter::Chain;

/// Type alias for the padding both iterator.
pub type BothPadding<NG, S> = Chain<
    Chain<<<NG as Ngram>::Pad as IntoIterator>::IntoIter, S>,
    <<NG as Ngram>::Pad as IntoIterator>::IntoIter,
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
    /// An example for when using an iterator of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = vec![b'a', b'b', b'c'].into_iter();
    /// let padded_left = iter.left_padding::<BiGram<u8>>();
    /// let padded: Vec<_> = padded_left.collect();
    /// assert_eq!(padded, vec![b'\0', b'a', b'b', b'c']);
    /// ```
    ///
    /// An example for when using an iterator of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "abc".chars();
    /// let padded_left = iter.left_padding::<BiGram<char>>();
    /// let padded: String = padded_left.collect();
    /// assert_eq!(padded, "\0abc");
    /// ```
    ///
    /// An example for when using an iterator of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "ab∂Ωc".chars().filter_map(|c| ASCIIChar::try_from(c).ok());
    /// let padded_left = iter.left_padding::<BiGram<ASCIIChar>>();
    /// let padded: Vec<_> = padded_left.collect();
    /// assert_eq!(
    ///     padded,
    ///     vec![
    ///         ASCIIChar::from(b'\0'),
    ///         ASCIIChar::from(b'a'),
    ///         ASCIIChar::from(b'b'),
    ///         ASCIIChar::from(b'c')
    ///     ]
    /// );
    /// ```
    fn left_padding<NG>(self) -> Chain<<<NG as Ngram>::Pad as IntoIterator>::IntoIter, Self>
    where
        NG: Ngram<G = Self::Item>,
    {
        NG::PADDING.into_iter().chain(self)
    }

    /// Adds padding to the right (end) of the iterator.
    ///
    /// # Example
    ///
    /// An example for when using an iterator of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = vec![b'a', b'b', b'c'].into_iter();
    /// let padded_right = iter.right_padding::<BiGram<u8>>();
    /// let padded: Vec<_> = padded_right.collect();
    /// assert_eq!(padded, vec![b'a', b'b', b'c', b'\0']);
    /// ```
    ///
    /// An example for when using an iterator of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "abc".chars();
    /// let padded_right = iter.right_padding::<BiGram<char>>();
    /// let padded: String = padded_right.collect();
    /// assert_eq!(padded, "abc\0");
    /// ```
    ///
    /// An example for when using an iterator of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "ab∂Ωc".chars().filter_map(|c| ASCIIChar::try_from(c).ok());
    /// let padded_right = iter.right_padding::<BiGram<ASCIIChar>>();
    /// let padded: Vec<_> = padded_right.collect();
    /// assert_eq!(
    ///     padded,
    ///     vec![
    ///         ASCIIChar::from(b'a'),
    ///         ASCIIChar::from(b'b'),
    ///         ASCIIChar::from(b'c'),
    ///         ASCIIChar::from(b'\0')
    ///     ]
    /// );
    /// ```
    fn right_padding<NG>(self) -> Chain<Self, <<NG as Ngram>::Pad as IntoIterator>::IntoIter>
    where
        NG: Ngram<G = Self::Item>,
    {
        self.chain(NG::PADDING)
    }

    /// Adds padding to both sides of the iterator.
    ///
    /// # Example
    /// An example for when using an iterator of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = vec![b'a', b'b', b'c'].into_iter();
    /// let padded_both = iter.both_padding::<BiGram<u8>>();
    /// let padded: Vec<_> = padded_both.collect();
    /// assert_eq!(padded, vec![b'\0', b'a', b'b', b'c', b'\0']);
    /// ```
    ///
    /// An example for when using an iterator of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "abc".chars();
    /// let padded_both = iter.both_padding::<BiGram<char>>();
    /// let padded: String = padded_both.collect();
    /// assert_eq!(padded, "\0abc\0");
    /// ```
    ///
    /// An example for when using an iterator of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let iter = "ab∂Ωc".chars().filter_map(|c| ASCIIChar::try_from(c).ok());
    /// let padded_both = iter.both_padding::<BiGram<ASCIIChar>>();
    /// let padded: Vec<_> = padded_both.collect();
    /// assert_eq!(
    ///     padded,
    ///     vec![
    ///         ASCIIChar::from(b'\0'),
    ///         ASCIIChar::from(b'a'),
    ///         ASCIIChar::from(b'b'),
    ///         ASCIIChar::from(b'c'),
    ///         ASCIIChar::from(b'\0')
    ///     ]
    /// );
    /// ```
    fn both_padding<NG>(self) -> BothPadding<NG, Self>
    where
        NG: Ngram<G = Self::Item>,
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
