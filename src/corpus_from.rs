//! Submodule implementing the `From` trait for the `Corpus` struct.
use std::collections::HashSet;

use fxhash::FxBuildHasher;
use sux::prelude::*;

use crate::{
    bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph, traits::*, AdaptativeVector,
};

use crate::Corpus;

impl<KS, NG, K> Corpus<KS, NG, K, WeightedBitFieldBipartiteGraph>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
{
    pub(crate) fn parse_keys(
        keys: &KS,
    ) -> (Vec<NG>, AdaptativeVector, AdaptativeVector, usize, Vec<NG>) {
        // Sorted vector of ngrams.
        let mut ngrams: HashSet<NG, FxBuildHasher> = HashSet::with_capacity_and_hasher(
            (keys.len() as f32).sqrt() as usize,
            FxBuildHasher::default(),
        );
        let mut cooccurrences = AdaptativeVector::with_capacity(keys.len());
        let mut maximal_cooccurrence: usize = 0;
        let mut key_offsets = AdaptativeVector::with_capacity(keys.len() + 1);
        // TODO: The adaptative vector needs to be allocated in such a way it
        // can contain AT LEAST a value as large as the number of keys.
        key_offsets.push(0_u8);
        let mut key_to_ngrams: Vec<NG> = Vec::with_capacity(keys.len());

        log::info!("Building ngrams from keys.");

        for key in keys.iter() {
            // First, we get the reference to the inner key.
            let key: &K = key.as_ref();

            // We create a hashmap to store the ngrams of the key and their counts.
            let ngram_counts = key.counts();

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
                // We check that the provided count is greater or equal to one.
                assert!(
                    count > 0,
                    "The count of an ngram must be greater than zero."
                );
                // We insert the ngram in the sorted btreeset.
                ngrams.insert(ngram);
                // We store the count of the ngram in the current key in the cooccurrences vector.
                // Since we know that the count is at least one, we can safely subtract one from it
                // and encode all values shifted by one. This way, we save one bit per value.
                cooccurrences.push(count - 1);
                // We save the maximal co-occurrence.
                if count - 1 > maximal_cooccurrence {
                    maximal_cooccurrence = count - 1;
                }
                // And finally we store the index of the ngram in the key_to_ngrams vector.
                key_to_ngrams.push(ngram);
            }
            // We store the number of edges from the current key in the key_offsets vector.
            key_offsets.push(cooccurrences.len());
        }

        assert!(
            !ngrams.is_empty(),
            "The corpus must contain at least one ngram."
        );

        // We convert the ngram set into a vector.
        let ngrams: Vec<NG> = ngrams.into_iter().collect();

        (
            ngrams,
            cooccurrences,
            key_offsets,
            maximal_cooccurrence,
            key_to_ngrams,
        )
    }
}

