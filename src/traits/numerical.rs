//! Numerical traits.

/// Trait defining the value zero.
pub trait Zero {
    /// The value zero for the type.
    const ZERO: Self;

    /// Check if the value is zero or nearly zero (in the case of floating point numbers).
    fn is_zero(&self) -> bool;
}

/// Trait defining the value one.
pub trait One {
    /// The value one for the type.
    const ONE: Self;

    /// Check if the value is one.
    fn is_one(&self) -> bool;
}

/// Trait defining the value three.
pub trait Three {
    /// The value three for the type.
    const THREE: Self;
}

/// Trait defining a value between one and three.
pub trait BetweenOneAndThree: PartialOrd + One + Three + Sized {
    /// Check if the value is between one and three.
    ///
    /// # Examples
    ///
    /// The following example demonstrates how to check if a value is between one and three:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let value = 2;
    /// assert!(value.is_between_one_and_three());
    /// ```
    ///
    /// The following example demonstrates how to check if a value is not between one and three:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let value = 4;
    /// assert!(!value.is_between_one_and_three());
    /// ```
    fn is_between_one_and_three(&self) -> bool {
        (Self::ONE..=Self::THREE).contains(self)
    }
}

impl<T> BetweenOneAndThree for T where T: PartialOrd + One + Three + Sized {}

/// Macro defining the `Zero`, `One`, and `Three` traits for signed integers.
#[macro_export]
macro_rules! impl_signed_integer {
    ($type:ty) => {
        impl Zero for $type {
            const ZERO: Self = 0;

            fn is_zero(&self) -> bool {
                *self == 0
            }
        }

        impl One for $type {
            const ONE: Self = 1;

            fn is_one(&self) -> bool {
                *self == 1
            }
        }

        impl Three for $type {
            const THREE: Self = 3;
        }
    };
}

/// Macro defining the `Zero`, `One`, and `Three` traits for several signed integer types.
#[macro_export]
macro_rules! impl_signed_integers {
    ($($type:ty),*) => {
        $(impl_signed_integer!($type);)*
    };
}

impl_signed_integers!(i8, i16, i32, i64, i128, isize);
