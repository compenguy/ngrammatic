//! Trait definition for floating point numbers.

use crate::{One, Zero, Three};

/// Trait defining a floating point number.
pub trait Float:
    Copy
    + One
    + Zero
    + Three
    + PartialOrd
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Neg<Output = Self>
    + core::fmt::Debug
{
    /// Returns the absolute value of the float.
    fn abs(self) -> Self;

    /// Returns an f64 from the provided value.
    fn to_f64(self) -> f64;

    /// Converts a given f64 to the float type.
    fn from_f64(value: f64) -> Self;
}

#[cfg(feature = "half")]
impl One for half::f16 {
    const ONE: Self = half::f16::from_f32_const(1.0);

    fn is_one(&self) -> bool {
        (self - half::f16::ONE).is_zero()
    }
}

#[cfg(feature = "half")]
impl Zero for half::f16 {
    const ZERO: Self = half::f16::from_f32_const(0.0);

    fn is_zero(&self) -> bool {
        self.abs() < half::f16::EPSILON
    }
}

#[cfg(feature = "half")]
impl Three for half::f16 {
    const THREE: Self = half::f16::from_f32_const(3.0);
}

#[cfg(feature = "half")]
impl One for half::bf16 {
    const ONE: Self = half::bf16::from_f32_const(1.0);

    fn is_one(&self) -> bool {
        (self - half::bf16::ONE).is_zero()
    }
}

#[cfg(feature = "half")]
impl Zero for half::bf16 {
    const ZERO: Self = half::bf16::from_f32_const(0.0);

    fn is_zero(&self) -> bool {
        self.abs() < half::bf16::EPSILON
    }
}

#[cfg(feature = "half")]
impl Three for half::bf16 {
    const THREE: Self = half::bf16::from_f32_const(3.0);
}

impl One for f32 {
    const ONE: Self = 1.0;

    fn is_one(&self) -> bool {
        (self - f32::ONE).is_zero()
    }
}

impl Zero for f32 {
    const ZERO: Self = 0.0;

    fn is_zero(&self) -> bool {
        self.abs() < f32::EPSILON
    }
}

impl Three for f32 {
    const THREE: Self = 3.0;
}

impl One for f64 {
    const ONE: Self = 1.0;

    fn is_one(&self) -> bool {
        (self - f64::ONE).is_zero()
    }
}

impl Zero for f64 {
    const ZERO: Self = 0.0;

    fn is_zero(&self) -> bool {
        self.abs() < f64::EPSILON
    }
}

impl Three for f64 {
    const THREE: Self = 3.0;
}


#[cfg(feature = "half")]
/// Implement the `Float` trait for the `half::f16` type.
impl Float for half::f16 {
    fn abs(self) -> Self {
        half::f16::abs(self)
    }

    fn to_f64(self) -> f64 {
        f64::from(self)
    }

    fn from_f64(value: f64) -> Self {
        half::f16::from_f64(value)
    }
}

#[cfg(feature = "half")]
/// Implement the `Float` trait for the `half::bf16` type.
impl Float for half::bf16 {
    fn abs(self) -> Self {
        half::bf16::abs(self)
    }

    fn to_f64(self) -> f64 {
        f64::from(self)
    }

    fn from_f64(value: f64) -> Self {
        half::bf16::from_f64(value)
    }
}

impl Float for f32 {
    fn abs(self) -> Self {
        f32::abs(self)
    }

    fn to_f64(self) -> f64 {
        f64::from(self)
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }
}
impl Float for f64 {
    fn abs(self) -> Self {
        f64::abs(self)
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn from_f64(value: f64) -> Self {
        value
    }
}
