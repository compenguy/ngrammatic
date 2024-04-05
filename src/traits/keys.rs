//! Submodule defining trait for a container of Keys.

use std::ops::Index;

use crate::{Gram, Key};

/// Trait defining a container of keys.
pub trait Keys<G: Gram>: Index<usize, Output = <Self as Keys<G>>::K> {
    /// The type of the key.
    type K: Key<G>;
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
