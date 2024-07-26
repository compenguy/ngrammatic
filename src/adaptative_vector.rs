//! Module providing a vector that adaptatively grows in data type.

use sux::dict::EliasFanoBuilder;

/// Trait defining a bounded type.
pub trait Bounded {
    /// The maximum value of the type.
    const MAX: usize;
}

/// Trait defining a primitive conversion type.
pub trait Convert<T> {
    /// Convert the type to the primitive type.
    fn convert(self) -> T;
}

/// Macro to implement the Upgrade type for all the combinations
/// of a set of given types.
#[macro_export]
macro_rules! impl_convert {
    ($($from:ty => $to:ty),*) => {
        $(impl Convert<$to> for $from {
            fn convert(self) -> $to {
                self as $to
            }
        })*
    };
}

impl_convert!(u8 => u16, u8 => u32, u8 => u64, u16 => u8, u16 => u16, u16 => u32, u16 => u64, u32 => u16, u32 => u32, u32 => u64, u64 => u16, u64 => u32, u64 => u64);

/// Macro to implement the Bounded trait for a given type.
#[macro_export]
macro_rules! impl_bounded {
    ($type:ty) => {
        impl Bounded for $type {
            const MAX: usize = <$type>::MAX as usize;
        }
    };
}

/// Macro to implement the Bounded trait for several types.
#[macro_export]
macro_rules! impl_bounded_types {
    ($($type:ty),*) => {
        $(impl_bounded!($type);)*
    };
}

impl_bounded_types!(u8, u16, u32, u64);

pub(crate) enum AdaptativeVector {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<u64>),
}

impl From<Vec<u8>> for AdaptativeVector {
    fn from(vector: Vec<u8>) -> Self {
        AdaptativeVector::U8(vector)
    }
}

impl From<Vec<u16>> for AdaptativeVector {
    fn from(vector: Vec<u16>) -> Self {
        AdaptativeVector::U16(vector)
    }
}

impl From<Vec<u32>> for AdaptativeVector {
    fn from(vector: Vec<u32>) -> Self {
        AdaptativeVector::U32(vector)
    }
}

impl From<Vec<u64>> for AdaptativeVector {
    fn from(vector: Vec<u64>) -> Self {
        AdaptativeVector::U64(vector)
    }
}

impl AdaptativeVector {
    fn type_max(&self) -> usize {
        match self {
            AdaptativeVector::U8(_) => u8::MAX as usize,
            AdaptativeVector::U16(_) => u16::MAX as usize,
            AdaptativeVector::U32(_) => u32::MAX as usize,
            AdaptativeVector::U64(_) => u64::MAX as usize,
        }
    }

    fn push_upgrade_vector<U>(&mut self, value: U)
    where
        u8: Convert<U>,
        u16: Convert<U>,
        u32: Convert<U>,
        u64: Convert<U>,
        U: Bounded,
        AdaptativeVector: From<Vec<U>>,
    {
        assert!(
            U::MAX > self.type_max(),
            "The new type must be bigger than the old one."
        );

        // We allocate the new vector with the provided capacity.
        let mut new_vector = Vec::with_capacity(self.len() + 1);

        // We swap the vector inplace with the new vector of the bigger type so to
        // be able to consume the old vector in place.
        let old_vector = core::mem::replace(self, AdaptativeVector::from(Vec::new()));

        // We populate the new vector by consuming the old one and upgrading the values.
        match old_vector {
            AdaptativeVector::U8(vector) => {
                new_vector.extend(vector.into_iter().map(u8::convert));
            }
            AdaptativeVector::U16(vector) => {
                new_vector.extend(vector.into_iter().map(u16::convert));
            }
            AdaptativeVector::U32(vector) => {
                new_vector.extend(vector.into_iter().map(u32::convert));
            }
            AdaptativeVector::U64(vector) => {
                new_vector.extend(vector.into_iter().map(u64::convert));
            }
        }

        // We push the new value to the new vector.
        new_vector.push(value);

        // We replace the vector with the new one.
        *self = AdaptativeVector::from(new_vector);
    }

    /// Creates a new adaptative vector.
    ///
    /// # Arguments
    /// * `capacity` - The capacity of the vector.
    /// * `value_type` - The type of the values to store in the vector.
    ///
    /// # Implementation details
    /// By default, the adaptative vector starts with the
    /// smallest possible data type, i.e. `u8`. As soon as
    /// the data type does not fit any of the provided values,
    /// the vector is converted to the next bigger data type.
    pub(crate) fn with_capacity<A>(capacity: usize, value_type: A) -> Self
    where
        A: Into<AdaptativeVectorValue>,
    {
        match AdaptativeVectorValue::smallest(value_type) {
            AdaptativeVectorValue::U8(_) => AdaptativeVector::U8(Vec::with_capacity(capacity)),
            AdaptativeVectorValue::U16(_) => AdaptativeVector::U16(Vec::with_capacity(capacity)),
            AdaptativeVectorValue::U32(_) => AdaptativeVector::U32(Vec::with_capacity(capacity)),
            AdaptativeVectorValue::U64(_) => AdaptativeVector::U64(Vec::with_capacity(capacity)),
        }
    }

