use crate::KeyTransformer;
use std::fmt::Display;
use crate::Linked;

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// PadBoth key transformer, adding the provided padding to both sides of the key.
/// 
/// # Example
/// 
/// ```rust
/// use ngrammatic::key_transformers::PadBoth;
/// 
/// let pad_both = PadBoth::<"-">;
/// assert_eq!(pad_both.transform(&"ab"), "-ab-");
/// ```
/// 
pub struct PadBoth<const Padding: &'static [char]>;

impl<const Padding: &'static str> Display for PadBoth<Padding> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PadBoth<{}>", Padding)
    }
}

impl<const Padding: &'static str> KeyTransformer<String> for PadBoth<Padding> {
    type Linked<Dst> = Linked<Self, Dst> where Dst: KeyTransformer<String>;
    type Target = String;

    fn transform(&self, key: &String) -> String {
        format!("{}{}{}", Padding, key, Padding)        
    }

    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: for<'a> KeyTransformer<Self::Target>,
    {
        Linked::new(self, dst)
    }
}

impl<const Padding: &'static str> KeyTransformer<&str> for PadBoth<Padding> {
    type Linked<Dst> = Linked<Self, Dst> where Dst: KeyTransformer<Self::Target>;
    type Target = String;

    fn transform(&self, key: &&str) -> Self::Target {
        format!("{}{}{}", Padding, key, Padding)
    }

    fn link<Dst>(self, dst: Dst) -> Self::Linked<Dst>
    where
        Dst: KeyTransformer<Self::Target>,
    {
        Linked::new(self, dst)
    }
}