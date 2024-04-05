//! Submodule providing traits to normalize iterators of char-like items.

use crate::CharLike;

/// Trait defining an iterator to lowercase.
pub struct Lowercase<I>(I);

impl<I> From<I> for Lowercase<I> {
    fn from(iter: I) -> Self {
        Lowercase(iter)
    }
}

impl<I> Iterator for Lowercase<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(CharLike::to_lowercase)
    }
}

/// Trait defining an iterator to uppercase.
pub struct Uppercase<I>(I);

impl<I> From<I> for Uppercase<I> {
    fn from(iter: I) -> Self {
        Uppercase(iter)
    }
}

impl<I> Iterator for Uppercase<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(CharLike::to_uppercase)
    }
}

/// Trait defining a char normalizer.
pub trait CharNormalizer: DoubleEndedIterator + Sized
where
    <Self as Iterator>::Item: CharLike,
{
    #[inline(always)]
    /// Trims spaces from the left of the iterator.
    fn trim_left(mut self) -> Self {
        let mut peekable = self.by_ref().peekable();

        while let Some(c) = peekable.peek() {
            if c.is_space_like() {
                peekable.next();
            } else {
                break;
            }
        }

        self
    }

    #[inline(always)]
    /// Trims spaces from the right of the iterator.
    fn trim_right(mut self) -> Self {
        let mut peekable = self.by_ref().rev().peekable();

        while let Some(c) = peekable.peek() {
            if c.is_space_like() {
                peekable.next();
            } else {
                break;
            }
        }

        self
    }

    #[inline(always)]
    /// Trims spaces from both sides of the iterator.
    fn trim(self) -> Self {
        self.trim_left().trim_right()
    }

    #[inline(always)]
    /// Trims null characters from the left of the iterator.
    fn trim_null_left(mut self) -> Self {
        let mut peekable = self.by_ref().peekable();

        while let Some(c) = peekable.peek() {
            if c.is_nul() {
                peekable.next();
            } else {
                break;
            }
        }

        self
    }

    #[inline(always)]
    /// Trims null characters from the right of the iterator.
    fn trim_null_right(mut self) -> Self {
        let mut peekable = self.by_ref().rev().peekable();

        while let Some(c) = peekable.peek() {
            if c.is_nul() {
                peekable.next();
            } else {
                break;
            }
        }

        self
    }

    #[inline(always)]
    /// Trims null characters from both sides of the iterator.
    fn trim_null(self) -> Self {
        self.trim_null_left().trim_null_right()
    }

    #[inline(always)]
    /// Converts all characters to lowercase.
    fn lower(self) -> Lowercase<Self> {
        Lowercase::from(self)
    }

    #[inline(always)]
    /// Converts all characters to uppercase.
    fn upper(self) -> Uppercase<Self> {
        Uppercase::from(self)
    }
}


/// Blanket implementation of `CharNormalizer` for all iterators yielding `CharLike` items.
impl<I> CharNormalizer for I
where
    I: DoubleEndedIterator,
    <Self as Iterator>::Item: CharLike,
{
}
