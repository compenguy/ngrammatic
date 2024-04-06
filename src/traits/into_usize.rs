//! Trait to convert objects into usize, occasionally with zero padding on the left side.

use crate::{
    ASCIIChar, BiGram, HeptaGram, HexaGram, MonoGram, OctaGram, PentaGram, TetraGram, TriGram,
};

/// Trait to convert objects into usize, occasionally with zero padding on the left side.
pub trait IntoUsize {
    /// Convert the object into a usize.
    fn into_usize(self) -> usize;

    /// Convert the object from an usize.
    fn from_usize(value: usize) -> Self;
}

impl IntoUsize for u8 {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        value as u8
    }
}

impl IntoUsize for ASCIIChar {
    #[inline(always)]
    fn into_usize(self) -> usize {
        let value: u8 = self.into();
        value.into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        ASCIIChar::from(value as u8)
    }
}

impl IntoUsize for char {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        char::from_u32(value as u32).unwrap()
    }
}

impl IntoUsize for MonoGram<u8> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self[0] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [value as u8]
    }
}

impl IntoUsize for MonoGram<ASCIIChar> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self[0].into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [ASCIIChar::from_usize(value)]
    }
}

impl IntoUsize for MonoGram<char> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self[0] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [char::from_usize(value)]
    }
}

impl IntoUsize for BiGram<u8> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0] as usize) << 8 | self[1] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [(value >> 8) as u8, value as u8]
    }
}

impl IntoUsize for BiGram<ASCIIChar> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0].into_usize() << 8) | self[1].into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            ASCIIChar::from_usize(value >> 8),
            ASCIIChar::from_usize(value),
        ]
    }
}

impl IntoUsize for BiGram<char> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0] as usize) << 32 | self[1] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [char::from_usize(value >> 32), char::from_usize(value)]
    }
}

impl IntoUsize for TriGram<u8> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0] as usize) << 16 | (self[1] as usize) << 8 | self[2] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [(value >> 16) as u8, (value >> 8) as u8, value as u8]
    }
}

impl IntoUsize for TriGram<ASCIIChar> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0].into_usize() << 16) | (self[1].into_usize() << 8) | self[2].into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            ASCIIChar::from_usize(value >> 16),
            ASCIIChar::from_usize(value >> 8),
            ASCIIChar::from_usize(value),
        ]
    }
}

impl IntoUsize for TetraGram<u8> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0] as usize) << 24
            | (self[1] as usize) << 16
            | (self[2] as usize) << 8
            | self[3] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        ]
    }
}

impl IntoUsize for TetraGram<ASCIIChar> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0].into_usize() << 24)
            | (self[1].into_usize() << 16)
            | (self[2].into_usize() << 8)
            | self[3].into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            ASCIIChar::from_usize(value >> 24),
            ASCIIChar::from_usize(value >> 16),
            ASCIIChar::from_usize(value >> 8),
            ASCIIChar::from_usize(value),
        ]
    }
}

impl IntoUsize for PentaGram<u8> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0] as usize) << 32
            | (self[1] as usize) << 24
            | (self[2] as usize) << 16
            | (self[3] as usize) << 8
            | self[4] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            (value >> 32) as u8,
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        ]
    }
}

impl IntoUsize for PentaGram<ASCIIChar> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0].into_usize() << 32)
            | (self[1].into_usize() << 24)
            | (self[2].into_usize() << 16)
            | (self[3].into_usize() << 8)
            | self[4].into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            ASCIIChar::from_usize(value >> 32),
            ASCIIChar::from_usize(value >> 24),
            ASCIIChar::from_usize(value >> 16),
            ASCIIChar::from_usize(value >> 8),
            ASCIIChar::from_usize(value),
        ]
    }
}

impl IntoUsize for HexaGram<u8> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0] as usize) << 40
            | (self[1] as usize) << 32
            | (self[2] as usize) << 24
            | (self[3] as usize) << 16
            | (self[4] as usize) << 8
            | self[5] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            (value >> 40) as u8,
            (value >> 32) as u8,
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        ]
    }
}

