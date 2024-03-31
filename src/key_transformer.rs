//! Submodule providing linkable key transformers.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Trait for key transformers.
///
/// Key transformers are used to transform keys before they are used in a map,
/// and are designed to be chained together.
pub trait KeyTransformer {
    /// Transform a key.
    ///
    /// # Arguments
    /// * `key` - The key to transform.
    fn transform(&self, key: &str) -> String;

    /// Link this key transformer to another.
    ///
    /// # Arguments
    /// * `dst` - The key transformer to link to itself.
    fn link<Dst>(self, dst: Dst) -> LinkedKeyTransformer<Self, Dst>
    where
        Self: Sized,
    {
        LinkedKeyTransformer::new(self, dst)
    }

    /// Chain this key transformer with a lowercasing key transformer.
    fn lower(self) -> LinkedKeyTransformer<Self, LowerKeyTransformer>
    where
        Self: Sized,
    {
        self.link(LowerKeyTransformer)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Identity key transformer.
pub struct IdentityKeyTransformer;

impl KeyTransformer for IdentityKeyTransformer {
    fn transform(&self, key: &str) -> String {
        key.to_string()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Lowercasing key transformer.
pub struct LowerKeyTransformer;

impl KeyTransformer for LowerKeyTransformer {
    fn transform(&self, key: &str) -> String {
        key.to_lowercase()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Linked key transformer.
pub struct LinkedKeyTransformer<Src, Dst> {
    src: Src,
    dst: Dst,
}

impl<Src, Dst> LinkedKeyTransformer<Src, Dst> {
    /// Create a new linked key transformer.
    ///
    /// # Arguments
    /// * `src` - The source key transformer.
    /// * `dst` - The destination key transformer.
    pub fn new(src: Src, dst: Dst) -> Self {
        LinkedKeyTransformer { src, dst }
    }
}

impl<Src, Dst> KeyTransformer for LinkedKeyTransformer<Src, Dst>
where
    Src: KeyTransformer,
    Dst: KeyTransformer,
{
    fn transform(&self, key: &str) -> String {
        self.dst.transform(&self.src.transform(key))
    }
}
