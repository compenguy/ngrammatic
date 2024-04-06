//! Submodule providing traits to normalize iterators of char-like items.

use std::mem::transmute;

use crate::CharLike;

/// Struct defining an iterator to lowercase.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Lowercase<I: ?Sized>(I);

impl<E: ?Sized, I: ?Sized> AsRef<I> for Lowercase<E>
where
    E: AsRef<I>,
{
    #[inline(always)]
    fn as_ref(&self) -> &I {
        self.0.as_ref()
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

impl<E: ?Sized> AsRef<Lowercase<E>> for str
where
    str: AsRef<E>,
{
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

impl<I> DoubleEndedIterator for Lowercase<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(CharLike::to_lowercase)
    }
}

impl<I> ExactSizeIterator for Lowercase<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: CharLike,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// Struct defining an iterator that replaces characters that are not alphanumeric with spaces.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Alphanumeric<I: ?Sized>(I);

impl<E: ?Sized, I: ?Sized> AsRef<I> for Alphanumeric<E>
where
    E: AsRef<I>,
{
    #[inline(always)]
    fn as_ref(&self) -> &I {
        self.0.as_ref()
    }
}

impl<E: ?Sized> AsRef<Alphanumeric<E>> for String
where
    String: AsRef<E>,
{
    #[inline(always)]
    fn as_ref(&self) -> &Alphanumeric<E> {
        let reference: &E = self.as_ref();
        unsafe { transmute(reference) }
    }
}

impl<E: ?Sized> AsRef<Alphanumeric<E>> for str
where
    str: AsRef<E>,
{
    #[inline(always)]
    fn as_ref(&self) -> &Alphanumeric<E> {
        let reference: &E = self.as_ref();
        unsafe { transmute(reference) }
    }
}

impl<I: ?Sized> Alphanumeric<I> {
    #[inline(always)]
    /// Returns a reference to the inner iterator.
    pub fn inner(&self) -> &I {
        &self.0
    }
}

impl<I> From<I> for Alphanumeric<I> {
    #[inline(always)]
    fn from(iter: I) -> Self {
        Alphanumeric(iter)
    }
}

impl<I> Iterator for Alphanumeric<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|c| {
            if c.is_alphanumeric() {
                c
            } else {
                <I as Iterator>::Item::SPACE
            }
        })
    }
}

impl<I> DoubleEndedIterator for Alphanumeric<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|c| {
            if c.is_alphanumeric() {
                c
            } else {
                <I as Iterator>::Item::SPACE
            }
        })
    }
}

impl<I> ExactSizeIterator for Alphanumeric<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: CharLike,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Struct defining an iterator that removes subsequent spaces.
pub struct SpaceNormalizer<I> {
    iter: I,
    last_was_space: bool,
}

impl<I> Iterator for SpaceNormalizer<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.iter.next();

        while let Some(c) = next {
            if c.is_space_like() {
                if self.last_was_space {
                    next = self.iter.next();
                } else {
                    self.last_was_space = true;
                    break;
                }
            } else {
                self.last_was_space = false;
                break;
            }
        }

        next
    }
}

impl<I> DoubleEndedIterator for SpaceNormalizer<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut next = self.iter.next_back();

        while let Some(c) = next {
            if c.is_space_like() {
                if self.last_was_space {
                    next = self.iter.next_back();
                } else {
                    self.last_was_space = true;
                    break;
                }
            } else {
                self.last_was_space = false;
                break;
            }
        }

        next
    }
}

impl<I> ExactSizeIterator for SpaceNormalizer<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: CharLike,
{
    fn len(&self) -> usize {
        self.iter.len()
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

    #[inline(always)]
    /// Converts all non-alpha characters to spaces.
    fn alphanumeric(self) -> Alphanumeric<Self> {
        Alphanumeric::from(self)
    }

    #[inline(always)]
    /// Normalizes spaces, removing subsequent spaces.
    fn dedup_spaces(self) -> SpaceNormalizer<Self> {
        SpaceNormalizer {
            iter: self,
            last_was_space: false,
        }
    }
}

/// Blanket implementation of `CharNormalizer` for all iterators yielding `CharLike` items.
impl<I> CharNormalizer for I
where
    I: Iterator,
    <Self as Iterator>::Item: CharLike,
{
}