impl IntoUsize for HexaGram<ASCIIChar> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0].into_usize() << 40)
            | (self[1].into_usize() << 32)
            | (self[2].into_usize() << 24)
            | (self[3].into_usize() << 16)
            | (self[4].into_usize() << 8)
            | self[5].into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            ASCIIChar::from_usize(value >> 40),
            ASCIIChar::from_usize(value >> 32),
            ASCIIChar::from_usize(value >> 24),
            ASCIIChar::from_usize(value >> 16),
            ASCIIChar::from_usize(value >> 8),
            ASCIIChar::from_usize(value),
        ]
    }
}

impl IntoUsize for HeptaGram<u8> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0] as usize) << 48
            | (self[1] as usize) << 40
            | (self[2] as usize) << 32
            | (self[3] as usize) << 24
            | (self[4] as usize) << 16
            | (self[5] as usize) << 8
            | self[6] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            (value >> 48) as u8,
            (value >> 40) as u8,
            (value >> 32) as u8,
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        ]
    }
}

impl IntoUsize for HeptaGram<ASCIIChar> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0].into_usize() << 48)
            | (self[1].into_usize() << 40)
            | (self[2].into_usize() << 32)
            | (self[3].into_usize() << 24)
            | (self[4].into_usize() << 16)
            | (self[5].into_usize() << 8)
            | self[6].into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            ASCIIChar::from_usize(value >> 48),
            ASCIIChar::from_usize(value >> 40),
            ASCIIChar::from_usize(value >> 32),
            ASCIIChar::from_usize(value >> 24),
            ASCIIChar::from_usize(value >> 16),
            ASCIIChar::from_usize(value >> 8),
            ASCIIChar::from_usize(value),
        ]
    }
}

impl IntoUsize for OctaGram<u8> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0] as usize) << 56
            | (self[1] as usize) << 48
            | (self[2] as usize) << 40
            | (self[3] as usize) << 32
            | (self[4] as usize) << 24
            | (self[5] as usize) << 16
            | (self[6] as usize) << 8
            | self[7] as usize
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            (value >> 56) as u8,
            (value >> 48) as u8,
            (value >> 40) as u8,
            (value >> 32) as u8,
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        ]
    }
}

