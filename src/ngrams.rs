use std::{cmp::Ordering, collections::HashMap};

use crate::traits::*;

#[cfg(feature = "mem_dbg")]
use mem_dbg::{MemDbg, MemSize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "mem_dbg", derive(MemSize, MemDbg))]
/// An ngram, holding a text string and a map of ngrams to their counts.
pub struct Ngram<A: Arity = ArityTwo, Counter: UnsignedInteger = usize> {
    /// Vector of the ngrams sorted by the grams.
    grams: Vec<(A::Gram, Counter)>,
}

impl<A: Arity, Counter: UnsignedInteger> From<HashMap<A::Gram, Counter>> for Ngram<A, Counter> {
    /// Create a new `Ngram` from a `HashMap` of grams to their counts.
    fn from(map: HashMap<A::Gram, Counter>) -> Self {
        let mut grams: Vec<(A::Gram, Counter)> = map.into_iter().collect();
        grams.sort_by(|a, b| a.0.cmp(&b.0));
        Ngram { grams }
    }
}

impl<A: Arity, Counter: UnsignedInteger> From<&[u8]> for Ngram<A, Counter> {
    fn from(bytes: &[u8]) -> Self {
        let mut counts: HashMap<A::Gram, Counter> = HashMap::new();
        for window in bytes.windows(A::ARITY) {
            let mut gram = A::Gram::default();
            (0..A::ARITY).for_each(|i| gram[i] = window[i]);
            counts
                .entry(gram)
                .and_modify(|c| *c += Counter::ONE)
                .or_insert(Counter::ONE);
        }
        Ngram::from(counts)
    }
}

impl<A: Arity, Counter> Ngram<A, Counter>
where
    Counter: UnsignedInteger,
{
    /// Returns the number of grams in the `Ngram`.
    fn number_of_grams(&self) -> usize {
        self.grams
            .iter()
            .map(|(_, count)| count.as_usize())
            .sum::<usize>()
    }

    /// Returns a count of grams that are common between this
    /// `Ngram` and the `other` `Ngram`.
    ///
    /// # Arguments
    /// * `other` - The other `Ngram` to compare against
    ///
    /// # Implementative details
    /// This function iterates over the grams in both ngrams, which are both
    /// sorted vectors of grams. It sums the minimum count of each gram in
    /// both ngrams, and returns the total. Since the grams are unique, there
    /// cannot be subsequent grams that are the same, so the function does
    /// not have to support that corner case.
    ///
    /// # Returns
    /// The number of grams that are common between this `Ngram` and the `other`
    fn number_of_shared_grams(&self, other: &Self) -> usize {
        let mut count = 0;
        let mut self_iter = self.grams.iter();
        let mut other_iter = other.grams.iter();
        let mut self_next = self_iter.next();
        let mut other_next = other_iter.next();
        while let (Some((self_gram, self_count)), Some((other_gram, other_count))) =
            (self_next, other_next)
        {
            match self_gram.cmp(other_gram) {
                Ordering::Less => {
                    self_next = self_iter.next();
                }
                Ordering::Greater => {
                    other_next = other_iter.next();
                }
                Ordering::Equal => {
                    count += self_count.min(other_count).as_usize();
                    self_next = self_iter.next();
                    other_next = other_iter.next();
                }
            }
        }
        count
    }

    /// Returns an iterator over the grams in the `Ngram`.
    pub(crate) fn iter_grams(&self) -> impl Iterator<Item = &A::Gram> {
        self.grams.iter().map(|(gram, _)| gram)
    }

    /// Returns whether any of the grams in the provided iterator are in the `Ngram`.
    ///
    /// # Arguments
    /// * `sorted_grams` - An iterator over the sorted grams to check for.
    /// 
    /// # Implementative details
    /// Since both the grams in the `Ngram` and the grams in the provided iterator are sorted,
    /// when either of the vectors is exhausted, the function can return early.
    pub (crate) fn contains_any_grams<I>(&self, mut sorted_grams: I) -> bool
    where
        I: Iterator<Item = A::Gram>,
    {
        let mut self_grams_iterator = self.iter_grams();

        let mut self_next = self_grams_iterator.next();
        let mut other_next = sorted_grams.next();

        while let (Some(self_gram), Some(other_gram)) = (self_next, other_next) {
            match self_gram.cmp(&other_gram) {
                Ordering::Less => {
                    self_next = self_grams_iterator.next();
                }
                Ordering::Greater => {
                    other_next = sorted_grams.next();
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }
       
        false
    }
}

pub trait Similarity<Warp> {
    fn similarity(&self, other: &Self, warp: Warp) -> f32;
}

impl<A: Arity, C: UnsignedInteger> Similarity<f32> for Ngram<A, C> {
    fn similarity(&self, other: &Self, warp: f32) -> f32 {
        assert!(
            !(1.0..=3.0).contains(&warp),
            "Warp factor must be in the range 1.0 to 3.0"
        );

        // This is a shortcut that counts all grams between both ngrams
        // Then subtracts out one instance of the grams that are in common
        let self_length = self.number_of_grams();
        let other_length = other.number_of_grams();

        if self_length < A::ARITY || other_length < A::ARITY {
            return 0.0;
        }

        let number_of_shared_grams = self.number_of_shared_grams(other) as f32;
        let number_of_unique_shared_grams =
            (self_length + other_length + 2 - 2 * A::ARITY) as f32 - number_of_shared_grams;

        if (warp - 1.0).abs() < 0.0000000001 {
            number_of_shared_grams / number_of_unique_shared_grams
        } else {
            let diffgrams = number_of_unique_shared_grams - number_of_shared_grams;
            (number_of_unique_shared_grams.powf(warp) - diffgrams.powf(warp))
                / (number_of_unique_shared_grams.powf(warp))
        }
    }
}

impl<A: Arity, C: UnsignedInteger> Similarity<i32> for Ngram<A, C> {
    fn similarity(&self, other: &Self, warp: i32) -> f32 {
        assert!(
            !(1..=3).contains(&warp),
            "Warp factor must be in the range 1 to 3"
        );

        // This is a shortcut that counts all grams between both ngrams
        // Then subtracts out one instance of the grams that are in common
        let self_length = self.number_of_grams();
        let other_length = other.number_of_grams();

        if self_length < A::ARITY || other_length < A::ARITY {
            return 0.0;
        }

        let number_of_shared_grams = self.number_of_shared_grams(other) as f32;
        let number_of_unique_shared_grams =
            (self_length + other_length + 2 - 2 * A::ARITY) as f32 - number_of_shared_grams;

        if warp == 1 {
            number_of_shared_grams / number_of_unique_shared_grams
        } else {
            let diffgrams = number_of_unique_shared_grams - number_of_shared_grams;
            (number_of_unique_shared_grams.powi(warp) - diffgrams.powi(warp))
                / (number_of_unique_shared_grams.powi(warp))
        }
    }
}
