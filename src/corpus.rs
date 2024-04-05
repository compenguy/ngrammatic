//! Submodule providing the Corpus data structure.
use std::{
    collections::{BTreeSet, HashMap},
    iter::Take,
};

use sux::prelude::*;

// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

use crate::{traits::*, AdaptativeVector};

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// Rasterized corpus.
///
/// # Implementation details
/// This corpus is represented as a sparse graph, using a CSR format. The
/// links between keys and grams are weighted by the number of times a given
/// gram appears in a given key: we call this vector the `cooccurrences`.
///
pub struct Corpus<
    KS: Keys<NG>,
    NG: Ngram,
    K: Key<NG, NG::G> + ?Sized = <<KS as Keys<NG>>::K as Key<NG, <NG as Ngram>::G>>::Ref,
> {
    /// Vector of unique keys in the corpus.
    keys: KS,
    /// Vector of unique ngrams in the corpus.
    ngrams: Vec<NG>,
    /// Vector containing the number of times a given gram appears in a given key.
    /// This is a descriptor of an edge from a Key to a Gram.
    cooccurrences: BitFieldVec,
    /// Vector containing the comulative outbound degree from a given key to grams.
    /// This is a vector with the same length as the keys vector PLUS ONE, and the value at
    /// index `i` is the sum of the oubound degrees before index `i`. The last element of this
    /// vector is the total number of edges in the bipartite graph from keys to grams.
    /// We use this vector alongside the `cooccurrences` vector to find the weighted edges
    /// of a given key. The destinations, i.e. the grams, are found in the `grams` vector.
    key_offsets: BitFieldVec,
    /// Vector contain the comulative inbound degree from a given gram to keys.
    /// This is a vector with the same length as the grams vector PLUS ONE, and the value at
    /// index `i` is the sum of the inbound degrees before index `i`. The last element of this
    /// vector is the total number of edges in the bipartite graph from grams to keys.
    /// These edges are NOT weighted, as the weights are stored in the `cooccurrences` vector and
    /// solely refer to the edges from keys to grams.
    ngram_offsets: BitFieldVec,
    /// Vector containing the destinations of the edges from keys to grams.
    key_to_ngram_edges: BitFieldVec,
    /// Vector containing the sources of the edges from grams to keys.
    gram_to_key_edges: BitFieldVec,
    /// Phantom type to store the type of the keys.
    _phantom: std::marker::PhantomData<K>,
}

