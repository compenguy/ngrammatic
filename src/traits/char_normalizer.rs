//! Submodule providing traits to normalize iterators of char-like items.

use std::{iter::Rev, mem::transmute};

use crate::CharLike;
use std::iter::Peekable;

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

/// Struct defining an iterator to trim spaces from left.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct TrimLeft<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    peekable: Peekable<I>,
}

impl<I> From<I> for TrimLeft<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn from(iter: I) -> Self {
        let mut peekable = iter.peekable();
        while let Some(c) = peekable.peek() {
            if c.is_space_like() {
                peekable.next();
            } else {
                break;
            }
        }
        TrimLeft { peekable }
    }
}

impl<I> Iterator for TrimLeft<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.peekable.next()
    }
}

impl<I> DoubleEndedIterator for TrimLeft<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.peekable.next_back()
    }
}

impl<I> ExactSizeIterator for TrimLeft<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: CharLike,
{
    fn len(&self) -> usize {
        self.peekable.len()
    }
}

/// Struct defining an iterator to trim spaces from right.
pub struct TrimRight<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    peekable: Rev<TrimLeft<Rev<I>>>,
}

impl<I> From<I> for TrimRight<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn from(iter: I) -> Self {
        TrimRight {
            peekable: iter.rev().trim_left().rev(),
        }
    }
}

impl<I> Iterator for TrimRight<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.peekable.next()
    }
}

impl<I> DoubleEndedIterator for TrimRight<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.peekable.next_back()
    }
}

impl<I> ExactSizeIterator for TrimRight<I>
where
    I: ExactSizeIterator + DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    fn len(&self) -> usize {
        self.peekable.len()
    }
}

/// Struct defining an iterator to trim spaces from both sides.
pub struct Trim<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    iter: TrimRight<TrimLeft<I>>,
}

impl<I> From<I> for Trim<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn from(iter: I) -> Self {
        Trim {
            iter: iter.trim_left().trim_right(),
        }
    }
}

impl<I> Iterator for Trim<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I> DoubleEndedIterator for Trim<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<I> ExactSizeIterator for Trim<I>
where
    I: ExactSizeIterator + DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// Struct defining an iterator to trim null characters from left.
pub struct TrimNullLeft<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    iter: Peekable<I>,
}

impl<I> From<I> for TrimNullLeft<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn from(iter: I) -> Self {
        let mut peekable = iter.peekable();
        while let Some(c) = peekable.peek() {
            if c.is_nul() {
                peekable.next();
            } else {
                break;
            }
        }
        TrimNullLeft { iter: peekable }
    }
}

impl<I> Iterator for TrimNullLeft<I>
where
    I: Iterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I> DoubleEndedIterator for TrimNullLeft<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<I> ExactSizeIterator for TrimNullLeft<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: CharLike,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// Struct defining an iterator to trim null characters from right.
pub struct TrimNullRight<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    iter: Rev<TrimNullLeft<Rev<I>>>,
}

impl<I> From<I> for TrimNullRight<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn from(iter: I) -> Self {
        TrimNullRight {
            iter: iter.rev().trim_null_left().rev(),
        }
    }
}

impl<I> Iterator for TrimNullRight<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I> DoubleEndedIterator for TrimNullRight<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<I> ExactSizeIterator for TrimNullRight<I>
where
    I: ExactSizeIterator + DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// Struct defining an iterator to trim null characters from both sides.
pub struct TrimNull<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    iter: TrimNullRight<TrimNullLeft<I>>,
}

impl<I> From<I> for TrimNull<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn from(iter: I) -> Self {
        TrimNull {
            iter: iter.trim_null_left().trim_null_right(),
        }
    }
}

impl<I> Iterator for TrimNull<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    type Item = <I as Iterator>::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I> DoubleEndedIterator for TrimNull<I>
where
    I: DoubleEndedIterator,
    <I as Iterator>::Item: CharLike,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<I> ExactSizeIterator for TrimNull<I>