impl IntoUsize for OctaGram<ASCIIChar> {
    #[inline(always)]
    fn into_usize(self) -> usize {
        (self[0].into_usize() << 56)
            | (self[1].into_usize() << 48)
            | (self[2].into_usize() << 40)
            | (self[3].into_usize() << 32)
            | (self[4].into_usize() << 24)
            | (self[5].into_usize() << 16)
            | (self[6].into_usize() << 8)
            | self[7].into_usize()
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        [
            ASCIIChar::from_usize(value >> 56),
            ASCIIChar::from_usize(value >> 48),
            ASCIIChar::from_usize(value >> 40),
            ASCIIChar::from_usize(value >> 32),
            ASCIIChar::from_usize(value >> 24),
            ASCIIChar::from_usize(value >> 16),
            ASCIIChar::from_usize(value >> 8),
            ASCIIChar::from_usize(value),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let value = 42_u8;
        let converted = value.into_usize();
        let expected = 42;
        assert_eq!(converted, expected);
        let value = 42;
        let converted = u8::from_usize(value);
        let expected = 42_u8;
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_ascii_char() {
        let value = ASCIIChar::from(42_u8);
        let converted = value.into_usize();
        let expected = 42;
        assert_eq!(converted, expected);
        let value = 42;
        let converted = ASCIIChar::from_usize(value);
        let expected = ASCIIChar::from(42_u8);
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_char() {
        let value = 'a';
        let converted = value.into_usize();
        let expected = 97;
        assert_eq!(converted, expected);
        let value = 97;
        let converted = char::from_usize(value);
        let expected = 'a';
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_monogram_u8() {
        let value = [42_u8];
        let converted = value.into_usize();
        let expected = 42;
        assert_eq!(converted, expected);
        let value = 42;
        let converted = MonoGram::<u8>::from_usize(value);
        let expected = [42_u8];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_monogram_ascii_char() {
        let value = [ASCIIChar::from(42_u8)];
        let converted = value.into_usize();
        let expected = 42;
        assert_eq!(converted, expected);
        let value = 42;
        let converted = MonoGram::<ASCIIChar>::from_usize(value);
        let expected = [ASCIIChar::from(42_u8)];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_monogram_char() {
        let value = ['a'];
        let converted = value.into_usize();
        let expected = 97;
        assert_eq!(converted, expected);
        let value = 97;
        let converted = MonoGram::<char>::from_usize(value);
        let expected = ['a'];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_bigram_u8() {
        let value = [42_u8, 43_u8];
        let converted = value.into_usize();
        let expected = 42 << 8 | 43;
        assert_eq!(converted, expected);
        let value = 42 << 8 | 43;
        let converted = BiGram::<u8>::from_usize(value);
        let expected = [42_u8, 43_u8];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_bigram_ascii_char() {
        let value = [ASCIIChar::from(42_u8), ASCIIChar::from(43_u8)];
        let converted = value.into_usize();
        let expected = 42 << 8 | 43;
        assert_eq!(converted, expected);
        let value = 42 << 8 | 43;
        let converted = BiGram::<ASCIIChar>::from_usize(value);
        let expected = [ASCIIChar::from(42_u8), ASCIIChar::from(43_u8)];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_bigram_char() {
        let value = ['a', 'b'];
        let converted = value.into_usize();
        let expected = 97 << 32 | 98;
        assert_eq!(converted, expected);
        let value = 97 << 32 | 98;
        let converted = BiGram::<char>::from_usize(value);
        let expected = ['a', 'b'];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_trigram_u8() {
        let value = [42_u8, 43_u8, 44_u8];
        let converted = value.into_usize();
        let expected = 42 << 16 | 43 << 8 | 44;
        assert_eq!(converted, expected);
        let value = 42 << 16 | 43 << 8 | 44;
        let converted = TriGram::<u8>::from_usize(value);
        let expected = [42_u8, 43_u8, 44_u8];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_trigram_ascii_char() {
        let value = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
        ];
        let converted = value.into_usize();
        let expected = 42 << 16 | 43 << 8 | 44;
        assert_eq!(converted, expected);
        let value = 42 << 16 | 43 << 8 | 44;
        let converted = TriGram::<ASCIIChar>::from_usize(value);
        let expected = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
        ];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_tetragram_u8() {
        let value = [42_u8, 43_u8, 44_u8, 45_u8];
        let converted = value.into_usize();
        let expected = 42 << 24 | 43 << 16 | 44 << 8 | 45;
        assert_eq!(converted, expected);
        let value = 42 << 24 | 43 << 16 | 44 << 8 | 45;
        let converted = TetraGram::<u8>::from_usize(value);
        let expected = [42_u8, 43_u8, 44_u8, 45_u8];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_tetragram_ascii_char() {
        let value = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
        ];
        let converted = value.into_usize();
        let expected = 42 << 24 | 43 << 16 | 44 << 8 | 45;
        assert_eq!(converted, expected);
        let value = 42 << 24 | 43 << 16 | 44 << 8 | 45;
        let converted = TetraGram::<ASCIIChar>::from_usize(value);
        let expected = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
        ];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_pentagram_u8() {
        let value = [42_u8, 43_u8, 44_u8, 45_u8, 46_u8];
        let converted = value.into_usize();
        let expected = 42 << 32 | 43 << 24 | 44 << 16 | 45 << 8 | 46;
        assert_eq!(converted, expected);
        let value = 42 << 32 | 43 << 24 | 44 << 16 | 45 << 8 | 46;
        let converted = PentaGram::<u8>::from_usize(value);
        let expected = [42_u8, 43_u8, 44_u8, 45_u8, 46_u8];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_pentagram_ascii_char() {
        let value = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
            ASCIIChar::from(46_u8),
        ];
        let converted = value.into_usize();
        let expected = 42 << 32 | 43 << 24 | 44 << 16 | 45 << 8 | 46;
        assert_eq!(converted, expected);
        let value = 42 << 32 | 43 << 24 | 44 << 16 | 45 << 8 | 46;
        let converted = PentaGram::<ASCIIChar>::from_usize(value);
        let expected = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
            ASCIIChar::from(46_u8),
        ];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_hexagram_u8() {
        let value = [42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8];
        let converted = value.into_usize();
        let expected = 42 << 40 | 43 << 32 | 44 << 24 | 45 << 16 | 46 << 8 | 47;
        assert_eq!(converted, expected);
        let value = 42 << 40 | 43 << 32 | 44 << 24 | 45 << 16 | 46 << 8 | 47;
        let converted = HexaGram::<u8>::from_usize(value);
        let expected = [42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_hexagram_ascii_char() {
        let value = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
            ASCIIChar::from(46_u8),
            ASCIIChar::from(47_u8),
        ];
        let converted = value.into_usize();
        let expected = 42 << 40 | 43 << 32 | 44 << 24 | 45 << 16 | 46 << 8 | 47;
        assert_eq!(converted, expected);
        let value = 42 << 40 | 43 << 32 | 44 << 24 | 45 << 16 | 46 << 8 | 47;
        let converted = HexaGram::<ASCIIChar>::from_usize(value);
        let expected = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
            ASCIIChar::from(46_u8),
            ASCIIChar::from(47_u8),
        ];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_heptagram_u8() {
        let value = [42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8];
        let converted = value.into_usize();
        let expected = 42 << 48 | 43 << 40 | 44 << 32 | 45 << 24 | 46 << 16 | 47 << 8 | 48;
        assert_eq!(converted, expected);
        let value = 42 << 48 | 43 << 40 | 44 << 32 | 45 << 24 | 46 << 16 | 47 << 8 | 48;
        let converted = HeptaGram::<u8>::from_usize(value);
        let expected = [42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_heptagram_ascii_char() {
        let value = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
            ASCIIChar::from(46_u8),
            ASCIIChar::from(47_u8),
            ASCIIChar::from(48_u8),
        ];
        let converted = value.into_usize();
        let expected = 42 << 48 | 43 << 40 | 44 << 32 | 45 << 24 | 46 << 16 | 47 << 8 | 48;
        assert_eq!(converted, expected);
        let value = 42 << 48 | 43 << 40 | 44 << 32 | 45 << 24 | 46 << 16 | 47 << 8 | 48;
        let converted = HeptaGram::<ASCIIChar>::from_usize(value);
        let expected = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
            ASCIIChar::from(46_u8),
            ASCIIChar::from(47_u8),
            ASCIIChar::from(48_u8),
        ];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_octagram_u8() {
        let value = [42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8];
        let converted = value.into_usize();
        let expected =
            42 << 56 | 43 << 48 | 44 << 40 | 45 << 32 | 46 << 24 | 47 << 16 | 48 << 8 | 49;
        assert_eq!(converted, expected);
        let value = 42 << 56 | 43 << 48 | 44 << 40 | 45 << 32 | 46 << 24 | 47 << 16 | 48 << 8 | 49;
        let converted = OctaGram::<u8>::from_usize(value);
        let expected = [42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8];
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_octagram_ascii_char() {
        let value = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
            ASCIIChar::from(46_u8),
            ASCIIChar::from(47_u8),
            ASCIIChar::from(48_u8),
            ASCIIChar::from(49_u8),
        ];
        let converted = value.into_usize();
        let expected =
            42 << 56 | 43 << 48 | 44 << 40 | 45 << 32 | 46 << 24 | 47 << 16 | 48 << 8 | 49;
        assert_eq!(converted, expected);
        let value = 42 << 56 | 43 << 48 | 44 << 40 | 45 << 32 | 46 << 24 | 47 << 16 | 48 << 8 | 49;
        let converted = OctaGram::<ASCIIChar>::from_usize(value);
        let expected = [
            ASCIIChar::from(42_u8),
            ASCIIChar::from(43_u8),
            ASCIIChar::from(44_u8),
            ASCIIChar::from(45_u8),
            ASCIIChar::from(46_u8),
            ASCIIChar::from(47_u8),
            ASCIIChar::from(48_u8),
            ASCIIChar::from(49_u8),
        ];
        assert_eq!(converted, expected);
    }
}