    /// Returns the length of the vector.
    pub(crate) fn len(&self) -> usize {
        match self {
            AdaptativeVector::U8(vector) => vector.len(),
            AdaptativeVector::U16(vector) => vector.len(),
            AdaptativeVector::U32(vector) => vector.len(),
            AdaptativeVector::U64(vector) => vector.len(),
        }
    }

    /// Pushes a value to the vector.
    ///
    /// # Arguments
    /// * `value` - The value to push to the vector.
    ///
    /// # Implementation details
    /// The value provided must be an AdaptativeVectorValue.
    /// If the value does not fit the current data type, the
    /// vector is converted to the next bigger data type.
    ///
    /// # Returns
    /// A boolean indicating whether it was necessary to
    /// convert the vector to a bigger data type.
    pub(crate) fn push<A>(&mut self, value: A) -> bool
    where
        A: Into<AdaptativeVectorValue>,
    {
        let value = AdaptativeVectorValue::smallest(value);
        match self {
            AdaptativeVector::U8(vector) => match value {
                AdaptativeVectorValue::U8(value) => {
                    vector.push(value);
                    false
                }
                AdaptativeVectorValue::U16(value) => {
                    self.push_upgrade_vector(value);
                    true
                }
                AdaptativeVectorValue::U32(value) => {
                    self.push_upgrade_vector(value);
                    true
                }
                AdaptativeVectorValue::U64(value) => {
                    self.push_upgrade_vector(value);
                    true
                }
            },
            AdaptativeVector::U16(vector) => match value {
                AdaptativeVectorValue::U8(value) => {
                    vector.push(value as u16);
                    false
                }
                AdaptativeVectorValue::U16(value) => {
                    vector.push(value);
                    false
                }
                AdaptativeVectorValue::U32(value) => {
                    self.push_upgrade_vector(value);
                    true
                }
                AdaptativeVectorValue::U64(value) => {
                    self.push_upgrade_vector(value);
                    true
                }
            },
            AdaptativeVector::U32(vector) => match value {
                AdaptativeVectorValue::U8(value) => {
                    vector.push(value as u32);
                    false
                }
                AdaptativeVectorValue::U16(value) => {
                    vector.push(value as u32);
                    false
                }
                AdaptativeVectorValue::U32(value) => {
                    vector.push(value);
                    false
                }
                AdaptativeVectorValue::U64(value) => {
                    self.push_upgrade_vector(value);
                    true
                }
            },
            AdaptativeVector::U64(vector) => match value {
                AdaptativeVectorValue::U8(value) => {
                    vector.push(value as u64);
                    false
                }
                AdaptativeVectorValue::U16(value) => {
                    vector.push(value as u64);
                    false
                }
                AdaptativeVectorValue::U32(value) => {
                    vector.push(value as u64);
                    false
                }
                AdaptativeVectorValue::U64(value) => {
                    vector.push(value);
                    false
                }
            },
        }
    }

