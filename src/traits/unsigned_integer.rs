//! Submodule providing traits to define an unsigned integer type.

/// Trait defining the value one.
pub trait One {
    /// The value one for the type.
    const ONE: Self;
}


/// Trait defining an unsigned integer type.
pub trait UnsignedInteger:
    Copy
    + Eq
    + One
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
}

/// Macro to implement the UnsignedInteger trait for a given type.
#[macro_export]
macro_rules! impl_unsigned_integer {
    ($type:ty) => {
        impl UnsignedInteger for $type {
            fn as_usize(&self) -> usize {
                *self as usize
            }
        }

        impl One for $type {
            const ONE: Self = 1;
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