//! Submodule providing linkable key transformers.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

/// Trait for key transformers.
///
/// Key transformers are used to transform keys before they are used in a map,
/// and are designed to be chained together.
pub trait KeyTransformer {
    /// The linked key transformer type.
    type Linked<Dst>: KeyTransformer
    where
        Dst: KeyTransformer;

    /// Transform a key.
    ///
    /// # Arguments
    /// * `key` - The key to transform.
    fn transform<S: ToString + AsRef<str>>(&self, key: S) -> String;

    /// Link this key transformer to another.
    ///
    /// # Arguments
    /// * `dst` - The key transformer to link to itself.
    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: KeyTransformer;

    /// Chain this key transformer with a lowercasing key transformer.
    fn lower(self) -> Self::Linked<LowerKeyTransformer>
    where
        Self: Sized,
    {
        self.link(LowerKeyTransformer)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Identity key transformer.
pub struct IdentityKeyTransformer;

impl KeyTransformer for IdentityKeyTransformer {
    type Linked<Dst> = Dst where Dst: KeyTransformer;

    fn transform<S: ToString>(&self, key: S) -> String {
        key.to_string()
    }

    fn link<Dst>(self, dst: Dst) -> Dst
    where
        Dst: KeyTransformer,
    {
        dst
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Lowercasing key transformer.
pub struct LowerKeyTransformer;

impl KeyTransformer for LowerKeyTransformer {
    type Linked<Dst> = LinkedKeyTransformer<Self, Dst> where Dst: KeyTransformer;

    fn transform<S: ToString + AsRef<str>>(&self, key: S) -> String {
        let key: &str = key.as_ref();
        key.to_lowercase()
    }

    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: KeyTransformer,
    {
        LinkedKeyTransformer::new(self, dst)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
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
    type Linked<Next> = LinkedKeyTransformer<Self, Next> where Next: KeyTransformer;

    fn transform<S: ToString + AsRef<str>>(&self, key: S) -> String {
        self.dst.transform(self.src.transform(key))
    }

    fn link<Next>(self, next: Next) -> Self::Linked<Next>
    where
        Next: KeyTransformer,
    {
        LinkedKeyTransformer::new(self, next)
    }
}
