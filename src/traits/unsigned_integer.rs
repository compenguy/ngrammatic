//! Submodule providing traits to define an unsigned integer type.
use crate::{One, Zero};

/// Trait defining an unsigned integer type.
pub trait UnsignedInteger:
    Copy
    + Eq
    + One
    + Zero
    + Ord
    + core::ops::Add
    + core::ops::Sub
    + core::ops::Mul
    + core::ops::Div
    + core::ops::Rem
    + core::ops::Shl
    + core::ops::Shr
    + core::ops::BitAnd
    + core::ops::BitOr
    + core::ops::BitXor
    + core::ops::Not
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + core::fmt::Debug
    + core::fmt::Display
    + core::fmt::Octal
    + core::iter::Sum
{
    /// Convert the integer to a usize.
    fn as_usize(&self) -> usize;

    /// Add one to the integer in a saturating manner.
    fn saturating_add_one(&self) -> Self;
}

/// Macro to implement the UnsignedInteger trait for a given type.
#[macro_export]
macro_rules! impl_unsigned_integer {
    ($type:ty) => {
        impl UnsignedInteger for $type {
            fn as_usize(&self) -> usize {
                *self as usize
            }

            fn saturating_add_one(&self) -> Self {
                self.saturating_add(Self::ONE)
            }
        }

        impl One for $type {
            const ONE: Self = 1;

            fn is_one(&self) -> bool {
                *self == Self::ONE
            }
        }

        impl Zero for $type {
            const ZERO: Self = 0;

            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }
    };
}

/// Macro to implement the UnsignedInteger trait for several types.
#[macro_export]
macro_rules! impl_unsigned_integers {
    ($($type:ty),*) => {
        $(impl_unsigned_integer!($type);)*
    };
}

impl_unsigned_integers!(u8, u16, u32, u64, u128, usize);