where
    I: ExactSizeIterator + DoubleEndedIterator,
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
    ///
    /// # Example
    ///
    /// The following example demonstrates how to trim spaces from the left of a string
    /// composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: String = string.chars().trim_left().collect();
    /// assert_eq!(trimmed, "abc  ");
    /// ```
    ///
    /// The following example demonstrates how to trim spaces from the left of a string
    /// composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .trim_left()
    ///     .collect();
    /// assert_eq!(trimmed, "abc  ");
    /// ```
    ///
    /// The following example demonstrates how to trim spaces from the left of a string
    /// composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: Vec<u8> = string.bytes().trim_left().collect();
    /// assert_eq!(trimmed, vec![b'a', b'b', b'c', b' ', b' ']);
    /// ```
    fn trim_left(self) -> TrimLeft<Self> {
        TrimLeft::from(self)
    }

    #[inline(always)]
    /// Trims spaces from the right of the iterator.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to trim spaces from the right of a string
    /// composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: String = string.chars().trim_right().collect();
    /// assert_eq!(trimmed, "  abc");
    /// ```
    ///
    /// The following example demonstrates how to trim spaces from the right of a string
    /// composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .trim_right()
    ///     .collect();
    /// assert_eq!(trimmed, "  abc");
    /// ```
    ///
    /// The following example demonstrates how to trim spaces from the right of a string
    /// composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: Vec<u8> = string.bytes().trim_right().collect();
    /// assert_eq!(trimmed, vec![b' ', b' ', b'a', b'b', b'c']);
    /// ```
    fn trim_right(self) -> TrimRight<Self>
    where
        Self: DoubleEndedIterator,
    {
        TrimRight::from(self)
    }

    #[inline(always)]
    /// Trims spaces from both sides of the iterator.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to trim spaces from both sides of a string
    /// composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: String = string.chars().trim().collect();
    /// assert_eq!(trimmed, "abc");
    /// ```
    ///
    /// The following example demonstrates how to trim spaces from both sides of a string
    /// composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .trim()
    ///     .collect();
    /// assert_eq!(trimmed, "abc");
    /// ```
    ///
    /// The following example demonstrates how to trim spaces from both sides of a string
    /// composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "  abc  ";
    /// let trimmed: Vec<u8> = string.bytes().trim().collect();
    /// assert_eq!(trimmed, vec![b'a', b'b', b'c']);
    /// ```
    fn trim(self) -> Trim<Self>
    where
        Self: DoubleEndedIterator,
    {
        Trim::from(self)
    }

    #[inline(always)]
    /// Trims null characters from the left of the iterator.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to trim null characters from the left of a string
    /// composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: String = string.chars().trim_null_left().collect();
    /// assert_eq!(trimmed, "abc\0\0");
    /// ```
    ///
    /// The following example demonstrates how to trim null characters from the left of a string
    /// composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .trim_null_left()
    ///     .collect();
    /// assert_eq!(trimmed, "abc\0\0");
    /// ```
    ///
    /// The following example demonstrates how to trim null characters from the left of a string
    /// composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: Vec<u8> = string.bytes().trim_null_left().collect();
    /// assert_eq!(trimmed, vec![b'a', b'b', b'c', b'\0', b'\0']);
    /// ```
    fn trim_null_left(self) -> TrimNullLeft<Self> {
        TrimNullLeft::from(self)
    }

    #[inline(always)]
    /// Trims null characters from the right of the iterator.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to trim null characters from the right of a string
    /// composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: String = string.chars().trim_null_right().collect();
    /// assert_eq!(trimmed, "\0\0abc");
    /// ```
    ///
    /// The following example demonstrates how to trim null characters from the right of a string
    /// composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .trim_null_right()
    ///     .collect();
    /// assert_eq!(trimmed, "\0\0abc");
    /// ```
    ///
    /// The following example demonstrates how to trim null characters from the right of a string
    /// composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: Vec<u8> = string.bytes().trim_null_right().collect();
    /// assert_eq!(trimmed, vec![b'\0', b'\0', b'a', b'b', b'c']);
    /// ```
    fn trim_null_right(self) -> TrimNullRight<Self>
    where
        Self: DoubleEndedIterator,
    {
        TrimNullRight::from(self)
    }

    #[inline(always)]
    /// Trims null characters from both sides of the iterator.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to trim null characters from both sides of a string
    /// composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: String = string.chars().trim_null().collect();
    /// assert_eq!(trimmed, "abc");
    /// ```
    ///
    /// The following example demonstrates how to trim null characters from both sides of a string
    /// composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .trim_null()
    ///     .collect();
    /// assert_eq!(trimmed, "abc");
    /// ```
    ///
    /// The following example demonstrates how to trim null characters from both sides of a string
    /// composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "\0\0abc\0\0";
    /// let trimmed: Vec<u8> = string.bytes().trim_null().collect();
    /// assert_eq!(trimmed, vec![b'a', b'b', b'c']);
    /// ```
    fn trim_null(self) -> TrimNull<Self>
    where
        Self: DoubleEndedIterator,
    {
        TrimNull::from(self)
    }

    #[inline(always)]
    /// Converts all characters to lowercase.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to convert all characters to lowercase
    /// of a string composed of `char`:
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "AbC";
    /// let lowercase: String = string.chars().lower().collect();
    /// assert_eq!(lowercase, "abc");
    /// ```
    ///
    /// The following example demonstrates how to convert all characters to lowercase
    /// of a string composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "AbC";
    /// let lowercase: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .lower()
    ///     .collect();
    /// assert_eq!(lowercase, "abc");
    /// ```
    ///
    /// The following example demonstrates how to convert all characters to lowercase
    /// of a string composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "AbC";
    /// let lowercase: Vec<u8> = string.bytes().lower().collect();
    /// assert_eq!(lowercase, vec![b'a', b'b', b'c']);
    /// ```
    fn lower(self) -> Lowercase<Self> {
        Lowercase::from(self)
    }

    #[inline(always)]
    /// Converts all non-alphanumerical characters to spaces.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to convert all non-alphanumerical characters to spaces
    /// of a string composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "a89???b#c";
    /// let alphanumeric: String = string.chars().alphanumeric().collect();
    /// assert_eq!(alphanumeric, "a89   b c");
    /// ```
    ///
    /// The following example demonstrates how to convert all non-alphanumerical characters to spaces
    /// of a string composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "a89???∂™b#c";
    /// let alphanumeric: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .alphanumeric()
    ///     .collect();
    /// assert_eq!(alphanumeric, "a89   b c");
    /// ```
    ///
    /// The following example demonstrates how to convert all non-alphanumerical characters to spaces
    /// of a string composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "a89???b#c";
    /// let alphanumeric: Vec<u8> = string.bytes().alphanumeric().collect();
    /// assert_eq!(
    ///     alphanumeric,
    ///     vec![b'a', b'8', b'9', b' ', b' ', b' ', b'b', b' ', b'c']
    /// );
    /// ```
    fn alphanumeric(self) -> Alphanumeric<Self> {
        Alphanumeric::from(self)
    }

    #[inline(always)]
    /// Normalizes spaces, removing subsequent spaces.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to normalize spaces, removing subsequent spaces
    /// of a string composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "a  b   c g g ";
    /// let normalized: String = string.chars().dedup_spaces().collect();
    /// assert_eq!(normalized, "a b c g g ");
    /// ```
    ///
    /// The following example demonstrates how to normalize spaces, removing subsequent spaces
    /// of a string composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "a  b   c g ∞Ωg ";
    /// let normalized: String = string
    ///     .chars()
    ///     .filter_map(|c| ASCIIChar::try_from(c).ok())
    ///     .dedup_spaces()
    ///     .collect();
    /// assert_eq!(normalized, "a b c g g ");
    /// ```
    ///
    /// The following example demonstrates how to normalize spaces, removing subsequent spaces
    /// of a string composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let string = "a  b   c g g ";
    /// let normalized: Vec<u8> = string.bytes().dedup_spaces().collect();
    /// assert_eq!(
    ///     normalized,
    ///     vec![b'a', b' ', b'b', b' ', b'c', b' ', b'g', b' ', b'g', b' ']
    /// );
    /// ```
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
