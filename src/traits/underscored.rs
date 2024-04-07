//! Submodule providing the Underscored trait.
//!
//! The Underscored trait provides a method to format integers with underscores.

/// Trait providing a method to format integers with underscores.
pub trait Underscored {
    /// Returns the integer formatted with underscores.
    fn underscored(&self) -> String;
}

/// Macro implementing the Underscored trait for the given integer
/// type.
#[macro_export]
macro_rules! impl_underscored {
    ($t:ty) => {
        impl Underscored for $t {
            fn underscored(&self) -> String {
                let s = self.to_string();
                let mut result = String::new();
                let mut count = 0;
                for c in s.chars().rev() {
                    if count % 3 == 0 && count != 0 {
                        result.push('_');
                    }
                    result.push(c);
                    count += 1;
                }
                result.chars().rev().collect()
            }
        }
    };
}

/// Macro implementing the Underscored trait for several integers.
#[macro_export]
macro_rules! impl_underscored_for_integers {
    ($($t:ty),*) => {
        $(impl_underscored!($t);)*
    };
}

impl_underscored_for_integers!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
