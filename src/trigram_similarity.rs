use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

use crate::{corpus::Corpus, search::QueryHashmap, traits::*};

impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph,
{
    #[inline(always)]
    /// Returns whether any of the ngrams provided appear in the provided key.
    ///
    /// # Arguments
    /// * `sorted_grams` - An iterator over the sorted ngrams to check for.
    /// * `key_id` - The id of the key to check for ngrams in.
    ///
    /// # Implementative details
    /// Since both iterators of ngrams are sorted, we can iterate over both
    /// iterators at once, comparing the current ngram in each iterator. If
    /// the ngrams are equal, we return `true`. If the ngrams are not equal,
    /// we advance the iterator that has the smaller ngram.
    pub fn contains_any_ngram_ids<I>(&self, mut right_iterator: I, key_id: usize) -> bool
    where
        I: Iterator<Item = usize>,
    {
        let mut left_iterator = self.ngram_ids_from_key(key_id);

        let mut left_next = left_iterator.next();
        let mut right_next = right_iterator.next();

        while let (Some(left), Some(right)) = (left_next, right_next) {
            match left.cmp(&right) {
                Ordering::Less => {
                    left_next = left_iterator.next();
                }
                Ordering::Greater => {
                    right_next = right_iterator.next();
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }

        false
    }
}

/// Returns the number of shared ngrams between two iterators.
///
/// # Arguments
/// * `left` - The first iterator of ngrams.
/// * `right` - The second iterator of ngrams.
fn number_of_shared_items<I, J>(mut left: I, mut right: J) -> (usize, usize)
where
    I: Iterator<Item = (usize, usize)>,
    J: Iterator<Item = (usize, usize)>,
{
    let mut count = 0;
    let mut other_count = 0;
    let mut left_next = left.next();
    let mut right_next = right.next();

    if let Some((_, right_count)) = &right_next {
        other_count += *right_count;
    }

    while let (Some((left_gram, left_count)), Some((right_gram, right_count))) =
        (&left_next, &right_next)
    {
        match left_gram.cmp(right_gram) {
            Ordering::Less => {
                left_next = left.next();
            }
            Ordering::Greater => {
                right_next = right.next();
                if let Some((_, right_count)) = &right_next {
                    other_count += *right_count;
                }
            }
            Ordering::Equal => {
                count += left_count.min(right_count);
                left_next = left.next();
                right_next = right.next();
                if let Some((_, right_count)) = &right_next {
                    other_count += *right_count;
                }
            }
        }
    }

    right.for_each(|(_, count)| other_count += count);

    (count, other_count)
}

/// Test that number_of_shared_items works correctly.
#[cfg(test)]
mod test_number_of_shared_items {
    use super::*;

    #[test]
    fn test_number_of_shared_items() {
        let left = vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)];
        let right = vec![(1, 1), (3, 1), (5, 1), (7, 1), (9, 1)];

        let (count, other_count) = number_of_shared_items(left.into_iter(), right.into_iter());

        assert_eq!(count, 3);
        assert_eq!(other_count, 5);

        let left = vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)];
        let right = vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)];

        let (count, other_count) = number_of_shared_items(left.into_iter(), right.into_iter());

        assert_eq!(count, 5);
        assert_eq!(other_count, 5);

        let left = vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)];
        let right = vec![(6, 1), (7, 1), (8, 1), (9, 1), (10, 1)];

        let (count, other_count) = number_of_shared_items(left.into_iter(), right.into_iter());

        assert_eq!(count, 0);
        assert_eq!(other_count, 5);
    }
}

