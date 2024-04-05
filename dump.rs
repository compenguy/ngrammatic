#![feature(prelude_import)]
/*!# Ngrammatic
[![Build status](https://github.com/compenguy/ngrammatic/actions/workflows/clippy.yml/badge.svg)](https://github.com/compenguy/ngrammatic/actions)
[![Crates.io](https://img.shields.io/crates/v/ngrammatic.svg)](https://crates.io/crates/ngrammatic)
[![Documentation](https://docs.rs/ngrammatic/badge.svg)](https://docs.rs/ngrammatic/)

This crate provides fuzzy search/string matching using N-grams.

This implementation is character-based, rather than word based, matching
solely based on string similarity. It is modelled somewhat after the
[python ngram module](https://pythonhosted.org/ngram/ngram.html) with some inspiration from
[chappers' blog post on fuzzy matching with ngrams](http://chappers.github.io/web%20micro%20log/2015/04/29/comparison-of-ngram-fuzzy-matching-approaches/).

The crate is implemented in three parts: the `Corpus`, which is an
index connecting strings (words, symbols, whatever) to their `Ngrams`,
and `SearchResult`s, which contains a fuzzy match result, with the
word and a similarity measure in the range of 0.0 to 1.0.

The general usage pattern is to construct a `Corpus`, `.add()` your
list of valid symbols to it, and then perform `.search()`es of valid,
unknown, misspelled, etc symbols on the `Corpus`. The results come
back as a vector of up to `limit` results, sorted from highest similarity
to lowest.

Licensed under the MIT license.

## Installation

This crate is published on [crates.io](https://crates.io/crates/).

To use it, add this to your Cargo.toml:

```toml
[dependencies]
ngrammatic = "0.4.0"
```

## Usage example
To do fuzzy matching, build up your corpus of valid symbols like this:

```rust
use ngrammatic::{CorpusBuilder, Pad};

let mut corpus = CorpusBuilder::<2>::default()
    .pad_full(Pad::Auto)
    .finish();

// Build up the list of known words
corpus.add_text("pie");
corpus.add_text("animal");
corpus.add_text("tomato");
corpus.add_text("seven");
corpus.add_text("carbon");

// Now we can try an unknown/misspelled word, and find a similar match
// in the corpus
let results = corpus.search("tomacco", 0.25, 10);
let top_match = results.first();

assert!(top_match.is_some());
assert!(top_match.unwrap().similarity > 0.5);
assert_eq!(top_match.unwrap().text,String::from("tomato"));
```
*/
#![deny(missing_docs)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod traits {
    //! This module contains the traits that are used in the library.
    pub mod unsigned_integer {
        //! Submodule providing traits to define an unsigned integer type.
        use crate::{One, Zero};
        /// Trait defining an unsigned integer type.
        pub trait UnsignedInteger: Copy + Eq + One + Zero + Ord + core::ops::Add + core::ops::Sub + core::ops::Mul + core::ops::Div + core::ops::Rem + core::ops::Shl + core::ops::Shr + core::ops::BitAnd + core::ops::BitOr + core::ops::BitXor + core::ops::Not + core::ops::AddAssign + core::ops::SubAssign + core::ops::MulAssign + core::ops::DivAssign + core::fmt::Debug + core::fmt::Display + core::fmt::Octal + core::iter::Sum {
            /// Convert the integer to a usize.
            fn as_usize(&self) -> usize;
            /// Add one to the integer in a saturating manner.
            fn saturating_add_one(&self) -> Self;
        }
        impl UnsignedInteger for u8 {
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn saturating_add_one(&self) -> Self {
                self.saturating_add(Self::ONE)
            }
        }
        impl One for u8 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == Self::ONE
            }
        }
        impl Zero for u8 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }
        impl UnsignedInteger for u16 {
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn saturating_add_one(&self) -> Self {
                self.saturating_add(Self::ONE)
            }
        }
        impl One for u16 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == Self::ONE
            }
        }
        impl Zero for u16 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }
        impl UnsignedInteger for u32 {
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn saturating_add_one(&self) -> Self {
                self.saturating_add(Self::ONE)
            }
        }
        impl One for u32 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == Self::ONE
            }
        }
        impl Zero for u32 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }
        impl UnsignedInteger for u64 {
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn saturating_add_one(&self) -> Self {
                self.saturating_add(Self::ONE)
            }
        }
        impl One for u64 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == Self::ONE
            }
        }
        impl Zero for u64 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }
        impl UnsignedInteger for u128 {
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn saturating_add_one(&self) -> Self {
                self.saturating_add(Self::ONE)
            }
        }
        impl One for u128 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == Self::ONE
            }
        }
        impl Zero for u128 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }
        impl UnsignedInteger for usize {
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn saturating_add_one(&self) -> Self {
                self.saturating_add(Self::ONE)
            }
        }
        impl One for usize {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == Self::ONE
            }
        }
        impl Zero for usize {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }
    }
    pub mod key {
        //! Trait defining a key and its hasher.
        use std::collections::HashMap;
        use crate::traits::ascii_char::ToASCIICharIterator;
        use crate::traits::iter_ngrams::IntoNgrams;
        use crate::{ASCIIChar, ASCIICharIterator, Gram, Ngram};
        /// Trait defining a key.
        pub trait Key<G: Gram>: Clone + PartialEq + Eq {
            /// The type of the grams iterator.
            type Grams<'a>: Iterator<Item = G> where Self: 'a;
            /// Returns an iterator over the grams of the key.
            fn grams(&self) -> Self::Grams<'_>;
            /// Returns the counts of the ngrams.
            fn counts<NG: Ngram<G = G>>(&self) -> HashMap<NG, usize> {
                let mut ngram_counts: HashMap<NG, usize> = HashMap::new();
                for ngram in self.grams().ngrams::<NG>() {
                    ngram_counts
                        .entry(ngram)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                ngram_counts
            }
        }
        impl Key<char> for String {
            type Grams<'a> = std::str::Chars<'a>;
            fn grams(&self) -> Self::Grams<'_> {
                self.chars()
            }
        }
        impl Key<char> for &str {
            type Grams<'a> = std::str::Chars<'a> where Self: 'a;
            fn grams(&self) -> Self::Grams<'_> {
                self.chars()
            }
        }
        impl Key<u8> for String {
            type Grams<'a> = std::str::Bytes<'a>;
            fn grams(&self) -> Self::Grams<'_> {
                self.bytes()
            }
        }
        impl Key<u8> for &str {
            type Grams<'a> = std::str::Bytes<'a> where Self: 'a;
            fn grams(&self) -> Self::Grams<'_> {
                self.bytes()
            }
        }
        impl Key<ASCIIChar> for String {
            type Grams<'a> = ASCIICharIterator<std::str::Chars<'a>> where Self: 'a;
            fn grams(&self) -> Self::Grams<'_> {
                self.chars().ascii()
            }
        }
        impl Key<ASCIIChar> for &str {
            type Grams<'a> = ASCIICharIterator<std::str::Chars<'a>> where Self: 'a;
            fn grams(&self) -> Self::Grams<'_> {
                self.chars().ascii()
            }
        }
    }
    pub use key::*;
    pub use unsigned_integer::*;
    pub mod floats {
        //! Trait definition for floating point numbers.
        use crate::{One, Zero, Three};
        /// Trait defining a floating point number.
        pub trait Float: Copy + One + Zero + Three + PartialOrd + core::ops::Add<
                Output = Self,
            > + core::ops::Sub<
                Output = Self,
            > + core::ops::Mul<
                Output = Self,
            > + core::ops::Div<
                Output = Self,
            > + core::ops::Neg<Output = Self> + core::fmt::Debug {
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
    }
    pub use floats::*;
    pub mod numerical {
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
            fn is_between_one_and_three(&self) -> bool {
                (Self::ONE..=Self::THREE).contains(self)
            }
        }
        impl<T> BetweenOneAndThree for T
        where
            T: PartialOrd + One + Three + Sized,
        {}
        impl Zero for i8 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
        impl One for i8 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == 1
            }
        }
        impl Three for i8 {
            const THREE: Self = 3;
        }
        impl Zero for i16 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
        impl One for i16 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == 1
            }
        }
        impl Three for i16 {
            const THREE: Self = 3;
        }
        impl Zero for i32 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
        impl One for i32 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == 1
            }
        }
        impl Three for i32 {
            const THREE: Self = 3;
        }
        impl Zero for i64 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
        impl One for i64 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == 1
            }
        }
        impl Three for i64 {
            const THREE: Self = 3;
        }
        impl Zero for i128 {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
        impl One for i128 {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == 1
            }
        }
        impl Three for i128 {
            const THREE: Self = 3;
        }
        impl Zero for isize {
            const ZERO: Self = 0;
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
        impl One for isize {
            const ONE: Self = 1;
            fn is_one(&self) -> bool {
                *self == 1
            }
        }
        impl Three for isize {
            const THREE: Self = 3;
        }
    }
    pub use numerical::*;
    pub mod keys {
        //! Submodule defining trait for a container of Keys.
        use std::ops::Index;
        use crate::{Gram, Key};
        /// Trait defining a container of keys.
        pub trait Keys<G: Gram>: Index<usize, Output = <Self as Keys<G>>::K> {
            /// The type of the key.
            type K: Key<G>;
            /// The iterator to iter the keys.
            type IterKeys<'a>: Iterator<Item = &'a Self::K> where Self::K: 'a, Self: 'a;
            /// Returns the number of keys.
            fn len(&self) -> usize;
            /// Returns whether the container is empty.
            fn is_empty(&self) -> bool {
                self.len() == 0
            }
            /// Returns an iterator over the keys.
            fn iter(&self) -> Self::IterKeys<'_>;
        }
        impl<G: Gram, K: Key<G>> Keys<G> for Vec<K> {
            type K = K;
            type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;
            fn len(&self) -> usize {
                self.len()
            }
            fn iter(&self) -> Self::IterKeys<'_> {
                <[K]>::iter(self)
            }
        }
    }
    pub use keys::*;
    pub mod gram {
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
        pub trait Gram: Copy + Clone + Default + Hash + Eq + PartialEq + Ord {}
        impl Gram for u8 {}
        impl Gram for char {}
        impl Gram for ASCIIChar {}
        /// Trait defining a
        pub trait Ngram: Default + Clone + Copy + Ord + Eq + PartialEq + Hash + Index<
                usize,
                Output = <Self as Ngram>::G,
            > + IndexMut<usize, Output = <Self as Ngram>::G> {
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
            fn rotate_left(&mut self) {}
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
    }
    pub use gram::*;
    pub mod iter_ngrams {
        //! Submodule providing an iterator to convert an iterator of grams to an iterator of n-grams.
        use crate::{Gram, Ngram};
        /// Struct implementing an iterator to convert an iterator
        /// of grams to an iterator of n-grams.
        pub struct IterNgrams<I, NG>
        where
            I: Iterator<Item = <NG as Ngram>::G>,
            NG: Ngram,
        {
            iter: I,
            ngram: NG,
        }
        impl<I, NG> From<I> for IterNgrams<I, NG>
        where
            I: Iterator<Item = <NG as Ngram>::G>,
            NG: Ngram,
        {
            fn from(mut iter: I) -> Self {
                let mut ngram: NG = Default::default();
                for i in 0..(NG::ARITY - 1) {
                    ngram[i + 1] = iter.next().unwrap();
                }
                IterNgrams { iter, ngram }
            }
        }
        impl<I, NG> Iterator for IterNgrams<I, NG>
        where
            I: Iterator<Item = <NG as Ngram>::G>,
            NG: Ngram,
        {
            type Item = NG;
            fn next(&mut self) -> Option<Self::Item> {
                self.iter
                    .next()
                    .map(|gram| {
                        self.ngram.rotate_left();
                        self.ngram[NG::ARITY - 1] = gram;
                        self.ngram
                    })
            }
        }
        /// Trait defining an iterator to convert an
        /// iterator of grams to an iterator of n-grams.
        pub trait IntoNgrams: Iterator
        where
            <Self as Iterator>::Item: Gram,
        {
            /// Converts an iterator of grams to an iterator of n-grams.
            fn ngrams<NG>(self) -> IterNgrams<Self, NG>
            where
                NG: Ngram<G = <Self as Iterator>::Item>,
                Self: Sized,
            {
                IterNgrams::from(self)
            }
        }
        impl<I> IntoNgrams for I
        where
            I: Iterator,
            <I as std::iter::Iterator>::Item: Gram,
        {}
    }
    pub use iter_ngrams::*;
    pub mod char_normalizer {
        //! Submodule providing traits to normalize iterators of char-like items.
        use crate::CharLike;
        /// Trait defining an iterator to lowercase.
        pub struct Lowercase<I>(I);
        impl<I> From<I> for Lowercase<I> {
            fn from(iter: I) -> Self {
                Lowercase(iter)
            }
        }
        impl<I> Iterator for Lowercase<I>
        where
            I: Iterator,
            <I as Iterator>::Item: CharLike,
        {
            type Item = <I as Iterator>::Item;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(CharLike::to_lowercase)
            }
        }
        /// Trait defining an iterator to uppercase.
        pub struct Uppercase<I>(I);
        impl<I> From<I> for Uppercase<I> {
            fn from(iter: I) -> Self {
                Uppercase(iter)
            }
        }
        impl<I> Iterator for Uppercase<I>
        where
            I: Iterator,
            <I as Iterator>::Item: CharLike,
        {
            type Item = <I as Iterator>::Item;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(CharLike::to_uppercase)
            }
        }
        /// Trait defining a char normalizer.
        pub trait CharNormalizer: DoubleEndedIterator + Sized
        where
            <Self as Iterator>::Item: CharLike,
        {
            #[inline(always)]
            /// Trims spaces from the left of the iterator.
            fn trim_left(mut self) -> Self {
                let mut peekable = self.by_ref().peekable();
                while let Some(c) = peekable.peek() {
                    if c.is_space_like() {
                        peekable.next();
                    } else {
                        break;
                    }
                }
                self
            }
            #[inline(always)]
            /// Trims spaces from the right of the iterator.
            fn trim_right(mut self) -> Self {
                let mut peekable = self.by_ref().rev().peekable();
                while let Some(c) = peekable.peek() {
                    if c.is_space_like() {
                        peekable.next();
                    } else {
                        break;
                    }
                }
                self
            }
            #[inline(always)]
            /// Trims spaces from both sides of the iterator.
            fn trim(self) -> Self {
                self.trim_left().trim_right()
            }
            #[inline(always)]
            /// Trims null characters from the left of the iterator.
            fn trim_null_left(mut self) -> Self {
                let mut peekable = self.by_ref().peekable();
                while let Some(c) = peekable.peek() {
                    if c.is_nul() {
                        peekable.next();
                    } else {
                        break;
                    }
                }
                self
            }
            #[inline(always)]
            /// Trims null characters from the right of the iterator.
            fn trim_null_right(mut self) -> Self {
                let mut peekable = self.by_ref().rev().peekable();
                while let Some(c) = peekable.peek() {
                    if c.is_nul() {
                        peekable.next();
                    } else {
                        break;
                    }
                }
                self
            }
            #[inline(always)]
            /// Trims null characters from both sides of the iterator.
            fn trim_null(self) -> Self {
                self.trim_null_left().trim_null_right()
            }
            #[inline(always)]
            /// Converts all characters to lowercase.
            fn lower(self) -> Lowercase<Self> {
                Lowercase::from(self)
            }
            #[inline(always)]
            /// Converts all characters to uppercase.
            fn upper(self) -> Uppercase<Self> {
                Uppercase::from(self)
            }
        }
        /// Blanket implementation of `CharNormalizer` for all iterators yielding `CharLike` items.
        impl<I> CharNormalizer for I
        where
            I: DoubleEndedIterator,
            <Self as Iterator>::Item: CharLike,
        {}
    }
    pub use char_normalizer::*;
    pub mod ascii_char {
        //! Submodule providing a small implementation of an ASCII character.
        //!
        //! # Implementative details
        //! While we are aware that there is an unstable features [`ascii_char`](https://doc.rust-lang.org/std/ascii/enum.Char.html), that
        //! will, when it stabilizes, provide a more complete implementation of ASCII characters, we provide a small implementation
        //! that provide all that is needed for the library.
        //!
        use std::fmt::Display;
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};
        #[cfg(feature = "mem_dbg")]
        use mem_dbg::{MemDbg, MemSize};
        #[repr(transparent)]
        /// Represents an ASCII character.
        pub struct ASCIIChar {
            /// The character.
            character: u8,
        }
        #[automatically_derived]
        impl mem_dbg::CopyType for ASCIIChar
        where
            u8: mem_dbg::MemSize,
        {
            type Copy = mem_dbg::False;
        }
        #[automatically_derived]
        impl mem_dbg::MemSize for ASCIIChar
        where
            u8: mem_dbg::MemSize,
        {
            fn mem_size(&self, _memsize_flags: mem_dbg::SizeFlags) -> usize {
                let mut bytes = core::mem::size_of::<Self>();
                bytes
                    += self.character.mem_size(_memsize_flags)
                        - core::mem::size_of::<u8>();
                bytes
            }
        }
        #[automatically_derived]
        impl mem_dbg::MemDbgImpl for ASCIIChar
        where
            u8: mem_dbg::MemDbgImpl,
        {
            #[inline(always)]
            fn _mem_dbg_rec_on(
                &self,
                _memdbg_writer: &mut impl core::fmt::Write,
                _memdbg_total_size: usize,
                _memdbg_max_depth: usize,
                _memdbg_prefix: &mut String,
                _memdbg_is_last: bool,
                _memdbg_flags: mem_dbg::DbgFlags,
            ) -> core::fmt::Result {
                self.character
                    .mem_dbg_depth_on(
                        _memdbg_writer,
                        _memdbg_total_size,
                        _memdbg_max_depth,
                        _memdbg_prefix,
                        Some("character"),
                        true,
                        _memdbg_flags,
                    )?;
                Ok(())
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ASCIIChar {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ASCIIChar",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "character",
                        &self.character,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ASCIIChar {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "character" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"character" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ASCIIChar>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ASCIIChar;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ASCIIChar",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u8,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ASCIIChar with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ASCIIChar { character: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "character",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("character")?
                                }
                            };
                            _serde::__private::Ok(ASCIIChar { character: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["character"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ASCIIChar",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ASCIIChar>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ASCIIChar {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ASCIIChar",
                    "character",
                    &&self.character,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ASCIIChar {
            #[inline]
            fn default() -> ASCIIChar {
                ASCIIChar {
                    character: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ASCIIChar {
            #[inline]
            fn clone(&self) -> ASCIIChar {
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ASCIIChar {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ASCIIChar {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ASCIIChar {
            #[inline]
            fn eq(&self, other: &ASCIIChar) -> bool {
                self.character == other.character
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ASCIIChar {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u8>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ASCIIChar {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ASCIIChar,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.character, &other.character)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ASCIIChar {
            #[inline]
            fn cmp(&self, other: &ASCIIChar) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.character, &other.character)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for ASCIIChar {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.character, state)
            }
        }
        impl From<u8> for ASCIIChar {
            fn from(character: u8) -> Self {
                ASCIIChar { character }
            }
        }
        impl From<ASCIIChar> for u8 {
            fn from(ascii_char: ASCIIChar) -> u8 {
                ascii_char.character
            }
        }
        impl TryFrom<char> for ASCIIChar {
            type Error = &'static str;
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
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0}", self.character as char))
            }
        }
        /// Provides character operations by manipulation of the underlying `u8` value.
        impl ASCIIChar {
            /// The NUL character.
            pub const NUL: Self = ASCIIChar { character: 0 };
            /// The space character.
            pub const SPACE: Self = ASCIIChar { character: b' ' };
            /// Returns the lowercase version of the character.
            pub fn to_lowercase(self) -> Self {
                ASCIIChar {
                    character: self.character.to_ascii_lowercase(),
                }
            }
            /// Returns the uppercase version of the character.
            pub fn to_uppercase(self) -> Self {
                ASCIIChar {
                    character: self.character.to_ascii_uppercase(),
                }
            }
            /// Returns whether the current character is a space-like.
            pub fn is_space_like(self) -> bool {
                self.character.is_ascii_whitespace()
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
            fn from(iterator: I) -> Self {
                ASCIICharIterator { iterator }
            }
        }
        impl<I> Iterator for ASCIICharIterator<I>
        where
            I: Iterator<Item = char>,
        {
            type Item = ASCIIChar;
            fn next(&mut self) -> Option<Self::Item> {
                self.iterator
                    .next()
                    .and_then(|character| match ASCIIChar::try_from(character) {
                        Ok(ascii_char) => Some(ascii_char),
                        Err(_) => self.next(),
                    })
            }
        }
        /// Trait to be implemented for all iterators that yield `char`
        /// so that they can be converted to `ASCIICharIterator`.
        pub trait ToASCIICharIterator: IntoIterator<Item = char> {
            /// Converts the iterator to an `ASCIICharIterator`.
            fn ascii(self) -> ASCIICharIterator<Self>
            where
                Self: Sized;
        }
        impl<I> ToASCIICharIterator for I
        where
            I: IntoIterator<Item = char>,
        {
            fn ascii(self) -> ASCIICharIterator<Self>
            where
                Self: Sized,
            {
                ASCIICharIterator::from(self)
            }
        }
    }
    pub use ascii_char::*;
    pub mod padder {
        //! Submodule providing the Padder trait and relative structs.
        //!
        //! # Implementative details
        //! The goal of the Padder trait and structs is to provide a way to pad iterators
        //! of paddable grams, i.e. the types that implement the trait Paddable.
        use crate::{Gram, Paddable, PaddableNgram};
        use std::iter::Chain;
        /// Type alias for the padding both iterator.
        pub type BothPadding<NG, S> = Chain<
            Chain<<<NG as PaddableNgram>::Pad as IntoIterator>::IntoIter, S>,
            <<NG as PaddableNgram>::Pad as IntoIterator>::IntoIter,
        >;
        /// Trait defining a padder.
        pub trait IntoPadder: Iterator + Sized
        where
            <Self as Iterator>::Item: Paddable + Gram,
        {
            /// Adds padding to the left (beginning) of the iterator.
            ///
            /// # Example
            ///
            /// ```rust
            /// use ngrammatic::prelude::*;
            ///
            /// let iter = vec![b'a', b'b', b'c'].into_iter();
            /// let padded_left = iter.left_padding::<BiGram<u8>>();
            /// let padded: Vec<_> = padded_left.collect();
            /// assert_eq!(padded, vec![b' ', b'a', b'b', b'c']);
            /// ```
            fn left_padding<NG>(
                self,
            ) -> Chain<<<NG as PaddableNgram>::Pad as IntoIterator>::IntoIter, Self>
            where
                NG: PaddableNgram<G = Self::Item>,
            {
                NG::PADDING.into_iter().chain(self)
            }
            /// Adds padding to the right (end) of the iterator.
            ///
            /// # Example
            ///
            /// ```rust
            /// use ngrammatic::prelude::*;
            ///
            /// let iter = vec![b'a', b'b', b'c'].into_iter();
            /// let padded_right = iter.right_padding::<BiGram<u8>>();
            /// let padded: Vec<_> = padded_right.collect();
            /// assert_eq!(padded, vec![b'a', b'b', b'c', b' ']);
            /// ```
            ///
            fn right_padding<NG>(
                self,
            ) -> Chain<Self, <<NG as PaddableNgram>::Pad as IntoIterator>::IntoIter>
            where
                NG: PaddableNgram<G = Self::Item>,
            {
                self.chain(NG::PADDING)
            }
            /// Adds padding to both sides of the iterator.
            ///
            /// # Example
            ///
            /// ```rust
            /// use ngrammatic::prelude::*;
            ///
            /// let iter = vec![b'a', b'b', b'c'].into_iter();
            /// let padded_both = iter.both_padding::<BiGram<u8>>();
            /// let padded: Vec<_> = padded_both.collect();
            /// assert_eq!(padded, vec![b' ', b'a', b'b', b'c', b' ']);
            /// ```
            fn both_padding<NG>(self) -> BothPadding<NG, Self>
            where
                NG: PaddableNgram<G = Self::Item>,
            {
                NG::PADDING.into_iter().chain(self).chain(NG::PADDING)
            }
        }
        impl<I> IntoPadder for I
        where
            I: Iterator,
            <I as Iterator>::Item: Paddable + Gram,
        {}
    }
    pub use padder::*;
    pub mod paddable {
        //! Submodule providing the paddable trait.
        /// Trait defining a paddable item.
        pub trait Paddable {
            /// The padding value for the type.
            const PADDING: Self;
        }
        impl Paddable for u8 {
            const PADDING: Self = 0;
        }
        impl Paddable for u16 {
            const PADDING: Self = 0;
        }
        impl Paddable for u32 {
            const PADDING: Self = 0;
        }
        impl Paddable for u64 {
            const PADDING: Self = 0;
        }
        impl Paddable for u128 {
            const PADDING: Self = 0;
        }
        impl Paddable for usize {
            const PADDING: Self = 0;
        }
        impl Paddable for i8 {
            const PADDING: Self = 0;
        }
        impl Paddable for i16 {
            const PADDING: Self = 0;
        }
        impl Paddable for i32 {
            const PADDING: Self = 0;
        }
        impl Paddable for i64 {
            const PADDING: Self = 0;
        }
        impl Paddable for i128 {
            const PADDING: Self = 0;
        }
        impl Paddable for isize {
            const PADDING: Self = 0;
        }
        impl Paddable for char {
            const PADDING: Self = '\0';
        }
        impl Paddable for crate::ASCIIChar {
            const PADDING: Self = crate::ASCIIChar::NUL;
        }
    }
    pub use paddable::*;
    pub mod char_like {
        //! Submodule defining char-like types.
        use crate::ASCIIChar;
        /// Trait defining a char-like type.
        pub trait CharLike: Copy + Clone + Default + PartialEq + Eq + Ord + PartialOrd + std::fmt::Debug + std::fmt::Display + std::hash::Hash {
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
        }
    }
    pub use char_like::*;
}
pub use traits::*;
pub mod search_result {
    //! Contains the `SearchResult` struct, which holds a fuzzy match search result string, and its associated similarity to the query text.
    use std::cmp::Ordering;
    use crate::traits::Float;
    #[cfg(feature = "mem_dbg")]
    use mem_dbg::{MemDbg, MemSize};
    /// Holds a fuzzy match search result string, and its associated similarity
    /// to the query text.
    pub struct SearchResult<'a, K, F: Float> {
        /// The key of a fuzzy match
        key: &'a K,
        /// A similarity value indicating how closely the other term matched
        similarity: F,
    }
    #[automatically_derived]
    impl<'a, K, F: Float> mem_dbg::CopyType for SearchResult<'a, K, F>
    where
        &'a K: mem_dbg::MemSize,
        F: mem_dbg::MemSize,
    {
        type Copy = mem_dbg::False;
    }
    #[automatically_derived]
    impl<'a, K, F: Float> mem_dbg::MemSize for SearchResult<'a, K, F>
    where
        &'a K: mem_dbg::MemSize,
        F: mem_dbg::MemSize,
    {
        fn mem_size(&self, _memsize_flags: mem_dbg::SizeFlags) -> usize {
            let mut bytes = core::mem::size_of::<Self>();
            bytes += self.key.mem_size(_memsize_flags) - core::mem::size_of::<&'a K>();
            bytes
                += self.similarity.mem_size(_memsize_flags) - core::mem::size_of::<F>();
            bytes
        }
    }
    #[automatically_derived]
    impl<'a, K, F: Float> mem_dbg::MemDbgImpl for SearchResult<'a, K, F>
    where
        &'a K: mem_dbg::MemDbgImpl,
        F: mem_dbg::MemDbgImpl,
    {
        #[inline(always)]
        fn _mem_dbg_rec_on(
            &self,
            _memdbg_writer: &mut impl core::fmt::Write,
            _memdbg_total_size: usize,
            _memdbg_max_depth: usize,
            _memdbg_prefix: &mut String,
            _memdbg_is_last: bool,
            _memdbg_flags: mem_dbg::DbgFlags,
        ) -> core::fmt::Result {
            self.key
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("key"),
                    false,
                    _memdbg_flags,
                )?;
            self.similarity
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("similarity"),
                    true,
                    _memdbg_flags,
                )?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl<'a, K: ::core::fmt::Debug, F: ::core::fmt::Debug + Float> ::core::fmt::Debug
    for SearchResult<'a, K, F> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "SearchResult",
                "key",
                &self.key,
                "similarity",
                &&self.similarity,
            )
        }
    }
    #[automatically_derived]
    impl<
        'a,
        K: ::core::clone::Clone,
        F: ::core::clone::Clone + Float,
    > ::core::clone::Clone for SearchResult<'a, K, F> {
        #[inline]
        fn clone(&self) -> SearchResult<'a, K, F> {
            SearchResult {
                key: ::core::clone::Clone::clone(&self.key),
                similarity: ::core::clone::Clone::clone(&self.similarity),
            }
        }
    }
    impl<'a, K, F: Float> PartialOrd for SearchResult<'a, K, F> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.similarity.partial_cmp(&other.similarity)
        }
    }
    impl<'a, K, F: Float> PartialEq for SearchResult<'a, K, F> {
        fn eq(&self, other: &Self) -> bool {
            self.similarity == other.similarity
        }
    }
    impl<'a, K, F: Float> SearchResult<'a, K, F> {
        /// Trivial constructor used internally to build search results
        ///
        /// # Arguments
        /// * `key` - The key of a fuzzy match
        /// * `similarity` - A similarity value indicating how closely the other term matched
        pub(crate) fn new(key: &'a K, similarity: F) -> Self {
            Self { key, similarity }
        }
        /// Returns the key of a fuzzy match
        pub fn key(&self) -> &'a K {
            self.key
        }
        /// Returns a similarity value indicating how closely the other term matched
        pub fn similarity(&self) -> F {
            self.similarity
        }
    }
}
pub use search_result::*;
pub mod corpus {
    //! Submodule providing the Corpus data structure.
    use std::{
        collections::{BTreeSet, HashMap},
        iter::Take,
    };
    use sux::prelude::*;
    #[cfg(feature = "mem_dbg")]
    use mem_dbg::{MemDbg, MemSize};
    use crate::{traits::*, AdaptativeVector};
    /// Rasterized corpus.
    ///
    /// # Implementation details
    /// This corpus is represented as a sparse graph, using a CSR format. The
    /// links between keys and grams are weighted by the number of times a given
    /// gram appears in a given key: we call this vector the `cooccurrences`.
    ///
    pub struct Corpus<K: Keys<NG::G>, NG: Ngram> {
        /// Vector of unique keys in the corpus.
        keys: K,
        /// Vector of unique ngrams in the corpus.
        ngrams: Vec<NG>,
        /// Vector containing the number of times a given gram appears in a given key.
        /// This is a descriptor of an edge from a Key to a Gram.
        cooccurrences: BitFieldVec,
        /// Vector containing the comulative outbound degree from a given key to grams.
        /// This is a vector with the same length as the keys vector PLUS ONE, and the value at
        /// index `i` is the sum of the oubound degrees before index `i`. The last element of this
        /// vector is the total number of edges in the bipartite graph from keys to grams.
        /// We use this vector alongside the `cooccurrences` vector to find the weighted edges
        /// of a given key. The destinations, i.e. the grams, are found in the `grams` vector.
        key_offsets: BitFieldVec,
        /// Vector contain the comulative inbound degree from a given gram to keys.
        /// This is a vector with the same length as the grams vector PLUS ONE, and the value at
        /// index `i` is the sum of the inbound degrees before index `i`. The last element of this
        /// vector is the total number of edges in the bipartite graph from grams to keys.
        /// These edges are NOT weighted, as the weights are stored in the `cooccurrences` vector and
        /// solely refer to the edges from keys to grams.
        ngram_offsets: BitFieldVec,
        /// Vector containing the destinations of the edges from keys to grams.
        key_to_ngram_edges: BitFieldVec,
        /// Vector containing the sources of the edges from grams to keys.
        gram_to_key_edges: BitFieldVec,
    }
    #[automatically_derived]
    impl<K: Keys<NG::G>, NG: Ngram> mem_dbg::CopyType for Corpus<K, NG>
    where
        K: mem_dbg::MemSize,
        Vec<NG>: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
    {
        type Copy = mem_dbg::False;
    }
    #[automatically_derived]
    impl<K: Keys<NG::G>, NG: Ngram> mem_dbg::MemSize for Corpus<K, NG>
    where
        K: mem_dbg::MemSize,
        Vec<NG>: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
        BitFieldVec: mem_dbg::MemSize,
    {
        fn mem_size(&self, _memsize_flags: mem_dbg::SizeFlags) -> usize {
            let mut bytes = core::mem::size_of::<Self>();
            bytes += self.keys.mem_size(_memsize_flags) - core::mem::size_of::<K>();
            bytes
                += self.ngrams.mem_size(_memsize_flags)
                    - core::mem::size_of::<Vec<NG>>();
            bytes
                += self.cooccurrences.mem_size(_memsize_flags)
                    - core::mem::size_of::<BitFieldVec>();
            bytes
                += self.key_offsets.mem_size(_memsize_flags)
                    - core::mem::size_of::<BitFieldVec>();
            bytes
                += self.ngram_offsets.mem_size(_memsize_flags)
                    - core::mem::size_of::<BitFieldVec>();
            bytes
                += self.key_to_ngram_edges.mem_size(_memsize_flags)
                    - core::mem::size_of::<BitFieldVec>();
            bytes
                += self.gram_to_key_edges.mem_size(_memsize_flags)
                    - core::mem::size_of::<BitFieldVec>();
            bytes
        }
    }
    #[automatically_derived]
    impl<K: Keys<NG::G>, NG: Ngram> mem_dbg::MemDbgImpl for Corpus<K, NG>
    where
        K: mem_dbg::MemDbgImpl,
        Vec<NG>: mem_dbg::MemDbgImpl,
        BitFieldVec: mem_dbg::MemDbgImpl,
        BitFieldVec: mem_dbg::MemDbgImpl,
        BitFieldVec: mem_dbg::MemDbgImpl,
        BitFieldVec: mem_dbg::MemDbgImpl,
        BitFieldVec: mem_dbg::MemDbgImpl,
    {
        #[inline(always)]
        fn _mem_dbg_rec_on(
            &self,
            _memdbg_writer: &mut impl core::fmt::Write,
            _memdbg_total_size: usize,
            _memdbg_max_depth: usize,
            _memdbg_prefix: &mut String,
            _memdbg_is_last: bool,
            _memdbg_flags: mem_dbg::DbgFlags,
        ) -> core::fmt::Result {
            self.keys
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("keys"),
                    false,
                    _memdbg_flags,
                )?;
            self.ngrams
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("ngrams"),
                    false,
                    _memdbg_flags,
                )?;
            self.cooccurrences
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("cooccurrences"),
                    false,
                    _memdbg_flags,
                )?;
            self.key_offsets
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("key_offsets"),
                    false,
                    _memdbg_flags,
                )?;
            self.ngram_offsets
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("ngram_offsets"),
                    false,
                    _memdbg_flags,
                )?;
            self.key_to_ngram_edges
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("key_to_ngram_edges"),
                    false,
                    _memdbg_flags,
                )?;
            self.gram_to_key_edges
                .mem_dbg_depth_on(
                    _memdbg_writer,
                    _memdbg_total_size,
                    _memdbg_max_depth,
                    _memdbg_prefix,
                    Some("gram_to_key_edges"),
                    true,
                    _memdbg_flags,
                )?;
            Ok(())
        }
    }
    impl<K, NG> From<K> for Corpus<K, NG>
    where
        NG: Ngram,
        K: Keys<NG::G>,
    {
        fn from(keys: K) -> Self {
            let mut ngrams = BTreeSet::new();
            let mut cooccurrences = AdaptativeVector::with_capacity(keys.len());
            let mut maximal_cooccurrence = 0;
            let mut key_offsets = AdaptativeVector::with_capacity(keys.len());
            key_offsets.push(0_u8);
            let mut key_to_ngrams: Vec<NG> = Vec::with_capacity(keys.len());
            for key in keys.iter() {
                let ngram_counts: HashMap<NG, usize> = key.counts();
                let mut ngram_counts: Vec<(NG, usize)> = ngram_counts
                    .into_iter()
                    .collect();
                ngram_counts
                    .sort_unstable_by(|(ngram_a, _), (ngram_b, _)| ngram_a.cmp(ngram_b));
                for (ngram, count) in ngram_counts {
                    ngrams.insert(ngram);
                    cooccurrences.push(count);
                    key_to_ngrams.push(ngram);
                }
                key_offsets.push(cooccurrences.len());
                maximal_cooccurrence = maximal_cooccurrence.max(cooccurrences.len());
            }
            let cooccurrences = BitFieldVec::from(cooccurrences);
            let key_offsets = BitFieldVec::from(key_offsets);
            let ngrams: Vec<NG> = ngrams.into_iter().collect();
            let mut ngram_offsets = BitFieldVec::new(
                cooccurrences.len().next_power_of_two().trailing_zeros() as usize,
                ngrams.len() + 1,
            );
            let mut key_to_ngram_edges = BitFieldVec::new(
                ngrams.len().next_power_of_two().trailing_zeros() as usize,
                key_to_ngrams.len(),
            );
            for (edge_id, ngram) in key_to_ngrams.into_iter().enumerate() {
                let ngram_index = ngrams.binary_search(&ngram).unwrap();
                unsafe { key_to_ngram_edges.set_unchecked(edge_id, ngram_index) };
                unsafe {
                    ngram_offsets
                        .set_unchecked(
                            ngram_index + 1,
                            ngram_offsets.get_unchecked(ngram_index + 1) + 1,
                        )
                }
            }
            let mut comulative_sum = 0;
            for i in 0..ngram_offsets.len() {
                unsafe {
                    comulative_sum += ngram_offsets.get_unchecked(i);
                }
                unsafe {
                    ngram_offsets.set_unchecked(i, comulative_sum);
                }
            }
            let mut gram_to_key_edges = BitFieldVec::new(
                keys.len().next_power_of_two().trailing_zeros() as usize,
                cooccurrences.len(),
            );
            let mut ngram_degrees = BitFieldVec::new(
                cooccurrences.len().next_power_of_two().trailing_zeros() as usize,
                ngrams.len() + 1,
            );
            let mut ngram_iterator = key_to_ngram_edges.into_iter_from(0);
            for (key_id, (key_offset_start, key_offset_end)) in key_offsets
                .into_iter_from(0)
                .zip(key_offsets.into_iter_from(1))
                .enumerate()
            {
                for _ in key_offset_start..key_offset_end {
                    let ngram_id = ngram_iterator.next().unwrap();
                    let ngram_degree: usize = unsafe {
                        ngram_degrees.get_unchecked(ngram_id)
                    };
                    let inbound_edge_id = unsafe {
                        ngram_offsets.get_unchecked(ngram_id)
                    } + ngram_degree;
                    unsafe { gram_to_key_edges.set_unchecked(inbound_edge_id, key_id) };
                    unsafe { ngram_degrees.set_unchecked(ngram_id, ngram_degree + 1) };
                }
            }
            Corpus {
                keys,
                ngrams,
                cooccurrences,
                key_offsets,
                ngram_offsets,
                key_to_ngram_edges,
                gram_to_key_edges,
            }
        }
    }
    impl<K, NG> Corpus<K, NG>
    where
        NG: Ngram,
        K: Keys<NG::G>,
    {
        #[inline(always)]
        /// Returns the number of keys in the corpus.
        pub fn number_of_keys(&self) -> usize {
            self.keys.len()
        }
        #[inline(always)]
        /// Returns the number of ngrams in the corpus.
        pub fn number_of_ngrams(&self) -> usize {
            self.ngrams.len()
        }
        #[inline(always)]
        /// Returns the number of edges in the corpus.
        pub fn number_of_edges(&self) -> usize {
            self.cooccurrences.len()
        }
        #[inline(always)]
        /// Returns a reference to the key at a given key id.
        ///
        /// # Arguments
        /// * `key_id` - The id of the key to get.
        ///
        pub fn key_from_id(&self, key_id: usize) -> &K::K {
            &self.keys[key_id]
        }
        #[inline(always)]
        /// Returns the ngram curresponding to a given ngram id.
        ///
        /// # Arguments
        /// * `ngram_id` - The id of the ngram to get.
        ///
        pub fn ngram_from_id(&self, ngram_id: usize) -> NG {
            self.ngrams[ngram_id]
        }
        #[inline(always)]
        /// Returns the ngram id curresponding to a given ngram,
        /// if it exists in the corpus. If it does not exist, the
        /// function returns the index where the ngram should be
        /// inserted to keep the ngrams sorted.
        ///
        /// # Arguments
        /// * `ngram` - The ngram to get the id from.
        pub fn ngram_id_from_ngram(&self, ngram: &NG) -> Result<usize, usize> {
            self.ngrams.binary_search(ngram)
        }
        #[inline(always)]
        /// Returns the number of ngrams from a given key.
        ///
        /// # Arguments
        /// * `key_id` - The id of the key to get the number of ngrams from.
        pub fn number_of_ngrams_from_key_id(&self, key_id: usize) -> usize {
            let key_offset_start = self.key_offsets.get(key_id);
            let key_offset_end = self.key_offsets.get(key_id + 1);
            key_offset_end - key_offset_start
        }
        #[inline(always)]
        /// Returns the number of keys from a given ngram.
        ///
        /// # Arguments
        /// * `ngram_id` - The id of the ngram to get the number of keys from.
        pub fn number_of_keys_from_ngram_id(&self, ngram_id: usize) -> usize {
            let ngram_offset_start = self.ngram_offsets.get(ngram_id);
            let ngram_offset_end = self.ngram_offsets.get(ngram_id + 1);
            ngram_offset_end - ngram_offset_start
        }
        #[inline(always)]
        /// Returns the key ids associated to a given ngram.
        ///
        /// # Arguments
        /// * `ngram_id` - The id of the ngram to get the key ids from.
        ///
        pub fn key_ids_from_ngram_id(
            &self,
            ngram_id: usize,
        ) -> Take<BitFieldVecIterator<'_, usize, Vec<usize>>> {
            let ngram_offset_start = self.ngram_offsets.get(ngram_id);
            let ngram_offset_end = self.ngram_offsets.get(ngram_id + 1);
            self.gram_to_key_edges
                .into_iter_from(ngram_offset_start)
                .take(ngram_offset_end - ngram_offset_start)
        }
        #[inline(always)]
        /// Returns the ngram ids associated to a given key.
        ///
        /// # Arguments
        /// * `key_id` - The id of the key to get the ngram ids from.
        pub fn ngram_ids_from_key(
            &self,
            key_id: usize,
        ) -> Take<BitFieldVecIterator<'_, usize, Vec<usize>>> {
            let key_offset_start = self.key_offsets.get(key_id);
            let key_offset_end = self.key_offsets.get(key_id + 1);
            self.key_to_ngram_edges
                .into_iter_from(key_offset_start)
                .take(key_offset_end - key_offset_start)
        }
        #[inline(always)]
        /// Returns the ngram co-oocurrences of a given key.
        ///
        /// # Arguments
        /// * `key_id` - The id of the key to get the ngram co-occurrences from.
        pub fn ngram_cooccurrences_from_key(
            &self,
            key_id: usize,
        ) -> Take<BitFieldVecIterator<'_, usize, Vec<usize>>> {
            let key_offset_start = self.key_offsets.get(key_id);
            let key_offset_end = self.key_offsets.get(key_id + 1);
            self.cooccurrences
                .into_iter_from(key_offset_start)
                .take(key_offset_end - key_offset_start)
        }
        #[inline(always)]
        /// Returns the ngrams ids and their co-occurrences in a given key.
        ///
        /// # Arguments
        /// * `key_id` - The id of the key to get the ngrams and their co-occurrences from.
        ///
        pub fn ngram_ids_and_cooccurrences_from_key(
            &self,
            key_id: usize,
        ) -> impl ExactSizeIterator<Item = (usize, usize)> + '_ {
            let key_offset_start = self.key_offsets.get(key_id);
            let key_offset_end = self.key_offsets.get(key_id + 1);
            self.cooccurrences
                .into_iter_from(key_offset_start)
                .zip(self.key_to_ngram_edges.into_iter_from(key_offset_start))
                .take(key_offset_end - key_offset_start)
        }
        #[inline(always)]
        /// Returns the ngrams and their co-occurrences in a given key.
        ///
        /// # Arguments
        /// * `key_id` - The id of the key to get the ngrams and their co-occurrences from.
        pub fn ngrams_and_cooccurrences_from_key(
            &self,
            key_id: usize,
        ) -> impl ExactSizeIterator<Item = (NG, usize)> + '_ {
            self.ngram_ids_and_cooccurrences_from_key(key_id)
                .map(move |(ngram_id, cooccurrence)| (
                    self.ngrams[ngram_id],
                    cooccurrence,
                ))
        }
        #[inline(always)]
        /// Returns the ngrams associated to a given key.
        ///
        /// # Arguments
        /// * `key_id` - The id of the key to get the ngrams from.
        pub fn ngrams_from_key(
            &self,
            key_id: usize,
        ) -> impl ExactSizeIterator<Item = NG> + '_ {
            self.ngram_ids_from_key(key_id).map(move |ngram_id| self.ngrams[ngram_id])
        }
    }
}
pub use corpus::*;
mod similarity {
    use std::{cmp::Ordering, fmt::{Display, Formatter}};
    use crate::{corpus::Corpus, traits::*};
    impl<K, NG> Corpus<K, NG>
    where
        NG: Ngram,
        K: Keys<NG::G>,
    {
        #[inline(always)]
        /// Returns whether any of the ngrams provided appear in the provided key.
        ///
        /// # Arguments
        /// * `sorted_grams` - An iterator over the sorted ngrams to check for.
        /// * `key_id` - The id of the key to check for ngrams in.
        ///
        /// # Implementative details
        /// Since both iterators of ngrams are sorted, we can iterate over both
        /// iterators at once, comparing the current ngram in each iterator. If
        /// the ngrams are equal, we return `true`. If the ngrams are not equal,
        /// we advance the iterator that has the smaller ngram.
        pub fn contains_any_ngrams<'a, I>(&self, sorted_ngrams: I, key_id: usize) -> bool
        where
            I: IntoIterator<Item = &'a NG>,
            NG: 'a,
        {
            let mut ngrams_iterator = self.ngrams_from_key(key_id);
            let mut left_next = ngrams_iterator.next();
            let mut sorted_ngrams = sorted_ngrams.into_iter();
            let mut next_ngram = sorted_ngrams.next();
            while let (Some(left_gram), Some(ngram)) = (left_next, next_ngram) {
                match left_gram.cmp(ngram) {
                    Ordering::Less => {
                        left_next = ngrams_iterator.next();
                    }
                    Ordering::Greater => {
                        next_ngram = sorted_ngrams.next();
                    }
                    Ordering::Equal => {
                        return true;
                    }
                }
            }
            false
        }
    }
    /// Returns the number of shared ngrams between two iterators.
    fn number_of_shared_items<I, J, Item>(mut left: I, mut right: J) -> usize
    where
        I: Iterator<Item = (Item, usize)>,
        J: Iterator<Item = (Item, usize)>,
        (Item, usize): Eq,
        Item: Ord,
    {
        let mut count = 0;
        let mut left_next = left.next();
        let mut right_next = right.next();
        while let (Some((left_gram, left_count)), Some((right_gram, right_count))) = (
            &left_next,
            &right_next,
        ) {
            match left_gram.cmp(right_gram) {
                Ordering::Less => {
                    left_next = left.next();
                }
                Ordering::Greater => {
                    right_next = right.next();
                }
                Ordering::Equal => {
                    count += left_count.min(right_count).as_usize();
                    left_next = left.next();
                    right_next = right.next();
                }
            }
        }
        count
    }
    #[inline(always)]
    /// Calculate the similarity between two iterators of ngrams.
    ///
    /// # Arguments
    /// * `warp` - The warp factor to use in the similarity calculation.
    /// * `left` - The first iterator of ngrams.
    /// * `right` - The second iterator of ngrams.
    ///
    pub(crate) fn similarity<I, J, W, F, NG>(warp: Warp<W>, left: I, right: J) -> F
    where
        I: ExactSizeIterator<Item = (NG, usize)>,
        J: ExactSizeIterator<Item = (NG, usize)>,
        NG: Ngram,
        F: Float,
        Warp<W>: Similarity + One + Zero + Three + PartialOrd,
    {
        if true {
            if !warp.is_between_one_and_three() {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Warp factor must be in the range 1 to 3"),
                    );
                }
            }
        }
        let left_number_of_ngrams = left.len();
        let right_number_of_ngrams = right.len();
        let number_of_shared_ngrams = number_of_shared_items(left, right) as f64;
        let number_of_unique_shared_grams = (left_number_of_ngrams
            + right_number_of_ngrams + 2 - 2 * NG::ARITY) as f64
            - number_of_shared_ngrams;
        if true {
            if !(number_of_unique_shared_grams >= 1.0) {
                ::core::panicking::panic(
                    "assertion failed: number_of_unique_shared_grams >= 1.0",
                )
            }
        }
        F::from_f64(
            if warp.is_one() {
                number_of_shared_ngrams / number_of_unique_shared_grams
            } else {
                let diffgrams = number_of_unique_shared_grams - number_of_shared_ngrams;
                (warp.pow(number_of_unique_shared_grams) - warp.pow(diffgrams))
                    / warp.pow(number_of_unique_shared_grams)
            },
        )
    }
    /// Struct representing a warp factor.
    pub struct Warp<W> {
        value: W,
    }
    #[automatically_derived]
    impl<W: ::core::fmt::Debug> ::core::fmt::Debug for Warp<W> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Warp",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl<W: ::core::clone::Clone> ::core::clone::Clone for Warp<W> {
        #[inline]
        fn clone(&self) -> Warp<W> {
            Warp {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    #[automatically_derived]
    impl<W: ::core::marker::Copy> ::core::marker::Copy for Warp<W> {}
    #[automatically_derived]
    impl<W> ::core::marker::StructuralPartialEq for Warp<W> {}
    #[automatically_derived]
    impl<W: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Warp<W> {
        #[inline]
        fn eq(&self, other: &Warp<W>) -> bool {
            self.value == other.value
        }
    }
    #[automatically_derived]
    impl<W: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for Warp<W> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Warp<W>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.value, &other.value)
        }
    }
    impl<W: One> One for Warp<W> {
        const ONE: Self = Warp { value: W::ONE };
        fn is_one(&self) -> bool {
            self.value.is_one()
        }
    }
    impl<W: Zero> Zero for Warp<W> {
        const ZERO: Self = Warp { value: W::ZERO };
        fn is_zero(&self) -> bool {
            self.value.is_zero()
        }
    }
    impl<W: Three> Three for Warp<W> {
        const THREE: Self = Warp { value: W::THREE };
    }
    impl<W: Display> Display for Warp<W> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0}", self.value))
        }
    }
    /// Trait defining the similarity calculation.
    pub trait Similarity {
        /// Calculate the power of a value.
        fn pow(&self, value: f64) -> f64;
        /// Calculate the similarity between two iterators of ngrams.
        fn similarity<I, J, F, NG>(self, left: I, right: J) -> F
        where
            I: ExactSizeIterator<Item = (NG, usize)>,
            J: ExactSizeIterator<Item = (NG, usize)>,
            NG: Ngram,
            F: Float;
    }
    impl Similarity for Warp<i32> {
        #[inline(always)]
        fn pow(&self, value: f64) -> f64 {
            value.powi(self.value)
        }
        #[inline(always)]
        fn similarity<I, J, F, NG>(self, left: I, right: J) -> F
        where
            I: ExactSizeIterator<Item = (NG, usize)>,
            J: ExactSizeIterator<Item = (NG, usize)>,
            NG: Ngram,
            F: Float,
        {
            similarity(self, left, right)
        }
    }
    impl Similarity for Warp<f64> {
        #[inline(always)]
        fn pow(&self, value: f64) -> f64 {
            value.powf(self.value)
        }
        #[inline(always)]
        fn similarity<I, J, F, NG>(self, left: I, right: J) -> F
        where
            I: ExactSizeIterator<Item = (NG, usize)>,
            J: ExactSizeIterator<Item = (NG, usize)>,
            NG: Ngram,
            F: Float,
        {
            similarity(self, left, right)
        }
    }
    #[cfg(feature = "half")]
    impl TryFrom<half::f16> for Warp<f64> {
        type Error = &'static str;
        fn try_from(value: half::f16) -> Result<Self, Self::Error> {
            if value.is_nan() {
                return Err("NaN is not a valid warp factor");
            }
            if value.is_infinite() {
                return Err("Infinity is not a valid warp factor");
            }
            if value < half::f16::from_f64(1.0) || value > half::f16::from_f64(3.0) {
                return Err("Warp factor must be in the range 1 to 3");
            }
            Ok(Warp { value: f64::from(value) })
        }
    }
    #[cfg(feature = "half")]
    impl TryFrom<half::bf16> for Warp<f64> {
        type Error = &'static str;
        fn try_from(value: half::bf16) -> Result<Self, Self::Error> {
            if value.is_nan() {
                return Err("NaN is not a valid warp factor");
            }
            if value.is_infinite() {
                return Err("Infinity is not a valid warp factor");
            }
            if value < half::bf16::from_f64(1.0) || value > half::bf16::from_f64(3.0) {
                return Err("Warp factor must be in the range 1 to 3");
            }
            Ok(Warp { value: f64::from(value) })
        }
    }
    impl TryFrom<i32> for Warp<i32> {
        type Error = &'static str;
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if !(1..=3).contains(&value) {
                return Err("Warp factor must be in the range 1 to 3");
            }
            Ok(Warp { value })
        }
    }
    impl TryFrom<f32> for Warp<f64> {
        type Error = &'static str;
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            if value.is_nan() {
                return Err("NaN is not a valid warp factor");
            }
            if value.is_infinite() {
                return Err("Infinity is not a valid warp factor");
            }
            if !(1.0..=3.0).contains(&value) {
                return Err("Warp factor must be in the range 1 to 3");
            }
            Ok(Warp { value: value as f64 })
        }
    }
    impl TryFrom<f64> for Warp<f64> {
        type Error = &'static str;
        fn try_from(value: f64) -> Result<Self, Self::Error> {
            if value.is_nan() {
                return Err("NaN is not a valid warp factor");
            }
            if value.is_infinite() {
                return Err("Infinity is not a valid warp factor");
            }
            if !(1.0..=3.0).contains(&value) {
                return Err("Warp factor must be in the range 1 to 3");
            }
            Ok(Warp { value })
        }
    }
}
pub use similarity::*;
pub mod search {
    //! This module contains the search functionality for the `Corpus` struct.
    use crate::traits::key::Key;
    use crate::{Corpus, Float, Keys, Ngram, SearchResult, Similarity, Warp};
    impl<K, NG> Corpus<K, NG>
    where
        NG: Ngram,
        K: Keys<NG::G>,
    {
        #[inline(always)]
        /// Perform a fuzzy search of the `Corpus` for `Ngrams` above some
        /// `threshold` of similarity to the supplied `key`.  Returns up to `limit`
        /// results, sorted by highest similarity to lowest.
        ///
        /// # Arguments
        /// * `key` - The key to search for in the corpus
        /// * `threshold` - The minimum similarity value for a result to be included in the
        /// output. This value should be in the range 0.0 to 1.0.
        /// * `limit` - The maximum number of results to return.
        ///
        pub fn search<F: Float>(
            &self,
            key: &K::K,
            threshold: F,
            limit: usize,
        ) -> Vec<SearchResult<'_, K::K, F>> {
            self.search_with_warp(key, 2_i32, threshold, limit).unwrap()
        }
        #[inline(always)]
        /// Perform a fuzzy search of the `Corpus` for `Ngrams` with a custom `warp` for
        /// results above some `threshold` of similarity to the supplied `key`.  Returns
        /// up to `limit` results, sorted by highest similarity to lowest.
        ///
        /// # Arguments
        /// * `key` - The key to search for in the corpus
        /// * `warp` - The warp factor to use in the similarity calculation. This value
        ///  should be in the range 1.0 to 3.0, with 2.0 being the default.
        /// * `threshold` - The minimum similarity value for a result to be included in the
        /// output. This value should be in the range 0.0 to 1.0.
        /// * `limit` - The maximum number of results to return.
        ///
        pub fn search_with_warp<W, F: Float>(
            &self,
            key: &K::K,
            warp: W,
            threshold: F,
            limit: usize,
        ) -> Result<Vec<SearchResult<'_, K::K, F>>, &'static str>
        where
            W: TryInto<Warp<W>, Error = &'static str>,
            Warp<W>: Similarity + Copy,
        {
            let warp = warp.try_into()?;
            let ngram_counts = key.counts::<NG>();
            let ngram_counts_ref = &ngram_counts;
            let mut matches = ngram_counts_ref
                .keys()
                .enumerate()
                .filter_map(|(gram_number, gram)| {
                    self.ngram_id_from_ngram(gram)
                        .ok()
                        .map(|ngram_id| (gram_number, ngram_id))
                })
                .flat_map(|(gram_number, ngram_id)| {
                    self.key_ids_from_ngram_id(ngram_id)
                        .filter_map(move |key_id| {
                            if self
                                .contains_any_ngrams(
                                    ngram_counts_ref.keys().take(gram_number),
                                    key_id,
                                )
                            {
                                return None;
                            }
                            let similarity = warp
                                .similarity(
                                    ngram_counts_ref
                                        .iter()
                                        .map(|(ngram, count)| (*ngram, *count)),
                                    self.ngrams_and_cooccurrences_from_key(key_id),
                                );
                            if similarity >= threshold {
                                Some(
                                    SearchResult::new(self.key_from_id(key_id), similarity),
                                )
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<SearchResult<'_, K::K, F>>>();
            matches.sort_by(|a, b| b.partial_cmp(a).unwrap());
            matches.truncate(limit);
            Ok(matches)
        }
    }
}
pub mod adaptative_vector {
    //! Module providing a vector that adaptatively grows in data type.
    use sux::bits::BitFieldVec;
    use sux::traits::BitFieldSliceMut;
    use crate::UnsignedInteger;
    /// Trait defining a bounded type.
    pub trait Bounded {
        /// The maximum value of the type.
        const MAX: usize;
    }
    /// Trait defining a primitive conversion type.
    pub trait Convert<T> {
        /// Convert the type to the primitive type.
        fn convert(self) -> T;
    }
    impl Convert<u16> for u8 {
        fn convert(self) -> u16 {
            self as u16
        }
    }
    impl Convert<u32> for u8 {
        fn convert(self) -> u32 {
            self as u32
        }
    }
    impl Convert<u64> for u8 {
        fn convert(self) -> u64 {
            self as u64
        }
    }
    impl Convert<u8> for u16 {
        fn convert(self) -> u8 {
            self as u8
        }
    }
    impl Convert<u16> for u16 {
        fn convert(self) -> u16 {
            self as u16
        }
    }
    impl Convert<u32> for u16 {
        fn convert(self) -> u32 {
            self as u32
        }
    }
    impl Convert<u64> for u16 {
        fn convert(self) -> u64 {
            self as u64
        }
    }
    impl Convert<u16> for u32 {
        fn convert(self) -> u16 {
            self as u16
        }
    }
    impl Convert<u32> for u32 {
        fn convert(self) -> u32 {
            self as u32
        }
    }
    impl Convert<u64> for u32 {
        fn convert(self) -> u64 {
            self as u64
        }
    }
    impl Convert<u16> for u64 {
        fn convert(self) -> u16 {
            self as u16
        }
    }
    impl Convert<u32> for u64 {
        fn convert(self) -> u32 {
            self as u32
        }
    }
    impl Convert<u64> for u64 {
        fn convert(self) -> u64 {
            self as u64
        }
    }
    impl Bounded for u8 {
        const MAX: usize = <u8>::MAX as usize;
    }
    impl Bounded for u16 {
        const MAX: usize = <u16>::MAX as usize;
    }
    impl Bounded for u32 {
        const MAX: usize = <u32>::MAX as usize;
    }
    impl Bounded for u64 {
        const MAX: usize = <u64>::MAX as usize;
    }
    pub(crate) enum AdaptativeVector {
        U8(Vec<u8>),
        U16(Vec<u16>),
        U32(Vec<u32>),
        U64(Vec<u64>),
    }
    impl From<Vec<u8>> for AdaptativeVector {
        fn from(vector: Vec<u8>) -> Self {
            AdaptativeVector::U8(vector)
        }
    }
    impl From<Vec<u16>> for AdaptativeVector {
        fn from(vector: Vec<u16>) -> Self {
            AdaptativeVector::U16(vector)
        }
    }
    impl From<Vec<u32>> for AdaptativeVector {
        fn from(vector: Vec<u32>) -> Self {
            AdaptativeVector::U32(vector)
        }
    }
    impl From<Vec<u64>> for AdaptativeVector {
        fn from(vector: Vec<u64>) -> Self {
            AdaptativeVector::U64(vector)
        }
    }
    impl AdaptativeVector {
        fn type_max(&self) -> usize {
            match self {
                AdaptativeVector::U8(_) => u8::MAX as usize,
                AdaptativeVector::U16(_) => u16::MAX as usize,
                AdaptativeVector::U32(_) => u32::MAX as usize,
                AdaptativeVector::U64(_) => u64::MAX as usize,
            }
        }
        fn push_upgrade_vector<U>(&mut self, value: U)
        where
            u8: Convert<U>,
            u16: Convert<U>,
            u32: Convert<U>,
            u64: Convert<U>,
            U: Bounded,
            AdaptativeVector: From<Vec<U>>,
        {
            if !(U::MAX > self.type_max()) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("The new type must be bigger than the old one."),
                    );
                }
            }
            let mut new_vector = Vec::with_capacity(self.len() + 1);
            let old_vector = core::mem::replace(
                self,
                AdaptativeVector::from(Vec::new()),
            );
            match old_vector {
                AdaptativeVector::U8(vector) => {
                    new_vector.extend(vector.into_iter().map(u8::convert));
                }
                AdaptativeVector::U16(vector) => {
                    new_vector.extend(vector.into_iter().map(u16::convert));
                }
                AdaptativeVector::U32(vector) => {
                    new_vector.extend(vector.into_iter().map(u32::convert));
                }
                AdaptativeVector::U64(vector) => {
                    new_vector.extend(vector.into_iter().map(u64::convert));
                }
            }
            new_vector.push(value);
            *self = AdaptativeVector::from(new_vector);
        }
        /// Creates a new adaptative vector.
        ///
        /// # Implementation details
        /// By default, the adaptative vector starts with the
        /// smallest possible data type, i.e. `u8`. As soon as
        /// the data type does not fit any of the provided values,
        /// the vector is converted to the next bigger data type.
        pub(crate) fn with_capacity(capacity: usize) -> Self {
            AdaptativeVector::U8(Vec::with_capacity(capacity))
        }
        /// Returns the maximum value in the vector.
        pub(crate) fn max(&self) -> AdaptativeVectorValue {
            match self {
                AdaptativeVector::U8(vector) => {
                    vector.iter().max().copied().unwrap_or_default().into()
                }
                AdaptativeVector::U16(vector) => {
                    vector.iter().max().copied().unwrap_or_default().into()
                }
                AdaptativeVector::U32(vector) => {
                    vector.iter().max().copied().unwrap_or_default().into()
                }
                AdaptativeVector::U64(vector) => {
                    vector.iter().max().copied().unwrap_or_default().into()
                }
            }
        }
        /// Returns the length of the vector.
        pub(crate) fn len(&self) -> usize {
            match self {
                AdaptativeVector::U8(vector) => vector.len(),
                AdaptativeVector::U16(vector) => vector.len(),
                AdaptativeVector::U32(vector) => vector.len(),
                AdaptativeVector::U64(vector) => vector.len(),
            }
        }
        /// Pushes a value to the vector.
        ///
        /// # Arguments
        /// * `value` - The value to push to the vector.
        ///
        /// # Implementation details
        /// The value provided must be an AdaptativeVectorValue.
        /// If the value does not fit the current data type, the
        /// vector is converted to the next bigger data type.
        ///
        /// # Returns
        /// A boolean indicating whether it was necessary to
        /// convert the vector to a bigger data type.
        pub(crate) fn push<A>(&mut self, value: A) -> bool
        where
            A: Into<AdaptativeVectorValue>,
        {
            let value = AdaptativeVectorValue::smallest(value);
            match self {
                AdaptativeVector::U8(vector) => {
                    match value {
                        AdaptativeVectorValue::U8(value) => {
                            vector.push(value);
                            false
                        }
                        AdaptativeVectorValue::U16(value) => {
                            self.push_upgrade_vector(value);
                            true
                        }
                        AdaptativeVectorValue::U32(value) => {
                            self.push_upgrade_vector(value);
                            true
                        }
                        AdaptativeVectorValue::U64(value) => {
                            self.push_upgrade_vector(value);
                            true
                        }
                    }
                }
                AdaptativeVector::U16(vector) => {
                    match value {
                        AdaptativeVectorValue::U8(value) => {
                            vector.push(value as u16);
                            false
                        }
                        AdaptativeVectorValue::U16(value) => {
                            vector.push(value);
                            false
                        }
                        AdaptativeVectorValue::U32(value) => {
                            self.push_upgrade_vector(value);
                            true
                        }
                        AdaptativeVectorValue::U64(value) => {
                            self.push_upgrade_vector(value);
                            true
                        }
                    }
                }
                AdaptativeVector::U32(vector) => {
                    match value {
                        AdaptativeVectorValue::U8(value) => {
                            vector.push(value as u32);
                            false
                        }
                        AdaptativeVectorValue::U16(value) => {
                            vector.push(value as u32);
                            false
                        }
                        AdaptativeVectorValue::U32(value) => {
                            vector.push(value);
                            false
                        }
                        AdaptativeVectorValue::U64(value) => {
                            self.push_upgrade_vector(value);
                            true
                        }
                    }
                }
                AdaptativeVector::U64(vector) => {
                    match value {
                        AdaptativeVectorValue::U8(value) => {
                            vector.push(value as u64);
                            false
                        }
                        AdaptativeVectorValue::U16(value) => {
                            vector.push(value as u64);
                            false
                        }
                        AdaptativeVectorValue::U32(value) => {
                            vector.push(value as u64);
                            false
                        }
                        AdaptativeVectorValue::U64(value) => {
                            vector.push(value);
                            false
                        }
                    }
                }
            }
        }
    }
    pub(crate) enum AdaptativeVectorValue {
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AdaptativeVectorValue {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AdaptativeVectorValue::U8(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "U8", &__self_0)
                }
                AdaptativeVectorValue::U16(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "U16",
                        &__self_0,
                    )
                }
                AdaptativeVectorValue::U32(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "U32",
                        &__self_0,
                    )
                }
                AdaptativeVectorValue::U64(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "U64",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AdaptativeVectorValue {
        #[inline]
        fn clone(&self) -> AdaptativeVectorValue {
            let _: ::core::clone::AssertParamIsClone<u8>;
            let _: ::core::clone::AssertParamIsClone<u16>;
            let _: ::core::clone::AssertParamIsClone<u32>;
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AdaptativeVectorValue {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AdaptativeVectorValue {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AdaptativeVectorValue {
        #[inline]
        fn eq(&self, other: &AdaptativeVectorValue) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        AdaptativeVectorValue::U8(__self_0),
                        AdaptativeVectorValue::U8(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        AdaptativeVectorValue::U16(__self_0),
                        AdaptativeVectorValue::U16(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        AdaptativeVectorValue::U32(__self_0),
                        AdaptativeVectorValue::U32(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        AdaptativeVectorValue::U64(__self_0),
                        AdaptativeVectorValue::U64(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for AdaptativeVectorValue {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
            let _: ::core::cmp::AssertParamIsEq<u16>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for AdaptativeVectorValue {
        #[inline]
        fn partial_cmp(
            &self,
            other: &AdaptativeVectorValue,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (
                    AdaptativeVectorValue::U8(__self_0),
                    AdaptativeVectorValue::U8(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                (
                    AdaptativeVectorValue::U16(__self_0),
                    AdaptativeVectorValue::U16(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                (
                    AdaptativeVectorValue::U32(__self_0),
                    AdaptativeVectorValue::U32(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                (
                    AdaptativeVectorValue::U64(__self_0),
                    AdaptativeVectorValue::U64(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for AdaptativeVectorValue {
        #[inline]
        fn cmp(&self, other: &AdaptativeVectorValue) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                ::core::cmp::Ordering::Equal => {
                    match (self, other) {
                        (
                            AdaptativeVectorValue::U8(__self_0),
                            AdaptativeVectorValue::U8(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            AdaptativeVectorValue::U16(__self_0),
                            AdaptativeVectorValue::U16(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            AdaptativeVectorValue::U32(__self_0),
                            AdaptativeVectorValue::U32(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            AdaptativeVectorValue::U64(__self_0),
                            AdaptativeVectorValue::U64(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
                }
                cmp => cmp,
            }
        }
    }
    impl AdaptativeVectorValue {
        pub(crate) fn smallest<A>(value: A) -> Self
        where
            A: Into<AdaptativeVectorValue>,
        {
            match value.into() {
                AdaptativeVectorValue::U8(value) => AdaptativeVectorValue::U8(value),
                AdaptativeVectorValue::U16(value) => {
                    if value < u8::MAX.into() {
                        AdaptativeVectorValue::U8(value as u8)
                    } else {
                        AdaptativeVectorValue::U16(value)
                    }
                }
                AdaptativeVectorValue::U32(value) => {
                    if value < u8::MAX.into() {
                        AdaptativeVectorValue::U8(value as u8)
                    } else if value < u16::MAX.into() {
                        AdaptativeVectorValue::U16(value as u16)
                    } else {
                        AdaptativeVectorValue::U32(value)
                    }
                }
                AdaptativeVectorValue::U64(value) => {
                    if value < u8::MAX.into() {
                        AdaptativeVectorValue::U8(value as u8)
                    } else if value < u16::MAX.into() {
                        AdaptativeVectorValue::U16(value as u16)
                    } else if value < u32::MAX.into() {
                        AdaptativeVectorValue::U32(value as u32)
                    } else {
                        AdaptativeVectorValue::U64(value)
                    }
                }
            }
        }
    }
    impl Default for AdaptativeVectorValue {
        fn default() -> Self {
            AdaptativeVectorValue::U8(0)
        }
    }
    impl core::ops::Add<AdaptativeVectorValue> for AdaptativeVectorValue {
        type Output = Self;
        /// Adds the value inplace and returns whether it was necessary to convert the value to a bigger data type.
        ///
        /// # Implementative details
        /// Whenever the provided amount is larger than the current data type, we convert the current value into
        /// the next bigger data type. This is done inplace, i.e. the current value is updated to the new data type.
        /// When the two data types are the same, we use an overflowing_add to add the two values. If the addition
        /// overflows, we convert the current value into the next bigger data type.
        ///
        /// # Returns
        /// A boolean indicating whether it was necessary to convert the value to a bigger data type.
        fn add(self, amount: Self) -> Self {
            match self {
                AdaptativeVectorValue::U8(value) => {
                    match amount {
                        AdaptativeVectorValue::U8(amount) => {
                            let (new_value, overflow) = value.overflowing_add(amount);
                            if overflow {
                                AdaptativeVectorValue::U16(value as u16 + amount as u16)
                            } else {
                                AdaptativeVectorValue::U8(new_value)
                            }
                        }
                        AdaptativeVectorValue::U16(amount) => {
                            AdaptativeVectorValue::U16(value as u16)
                                + AdaptativeVectorValue::U16(amount)
                        }
                        AdaptativeVectorValue::U32(amount) => {
                            AdaptativeVectorValue::U32(value as u32)
                                + AdaptativeVectorValue::U32(amount)
                        }
                        AdaptativeVectorValue::U64(amount) => {
                            AdaptativeVectorValue::U64(value as u64)
                                + AdaptativeVectorValue::U64(amount)
                        }
                    }
                }
                AdaptativeVectorValue::U16(value) => {
                    match amount {
                        AdaptativeVectorValue::U8(amount) => {
                            AdaptativeVectorValue::U16(value)
                                + AdaptativeVectorValue::U16(amount as u16)
                        }
                        AdaptativeVectorValue::U16(amount) => {
                            let (new_value, overflow) = value.overflowing_add(amount);
                            if overflow {
                                AdaptativeVectorValue::U32(value as u32 + amount as u32)
                            } else {
                                AdaptativeVectorValue::U16(new_value)
                            }
                        }
                        AdaptativeVectorValue::U32(amount) => {
                            AdaptativeVectorValue::U32(value as u32)
                                + AdaptativeVectorValue::U32(amount)
                        }
                        AdaptativeVectorValue::U64(amount) => {
                            AdaptativeVectorValue::U64(value as u64)
                                + AdaptativeVectorValue::U64(amount)
                        }
                    }
                }
                AdaptativeVectorValue::U32(value) => {
                    match amount {
                        AdaptativeVectorValue::U8(amount) => {
                            AdaptativeVectorValue::U32(value)
                                + AdaptativeVectorValue::U32(amount as u32)
                        }
                        AdaptativeVectorValue::U16(amount) => {
                            AdaptativeVectorValue::U32(value)
                                + AdaptativeVectorValue::U32(amount as u32)
                        }
                        AdaptativeVectorValue::U32(amount) => {
                            let (new_value, overflow) = value.overflowing_add(amount);
                            if overflow {
                                AdaptativeVectorValue::U64(value as u64 + amount as u64)
                            } else {
                                AdaptativeVectorValue::U32(new_value)
                            }
                        }
                        AdaptativeVectorValue::U64(amount) => {
                            AdaptativeVectorValue::U64(value as u64)
                                + AdaptativeVectorValue::U64(amount)
                        }
                    }
                }
                AdaptativeVectorValue::U64(value) => {
                    match amount {
                        AdaptativeVectorValue::U8(amount) => {
                            AdaptativeVectorValue::U64(value)
                                + AdaptativeVectorValue::U64(amount as u64)
                        }
                        AdaptativeVectorValue::U16(amount) => {
                            AdaptativeVectorValue::U64(value)
                                + AdaptativeVectorValue::U64(amount as u64)
                        }
                        AdaptativeVectorValue::U32(amount) => {
                            AdaptativeVectorValue::U64(value)
                                + AdaptativeVectorValue::U64(amount as u64)
                        }
                        AdaptativeVectorValue::U64(amount) => {
                            AdaptativeVectorValue::U64(value + amount)
                        }
                    }
                }
            }
        }
    }
    impl core::ops::AddAssign for AdaptativeVectorValue {
        /// Adds the value inplace and returns whether it was necessary to convert the value to a bigger data type.
        ///
        /// # Implementative details
        /// Whenever the provided amount is larger than the current data type, we convert the current value into
        /// the next bigger data type. This is done inplace, i.e. the current value is updated to the new data type.
        /// When the two data types are the same, we use an overflowing_add to add the two values. If the addition
        /// overflows, we convert the current value into the next bigger data type.
        fn add_assign(&mut self, amount: Self) {
            *self = *self + amount;
        }
    }
    impl From<u8> for AdaptativeVectorValue {
        fn from(value: u8) -> Self {
            AdaptativeVectorValue::U8(value)
        }
    }
    impl From<u16> for AdaptativeVectorValue {
        fn from(value: u16) -> Self {
            AdaptativeVectorValue::U16(value)
        }
    }
    impl From<u32> for AdaptativeVectorValue {
        fn from(value: u32) -> Self {
            AdaptativeVectorValue::U32(value)
        }
    }
    impl From<u64> for AdaptativeVectorValue {
        fn from(value: u64) -> Self {
            AdaptativeVectorValue::U64(value)
        }
    }
    impl From<usize> for AdaptativeVectorValue {
        fn from(value: usize) -> Self {
            AdaptativeVectorValue::U64(value as u64)
        }
    }
    impl From<AdaptativeVector> for BitFieldVec {
        fn from(vector: AdaptativeVector) -> Self {
            let maximum_value = vector.max();
            let number_of_bits_to_represent_maximum_value = match maximum_value {
                AdaptativeVectorValue::U8(value) => {
                    value.as_usize().next_power_of_two().trailing_zeros()
                }
                AdaptativeVectorValue::U16(value) => {
                    value.as_usize().next_power_of_two().trailing_zeros()
                }
                AdaptativeVectorValue::U32(value) => {
                    value.as_usize().next_power_of_two().trailing_zeros()
                }
                AdaptativeVectorValue::U64(value) => {
                    value.as_usize().next_power_of_two().trailing_zeros()
                }
            };
            unsafe {
                let mut bit_field = BitFieldVec::new_uninit(
                    number_of_bits_to_represent_maximum_value as usize,
                    vector.len(),
                );
                match vector {
                    AdaptativeVector::U8(vector) => {
                        for (index, value) in vector.into_iter().enumerate() {
                            bit_field.set_unchecked(index, value as usize);
                        }
                    }
                    AdaptativeVector::U16(vector) => {
                        for (index, value) in vector.into_iter().enumerate() {
                            bit_field.set_unchecked(index, value as usize);
                        }
                    }
                    AdaptativeVector::U32(vector) => {
                        for (index, value) in vector.into_iter().enumerate() {
                            bit_field.set_unchecked(index, value as usize);
                        }
                    }
                    AdaptativeVector::U64(vector) => {
                        for (index, value) in vector.into_iter().enumerate() {
                            bit_field.set_unchecked(index, value as usize);
                        }
                    }
                }
                bit_field
            }
        }
    }
}
pub use adaptative_vector::*;
/// Re-export of the most commonly used traits and structs.
pub mod prelude {
    pub use crate::traits::*;
    pub use crate::search_result::*;
    pub use crate::corpus::*;
    pub use crate::similarity::*;
    pub use crate::adaptative_vector::*;
}
