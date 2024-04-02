//! Trait defining a key and its hasher.

use std::hash::Hash;

use crate::KeyTransformer;

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

/// Trait defining a key.
pub trait Key: Clone + PartialEq + Eq {
    /// The type of the key.
    type Key: Hash;

    /// Returns the key.
    fn key(&self) -> Self::Key;
}

impl Key for String {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.clone()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Normalizer for a key.
pub struct Normalizer<K: Key, KT: KeyTransformer<K::Key>> {
    key: K,
    _phantom: std::marker::PhantomData<KT>,
}

impl<K: Key, KT: KeyTransformer<K::Key>> Normalizer<K, KT> {
    /// Returns the inner key.
    pub fn key(&self) -> &K {
        &self.key
    }
}

impl<K: Key, KT: KeyTransformer<K::Key>> From<K> for Normalizer<K, KT> {
    fn from(key: K) -> Self {
        Normalizer {
            key,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<K: Key, KT: KeyTransformer<K::Key>> Hash for Normalizer<K, KT> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let key_transformer = KT::default();
        key_transformer
            .transform(&self.key.key())
            .as_ref()
            .hash(state);
    }
}
