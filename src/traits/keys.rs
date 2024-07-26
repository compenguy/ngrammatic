//! Submodule defining trait for a container of Keys.

use crate::{Key, Ngram};
use sux::dict::rear_coded_list::RearCodedList;
use sux::traits::IndexedSeq;
use sux::dict::rear_coded_list::Iter as ValueIterator;

/// Trait defining a container of keys.
pub trait Keys<NG: Ngram> {
    /// The type of the key.
    type K: Key<NG, <NG as Ngram>::G>;
    /// The type of the reference of the key, if available.
    /// If not, just ignore the lifetime provided.
    type KeyRef<'a>: Clone + Key<NG, <NG as Ngram>::G>
    where
        Self: 'a;
    /// The iterator to iter the keys.
    type IterKeys<'a>: Iterator<Item = Self::KeyRef<'a>>
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
    fn get_ref(&self, index: usize) -> Self::KeyRef<'_>;

    /// Returns an iterator over the keys.
    fn iter(&self) -> Self::IterKeys<'_>;
}

impl<NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for Vec<K> {
    type K = K;
    type KeyRef<'a> = &'a K where K: 'a, Self: 'a;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        self.len()
    }

    fn get_ref(&self, index: usize) -> Self::KeyRef<'_> {
        &self[index]
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}

impl<const N: usize, NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for [K; N] {
    type K = K;
    type KeyRef<'a> = &'a K where K: 'a, Self: 'a;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        <[K]>::len(self)
    }

    fn get_ref(&self, index: usize) -> Self::KeyRef<'_> {
        &self[index]
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}

impl<NG: Ngram, K: Key<NG, NG::G>> Keys<NG> for [K] {
    type K = K;
    type KeyRef<'a> = &'a K where K: 'a, Self: 'a;
    type IterKeys<'a> = std::slice::Iter<'a, K> where K: 'a, Self: 'a;

    fn len(&self) -> usize {
        self.len()
    }

    fn get_ref(&self, index: usize) -> Self::KeyRef<'_> {
        &self[index]
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <[K]>::iter(self)
    }
}

impl<R, NG: Ngram> Keys<NG> for &R
where
    R: Keys<NG> + ?Sized,
{
    type K = R::K;
    type KeyRef<'a> = R::KeyRef<'a> where Self: 'a;
    type IterKeys<'a> = R::IterKeys<'a> where Self: 'a, Self::K: 'a;

    fn len(&self) -> usize {
        <R>::len(self)
    }

    fn get_ref(&self, index: usize) -> Self::KeyRef<'_> {
        <R>::get_ref(self, index)
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        <R>::iter(self)
    }
}

impl<NG: Ngram, D: AsRef<[u8]>, P: AsRef<[usize]>> Keys<NG> for RearCodedList<D, P>
where
    String: Key<NG, <NG as Ngram>::G>,
{
    type K = String;
    type KeyRef<'a> = String where Self: 'a;
    type IterKeys<'a> =  ValueIterator<'a, D, P> where Self: 'a;

    fn len(&self) -> usize {
        <Self as IndexedSeq>::len(self)
    }

    fn get_ref(&self, index: usize) -> Self::KeyRef<'_> {
        <Self as IndexedSeq>::get(self, index)
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        self.iter_from(0)
    }
}

#[cfg(feature = "trie-rs")]
impl<NG: Ngram> Keys<NG> for trie_rs::Trie<u8>
where
    String: Key<NG, <NG as Ngram>::G>,
{
    type K = String;
    type KeyRef<'a> = String where Self: 'a;
    type IterKeys<'a> = trie_rs::iter::Keys<
        trie_rs::iter::PostfixIter<'a, u8, (), String, trie_rs::try_collect::StringCollect>,
    >;

    fn len(&self) -> usize {
        <trie_rs::iter::Keys<
            trie_rs::iter::PostfixIter<'_, u8, (), String, trie_rs::try_collect::StringCollect>,
        > as std::iter::Iterator>::count(trie_rs::Trie::postfix_search(self, []))
    }

    fn get_ref(&self, index: usize) -> Self::KeyRef<'_> {
        trie_rs::Trie::postfix_search(self, []).nth(index).unwrap()
    }

    fn iter(&self) -> Self::IterKeys<'_> {
        trie_rs::Trie::postfix_search(self, [])
    }
}