impl<KS, NG, K> From<KS> for Corpus<KS, NG, K>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized
{
    fn from(keys: KS) -> Self {
        // Sorted vector of ngrams.
        let mut ngrams = BTreeSet::new();
        let mut cooccurrences = AdaptativeVector::with_capacity(keys.len());
        let mut maximal_cooccurrence: usize = 0;
        let mut key_offsets = AdaptativeVector::with_capacity(keys.len());
        key_offsets.push(0_u8);
        let mut key_to_ngrams: Vec<NG> = Vec::with_capacity(keys.len());

        for key in keys.iter() {
            // First, we get the reference to the inner key.
            let key: &K = key.as_ref();

            // We create a hashmap to store the ngrams of the key and their counts.
            let ngram_counts: HashMap<NG, usize> = key.counts();

            // Before digesting the hashmap, we convert it to a vector of tuples and we sort if
            // by ngram. This is done so that when we remap the ngrams to the overall sorted array,
            // we can also update the key to gram edges vector inplace without having to sort every
            // set of ngrams associated to a document as we are sure that, once replaced, any ngram
            // will already be in an ordering that is consistent with the overall ordering of ngrams.
            // This way we do not need to sort things such as the associated co-occurrences.
            let mut ngram_counts: Vec<(NG, usize)> = ngram_counts.into_iter().collect();

            // We sort the ngrams by ngram.
            ngram_counts.sort_unstable_by(|(ngram_a, _), (ngram_b, _)| ngram_a.cmp(ngram_b));

            // Then, we digest the sorted array of tuples.
            for (ngram, count) in ngram_counts {
                // We insert the ngram in the sorted btreeset.
                ngrams.insert(ngram);
                // We store the count of the ngram in the current key in the cooccurrences vector.
                cooccurrences.push(count);
                // We save the maximal co-occurrence.
                if count > maximal_cooccurrence {
                    maximal_cooccurrence = count;
                }
                // And finally we store the index of the ngram in the key_to_ngrams vector.
                key_to_ngrams.push(ngram);
            }
            // We store the number of edges from the current key in the key_offsets vector.
            key_offsets.push(cooccurrences.len());
        }

        // We can now start to compress several of the vectors into BitFieldVecs.
        let key_offsets = key_offsets.into_bitvec(cooccurrences.len());
        let cooccurrences = cooccurrences.into_bitvec(maximal_cooccurrence);

        // We create the ngram_offsets vector. Since we are using a btreeset, we already have the
        // ngrams sorted, so we can simply convert the btreeset into a vector.
        let ngrams: Vec<NG> = ngrams.into_iter().collect();

        // We now create the various required bitvectors, knowing all of their characteristics
        // such as the capacity and the largest value to fit in the bitvector, i.e. the number
        // of bits necessary to store the largest value in the vector.

        // We start by creating the ngram_offsets vector. This vector has as length the number of
        // ngrams plus one, and the value at index `i` is the sum of the inbound degrees before
        // index `i`. The last element of this vector is the total number of edges in the bipartite
        // graph from grams to keys, i.e. the total number of edges in the corpus. This value is also
        // the largest value contained in the vector.
        let mut ngram_offsets = BitFieldVec::new(
            cooccurrences.len().next_power_of_two().ilog2() as usize,
            ngrams.len() + 1,
        );

        // While populating the previous two vectors, we also populate the key_to_ngram_edges.
        // As it stands, this value is populated by the ngrams in the order they appear in the keys. We need
        // to replace these ngrams with their curresponding index, which means that we need to allocate a new
        // vector of the same length as the current key_to_ngram_edges vector, and as maximum value the number
        // of ngrams in the corpus.
        let mut key_to_ngram_edges = BitFieldVec::new(
            ngrams.len().next_power_of_two().ilog2() as usize,
            key_to_ngrams.len(),
        );

        // We iterate on the key_to_ngrams vector. For each ngram we encounter, we find the index of the ngram
        // in the ngram vector by employing a binary search, since we know that the ngrams are sorted.
        for (edge_id, ngram) in key_to_ngrams.into_iter().enumerate() {
            // We find the index of the ngram in the ngrams vector.
            // We can always unwrap since we know that the ngram is in the ngrams vector.
            let ngram_index = ngrams.binary_search(&ngram).unwrap();
            // We store the index in the key_to_ngram_edges vector.
            unsafe { key_to_ngram_edges.set_unchecked(edge_id, ngram_index) };
            // We increment the inbound degree of the ngram.
            unsafe {
                ngram_offsets.set_unchecked(
                    ngram_index + 1,
                    ngram_offsets.get_unchecked(ngram_index + 1) + 1,
                )
            }
        }

        // Now that we have fully populated the ngram_offsets vector, we need to compute the comulative
        // sum of the inbound degrees of the ngrams.
        let mut comulative_sum = 0;
        for i in 0..ngram_offsets.len() {
            unsafe {
                comulative_sum += ngram_offsets.get_unchecked(i);
            }
            unsafe {
                ngram_offsets.set_unchecked(i, comulative_sum);
            }
        }

        // Finally, we can allocate and populate the gram_to_key_edges vector. This vector has the same length
        // as the cooccurrences vector.
        let mut gram_to_key_edges = BitFieldVec::new(
            keys.len().next_power_of_two().ilog2() as usize,
            cooccurrences.len(),
        );

        let mut ngram_degrees = BitFieldVec::new(
            cooccurrences.len().next_power_of_two().ilog2() as usize,
            ngrams.len() + 1,
        );

        // We iterate on the key_to_ngram_edges while keeping track of the current key, as defined by the key_offsets.
        // For each ngram, by using the ngram_offsets, we can find the position of the key in the gram_to_key_edges vector.

        let mut ngram_iterator = key_to_ngram_edges.into_iter_from(0);

        for (key_id, (key_offset_start, key_offset_end)) in key_offsets
            .into_iter_from(0)
            .zip(key_offsets.into_iter_from(1))
            .enumerate()
        {
            // We iterate on the ngrams of the key.
            for _ in key_offset_start..key_offset_end {
                // We find the ngram index. We know we can always unwrap since the length of the
                // key_to_ngram_edges vector is the same as the maximal offset in the key_offsets vector.
                let ngram_id = ngram_iterator.next().unwrap();
                // We get the ngram current degree.
                let ngram_degree: usize = unsafe { ngram_degrees.get_unchecked(ngram_id) };

                // We find the position of the key in the gram_to_key_edges vector.
                let inbound_edge_id =
                    unsafe { ngram_offsets.get_unchecked(ngram_id) } + ngram_degree;

                // // We store the key index in the gram_to_key_edges vector.
                unsafe { gram_to_key_edges.set_unchecked(inbound_edge_id, key_id) };
                // // We increment the inbound degree of the key.
                unsafe { ngram_degrees.set_unchecked(ngram_id, ngram_degree + 1) };
            }
        }

        Corpus {
            keys,
            ngrams,
            cooccurrences,
            key_offsets,
            ngram_offsets,
            key_to_ngram_edges,
            gram_to_key_edges,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<KS, NG, K> Corpus<KS, NG, K>
where
    NG: Ngram,
    KS: Keys<NG>,
    K: Key<NG, NG::G> + ?Sized,
{
    #[inline(always)]
    /// Returns the number of keys in the corpus.
    pub fn number_of_keys(&self) -> usize {
        self.keys.len()
    }

    #[inline(always)]
    /// Returns the number of ngrams in the corpus.
    pub fn number_of_ngrams(&self) -> usize {
        self.ngrams.len()
    }

    #[inline(always)]
    /// Returns the number of edges in the corpus.
    pub fn number_of_edges(&self) -> usize {
        self.cooccurrences.len()
    }

    #[inline(always)]
    /// Returns a reference to the key at a given key id.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get.
    ///
    pub fn key_from_id(&self, key_id: usize) -> &KS::K {
        &self.keys[key_id]
    }

    #[inline(always)]
    /// Returns the ngram curresponding to a given ngram id.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get.
    ///
    pub fn ngram_from_id(&self, ngram_id: usize) -> NG {
        self.ngrams[ngram_id]
    }

    #[inline(always)]
    /// Returns the ngram id curresponding to a given ngram,
    /// if it exists in the corpus. If it does not exist, the
    /// function returns the index where the ngram should be
    /// inserted to keep the ngrams sorted.
    ///
    /// # Arguments
    /// * `ngram` - The ngram to get the id from.
    pub fn ngram_id_from_ngram(&self, ngram: &NG) -> Result<usize, usize> {
        self.ngrams.binary_search(ngram)
    }

    #[inline(always)]
    /// Returns the number of ngrams from a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the number of ngrams from.
    pub fn number_of_ngrams_from_key_id(&self, key_id: usize) -> usize {
        let key_offset_start = self.key_offsets.get(key_id);
        let key_offset_end = self.key_offsets.get(key_id + 1);
        key_offset_end - key_offset_start
    }

    #[inline(always)]
    /// Returns the number of keys from a given ngram.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get the number of keys from.
    pub fn number_of_keys_from_ngram_id(&self, ngram_id: usize) -> usize {
        let ngram_offset_start = self.ngram_offsets.get(ngram_id);
        let ngram_offset_end = self.ngram_offsets.get(ngram_id + 1);
        ngram_offset_end - ngram_offset_start
    }

    #[inline(always)]
    /// Returns the key ids associated to a given ngram.
    ///
    /// # Arguments
    /// * `ngram_id` - The id of the ngram to get the key ids from.
    ///
    pub fn key_ids_from_ngram_id(
        &self,
        ngram_id: usize,
    ) -> Take<BitFieldVecIterator<'_, usize, Vec<usize>>> {
        let ngram_offset_start = self.ngram_offsets.get(ngram_id);
        let ngram_offset_end = self.ngram_offsets.get(ngram_id + 1);
        self.gram_to_key_edges
            .into_iter_from(ngram_offset_start)
            .take(ngram_offset_end - ngram_offset_start)
    }

    #[inline(always)]
    /// Returns the ngram ids associated to a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngram ids from.
    pub fn ngram_ids_from_key(
        &self,
        key_id: usize,
    ) -> Take<BitFieldVecIterator<'_, usize, Vec<usize>>> {
        let key_offset_start = self.key_offsets.get(key_id);
        let key_offset_end = self.key_offsets.get(key_id + 1);
        self.key_to_ngram_edges
            .into_iter_from(key_offset_start)
            .take(key_offset_end - key_offset_start)
    }

    #[inline(always)]
    /// Returns the ngram co-oocurrences of a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngram co-occurrences from.
    pub fn ngram_cooccurrences_from_key(
        &self,
        key_id: usize,
    ) -> Take<BitFieldVecIterator<'_, usize, Vec<usize>>> {
        let key_offset_start = self.key_offsets.get(key_id);
        let key_offset_end = self.key_offsets.get(key_id + 1);
        self.cooccurrences
            .into_iter_from(key_offset_start)
            .take(key_offset_end - key_offset_start)
    }

    #[inline(always)]
    /// Returns the ngrams ids and their co-occurrences in a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngrams and their co-occurrences from.
    ///
    pub fn ngram_ids_and_cooccurrences_from_key(
        &self,
        key_id: usize,
    ) -> impl ExactSizeIterator<Item = (usize, usize)> + '_ {
        let key_offset_start = self.key_offsets.get(key_id);
        let key_offset_end = self.key_offsets.get(key_id + 1);
        self.cooccurrences
            .into_iter_from(key_offset_start)
            .zip(self.key_to_ngram_edges.into_iter_from(key_offset_start))
            .take(key_offset_end - key_offset_start)
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
            .map(move |(ngram_id, cooccurrence)| (self.ngrams[ngram_id], cooccurrence))
    }

    #[inline(always)]
    /// Returns the ngrams associated to a given key.
    ///
    /// # Arguments
    /// * `key_id` - The id of the key to get the ngrams from.
    pub fn ngrams_from_key(&self, key_id: usize) -> impl ExactSizeIterator<Item = NG> + '_ {
        self.ngram_ids_from_key(key_id)
            .map(move |ngram_id| self.ngrams[ngram_id])
    }
}