    #[cfg(feature = "rayon")]
    /// Converts the vector into an Elias Fano.
    ///
    /// # Safety
    /// This method assumes that the vector is sorted.
    pub(crate) unsafe fn par_into_elias_fano(self) -> crate::weights::PredEF {
        use rayon::prelude::*;
        use sux::dict::EliasFanoConcurrentBuilder;

        match self {
            AdaptativeVector::U8(vector) => {
                let builder = EliasFanoConcurrentBuilder::new(
                    vector.len(),
                    vector.last().copied().unwrap_or(0) as usize,
                );
                vector
                    .into_par_iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        builder.set(index, value as usize);
                    });
                let elias_fano = builder.build();
                unsafe {
                    elias_fano.map_high_bits(|high_bits| {
                        crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(
                            high_bits,
                        ))
                    })
                }
            }
            AdaptativeVector::U16(vector) => {
                let builder = EliasFanoConcurrentBuilder::new(
                    vector.len(),
                    vector.last().copied().unwrap_or(0) as usize,
                );
                vector
                    .into_par_iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        builder.set(index, value as usize);
                    });
                unsafe {
                    builder.build().map_high_bits(|high_bits| {
                        crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(
                            high_bits,
                        ))
                    })
                }
            }
            AdaptativeVector::U32(vector) => {
                let builder = EliasFanoConcurrentBuilder::new(
                    vector.len(),
                    vector.last().copied().unwrap_or(0) as usize,
                );
                vector
                    .into_par_iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        builder.set(index, value as usize);
                    });
                unsafe {
                    builder.build().map_high_bits(|high_bits| {
                        crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(
                            high_bits,
                        ))
                    })
                }
            }
            AdaptativeVector::U64(vector) => {
                let builder = EliasFanoConcurrentBuilder::new(
                    vector.len(),
                    vector.last().copied().unwrap_or(0) as usize,
                );
                vector
                    .into_par_iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        builder.set(index, value as usize);
                    });
                unsafe {
                    builder.build().map_high_bits(|high_bits| {
                        crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(
                            high_bits,
                        ))
                    })
                }
            }
        }
    }

    /// Converts the vector into an Elias Fano.
    ///
    /// # Safety
    /// This method assumes that the vector is sorted.
    pub(crate) unsafe fn into_elias_fano(self) -> crate::weights::PredEF {
        match self {
            AdaptativeVector::U8(vector) => {
                let mut builder = EliasFanoBuilder::new(
                    vector.len(),
                    vector.last().copied().unwrap_or(0) as usize,
                );
                for value in vector {
                    builder.push_unchecked(value as usize);
                }
                unsafe {
                    builder.build().map_high_bits(|high_bits| {
                        crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(
                            high_bits,
                        ))
                    })
                }
            }
            AdaptativeVector::U16(vector) => {
                let mut builder = EliasFanoBuilder::new(
                    vector.len(),
                    vector.last().copied().unwrap_or(0) as usize,
                );
                for value in vector {
                    builder.push_unchecked(value as usize);
                }
                unsafe {
                    builder.build().map_high_bits(|high_bits| {
                        crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(
                            high_bits,
                        ))
                    })
                }
            }
            AdaptativeVector::U32(vector) => {
                let mut builder = EliasFanoBuilder::new(
                    vector.len(),
                    vector.last().copied().unwrap_or(0) as usize,
                );
                for value in vector {
                    builder.push_unchecked(value as usize);
                }
                unsafe {
                    builder.build().map_high_bits(|high_bits| {
                        crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(
                            high_bits,
                        ))
                    })
                }
            }
            AdaptativeVector::U64(vector) => {
                let mut builder = EliasFanoBuilder::new(
                    vector.len(),
                    vector.last().copied().unwrap_or(0) as usize,
                );
                for value in vector {
                    builder.push_unchecked(value as usize);
                }
                unsafe {
                    builder.build().map_high_bits(|high_bits| {
                        crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(
                            high_bits,
                        ))
                    })
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum AdaptativeVectorValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl AdaptativeVectorValue {
    pub(crate) fn smallest<A>(value: A) -> Self
    where
        A: Into<AdaptativeVectorValue>,
    {
        match value.into() {
            AdaptativeVectorValue::U8(value) => AdaptativeVectorValue::U8(value),
            AdaptativeVectorValue::U16(value) => {
                if value < u8::MAX.into() {
                    AdaptativeVectorValue::U8(value as u8)
                } else {
                    AdaptativeVectorValue::U16(value)
                }
            }
            AdaptativeVectorValue::U32(value) => {
                if value < u8::MAX.into() {
                    AdaptativeVectorValue::U8(value as u8)
                } else if value < u16::MAX.into() {
                    AdaptativeVectorValue::U16(value as u16)
                } else {
                    AdaptativeVectorValue::U32(value)
                }
            }
            AdaptativeVectorValue::U64(value) => {
                if value < u8::MAX.into() {
                    AdaptativeVectorValue::U8(value as u8)
                } else if value < u16::MAX.into() {
                    AdaptativeVectorValue::U16(value as u16)
                } else if value < u32::MAX.into() {
                    AdaptativeVectorValue::U32(value as u32)
                } else {
                    AdaptativeVectorValue::U64(value)
                }
            }
        }
    }
}

impl Default for AdaptativeVectorValue {
    fn default() -> Self {
        AdaptativeVectorValue::U8(0)
    }
}

impl core::ops::Add<AdaptativeVectorValue> for AdaptativeVectorValue {
    type Output = Self;

    /// Adds the value inplace and returns whether it was necessary to convert the value to a bigger data type.
    ///
    /// # Implementative details
    /// Whenever the provided amount is larger than the current data type, we convert the current value into
    /// the next bigger data type. This is done inplace, i.e. the current value is updated to the new data type.
    /// When the two data types are the same, we use an overflowing_add to add the two values. If the addition
    /// overflows, we convert the current value into the next bigger data type.
    ///
    /// # Returns
    /// A boolean indicating whether it was necessary to convert the value to a bigger data type.
    fn add(self, amount: Self) -> Self {
        match self {
            AdaptativeVectorValue::U8(value) => match amount {
                AdaptativeVectorValue::U8(amount) => {
                    let (new_value, overflow) = value.overflowing_add(amount);
                    if overflow {
                        AdaptativeVectorValue::U16(value as u16 + amount as u16)
                    } else {
                        AdaptativeVectorValue::U8(new_value)
                    }
                }
                AdaptativeVectorValue::U16(amount) => {
                    AdaptativeVectorValue::U16(value as u16) + AdaptativeVectorValue::U16(amount)
                }
                AdaptativeVectorValue::U32(amount) => {
                    AdaptativeVectorValue::U32(value as u32) + AdaptativeVectorValue::U32(amount)
                }
                AdaptativeVectorValue::U64(amount) => {
                    AdaptativeVectorValue::U64(value as u64) + AdaptativeVectorValue::U64(amount)
                }
            },
            AdaptativeVectorValue::U16(value) => match amount {
                AdaptativeVectorValue::U8(amount) => {
                    AdaptativeVectorValue::U16(value) + AdaptativeVectorValue::U16(amount as u16)
                }
                AdaptativeVectorValue::U16(amount) => {
                    let (new_value, overflow) = value.overflowing_add(amount);
                    if overflow {
                        AdaptativeVectorValue::U32(value as u32 + amount as u32)
                    } else {
                        AdaptativeVectorValue::U16(new_value)
                    }
                }
                AdaptativeVectorValue::U32(amount) => {
                    AdaptativeVectorValue::U32(value as u32) + AdaptativeVectorValue::U32(amount)
                }
                AdaptativeVectorValue::U64(amount) => {
                    AdaptativeVectorValue::U64(value as u64) + AdaptativeVectorValue::U64(amount)
                }
            },
            AdaptativeVectorValue::U32(value) => match amount {
                AdaptativeVectorValue::U8(amount) => {
                    AdaptativeVectorValue::U32(value) + AdaptativeVectorValue::U32(amount as u32)
                }
                AdaptativeVectorValue::U16(amount) => {
                    AdaptativeVectorValue::U32(value) + AdaptativeVectorValue::U32(amount as u32)
                }
                AdaptativeVectorValue::U32(amount) => {
                    let (new_value, overflow) = value.overflowing_add(amount);
                    if overflow {
                        AdaptativeVectorValue::U64(value as u64 + amount as u64)
                    } else {
                        AdaptativeVectorValue::U32(new_value)
                    }
                }
                AdaptativeVectorValue::U64(amount) => {
                    AdaptativeVectorValue::U64(value as u64) + AdaptativeVectorValue::U64(amount)
                }
            },
            AdaptativeVectorValue::U64(value) => match amount {
                AdaptativeVectorValue::U8(amount) => {
                    AdaptativeVectorValue::U64(value) + AdaptativeVectorValue::U64(amount as u64)
                }
                AdaptativeVectorValue::U16(amount) => {
                    AdaptativeVectorValue::U64(value) + AdaptativeVectorValue::U64(amount as u64)
                }
                AdaptativeVectorValue::U32(amount) => {
                    AdaptativeVectorValue::U64(value) + AdaptativeVectorValue::U64(amount as u64)
                }
                AdaptativeVectorValue::U64(amount) => AdaptativeVectorValue::U64(value + amount),
            },
        }
    }
}

impl core::ops::AddAssign for AdaptativeVectorValue {
    /// Adds the value inplace and returns whether it was necessary to convert the value to a bigger data type.
    ///
    /// # Implementative details
    /// Whenever the provided amount is larger than the current data type, we convert the current value into
    /// the next bigger data type. This is done inplace, i.e. the current value is updated to the new data type.
    /// When the two data types are the same, we use an overflowing_add to add the two values. If the addition
    /// overflows, we convert the current value into the next bigger data type.
    fn add_assign(&mut self, amount: Self) {
        *self = *self + amount;
    }
}

impl From<u8> for AdaptativeVectorValue {
    fn from(value: u8) -> Self {
        AdaptativeVectorValue::U8(value)
    }
}

impl From<u16> for AdaptativeVectorValue {
    fn from(value: u16) -> Self {
        AdaptativeVectorValue::U16(value)
    }
}

impl From<u32> for AdaptativeVectorValue {
    fn from(value: u32) -> Self {
        AdaptativeVectorValue::U32(value)
    }
}

impl From<u64> for AdaptativeVectorValue {
    fn from(value: u64) -> Self {
        AdaptativeVectorValue::U64(value)
    }
}

impl From<usize> for AdaptativeVectorValue {
    fn from(value: usize) -> Self {
        AdaptativeVectorValue::U64(value as u64)
    }
}