#[inline(always)]
/// Calculate the similarity between two iterators of ngrams.
///
/// # Arguments
/// * `warp` - The warp value to use in the trigram similarity calculation.
/// Use warp greater than 1.0 to increase the similarity of shorter string pairs.
/// * `query` - The query hashmap.
/// * `ngrams` - The iterator of ngrams.
pub(crate) fn trigram_similarity<I, W, F>(warp: Warp<W>, query: &QueryHashmap, ngrams: I) -> F
where
    I: Iterator<Item = (usize, usize)>,
    F: Float,
    Warp<W>: TrigramSimilarity + One + Zero + Three + PartialOrd,
{
    debug_assert!(
        warp.is_between_one_and_three(),
        "Warp factor must be in the range 1 to 3"
    );

    let (sharegrams, other_count) = number_of_shared_items(query.ngram_ids_and_counts(), ngrams);

    debug_assert!(sharegrams <= query.total_count());
    debug_assert!(sharegrams <= other_count);

    let allgrams = query.total_count() + other_count - sharegrams;

    debug_assert!(allgrams >= 1);
    debug_assert!(
        allgrams >= sharegrams,
        "allgrams: {}, sharegrams: {}",
        allgrams,
        sharegrams
    );

    F::from_f64(if warp.is_one() {
        sharegrams as f64 / allgrams as f64
    } else {
        let exponentiated_allgrams = warp.pow(allgrams as f64);
        (exponentiated_allgrams - warp.pow(allgrams as f64 - sharegrams as f64))
            / exponentiated_allgrams
    })
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Struct representing a warp factor.
pub struct Warp<W> {
    value: W,
}

unsafe impl Send for Warp<i32> {}
unsafe impl Sync for Warp<i32> {}
unsafe impl Send for Warp<f64> {}
unsafe impl Sync for Warp<f64> {}

impl<W: One> One for Warp<W> {
    const ONE: Self = Warp { value: W::ONE };

    fn is_one(&self) -> bool {
        self.value.is_one()
    }
}

impl<W: Zero> Zero for Warp<W> {
    const ZERO: Self = Warp { value: W::ZERO };

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<W: Three> Three for Warp<W> {
    const THREE: Self = Warp { value: W::THREE };
}

impl<W: Display> Display for Warp<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Trait defining the similarity calculation.
pub trait TrigramSimilarity {
    /// Calculate the power of a value.
    fn pow(&self, value: f64) -> f64;

    /// Calculate the similarity between two iterators of ngrams.
    fn trigram_similarity<I, F>(self, query: &QueryHashmap, ngrams: I) -> F
    where
        I: Iterator<Item = (usize, usize)>,
        F: Float;
}

impl TrigramSimilarity for Warp<i32> {
    #[inline(always)]
    fn pow(&self, value: f64) -> f64 {
        value.powi(self.value)
    }

    #[inline(always)]
    fn trigram_similarity<I, F>(self, query: &QueryHashmap, ngrams: I) -> F
    where
        I: Iterator<Item = (usize, usize)>,
        F: Float,
    {
        trigram_similarity(self, query, ngrams)
    }
}

impl TrigramSimilarity for Warp<f64> {
    #[inline(always)]
    fn pow(&self, value: f64) -> f64 {
        value.powf(self.value)
    }

    #[inline(always)]
    fn trigram_similarity<I, F>(self, query: &QueryHashmap, ngrams: I) -> F
    where
        I: Iterator<Item = (usize, usize)>,
        F: Float,
    {
        trigram_similarity(self, query, ngrams)
    }
}

#[cfg(feature = "half")]
impl TryFrom<half::f16> for Warp<f64> {
    type Error = &'static str;

    fn try_from(value: half::f16) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err("NaN is not a valid warp factor");
        }

        if value.is_infinite() {
            return Err("Infinity is not a valid warp factor");
        }

        if value < half::f16::from_f64(1.0) || value > half::f16::from_f64(3.0) {
            return Err("Warp factor must be in the range 1 to 3");
        }

        Ok(Warp {
            value: f64::from(value),
        })
    }
}

#[cfg(feature = "half")]
impl TryFrom<half::bf16> for Warp<f64> {
    type Error = &'static str;

    fn try_from(value: half::bf16) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err("NaN is not a valid warp factor");
        }

        if value.is_infinite() {
            return Err("Infinity is not a valid warp factor");
        }

        if value < half::bf16::from_f64(1.0) || value > half::bf16::from_f64(3.0) {
            return Err("Warp factor must be in the range 1 to 3");
        }

        Ok(Warp {
            value: f64::from(value),
        })
    }
}

impl TryFrom<i32> for Warp<i32> {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            return Err("Warp factor must be in the range 1 to 3");
        }

        Ok(Warp { value })
    }
}

impl TryFrom<f32> for Warp<f64> {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err("NaN is not a valid warp factor");
        }

        if value.is_infinite() {
            return Err("Infinity is not a valid warp factor");
        }

        if !(1.0..=3.0).contains(&value) {
            return Err("Warp factor must be in the range 1 to 3");
        }

        Ok(Warp {
            value: value as f64,
        })
    }
}

impl TryFrom<f64> for Warp<f64> {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err("NaN is not a valid warp factor");
        }

        if value.is_infinite() {
            return Err("Infinity is not a valid warp factor");
        }

        if !(1.0..=3.0).contains(&value) {
            return Err("Warp factor must be in the range 1 to 3");
        }

        Ok(Warp { value })
    }
}