impl<KS, NG, K> From<KS> for Corpus<KS, NG, K, WeightedBitFieldBipartiteGraph>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
{
    fn from(keys: KS) -> Self {
        // We start by parsing the keys to extract the ngrams, the cooccurrences, the key offsets,
        // and the maximal cooccurrence.
        let (mut ngrams, cooccurrences, key_offsets, maximal_cooccurrence, key_to_ngrams) =
            Self::parse_keys(&keys);

        // We sort the ngrams.
        log::info!("Sorting ngrams.");
        ngrams.sort_unstable();

        // We can now start to compress several of the vectors into BitFieldVecs.
        log::info!("Compressing key offsets into Elias-Fano.");
        let key_offsets = unsafe { key_offsets.into_elias_fano() };
        log::info!("Compressing cooccurrence vector into BitFieldVec.");
        let cooccurrences = cooccurrences.into_bitvec(maximal_cooccurrence);

        // We create the ngrams vector. Since we are using a btreeset, we already have the
        // ngrams sorted, so we can simply convert the btreeset into a vector.
        log::info!(
            "Storing ngrams into {}.",
            std::any::type_name::<NG::SortedStorage>()
        );
        let mut ngram_builder = <<<NG as Ngram>::SortedStorage as SortedNgramStorage<NG>>::Builder>::new_storage_builder(ngrams.len(), *ngrams.last().unwrap());

        for ngram in ngrams {
            unsafe { ngram_builder.push_unchecked(ngram) };
        }

        let ngrams: NG::SortedStorage = ngram_builder.build();

        // We now create the various required bitvectors, knowing all of their characteristics
        // such as the capacity and the largest value to fit in the bitvector, i.e. the number
        // of bits necessary to store the largest value in the vector.

        // We start by creating the ngram_degrees vector. This vector has as length the number of
        // ngrams plus one, and the value at index `i` is the sum of the inbound degrees before
        // index `i`. Since we do not know the actual maximal value of the ngram degrees, we need
        // to use a value that is certainly larger than the maximal value. We can use the number of
        // keys, since at most an ngram will appear in all of the keys. Note that we will not store
        // the ngram comulative degrees in this array, which have their maximal value equal to the
        // number of edges from keys to ngrams, which we already have at this time (i.e. the length
        // of the cooccurrences vector).
        let mut ngram_degrees = BitFieldVec::new(
            keys.len().next_power_of_two().ilog2() as usize,
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

        log::info!("Building the key to ngram edges and computing ngram degrees.");

        // We iterate on the key_to_ngrams vector. For each ngram we encounter, we find the index of the ngram
        // in the ngram vector by employing a binary search, since we know that the ngrams are sorted.
        for (edge_id, ngram) in key_to_ngrams.into_iter().enumerate() {
            // We find the index of the ngram in the ngrams vector.
            // We can always unwrap since we know that the ngram is in the ngrams vector.
            let ngram_index = unsafe { ngrams.index_of_unchecked(ngram) };
            // We store the index in the key_to_ngram_edges vector.
            unsafe { key_to_ngram_edges.set_unchecked(edge_id, ngram_index) };
            // We increment the inbound degree of the ngram.
            unsafe {
                ngram_degrees.set_unchecked(
                    ngram_index + 1,
                    ngram_degrees.get_unchecked(ngram_index + 1) + 1,
                )
            }
        }

        log::info!("Computing ngrams degrees comulative sum.");

        // Now that we have fully populated the ngram_degrees vector, we need to compute the comulative
        // sum of the inbound degrees of the ngrams.
        let mut comulative_sum = 0;
        let mut ngram_offsets_builder =
            EliasFanoBuilder::new(ngram_degrees.len(), cooccurrences.len());

        // We iterate on the ngram_degrees vector, and we compute the comulative sum of the inbound degrees.
        for ngram_degree in ngram_degrees.iter_from(0) {
            debug_assert!(
                ngram_degree > 0,
                "Since all ngrams appear in at least one key, the degree of a ngram should be at least one."
            );
            debug_assert!(
                ngram_degree <= cooccurrences.len(),
                "The degree of a ngram should be less than or equal to the number of keys in the corpus."
            );
            comulative_sum += ngram_degree;
            unsafe { ngram_offsets_builder.push_unchecked(comulative_sum) };
        }

        // We check that the total comulative sum is equal to the number of edges from keys to ngrams.
        debug_assert_eq!(
            comulative_sum,
            cooccurrences.len(),
            "The comulative sum of the ngram degrees should be equal to the number of edges from keys to ngrams."
        );

        // We build the ngram_offsets vector.
        let ngram_offsets = ngram_offsets_builder.build().convert_to().unwrap();

        log::info!("Building edges from gram to key.");
        // Finally, we can allocate and populate the gram_to_key_edges vector. This vector has the same length
        // as the cooccurrences vector.
        let mut gram_to_key_edges = BitFieldVec::new(
            keys.len().next_power_of_two().ilog2() as usize,
            cooccurrences.len(),
        );

        // We reset the degrees to zeroes so that we can reuse the ngram_degrees vector.
        ngram_degrees.reset();

        // We iterate on the key_to_ngram_edges while keeping track of the current key, as defined by the key_offsets.
        // For each ngram, by using the ngram_degrees, we can find the position of the key in the gram_to_key_edges vector.

        let mut ngram_iterator = key_to_ngram_edges.iter();

        for (key_id, (key_offset_start, key_offset_end)) in key_offsets
            .into_iter_from(0)
            .zip(key_offsets.into_iter_from(1))
            .enumerate()
        {
            // Note that we check for the key offsets to be increasing or equal as
            // it may happen that a key does not contain any ngrams. This could be
            // caused for instance by key containing exclusively invalid characters
            // such as whitespaces or NULL characters.
            debug_assert!(
                key_offset_end >= key_offset_start,
                "The key offsets should be increasing."
            );
            // We iterate on the ngrams of the key.
            for _ in key_offset_start..key_offset_end {
                // We find the ngram index. We know we can always unwrap since the length of the
                // key_to_ngram_edges vector is the same as the maximal offset in the key_offsets vector.
                let ngram_id = ngram_iterator.next().unwrap();
                // We get the ngram current degree.
                let ngram_degree: usize = unsafe { ngram_degrees.get_unchecked(ngram_id) };

                // We find the position of the key in the gram_to_key_edges vector.
                let ngram_offset =
                    unsafe { sux::traits::IndexedDict::get_unchecked(&ngram_offsets, ngram_id) };
                let inbound_edge_id = ngram_offset + ngram_degree;

                // We store the key index in the gram_to_key_edges vector.
                unsafe { gram_to_key_edges.set_unchecked(inbound_edge_id, key_id) };
                //We increment the inbound degree of the key.
                unsafe { ngram_degrees.set_unchecked(ngram_id, ngram_degree + 1) };
            }
        }

        Corpus::new(
            keys,
            ngrams,
            WeightedBitFieldBipartiteGraph::new(
                cooccurrences,
                key_offsets,
                ngram_offsets,
                gram_to_key_edges,
                key_to_ngram_edges,
            ),
        )
    }
}
