//! Submodule providing the Corpus data structure.
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};

use std::{cmp::Reverse, iter::Map};

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

use crate::{bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph, traits::*};

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

impl<KS, NG, K, G> Clone for Corpus<KS, NG, K, G>
where
    KS: Keys<NG> + Clone,
    NG: Ngram + Clone,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph + Clone,
{
    fn clone(&self) -> Self {
        Corpus {
            keys: self.keys.clone(),
            ngrams: self.ngrams.clone(),
            graph: self.graph.clone(),
            average_key_length: self.average_key_length,
            _phantom: std::marker::PhantomData,
        }
    }
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
    /// assert_eq!(number_of_ngrams, 2530);
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
    /// assert_eq!(animals.key_from_id(0), "Aardvark");
    /// assert_eq!(animals.key_from_id(1), "Abyssinian");
    /// assert_eq!(animals.key_from_id(20), "Alligator");
    /// ```
    pub fn key_from_id(
        &self,
        key_id: usize,
    ) -> &<<KS as keys::Keys<NG>>::K as key::Key<NG, <NG as gram::Ngram>::G>>::Ref {
        self.keys.get_ref(key_id)
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
    /// assert_eq!(animals.ngram_from_id(0), ['\0', '\0', 'A']);
    /// assert_eq!(animals.ngram_from_id(1), ['\0', '\0', 'B']);
    /// assert_eq!(animals.ngram_from_id(20), ['\0', '\0', 'U']);
    ///
    /// for ngram_id in 0..animals.number_of_ngrams() {
    ///     let ngram = animals.ngram_from_id(ngram_id);
    ///     let ngram_id_from_ngram = animals.ngram_id_from_ngram(ngram);
    ///     assert_eq!(Some(ngram_id), ngram_id_from_ngram);
    /// }
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.ngram_id_from_ngram(['\0', '\0', 'A']), Some(0));
    /// assert_eq!(animals.ngram_id_from_ngram(['\0', '\0', 'B']), Some(1));
    /// assert_eq!(animals.ngram_id_from_ngram(['\0', '\0', 'U']), Some(20));
    ///
    /// for ngram_id in 0..animals.number_of_ngrams() {
    ///     let ngram = animals.ngram_from_id(ngram_id);
    ///     let ngram_id_from_ngram = animals.ngram_id_from_ngram(ngram);
    ///     assert_eq!(Some(ngram_id), ngram_id_from_ngram);
    /// }
    /// ```
    pub fn ngram_id_from_ngram(&self, ngram: NG) -> Option<usize> {
        self.ngrams.index_of(ngram)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the number of ngrams from.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.number_of_ngrams_from_key_id(0), 10);
    /// assert_eq!(animals.number_of_ngrams_from_key_id(1), 12);
    /// assert_eq!(animals.number_of_ngrams_from_key_id(20), 11);
    /// ```
    pub fn number_of_ngrams_from_key_id(&self, key_id: usize) -> usize {
        self.graph.src_degree(key_id)
    }

    #[inline(always)]
    /// Returns the number of keys from a given ngram.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get the number of keys from.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.number_of_keys_from_ngram_id(0), 59);
    /// assert_eq!(animals.number_of_keys_from_ngram_id(1), 78);
    /// assert_eq!(animals.number_of_keys_from_ngram_id(20), 4);
    /// ```
    pub fn number_of_keys_from_ngram_id(&self, ngram_id: usize) -> usize {
        self.graph.dst_degree(ngram_id)
    }

    #[inline(always)]
    /// Returns the key ids associated to a given ngram.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get the key ids from.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.key_ids_from_ngram_id(0).count(), 59);
    /// assert_eq!(animals.key_ids_from_ngram_id(1).count(), 78);
    /// assert_eq!(animals.key_ids_from_ngram_id(20).count(), 4);
    /// ```
    pub fn key_ids_from_ngram_id(&self, ngram_id: usize) -> G::Srcs<'_> {
        self.graph.srcs_from_dst(ngram_id)
    }

    #[inline(always)]
    /// Returns the ngram ids associated to a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngram ids from.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.ngram_ids_from_key(0).count(), 10);
    /// assert_eq!(animals.ngram_ids_from_key(1).count(), 12);
    /// assert_eq!(animals.ngram_ids_from_key(20).count(), 11);
    /// ```
    pub fn ngram_ids_from_key(&self, key_id: usize) -> G::Dsts<'_> {
        self.graph.dsts_from_src(key_id)
    }

    #[inline(always)]
    /// Returns the ngram co-oocurrences of a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngram co-occurrences from.
    ///
    /// # Example
    /// We check that all values are greater than 0.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.ngram_cooccurrences_from_key(0).count(), 10);
    /// assert_eq!(animals.ngram_cooccurrences_from_key(1).count(), 12);
    /// assert_eq!(animals.ngram_cooccurrences_from_key(20).count(), 11);
    ///
    /// assert!(animals.ngram_cooccurrences_from_key(0).all(|x| x > 0));
    /// assert!(animals.ngram_cooccurrences_from_key(1).all(|x| x > 0));
    /// assert!(animals.ngram_cooccurrences_from_key(20).all(|x| x > 0));
    /// ```
    pub fn ngram_cooccurrences_from_key(
        &self,
        key_id: usize,
    ) -> Map<G::WeightsSrc<'_>, fn(usize) -> usize> {
        self.graph.weights_from_src(key_id).map(|x| x + 1)
    }

    #[inline(always)]
    /// Returns all co-occurrences.
    ///
    /// # Example
    /// We check that all values are greater than 0.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.cooccurrences().count(), 9040);
    /// assert!(animals.cooccurrences().all(|x| x > 0));
    /// ```
    pub fn cooccurrences(&self) -> Map<G::Weights<'_>, fn(usize) -> usize> {
        self.graph.weights().map(|x| x + 1)
    }

    #[inline(always)]
    /// Returns the ngrams ids and their co-occurrences in a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngrams and their co-occurrences from.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.ngram_ids_and_cooccurrences_from_key(0).count(), 10);
    /// assert_eq!(animals.ngram_ids_and_cooccurrences_from_key(1).count(), 12);
    /// assert_eq!(animals.ngram_ids_and_cooccurrences_from_key(20).count(), 11);
    /// ```
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
    ///
    /// # Example
    /// We check that all of the ngrams returned appear in the corpus and
    /// that all of the co-occurrences are greater than 0.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.ngrams_and_cooccurrences_from_key(0).count(), 10);
    /// assert_eq!(animals.ngrams_and_cooccurrences_from_key(1).count(), 12);
    /// assert_eq!(animals.ngrams_and_cooccurrences_from_key(20).count(), 11);
    ///
    /// for (ngram, cooccurrence) in animals.ngrams_and_cooccurrences_from_key(0) {
    ///     assert!(animals.ngram_id_from_ngram(ngram).is_some());
    ///     assert!(cooccurrence > 0);
    /// }
    /// for (ngram, cooccurrence) in animals.ngrams_and_cooccurrences_from_key(1) {
    ///     assert!(animals.ngram_id_from_ngram(ngram).is_some());
    ///     assert!(cooccurrence > 0);
    /// }
    ///
    /// for (ngram, cooccurrence) in animals.ngrams_and_cooccurrences_from_key(20) {
    ///     assert!(animals.ngram_id_from_ngram(ngram).is_some());
    ///     assert!(cooccurrence > 0);
    /// }
    /// ```
    pub fn ngrams_and_cooccurrences_from_key(
        &self,
        key_id: usize,
    ) -> impl Iterator<Item = (NG, usize)> + '_ {
        self.ngram_ids_and_cooccurrences_from_key(key_id)
            .map(move |(ngram_id, cooccurrence)| (self.ngram_from_id(ngram_id), cooccurrence))
    }

    #[inline(always)]
    /// Returns the ngrams associated to a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngrams from.
    ///
    /// # Example
    /// We check that all of the ngrams returned appear in the corpus.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(animals.ngrams_from_key_id(0).count(), 10);
    /// assert_eq!(animals.ngrams_from_key_id(1).count(), 12);
    /// assert_eq!(animals.ngrams_from_key_id(20).count(), 11);
    ///
    /// for ngram in animals.ngrams_from_key_id(0) {
    ///     assert!(animals.ngram_id_from_ngram(ngram).is_some());
    /// }
    ///
    /// for ngram in animals.ngrams_from_key_id(1) {
    ///     assert!(animals.ngram_id_from_ngram(ngram).is_some());
    /// }
    ///
    /// for ngram in animals.ngrams_from_key_id(20) {
    ///     assert!(animals.ngram_id_from_ngram(ngram).is_some());
    /// }
    /// ```
    pub fn ngrams_from_key_id(&self, key_id: usize) -> impl ExactSizeIterator<Item = NG> + '_ {
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
    ///
    /// # Example
    /// We check that the keys returned by the keys_from_ngram_id method are the
    /// exactly same keys returned keys_from_ngram method.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// for ngram_id in 0..animals.number_of_ngrams() {
    ///     let ngram = animals.ngram_from_id(ngram_id);
    ///     for (left, right) in animals
    ///         .keys_from_ngram_id(ngram_id)
    ///         .zip(animals.keys_from_ngram(ngram).unwrap())
    ///     {
    ///         assert_eq!(left, right);
    ///     }
    /// }
    /// ```
    pub fn keys_from_ngram_id(
        &self,
        ngram_id: usize,
    ) -> impl ExactSizeIterator<
        Item = &<<KS as keys::Keys<NG>>::K as key::Key<NG, <NG as gram::Ngram>::G>>::Ref,
    > + '_ {
        self.key_ids_from_ngram_id(ngram_id)
            .map(move |key_id| self.key_from_id(key_id))
    }

    #[inline(always)]
    /// Returns the number of keys associated to a given ngram.
    ///
    /// # Arguments
    /// * `ngram` - The ngram to get the number of keys from.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// assert_eq!(
    ///     animals.number_of_keys_from_ngram(['\0', '\0', 'A']),
    ///     Some(59)
    /// );
    /// assert_eq!(
    ///     animals.number_of_keys_from_ngram(['\0', '\0', 'B']),
    ///     Some(78)
    /// );
    /// assert_eq!(
    ///     animals.number_of_keys_from_ngram(['\0', '\0', 'U']),
    ///     Some(4)
    /// );
    /// assert_eq!(
    ///     animals.number_of_keys_from_ngram(['\0', '\0', 'Y']),
    ///     Some(3)
    /// );
    /// assert_eq!(animals.number_of_keys_from_ngram(['X', 'X', 'X']), None);
    ///
    /// for ngram_id in 0..animals.number_of_ngrams() {
    ///     let ngram = animals.ngram_from_id(ngram_id);
    ///     let number_of_keys_from_ngram = animals.number_of_keys_from_ngram(ngram);
    ///     assert_eq!(
    ///         Some(animals.number_of_keys_from_ngram_id(ngram_id)),
    ///         number_of_keys_from_ngram
    ///     );
    ///     let keys_iterator = animals.keys_from_ngram(ngram).unwrap();
    ///     assert_eq!(
    ///         keys_iterator.len(),
    ///         animals.number_of_keys_from_ngram(ngram).unwrap()
    ///     );
    /// }
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// for ngram_id in 0..animals.number_of_ngrams() {
    ///     let ngram = animals.ngram_from_id(ngram_id);
    ///     for (left, right) in animals
    ///         .keys_from_ngram_id(ngram_id)
    ///         .zip(animals.keys_from_ngram(ngram).unwrap())
    ///     {
    ///         assert_eq!(left, right);
    ///     }
    /// }
    /// ```
    pub fn keys_from_ngram(
        &self,
        ngram: NG,
    ) -> Option<
        impl ExactSizeIterator<
                Item = &<<KS as keys::Keys<NG>>::K as key::Key<NG, <NG as gram::Ngram>::G>>::Ref,
            > + '_,
    > {
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    ///
    /// let top_10_ngrams = animals.top_k_ngrams(10);
    ///
    /// assert_eq!(top_10_ngrams.len(), 10);
    ///
    /// for (degree, ngram) in top_10_ngrams {
    ///     assert!(degree > 0);
    ///     assert!(animals.ngram_id_from_ngram(ngram).is_some());
    ///     assert_eq!(animals.number_of_keys_from_ngram(ngram).unwrap(), degree);
    /// }
    /// ```
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
