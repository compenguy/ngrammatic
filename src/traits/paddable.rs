//! Submodule providing the paddable trait.

/// Trait defining a paddable item.
pub trait Paddable {
    /// The padding value for the type.
    const PADDING: Self;
}

/// Macro to implement the Paddable trait for signed and unsigned integers.
#[macro_export]
macro_rules! impl_paddable {
    ($type:ty) => {
        impl Paddable for $type {
            const PADDING: Self = 0;
        }
    };
}

/// Macro to implement the Paddable trait for several signed and unsigned integer types.
#[macro_export]
macro_rules! impl_paddables {
    ($($type:ty),*) => {
        $(impl_paddable!($type);)*
    };
}

impl_paddables!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Paddable for char {
    const PADDING: Self = '\0';
}

impl Paddable for crate::ASCIIChar {
    const PADDING: Self = crate::ASCIIChar::NUL;
}