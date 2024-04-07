//! Submodule defining trait for a container of Keys.

use crate::{Key, Ngram};

/// Trait defining a container of keys.
pub trait Keys<NG: Ngram> {
    /// The type of the key.
    type K: Key<NG, <NG as Ngram>::G>;
    /// The iterator to iter the keys.
    type IterKeys<'a>: Iterator<Item = &'a Self::K>
    where
        Self::K: 'a,
        Self: 'a;

    /// Returns the number of keys.
    fn len(&self) -> usize;

    /// Returns whether the container is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the key at the given index.
    ///
    /// # Implementative details
    /// Note that this method does not return a &K, but the
    /// type Ref assocuated with the Key trait.
    fn get_ref(&self, index: usize) -> &<Self::K as Key<NG, <NG as Ngram>::G>>::Ref;

    /// Returns an iterator over the keys.
    fn iter(&self) -> Self::IterKeys<'_>;
}

impl<NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for Vec<K> {
    type K = K;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        self.len()
    }

    fn get_ref(&self, index: usize) -> &<Self::K as Key<NG, <NG as Ngram>::G>>::Ref {
        self[index].as_ref()
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}

impl<const N: usize, NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for [K; N] {
    type K = K;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        <[K]>::len(self)
    }

    fn get_ref(&self, index: usize) -> &<Self::K as Key<NG, <NG as Ngram>::G>>::Ref {
        self[index].as_ref()
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}

impl<const N: usize, NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for &[K; N] {
    type K = K;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        <[K]>::len(*self)
    }

    fn get_ref(&self, index: usize) -> &<Self::K as Key<NG, <NG as Ngram>::G>>::Ref {
        <[K]>::get_ref(*self, index)
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(*self)
    }
}

impl<NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for [K] {
    type K = K;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        self.len()
    }

    fn get_ref(&self, index: usize) -> &<Self::K as Key<NG, <NG as Ngram>::G>>::Ref {
        self[index].as_ref()
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}

impl<NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for &[K] {
    type K = K;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        <[K]>::len(*self)
    }

    fn get_ref(&self, index: usize) -> &<Self::K as Key<NG, <NG as Ngram>::G>>::Ref {
        self[index].as_ref()
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}
