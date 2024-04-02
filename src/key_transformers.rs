//! Submodule providing linkable key transformers.
use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

use crate::Key;
// pub mod pad_both;
// pub use pad_both::PadBoth;

/// Trait for key transformers.
///
/// Key transformers are used to transform keys before they are used in a map,
/// and are designed to be chained together.
pub trait KeyTransformer<K>: Display + Default + Debug + PartialEq + Eq{
    /// The linked key transformer type.
    type Linked<Dst>: KeyTransformer<K>
    where
        Dst: KeyTransformer<Self::Target>;
    /// The type of the transformed key.
    type Target: AsRef<[u8]>;

    /// Transform a key.
    ///
    /// # Arguments
    /// * `key` - The key to transform.
    fn transform(&self, key: &K) -> Self::Target;

    /// Link this key transformer to another.
    ///
    /// # Arguments
    /// * `dst` - The key transformer to link to itself.
    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: KeyTransformer<Self::Target>;

    /// Chain this key transformer with a lowercasing key transformer.
    fn lower(self) -> Self::Linked<Lower>
    where
        Self: Sized,
        Lower: KeyTransformer<Self::Target>,
    {
        self.link(Lower)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Identity key transformer.
pub struct Identity;

impl Display for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity")
    }
}

impl<K: Key + AsRef<[u8]>> KeyTransformer<K> for Identity {
    type Linked<Dst> = Dst where Dst: KeyTransformer<Self::Target>;
    type Target = K;

    fn transform(&self, key: &K) -> Self::Target {
        key.clone()
    }

    fn link<Dst>(self, dst: Dst) -> Dst
    where
        Dst: KeyTransformer<K>,
    {
        dst
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Lowercasing key transformer.
pub struct Lower;

impl Display for Lower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lower")
    }
}

impl KeyTransformer<String> for Lower {
    type Linked<Dst> = Linked<Self, Dst> where Dst: for<'a> KeyTransformer<Self::Target>;
    type Target = String;

    fn transform(&self, key: &String) -> String {
        key.to_lowercase()
    }

    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: for<'a> KeyTransformer<Self::Target>,
    {
        Linked::new(self, dst)
    }
}

impl KeyTransformer<&str> for Lower {
    type Linked<Dst> = Linked<Self, Dst> where Dst: KeyTransformer<Self::Target>;
    type Target = String;

    fn transform(&self, key: &&str) -> Self::Target {
        key.to_lowercase()
    }

    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: for<'a> KeyTransformer<Self::Target>,
    {
        Linked::new(self, dst)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Trim key transformer.
pub struct Trim;

impl Display for Trim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trim")
    }
}

impl KeyTransformer<String> for Trim {
    type Linked<Dst> = Linked<Self, Dst> where Dst: KeyTransformer<String>;
    type Target = String;

    fn transform(&self, key: &String) -> String {
        key.trim().to_string()
    }

    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: for<'a> KeyTransformer<Self::Target>,
    {
        Linked::new(self, dst)
    }
}

impl KeyTransformer<&str> for Trim {
    type Linked<Dst> = Linked<Self, Dst> where Dst: KeyTransformer<Self::Target>;
    type Target = String;

    fn transform(&self, key: &&str) -> Self::Target {
        key.trim().to_string()
    }

    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: KeyTransformer<Self::Target>,
    {
        Linked::new(self, dst)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Linked key transformer.
pub struct Linked<Src, Dst> {
    src: Src,
    dst: Dst,
}

impl<Src, Dst> Display for Linked<Src, Dst>
where
    Src: Display,
    Dst: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dst.fmt(f)?;
        write!(f, "<")?;
        self.src.fmt(f)?;
        write!(f, ">")
    }
}

impl<Src, Dst> Linked<Src, Dst> {
    /// Create a new linked key transformer.
    ///
    /// # Arguments
    /// * `src` - The source key transformer.
    /// * `dst` - The destination key transformer.
    pub fn new(src: Src, dst: Dst) -> Self {
        Linked { src, dst }
    }
}

impl<K, Src, Dst> KeyTransformer<K> for Linked<Src, Dst>
where
    Src: KeyTransformer<K>,
    Dst: KeyTransformer<<Src as KeyTransformer<K>>::Target>,
{
    type Linked<Next> = Linked<Self, Next> where Next: KeyTransformer<Self::Target>;
    type Target = <Dst as KeyTransformer<Src::Target>>::Target;

    fn transform(&self, key: &K) -> Self::Target {
        self.dst.transform(&self.src.transform(key))
    }

    fn link<Next>(self, next: Next) -> Self::Linked<Next>
    where
        Next: KeyTransformer<Self::Target>,
    {
        Linked::new(self, next)
    }
}
