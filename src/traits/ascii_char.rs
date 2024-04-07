//! Submodule providing a small implementation of an ASCII character.
//!
//! # Implementative details
//! While we are aware that there is an unstable features [`ascii_char`](https://doc.rust-lang.org/std/ascii/enum.Char.html), that
//! will, when it stabilizes, provide a more complete implementation of ASCII characters, we provide a small implementation
//! that provide all that is needed for the library.

use std::fmt::Debug;
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
#[repr(transparent)]
/// Represents an ASCII character.
pub struct ASCIIChar {
    /// The character.
    character: u8,
}

impl From<u8> for ASCIIChar {
    #[inline(always)]
    fn from(character: u8) -> Self {
        ASCIIChar { character }
    }
}

impl From<ASCIIChar> for u8 {
    #[inline(always)]
    fn from(ascii_char: ASCIIChar) -> u8 {
        ascii_char.character
    }
}

impl TryFrom<char> for ASCIIChar {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(character: char) -> Result<Self, Self::Error> {
        if character.is_ascii() {
            Ok(ASCIIChar {
                character: character as u8,
            })
        } else {
            Err("Character is not ASCII")
        }
    }
}

impl Display for ASCIIChar {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.character as char)
    }
}

impl Debug for ASCIIChar {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ASCIIChar({})", self.character as char)
    }
}

/// Provides character operations by manipulation of the underlying `u8` value.
impl ASCIIChar {
    /// The NUL character.
    pub const NUL: Self = ASCIIChar { character: 0 };
    /// The space character.
    pub const SPACE: Self = ASCIIChar { character: b' ' };

    #[inline(always)]
    /// Returns the lowercase version of the character.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let ascii_char = ASCIIChar::from(b'A');
    /// let lowercase = ascii_char.to_lowercase();
    /// assert_eq!(lowercase, ASCIIChar::from(b'a'));
    /// ```
    pub fn to_lowercase(self) -> Self {
        ASCIIChar {
            character: self.character.to_ascii_lowercase(),
        }
    }

    #[inline(always)]
    /// Returns the uppercase version of the character.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let ascii_char = ASCIIChar::from(b'a');
    /// let uppercase = ascii_char.to_uppercase();
    /// assert_eq!(uppercase, ASCIIChar::from(b'A'));
    /// ```
    pub fn to_uppercase(self) -> Self {
        ASCIIChar {
            character: self.character.to_ascii_uppercase(),
        }
    }

    #[inline(always)]
    /// Returns whether the current character is a space-like.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let ascii_char = ASCIIChar::from(b' ');
    /// assert!(ascii_char.is_space_like());
    /// let ascii_char = ASCIIChar::from(b'a');
    /// assert!(!ascii_char.is_space_like());
    /// ```
    pub fn is_space_like(self) -> bool {
        self.character.is_ascii_whitespace()
    }

    #[inline(always)]
    /// Returns whether the current character is alphanumeric.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let ascii_char = ASCIIChar::from(b'a');
    /// assert!(ascii_char.is_alphanumeric());
    /// let ascii_char = ASCIIChar::from(b' ');
    /// assert!(!ascii_char.is_alphanumeric());
    /// ```
    pub fn is_alphanumeric(self) -> bool {
        self.character.is_ascii_alphanumeric()
    }
}

/// Iterator that converts an iterator of `char` to an iterator of `ASCIIChar`.
///
/// # Implementative details
/// Since no all of the characters in the iterator are ASCII, we FILTER OUT all the characters that are not ASCII.
/// In some corner cases, this might yield an empty iterator. Note that chars in Rust are u32, and as such the conversion
/// will yield u8, which is the underlying representation of ASCII characters, occupying a fourth of the space.
pub struct ASCIICharIterator<I> {
    /// The iterator of characters.
    iterator: I,
}

impl<I> From<I> for ASCIICharIterator<I> {
    #[inline(always)]
    fn from(iterator: I) -> Self {
        ASCIICharIterator { iterator }
    }
}

impl<I> Iterator for ASCIICharIterator<I>
where
    I: Iterator<Item = char>,
{
    type Item = ASCIIChar;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .and_then(|character| match ASCIIChar::try_from(character) {
                // If the character is ASCII, we return it.
                Ok(ascii_char) => Some(ascii_char),
                // Otherwise we proceed to the next character.
                Err(_) => self.next(),
            })
    }
}

impl<I> ExactSizeIterator for ASCIICharIterator<I>
where
    I: ExactSizeIterator<Item = char>,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.iterator.len()
    }
}

impl<I> DoubleEndedIterator for ASCIICharIterator<I>
where
    I: DoubleEndedIterator<Item = char>,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator
            .next_back()
            .and_then(|character| match ASCIIChar::try_from(character) {
                // If the character is ASCII, we return it.
                Ok(ascii_char) => Some(ascii_char),
                // Otherwise we proceed to the next character.
                Err(_) => self.next_back(),
            })
    }
}

/// Trait to be implemented for all iterators that yield `char`
/// so that they can be converted to `ASCIICharIterator`.
pub trait ToASCIICharIterator: IntoIterator<Item = char> {
    /// Converts the iterator to an `ASCIICharIterator`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let ascii = "ab∂Ωc".chars().ascii().collect::<Vec<_>>();
    /// 
    /// assert_eq!(ascii, vec![ASCIIChar::from(b'a'), ASCIIChar::from(b'b'), ASCIIChar::from(b'c')]);
    /// ```
    fn ascii(self) -> ASCIICharIterator<Self>
    where
        Self: Sized;
}

impl<I> ToASCIICharIterator for I
where
    I: IntoIterator<Item = char>,
{
    #[inline(always)]
    fn ascii(self) -> ASCIICharIterator<Self>
    where
        Self: Sized,
    {
        ASCIICharIterator::from(self)
    }
}

/// Implements the collect to string of an iterator of `ASCIIChar`.
impl std::iter::FromIterator<ASCIIChar> for String {
    #[inline(always)]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = ASCIIChar>,
    {
        iter.into_iter().map(|ascii_char| ascii_char.character as char).collect()
    }
}