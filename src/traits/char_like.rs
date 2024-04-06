//! Submodule defining char-like types.

use crate::ASCIIChar;

/// Trait defining a char-like type.
pub trait CharLike:
    Copy
    + Clone
    + Default
    + PartialEq
    + Eq
    + Ord
    + PartialOrd
    + std::fmt::Debug
    + std::fmt::Display
    + std::hash::Hash
{
    /// The space character.
    const SPACE: Self;
    /// The NUL character.
    const NUL: Self;

    /// Returns the lowercase version of the character.
    fn to_lowercase(self) -> Self;

    /// Returns the uppercase version of the character.
    fn to_uppercase(self) -> Self;

    /// Returns whether the current character is a space-like.
    fn is_space_like(self) -> bool;

    /// Returns whether the current character is alphanumeric.
    fn is_alphanumeric(self) -> bool;

    #[inline(always)]
    /// Returns whether the current character is a NUL.
    fn is_nul(self) -> bool {
        self == Self::NUL
    }
}

impl CharLike for char {
    const SPACE: Self = ' ';
    const NUL: Self = '\0';

    #[inline(always)]
    fn to_lowercase(self) -> Self {
        self.to_ascii_lowercase()
    }

    #[inline(always)]
    fn to_uppercase(self) -> Self {
        self.to_ascii_uppercase()
    }

    #[inline(always)]
    fn is_space_like(self) -> bool {
        self.is_whitespace()
    }

    #[inline(always)]
    fn is_alphanumeric(self) -> bool {
        self.is_alphanumeric()
    }
}

impl CharLike for u8 {
    const SPACE: Self = b' ';
    const NUL: Self = b'\0';

    #[inline(always)]
    fn to_lowercase(self) -> Self {
        self.to_ascii_lowercase()
    }

    #[inline(always)]
    fn to_uppercase(self) -> Self {
        self.to_ascii_uppercase()
    }

    #[inline(always)]
    fn is_space_like(self) -> bool {
        self.is_ascii_whitespace()
    }

    #[inline(always)]
    fn is_alphanumeric(self) -> bool {
        self.is_ascii_alphanumeric()
    }
}

impl CharLike for ASCIIChar {
    const SPACE: Self = ASCIIChar::SPACE;
    const NUL: Self = ASCIIChar::NUL;

    #[inline(always)]
    fn to_lowercase(self) -> Self {
        self.to_lowercase()
    }

    #[inline(always)]
    fn to_uppercase(self) -> Self {
        self.to_uppercase()
    }

    #[inline(always)]
    fn is_space_like(self) -> bool {
        self.is_space_like()
    }

    #[inline(always)]
    fn is_alphanumeric(self) -> bool {
        self.is_alphanumeric()
    }
}