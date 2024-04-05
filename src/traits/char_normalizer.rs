//! Submodule providing traits to normalize iterators of char-like items.

use std::mem::transmute;

use crate::CharLike;

/// Trait defining an iterator to lowercase.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Lowercase<I: ?Sized>(I);

impl<I: ?Sized> AsRef<I> for Lowercase<I> {
    #[inline(always)]
    fn as_ref(&self) -> &I {
        &self.0
    }
}

impl<E: ?Sized> AsRef<Lowercase<E>> for String
where
    String: AsRef<E>,
{
    #[inline(always)]
    fn as_ref(&self) -> &Lowercase<E> {
        let reference: &E = self.as_ref();
        unsafe { transmute(reference) }
    }
}

impl<E: ?Sized> AsRef<Lowercase<E>> for str where str: AsRef<E> {
    #[inline(always)]
    fn as_ref(&self) -> &Lowercase<E> {
        let reference: &E = self.as_ref();
        unsafe { transmute(reference) }
    }
}

impl<I: ?Sized> Lowercase<I> {
    #[inline(always)]
    /// Returns a reference to the inner iterator.
    pub fn inner(&self) -> &I {
        &self.0
    }
}

impl<I> From<I> for Lowercase<I> {
    #[inline(always)]
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

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(CharLike::to_lowercase)
    }
}

/// Trait defining a char normalizer.
pub trait CharNormalizer: Iterator + Sized
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
    fn trim_right(mut self) -> Self
    where
        Self: DoubleEndedIterator,
    {
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
    fn trim(self) -> Self
    where
        Self: DoubleEndedIterator,
    {
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
    fn trim_null_right(mut self) -> Self
    where
        Self: DoubleEndedIterator,
    {
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
    fn trim_null(self) -> Self
    where
        Self: DoubleEndedIterator,
    {
        self.trim_null_left().trim_null_right()
    }

    #[inline(always)]
    /// Converts all characters to lowercase.
    fn lower(self) -> Lowercase<Self> {
        Lowercase::from(self)
    }
}

/// Blanket implementation of `CharNormalizer` for all iterators yielding `CharLike` items.
impl<I> CharNormalizer for I
where
    I: Iterator,
    <Self as Iterator>::Item: CharLike,
{
}
