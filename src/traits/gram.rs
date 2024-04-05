//! Trait defining the unit type for an ngram.

use std::{hash::Hash, ops::{Index, IndexMut}};

use crate::{ASCIIChar, Paddable};

/// Type alias for a monogram.
pub type MonoGram<T> = [T; 1];
/// Type alias for a bigram.
pub type BiGram<T> = [T; 2];
/// Type alias for a trigram.
pub type TriGram<T> = [T; 3];
/// Type alias for a tetragram.
pub type TetraGram<T> = [T; 4];
/// Type alias for a pentagram.
pub type PentaGram<T> = [T; 5];
/// Type alias for a hexagram.
pub type HexaGram<T> = [T; 6];
/// Type alias for a heptagram.
pub type HeptaGram<T> = [T; 7];
/// Type alias for an octagram.
pub type OctaGram<T> = [T; 8];

/// Trait defining
pub trait Gram: Copy + Clone + Default + Hash + Eq + PartialEq + Ord{}

impl Gram for u8 {}

impl Gram for char {}

impl Gram for ASCIIChar {}

/// Trait defining a
pub trait Ngram:
    Default
    + Clone
    + Copy
    + Ord
    + Eq
    + PartialEq
    + Hash
    + Index<usize, Output = <Self as Ngram>::G>
    + IndexMut<usize, Output = <Self as Ngram>::G>
{
    /// The type of the ngram.
    type G: Gram;

    /// The arity of the ngram.
    const ARITY: usize;

    /// Rotate the ngram to the left.
    fn rotate_left(&mut self);
}

impl<G: Gram> Ngram for MonoGram<G> {
    const ARITY: usize = 1;
    type G = G;

    #[inline(always)]
    fn rotate_left(&mut self) {
        // Do nothing.
    }
}

impl<G: Gram> Ngram for BiGram<G> {
    const ARITY: usize = 2;
    type G = G;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[G]>::rotate_left(self, 1);
    }
}

impl<G: Gram> Ngram for TriGram<G> {
    const ARITY: usize = 3;
    type G = G;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[G]>::rotate_left(self, 1);
    }
}

impl<G: Gram> Ngram for TetraGram<G> {
    const ARITY: usize = 4;
    type G = G;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[G]>::rotate_left(self, 1);
    }
}

impl<G: Gram> Ngram for PentaGram<G> {
    const ARITY: usize = 5;
    type G = G;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[G]>::rotate_left(self, 1);
    }
}

impl<G: Gram> Ngram for HexaGram<G> {
    const ARITY: usize = 6;
    type G = G;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[G]>::rotate_left(self, 1);
    }
}

impl<G: Gram> Ngram for HeptaGram<G> {
    const ARITY: usize = 7;
    type G = G;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[G]>::rotate_left(self, 1);
    }
}

impl<G: Gram> Ngram for OctaGram<G> {
    const ARITY: usize = 8;
    type G = G;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[G]>::rotate_left(self, 1);
    }
}

/// Trait defining a paddable ngram.
pub trait PaddableNgram: Ngram
where
    <Self as Ngram>::G: Paddable,
{
    /// The padding type. It will generally be an
    /// array of one unit smaller than the arity of Self.
    type Pad: IntoIterator<Item = Self::G>;
    /// The padding value.
    const PADDING: Self::Pad;
}

impl<G: Paddable + Gram> PaddableNgram for MonoGram<G> {
    type Pad = [G; 0];
    const PADDING: Self::Pad = [];
}

impl<G: Paddable + Gram> PaddableNgram for BiGram<G> {
    type Pad = [G; 1];
    const PADDING: Self::Pad = [G::PADDING];
}

impl<G: Paddable + Gram> PaddableNgram for TriGram<G> {
    type Pad = [G; 2];
    const PADDING: Self::Pad = [G::PADDING; 2];
}

impl<G: Paddable + Gram> PaddableNgram for TetraGram<G> {
    type Pad = [G; 3];
    const PADDING: Self::Pad = [G::PADDING; 3];
}

impl<G: Paddable + Gram> PaddableNgram for PentaGram<G> {
    type Pad = [G; 4];
    const PADDING: Self::Pad = [G::PADDING; 4];
}

impl<G: Paddable + Gram> PaddableNgram for HexaGram<G> {
    type Pad = [G; 5];
    const PADDING: Self::Pad = [G::PADDING; 5];
}

impl<G: Paddable + Gram> PaddableNgram for HeptaGram<G> {
    type Pad = [G; 6];
    const PADDING: Self::Pad = [G::PADDING; 6];
}

impl<G: Paddable + Gram> PaddableNgram for OctaGram<G> {
    type Pad = [G; 7];
    const PADDING: Self::Pad = [G::PADDING; 7];
}
