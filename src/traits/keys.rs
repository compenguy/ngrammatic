//! Submodule defining trait for a container of Keys.

use std::ops::Index;

use crate::{Key, Ngram};

/// Trait defining a container of keys.
pub trait Keys<NG: Ngram>: Index<usize, Output = <Self as Keys<NG>>::K> {
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

    /// Returns an iterator over the keys.
    fn iter(&self) -> Self::IterKeys<'_>;
}

impl<NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for Vec<K> {
    type K = K;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        self.len()
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

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}

impl<NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for [K] {
    type K = K;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}
