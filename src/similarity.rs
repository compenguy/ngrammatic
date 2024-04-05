use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

use crate::{corpus::Corpus, traits::*};

impl<K, NG> Corpus<K, NG>
where
    NG: Ngram,
    K: Keys<NG::G>,
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
    pub fn contains_any_ngrams<'a, I>(&self, sorted_ngrams: I, key_id: usize) -> bool
    where
        I: IntoIterator<Item = &'a NG>,
        NG: 'a,
    {
        let mut ngrams_iterator = self.ngrams_from_key(key_id);

        let mut left_next = ngrams_iterator.next();
        let mut sorted_ngrams = sorted_ngrams.into_iter();
        let mut next_ngram = sorted_ngrams.next();

        while let (Some(left_gram), Some(ngram)) = (left_next, next_ngram) {
            match left_gram.cmp(ngram) {
                Ordering::Less => {
                    left_next = ngrams_iterator.next();
                }
                Ordering::Greater => {
                    next_ngram = sorted_ngrams.next();
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
fn number_of_shared_items<I, J, Item>(mut left: I, mut right: J) -> usize
where
    I: Iterator<Item = (Item, usize)>,
    J: Iterator<Item = (Item, usize)>,
    (Item, usize): Eq,
    Item: Ord,
{
    let mut count = 0;
    let mut left_next = left.next();
    let mut right_next = right.next();
    while let (Some((left_gram, left_count)), Some((right_gram, right_count))) =
        (&left_next, &right_next)
    {
        match left_gram.cmp(right_gram) {
            Ordering::Less => {
                left_next = left.next();
            }
            Ordering::Greater => {
                right_next = right.next();
            }
            Ordering::Equal => {
                count += left_count.min(right_count).as_usize();
                left_next = left.next();
                right_next = right.next();
            }
        }
    }
    count
}

#[inline(always)]
/// Calculate the similarity between two iterators of ngrams.
///
/// # Arguments
/// * `warp` - The warp factor to use in the similarity calculation.
/// * `left` - The first iterator of ngrams.
/// * `right` - The second iterator of ngrams.
///
pub(crate) fn similarity<I, J, W, F, NG>(warp: Warp<W>, left: I, right: J) -> F
where
    I: ExactSizeIterator<Item = (NG, usize)>,
    J: ExactSizeIterator<Item = (NG, usize)>,
    NG: Ngram,
    F: Float,
    Warp<W>: Similarity + One + Zero + Three + PartialOrd
{
    debug_assert!(
        warp.is_between_one_and_three(),
        "Warp factor must be in the range 1 to 3"
    );

    // This is a shortcut that counts all grams between both ngrams
    // Then subtracts out one instance of the grams that are in common
    let left_number_of_ngrams = left.len();
    let right_number_of_ngrams = right.len();

    let number_of_shared_ngrams = number_of_shared_items(left, right) as f64;
    let number_of_unique_shared_grams = (left_number_of_ngrams + right_number_of_ngrams + 2
        - 2 * NG::ARITY) as f64
        - number_of_shared_ngrams;

    debug_assert!(number_of_unique_shared_grams >= 1.0);

    F::from_f64(if warp.is_one() {
        number_of_shared_ngrams / number_of_unique_shared_grams
    } else {
        let diffgrams = number_of_unique_shared_grams - number_of_shared_ngrams;
        (warp.pow(number_of_unique_shared_grams) - warp.pow(diffgrams))
            / warp.pow(number_of_unique_shared_grams)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Struct representing a warp factor.
pub struct Warp<W> {
    value: W,
}

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
pub trait Similarity {
    /// Calculate the power of a value.
    fn pow(&self, value: f64) -> f64;

    /// Calculate the similarity between two iterators of ngrams.
    fn similarity<I, J, F, NG>(self, left: I, right: J) -> F
    where
        I: ExactSizeIterator<Item = (NG, usize)>,
        J: ExactSizeIterator<Item = (NG, usize)>,
        NG: Ngram,
        F: Float;
}

impl Similarity for Warp<i32> {
    #[inline(always)]
    fn pow(&self, value: f64) -> f64 {
        value.powi(self.value)
    }

    #[inline(always)]
    fn similarity<I, J, F, NG>(self, left: I, right: J) -> F
    where
        I: ExactSizeIterator<Item = (NG, usize)>,
        J: ExactSizeIterator<Item = (NG, usize)>,
        NG: Ngram,
        F: Float,
    {
        similarity(self, left, right)
    }
}

impl Similarity for Warp<f64> {
    #[inline(always)]
    fn pow(&self, value: f64) -> f64 {
        value.powf(self.value)
    }

    #[inline(always)]
    fn similarity<I, J, F, NG>(self, left: I, right: J) -> F
    where
        I: ExactSizeIterator<Item = (NG, usize)>,
        J: ExactSizeIterator<Item = (NG, usize)>,
        NG: Ngram,
        F: Float,
    {
        similarity(self, left, right)
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

        Ok(Warp { value: value as f64})
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
