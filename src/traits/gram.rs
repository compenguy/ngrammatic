//! Trait defining the unit type for an ngram.

use std::{
    hash::Hash,
    ops::{Index, IndexMut},
};

use sux::traits::ConvertTo;
use sux::{dict::{EliasFano, EliasFanoBuilder}, rank_sel::SelectFixed2, traits::IndexedDict};

use crate::{ASCIIChar, IntoUsize, Paddable};

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

/// Trait defining a builder of a sorted storage for Ngrams.
pub trait SortedNgramStorageBuilder<NG: Ngram> {
    /// The type of the storage.
    type Storage: SortedNgramStorage<NG>;

    /// Create a new builder.
    ///
    /// # Arguments
    /// * `number_of_ngrams` - The number of ngrams to store.
    /// * `maximal_ngram` - The maximal ngram to store.
    fn new_storage_builder(number_of_ngrams: usize, maximal_ngram: NG) -> Self;

    /// Push a new ngram into the storage.
    ///
    /// # Arguments
    /// * `ngram` - The ngram to push.
    ///
    /// # Safety
    /// This function is unsafe because it does not check if the ngram is sorted,
    /// or that it fits within the maximal ngram or that the storage is not full.
    unsafe fn push_unchecked(&mut self, ngram: NG);

    /// Build the storage.
    fn build(self) -> Self::Storage;
}

impl<NG: Ngram> SortedNgramStorageBuilder<NG> for EliasFanoBuilder
where
    NG: IntoUsize,
{
    type Storage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn new_storage_builder(number_of_ngrams: usize, maximal_ngram: NG) -> Self {
        EliasFanoBuilder::new(number_of_ngrams, maximal_ngram.into_usize())
    }

    #[inline(always)]
    unsafe fn push_unchecked(&mut self, ngram: NG) {
        self.push_unchecked(ngram.into_usize());
    }

    #[inline(always)]
    fn build(self) -> Self::Storage {
        self.build().convert_to().unwrap()
    }
}

impl<NG: Ngram> SortedNgramStorageBuilder<NG> for Vec<NG> {
    type Storage = Self;

    #[inline(always)]
    fn new_storage_builder(number_of_ngrams: usize, _maximal_ngram: NG) -> Self {
        Vec::with_capacity(number_of_ngrams)
    }

    #[inline(always)]
    unsafe fn push_unchecked(&mut self, ngram: NG) {
        self.push(ngram);
    }

    #[inline(always)]
    fn build(self) -> Self::Storage {
        self
    }
}

/// Trait defined a sorted storage for Ngrams.
pub trait SortedNgramStorage<NG: Ngram> {
    /// The builder to use to build this storage.
    type Builder: SortedNgramStorageBuilder<NG, Storage = Self>;

    /// Returns the number of ngrams in the storage.
    fn len(&self) -> usize;

    /// Returns true if the storage is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the index curresponding to the provided Ngram.
    /// 
    /// # Arguments
    /// * `ngram` - The ngram to search for.
    fn index_of(&self, ngram: NG) -> Option<usize>;

    /// Returns the index curresponding to the provided Ngram without checking bounds.
    /// 
    /// # Arguments
    /// * `ngram` - The ngram to search for.
    /// 
    /// # Safety
    /// This function is unsafe because it does not check if the index is within bounds.
    /// The caller must ensure that the index is within bounds.
    unsafe fn index_of_unchecked(&self, ngram: NG) -> usize;

    /// Returns the i-th ngram in the storage without checking bounds.
    /// 
    /// # Arguments
    /// * `i` - The index of the ngram to return.
    /// 
    /// # Safety
    /// This function is unsafe because it does not check if the index is within bounds.
    /// The caller must ensure that the index is within bounds.
    unsafe fn get_unchecked(&self, i: usize) -> NG;
}

impl<NG: Ngram> SortedNgramStorage<NG> for EliasFano<SelectFixed2>
where
    NG: IntoUsize,
{
    type Builder = EliasFanoBuilder;

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn index_of(&self, ngram: NG) -> Option<usize> {
        <Self as IndexedDict>::index_of(self, &ngram.into_usize())
    }

    #[inline(always)]
    unsafe fn index_of_unchecked(&self, ngram: NG) -> usize {
        <Self as IndexedDict>::index_of(self, &ngram.into_usize()).unwrap()
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, i: usize) -> NG {
        NG::from_usize(<Self as IndexedDict>::get_unchecked(self, i))
    }
}

impl<NG: Ngram> SortedNgramStorage<NG> for Vec<NG> {
    type Builder = Self;

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn index_of(&self, ngram: NG) -> Option<usize> {
        // Since the ngrams are sorted, we can use binary search.
        self.binary_search(&ngram).ok()
    }

    #[inline(always)]
    unsafe fn index_of_unchecked(&self, ngram: NG) -> usize {
        // Since the ngrams are sorted, we can use binary search.
        self.binary_search(&ngram).unwrap()
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, i: usize) -> NG {
        *<[NG]>::get_unchecked(self, i)
    }
}

