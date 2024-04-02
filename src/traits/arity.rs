//! Submodule defining struct markers for Arity.

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Index, IndexMut},
};

/// Trait defining the arity of a struct.
/// 
/// # Implementative details
/// This is a struct marker for an arity of one.
/// It is used to explicitly define the arity of a struct,
/// and we use it to avoid allowing for illegal options,
/// such as arity zero.
pub trait Arity: Display + Default + Clone + Debug {
    /// The arity of the struct.
    const ARITY: usize;
    /// The padding for the struct.
    const PADDING: Self::Gram;
    /// The type of the n-gram.
    type Gram: Copy
        + Index<usize, Output = u8>
        + IndexMut<usize, Output = u8>
        + Hash
        + Debug
        + AsRef<[u8]>
        + Default
        + Ord
        + Eq
        + PartialEq;
}

#[derive(Clone, Copy, Default, Debug)]
/// Arity of one.
pub struct ArityOne;

#[derive(Clone, Copy, Default, Debug)]
/// Arity of two.
pub struct ArityTwo;

#[derive(Clone, Copy, Default, Debug)]
/// Arity of three.
pub struct ArityThree;

#[derive(Clone, Copy, Default, Debug)]
/// Arity of four.
pub struct ArityFour;

#[derive(Clone, Copy, Default, Debug)]
/// Arity of four.
pub struct ArityFive;

impl Arity for ArityOne {
    const ARITY: usize = 1;
    const PADDING: [u8; 1] = [b' '];
    type Gram = [u8; 1];
}

impl Display for ArityOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArityOne")
    }
}

impl Arity for ArityTwo {
    const ARITY: usize = 2;
    const PADDING: [u8; 2] = [b' ', b' '];
    type Gram = [u8; 2];
}

impl Display for ArityTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArityTwo")
    }
}

impl Arity for ArityThree {
    const ARITY: usize = 3;
    const PADDING: [u8; 3] = [b' ', b' ', b' '];
    type Gram = [u8; 3];
}

impl Display for ArityThree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArityThree")
    }
}

impl Arity for ArityFour {
    const ARITY: usize = 4;
    const PADDING: [u8; 4] = [b' ', b' ', b' ', b' '];
    type Gram = [u8; 4];
}

impl Display for ArityFour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArityFour")
    }
}

impl Arity for ArityFive {
    const ARITY: usize = 5;
    const PADDING: [u8; 5] = [b' ', b' ', b' ', b' ', b' '];
    type Gram = [u8; 5];
}

impl Display for ArityFive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArityFive")
    }
}
