//! Submodule providing the Corpus data structure.
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};

use std::{cmp::Reverse, iter::Map};

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

use crate::{bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph, traits::*};

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Rasterized corpus.
///
/// # Implementation details
/// This corpus is represented as a sparse graph, using a CSR format. The
/// links between keys and grams are weighted by the number of times a given
/// gram appears in a given key: we call this vector the `cooccurrences`.
pub struct Corpus<
    KS: Keys<NG>,
    NG: Ngram,
    K: Key<NG, NG::G> + ?Sized = <<KS as Keys<NG>>::K as Key<NG, <NG as Ngram>::G>>::Ref,
    G: WeightedBipartiteGraph = WeightedBitFieldBipartiteGraph,
> {
    /// Vector of unique keys in the corpus.
    pub(crate) keys: KS,
    /// Vector of unique ngrams in the corpus.
    pub(crate) ngrams: NG::SortedStorage,
    /// Graph describing the weighted bipapartite graph from keys to grams.
    pub(crate) graph: G,
    /// Average key length.
    pub(crate) average_key_length: f64,
    /// Phantom type to store the type of the keys.
    _phantom: std::marker::PhantomData<K>,
}

impl<KS, NG, K, G> AsRef<G> for Corpus<KS, NG, K, G>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph,
{
    fn as_ref(&self) -> &G {
        &self.graph
    }
}

impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph,
{
    /// Creates a new corpus from a set of keys, a set of ngrams and a weighted bipartite graph.
    ///
    /// # Arguments
    /// * `keys` - The keys of the corpus.
    /// * `ngrams` - The ngrams of the corpus.
    /// * `average_key_length` - The average key length.
    /// * `graph` - The weighted bipartite graph.
    pub(crate) fn new(
        keys: KS,
        ngrams: NG::SortedStorage,
        average_key_length: f64,
        graph: G,
    ) -> Self {
        Corpus {
            keys,
            ngrams,
            graph,
            average_key_length: average_key_length.max(1.0),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns a reference to underlying graph.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let animals: Corpus<_, TriGram<ASCIIChar>> = Corpus::from(ANIMALS);
    /// 
    /// let graph = animals.graph();
    /// ```
    pub fn graph(&self) -> &G {
        &self.graph
    }
}

/// Iterator over the ngram ids and their co-occurrences.
pub type NgramIdsAndCooccurrences<'a, G> = std::iter::Zip<
    <G as WeightedBipartiteGraph>::Dsts<'a>,
    Map<<G as WeightedBipartiteGraph>::WeightsSrc<'a>, fn(usize) -> usize>,
>;

impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram,
    KS: Keys<NG>,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph,
{
    #[inline(always)]
    /// Returns the number of keys in the corpus.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let animals: Corpus<_, TriGram<ASCIIChar>> = Corpus::from(ANIMALS);
    /// 
    /// let number_of_keys = animals.number_of_keys();
    /// 
    /// assert_eq!(number_of_keys, 699);
    /// ```
    pub fn number_of_keys(&self) -> usize {
        self.keys.len()
    }

    #[inline(always)]
    /// Returns the number of ngrams in the corpus.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let animals: Corpus<_, TriGram<ASCIIChar>> = Corpus::from(ANIMALS);
    /// 
    /// let number_of_ngrams = animals.number_of_ngrams();
    /// 
    /// assert_eq!(number_of_ngrams, 1897);
    /// ```
    pub fn number_of_ngrams(&self) -> usize {
        self.ngrams.len()
    }

    #[inline(always)]
    /// Returns a reference to the key at a given key id.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let animals: Corpus<_, TriGram<ASCIIChar>> = Corpus::from(ANIMALS);
    /// 
    /// assert_eq!(animals.key_from_id(0), &"Aardvark");
    /// assert_eq!(animals.key_from_id(1), &"Abyssinian");
    /// assert_eq!(animals.key_from_id(20), &"Alligator");
    /// ```
    pub fn key_from_id(&self, key_id: usize) -> &KS::K {
        &self.keys[key_id]
    }

    #[inline(always)]
    /// Returns the ngram curresponding to a given ngram id.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ngrammatic::prelude::*;
    /// 
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    /// 
    /// assert_eq!(animals.ngram_from_id(0), ['\0', '\0', '\0']);
    /// assert_eq!(animals.ngram_from_id(1), ['\0', '\0', 'R']);
    /// assert_eq!(animals.ngram_from_id(20),['\0', '\0', 't']);
    /// ```
    pub fn ngram_from_id(&self, ngram_id: usize) -> NG {
        unsafe { self.ngrams.get_unchecked(ngram_id) }
    }

    #[inline(always)]
    /// Returns the ngram id curresponding to a given ngram,
    /// if it exists in the corpus.
    ///
    /// # Arguments
    /// * `ngram` - The ngram to get the id from.
    pub fn ngram_id_from_ngram(&self, ngram: NG) -> Option<usize> {
        self.ngrams.index_of(ngram)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the number of ngrams from.
    pub fn number_of_ngrams_from_key_id(&self, key_id: usize) -> usize {
        self.graph.src_degree(key_id)
    }

    #[inline(always)]
    /// Returns the number of keys from a given ngram.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get the number of keys from.
    pub fn number_of_keys_from_ngram_id(&self, ngram_id: usize) -> usize {
        self.graph.dst_degree(ngram_id)
    }

    #[inline(always)]
    /// Returns the key ids associated to a given ngram.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get the key ids from.
    pub fn key_ids_from_ngram_id(&self, ngram_id: usize) -> G::Srcs<'_> {
        self.graph.srcs_from_dst(ngram_id)
    }

    #[inline(always)]
    /// Returns the ngram ids associated to a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngram ids from.
    pub fn ngram_ids_from_key(&self, key_id: usize) -> G::Dsts<'_> {
        self.graph.dsts_from_src(key_id)
    }

    #[inline(always)]
    /// Returns the ngram co-oocurrences of a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngram co-occurrences from.
    pub fn ngram_cooccurrences_from_key(
        &self,
        key_id: usize,
    ) -> Map<G::WeightsSrc<'_>, fn(usize) -> usize> {
        self.graph.weights_from_src(key_id).map(|x| x + 1)
    }

    #[inline(always)]
    /// Returns all co-occurrences.
    pub fn cooccurrences(&self) -> Map<G::Weights<'_>, fn(usize) -> usize> {
        self.graph.weights().map(|x| x + 1)
    }

    #[inline(always)]
    /// Returns the ngrams ids and their co-occurrences in a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngrams and their co-occurrences from.
    pub fn ngram_ids_and_cooccurrences_from_key(
        &self,
        key_id: usize,
    ) -> NgramIdsAndCooccurrences<'_, G> {
        self.ngram_ids_from_key(key_id)
            .zip(self.ngram_cooccurrences_from_key(key_id))
    }

    #[inline(always)]
    /// Returns the ngrams and their co-occurrences in a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngrams and their co-occurrences from.
    pub fn ngrams_and_cooccurrences_from_key(
        &self,
        key_id: usize,
    ) -> impl ExactSizeIterator<Item = (NG, usize)> + '_ {
        self.ngram_ids_and_cooccurrences_from_key(key_id)
            .map(move |(ngram_id, cooccurrence)| (self.ngram_from_id(ngram_id), cooccurrence))
    }

    #[inline(always)]
    /// Returns the ngrams associated to a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngrams from.
    pub fn ngrams_from_key(&self, key_id: usize) -> impl ExactSizeIterator<Item = NG> + '_ {
        self.ngram_ids_from_key(key_id)
            .map(move |ngram_id| self.ngram_from_id(ngram_id))
    }

    #[inline(always)]
    /// Returns the keys associated to a given ngram id.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get the keys from.
    ///
    /// # Returns
    /// An iterator over the keys associated to the ngram.
    pub fn keys_from_ngram_id(
        &self,
        ngram_id: usize,
    ) -> impl ExactSizeIterator<Item = &KS::K> + '_ {
        self.key_ids_from_ngram_id(ngram_id)
            .map(move |key_id| self.key_from_id(key_id))
    }

    #[inline(always)]
    /// Returns the number of keys associated to a given ngram.
    ///
    /// # Arguments
    /// * `ngram` - The ngram to get the number of keys from.
    pub fn number_of_keys_from_ngram(&self, ngram: NG) -> Option<usize> {
        self.ngram_id_from_ngram(ngram)
            .map(|ngram_id| self.number_of_keys_from_ngram_id(ngram_id))
    }

    #[inline(always)]
    /// Returns the keys associated to a given ngram.
    ///
    /// # Arguments
    /// * `ngram` - The ngram to get the keys from.
    ///
    /// # Returns
    /// An iterator over the keys associated to the ngram.
    pub fn keys_from_ngram(&self, ngram: NG) -> Option<impl ExactSizeIterator<Item = &KS::K> + '_> {
        self.ngram_id_from_ngram(ngram)
            .map(move |ngram_id| self.keys_from_ngram_id(ngram_id))
    }

    #[inline(always)]
    /// Returns the top k most common ngrams in the corpus.
    ///
    /// # Arguments
    /// * `k` - The number of ngrams to return.
    ///
    /// # Implementative details
    /// This function is implemented using a Binary Heap.
    pub fn top_k_ngrams(&self, k: usize) -> Vec<(usize, NG)> {
        let mut heap = std::collections::BinaryHeap::with_capacity(k);
        for (degree, ngram) in self
            .graph
            .degrees()
            .skip(self.number_of_keys())
            .zip(self.ngrams.iter())
        {
            if heap.len() < k {
                heap.push(Reverse((degree, ngram)));
            } else if heap.peek().unwrap().0 .0 < degree {
                heap.pop();
                heap.push(Reverse((degree, ngram)));
            }
        }
        heap.into_sorted_vec()
            .into_iter()
            .map(|Reverse(x)| x)
            .collect()
    }
}
