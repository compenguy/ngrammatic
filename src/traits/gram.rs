//! Trait defining the unit type for an ngram.

use std::{
    cell::UnsafeCell,
    fmt::Debug,
    hash::Hash,
    iter::Copied,
    ops::{Index, IndexMut},
};

use sux::{
    bits::BitFieldVec,
    dict::{elias_fano::EliasFanoIterator, EliasFanoConcurrentBuilder},
    traits::ConvertTo,
};
use sux::{
    dict::{EliasFano, EliasFanoBuilder},
    rank_sel::SelectFixed2,
    traits::IndexedDict,
};

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

/// Trait defining a concurrent builder of a sorted storage for Ngrams.
pub trait ConcurrentSortedNgramStorageBuilder<NG: Ngram> {
    /// The type of the storage.
    type Storage: SortedNgramStorage<NG> + Send + Sync;

    /// Create a new builder.
    ///
    /// # Arguments
    /// * `number_of_ngrams` - The number of ngrams to store.
    /// * `maximal_ngram` - The maximal ngram to store.
    fn new_storage_builder(number_of_ngrams: usize, maximal_ngram: NG) -> Self;

    /// Set a new ngram into the storage.
    ///
    /// # Arguments
    /// * `ngram` - The ngram to push.
    /// * `index` - The index of the ngram to set.
    ///
    /// # Safety
    /// This function is unsafe because it does not check if the ngram is sorted,
    /// or that it fits within the maximal ngram or that the storage is not full.
    unsafe fn set_unchecked(&self, ngram: NG, index: usize);

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

#[cfg(feature = "rayon")]
impl<NG: Ngram + IntoUsize> ConcurrentSortedNgramStorageBuilder<NG> for EliasFanoConcurrentBuilder {
    type Storage = EliasFano<SelectFixed2>;

    #[inline(always)]
    fn new_storage_builder(number_of_ngrams: usize, maximal_ngram: NG) -> Self {
        EliasFanoConcurrentBuilder::new(number_of_ngrams, maximal_ngram.into_usize())
    }

    #[inline(always)]
    unsafe fn set_unchecked(&self, ngram: NG, index: usize) {
        self.set(
            index,
            ngram.into_usize(),
            std::sync::atomic::Ordering::SeqCst,
        );
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

/// A shared vector to build a concurrent storage.
pub struct SharedVec<NG> {
    storage: UnsafeCell<Vec<NG>>,
}

unsafe impl<NG> Send for SharedVec<NG> {}
unsafe impl<NG> Sync for SharedVec<NG> {}

impl<NG: Ngram> ConcurrentSortedNgramStorageBuilder<NG> for SharedVec<NG> {
    type Storage = Vec<NG>;

    #[inline(always)]
    #[allow(clippy::uninit_vec)]
    fn new_storage_builder(number_of_ngrams: usize, _maximal_ngram: NG) -> Self {
        let mut storage = Vec::with_capacity(number_of_ngrams);
        unsafe {
            storage.set_len(number_of_ngrams);
        }
        SharedVec {
            storage: UnsafeCell::new(storage),
        }
    }

    #[inline(always)]
    unsafe fn set_unchecked(&self, ngram: NG, index: usize) {
        let storage = &mut *self.storage.get();
        storage[index] = ngram;
    }

    #[inline(always)]
    fn build(self) -> Self::Storage {
        self.storage.into_inner()
    }
}

/// Trait defined a sorted storage for Ngrams.
pub trait SortedNgramStorage<NG: Ngram>: Send + Sync {
    /// The builder to use to build this storage.
    type Builder: SortedNgramStorageBuilder<NG, Storage = Self>;

    #[cfg(feature = "rayon")]
    /// The concurrent builder to use to build this storage.
    type ConcurrentBuilder: ConcurrentSortedNgramStorageBuilder<NG, Storage = Self> + Send + Sync;

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

    /// Iterator over the ngrams in the storage.
    type Iter<'a>: Iterator<Item = NG>
    where
        Self: 'a;

    /// Returns an iterator over the ngrams in the storage.
    fn iter(&self) -> Self::Iter<'_>;
}

impl<NG: Ngram> SortedNgramStorage<NG> for EliasFano<SelectFixed2>
where
    NG: IntoUsize,
{
    type Builder = EliasFanoBuilder;

    #[cfg(feature = "rayon")]
    type ConcurrentBuilder = EliasFanoConcurrentBuilder;

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

    type Iter<'a> =
        std::iter::Map<EliasFanoIterator<'a, SelectFixed2, BitFieldVec>, fn(usize) -> NG>;

    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter_from(0).map(NG::from_usize)
    }
}

impl<NG: Ngram> SortedNgramStorage<NG> for Vec<NG> {
    type Builder = Self;

    #[cfg(feature = "rayon")]
    type ConcurrentBuilder = SharedVec<NG>;

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

