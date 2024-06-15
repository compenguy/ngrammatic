//! Submodule providing an offsettable iterator of usize.

#[derive(Clone, Debug)]
/// A struct handling the offsetting of ngram nodes in the WeightedBitFieldBipartiteGraph.
pub struct Offset<I> {
    /// The offset to apply to the nodes. It is expected to be zero for
    /// the source nodes representing the keys and equal to the number of
    /// source nodes for the destination nodes representing the ngrams.
    offset: isize,
    /// The iterator over the nodes.
    iterator: I,
}

impl<I> From<I> for Offset<I> {
    fn from(iterator: I) -> Self {
        Self::new(0, iterator)
    }
}

impl<I> Offset<I> {
    /// Returns a new Offset struct.
    pub fn new(offset: isize, iterator: I) -> Self {
        Offset { offset, iterator }
    }
}

impl<I> Iterator for Offset<I>
where
    I: Iterator<Item = usize>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|node| (node as isize + self.offset) as usize)
    }
}

impl<I> DoubleEndedIterator for Offset<I>
where
    I: DoubleEndedIterator<Item = usize>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator
            .next_back()
            .map(|node| (node as isize + self.offset) as usize)
    }
}

impl<I> ExactSizeIterator for Offset<I>
where
    I: ExactSizeIterator<Item = usize>,
{
    fn len(&self) -> usize {
        self.iterator.len()
    }
}

/// Trait implementing the offset method.
pub trait Offsettable {
    /// Returns an offsetted iterator.
    fn offset(self, offset: isize) -> Offset<Self>
    where
        Self: Sized;
}

impl<I> Offsettable for I
where
    I: Iterator<Item = usize>,
{
    fn offset(self, offset: isize) -> Offset<Self> {
        Offset::new(offset, self)
    }
}