/// Trait defining an Ngram.
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

    #[cfg(feature = "mem_dbg")]
    /// The type of structure to use to store the ngrams.
    /// This should be selected depending on the arity of the ngram,
    /// and the type of the gram. For example, any u8-based gram with an arity
    /// between 1 and 8 can be easily stored in an elias fano data structure
    /// since we can exploit the information that the ngram is sorted.
    /// Similarly, we can store within an u64 char-based grams with an arity
    /// between 1 and 2, and store these as well within an elias fano data structure.
    /// Larger ngrams need to be stored into vectors.
    type SortedStorage: SortedNgramStorage<Self> + mem_dbg::MemDbg + mem_dbg::MemSize;

    #[cfg(not(feature = "mem_dbg"))]
    /// The type of structure to use to store the ngrams.
    /// This should be selected depending on the arity of the ngram,
    /// and the type of the gram. For example, any u8-based gram with an arity
    /// between 1 and 8 can be easily stored in an elias fano data structure
    /// since we can exploit the information that the ngram is sorted.
    /// Similarly, we can store within an u64 char-based grams with an arity
    /// between 1 and 2, and store these as well within an elias fano data structure.
    /// Larger ngrams need to be stored into vectors.
    type SortedStorage: SortedNgramStorage<Self>;

    /// Rotate the ngram to the left.
    fn rotate_left(&mut self);
}

impl Ngram for MonoGram<u8> {
    const ARITY: usize = 1;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        // Do nothing.
    }
}

impl Ngram for MonoGram<ASCIIChar> {
    const ARITY: usize = 1;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        // Do nothing.
    }
}

impl Ngram for MonoGram<char> {
    const ARITY: usize = 1;
    type G = char;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        // Do nothing.
    }
}

impl Ngram for BiGram<u8> {
    const ARITY: usize = 2;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for BiGram<ASCIIChar> {
    const ARITY: usize = 2;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for BiGram<char> {
    const ARITY: usize = 2;
    type G = char;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for TriGram<u8> {
    const ARITY: usize = 3;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for TriGram<ASCIIChar> {
    const ARITY: usize = 3;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for TriGram<char> {
    const ARITY: usize = 3;
    type G = char;
    type SortedStorage = Vec<Self>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for TetraGram<u8> {
    const ARITY: usize = 4;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for TetraGram<ASCIIChar> {
    const ARITY: usize = 4;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for TetraGram<char> {
    const ARITY: usize = 4;
    type G = char;
    type SortedStorage = Vec<Self>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for PentaGram<u8> {
    const ARITY: usize = 5;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for PentaGram<ASCIIChar> {
    const ARITY: usize = 5;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for PentaGram<char> {
    const ARITY: usize = 5;
    type G = char;
    type SortedStorage = Vec<Self>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for HexaGram<u8> {
    const ARITY: usize = 6;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for HexaGram<ASCIIChar> {
    const ARITY: usize = 6;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for HexaGram<char> {
    const ARITY: usize = 6;
    type G = char;
    type SortedStorage = Vec<Self>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for HeptaGram<u8> {
    const ARITY: usize = 7;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for HeptaGram<ASCIIChar> {
    const ARITY: usize = 7;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for HeptaGram<char> {
    const ARITY: usize = 7;
    type G = char;
    type SortedStorage = Vec<Self>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for OctaGram<u8> {
    const ARITY: usize = 8;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for OctaGram<ASCIIChar> {
    const ARITY: usize = 8;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
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

impl<G: Paddable + Gram> PaddableNgram for MonoGram<G>
where
    Self: Ngram<G = G>,
{
    type Pad = [G; 0];
    const PADDING: Self::Pad = [];
}

impl<G: Paddable + Gram> PaddableNgram for BiGram<G>
where
    Self: Ngram<G = G>,
{
    type Pad = MonoGram<G>;
    const PADDING: Self::Pad = [G::PADDING];
}

impl<G: Paddable + Gram> PaddableNgram for TriGram<G>
where
    Self: Ngram<G = G>,
{
    type Pad = BiGram<G>;
    const PADDING: Self::Pad = [G::PADDING; 2];
}

impl<G: Paddable + Gram> PaddableNgram for TetraGram<G>
where
    Self: Ngram<G = G>,
{
    type Pad = TriGram<G>;
    const PADDING: Self::Pad = [G::PADDING; 3];
}

impl<G: Paddable + Gram> PaddableNgram for PentaGram<G>
where
    Self: Ngram<G = G>,
{
    type Pad = TetraGram<G>;
    const PADDING: Self::Pad = [G::PADDING; 4];
}

impl<G: Paddable + Gram> PaddableNgram for HexaGram<G>
where
    Self: Ngram<G = G>,
{
    type Pad = PentaGram<G>;
    const PADDING: Self::Pad = [G::PADDING; 5];
}

impl<G: Paddable + Gram> PaddableNgram for HeptaGram<G>
where
    Self: Ngram<G = G>,
{
    type Pad = HexaGram<G>;
    const PADDING: Self::Pad = [G::PADDING; 6];
}

impl<G: Paddable + Gram> PaddableNgram for OctaGram<G>
where
    Self: Ngram<G = G>,
{
    type Pad = HeptaGram<G>;
    const PADDING: Self::Pad = [G::PADDING; 7];
}