    type Iter<'a> = Copied<std::slice::Iter<'a, NG>> where Self: 'a;

    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        <[NG]>::iter(self).copied()
    }
}

/// Trait defining an Ngram.
pub trait Ngram:
    Default
    + Clone
    + Copy
    + Ord
    + Eq
    + Send
    + Sync
    + Debug
    + PartialEq
    + Hash
    + Index<usize, Output = <Self as Ngram>::G>
    + IndexMut<usize, Output = <Self as Ngram>::G>
{
    /// The type of the ngram.
    type G: Gram;

    /// The arity of the ngram.
    const ARITY: usize;

    /// The padding type. It will generally be an
    /// array of one unit smaller than the arity of Self.
    type Pad: IntoIterator<Item = Self::G>;

    /// The padding value.
    const PADDING: Self::Pad;

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

    type Pad = [Self::G; 0];
    const PADDING: Self::Pad = [Self::G::PADDING; 0];

    #[inline(always)]
    fn rotate_left(&mut self) {
        // Do nothing.
    }
}

impl Ngram for MonoGram<ASCIIChar> {
    const ARITY: usize = 1;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 0];
    const PADDING: Self::Pad = [Self::G::PADDING; 0];

    #[inline(always)]
    fn rotate_left(&mut self) {
        // Do nothing.
    }
}

impl Ngram for MonoGram<char> {
    const ARITY: usize = 1;
    type G = char;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 0];
    const PADDING: Self::Pad = [Self::G::PADDING; 0];

    #[inline(always)]
    fn rotate_left(&mut self) {
        // Do nothing.
    }
}

impl Ngram for BiGram<u8> {
    const ARITY: usize = 2;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 1];
    const PADDING: Self::Pad = [Self::G::PADDING; 1];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for BiGram<ASCIIChar> {
    const ARITY: usize = 2;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 1];
    const PADDING: Self::Pad = [Self::G::PADDING; 1];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for BiGram<char> {
    const ARITY: usize = 2;
    type G = char;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 1];
    const PADDING: Self::Pad = [Self::G::PADDING; 1];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for TriGram<u8> {
    const ARITY: usize = 3;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 2];
    const PADDING: Self::Pad = [Self::G::PADDING; 2];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for TriGram<ASCIIChar> {
    const ARITY: usize = 3;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 2];
    const PADDING: Self::Pad = [Self::G::PADDING; 2];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for TriGram<char> {
    const ARITY: usize = 3;
    type G = char;
    type SortedStorage = Vec<Self>;

    type Pad = [Self::G; 2];
    const PADDING: Self::Pad = [Self::G::PADDING; 2];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for TetraGram<u8> {
    const ARITY: usize = 4;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 3];
    const PADDING: Self::Pad = [Self::G::PADDING; 3];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for TetraGram<ASCIIChar> {
    const ARITY: usize = 4;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 3];
    const PADDING: Self::Pad = [Self::G::PADDING; 3];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for TetraGram<char> {
    const ARITY: usize = 4;
    type G = char;
    type SortedStorage = Vec<Self>;

    type Pad = [Self::G; 3];
    const PADDING: Self::Pad = [Self::G::PADDING; 3];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for PentaGram<u8> {
    const ARITY: usize = 5;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 4];
    const PADDING: Self::Pad = [Self::G::PADDING; 4];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for PentaGram<ASCIIChar> {
    const ARITY: usize = 5;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 4];
    const PADDING: Self::Pad = [Self::G::PADDING; 4];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for PentaGram<char> {
    const ARITY: usize = 5;
    type G = char;
    type SortedStorage = Vec<Self>;

    type Pad = [Self::G; 4];
    const PADDING: Self::Pad = [Self::G::PADDING; 4];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for HexaGram<u8> {
    const ARITY: usize = 6;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 5];
    const PADDING: Self::Pad = [Self::G::PADDING; 5];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for HexaGram<ASCIIChar> {
    const ARITY: usize = 6;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 5];
    const PADDING: Self::Pad = [Self::G::PADDING; 5];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for HexaGram<char> {
    const ARITY: usize = 6;
    type G = char;
    type SortedStorage = Vec<Self>;

    type Pad = [Self::G; 5];
    const PADDING: Self::Pad = [Self::G::PADDING; 5];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for HeptaGram<u8> {
    const ARITY: usize = 7;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 6];
    const PADDING: Self::Pad = [Self::G::PADDING; 6];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for HeptaGram<ASCIIChar> {
    const ARITY: usize = 7;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 6];
    const PADDING: Self::Pad = [Self::G::PADDING; 6];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for HeptaGram<char> {
    const ARITY: usize = 7;
    type G = char;
    type SortedStorage = Vec<Self>;

    type Pad = [Self::G; 6];
    const PADDING: Self::Pad = [Self::G::PADDING; 6];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}

impl Ngram for OctaGram<u8> {
    const ARITY: usize = 8;
    type G = u8;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 7];
    const PADDING: Self::Pad = [Self::G::PADDING; 7];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[u8]>::rotate_left(self, 1);
    }
}

impl Ngram for OctaGram<ASCIIChar> {
    const ARITY: usize = 8;
    type G = ASCIIChar;
    type SortedStorage = EliasFano<SelectFixed2>;

    type Pad = [Self::G; 7];
    const PADDING: Self::Pad = [Self::G::PADDING; 7];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[ASCIIChar]>::rotate_left(self, 1);
    }
}

impl Ngram for OctaGram<char> {
    const ARITY: usize = 8;
    type G = char;
    type SortedStorage = Vec<Self>;

    type Pad = [Self::G; 7];
    const PADDING: Self::Pad = [Self::G::PADDING; 7];

    #[inline(always)]
    fn rotate_left(&mut self) {
        <[char]>::rotate_left(self, 1);
    }
}