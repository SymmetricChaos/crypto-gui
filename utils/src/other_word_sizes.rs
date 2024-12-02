use itertools::Itertools;
use num::{
    traits::{FromBytes, ToBytes},
    One, Zero,
};
use paste::paste;
use std::ops::{Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul};

macro_rules! new_word {
    ($name: ident, $low_name: ident, $inner: ty, $inner_bytes: literal, $mask: literal, $bytes: literal, $bits: literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name($inner);

        impl Default for $name {
            fn default() -> Self {
                Self(0)
            }
        }

        impl $name {
            pub const MASK: $inner = $mask;
            pub const BYTES: $inner = $bytes;
            pub const BITS: $inner = $bits;

            pub fn new(n: $inner) -> $name {
                $name(n & $mask)
            }

            pub fn rotate_left(mut self, d: $inner) -> $name {
                self.0 = ((self.0 << d) | (self.0 >> ($bits - d))) & $mask;
                self
            }

            pub fn rotate_right(mut self, d: $inner) -> $name {
                self.0 = ((self.0 >> d) | (self.0 << ($bits - d))) & $mask;
                self
            }
        }

        impl ToBytes for $name {
            type Bytes = [u8; $bytes];

            fn to_be_bytes(&self) -> Self::Bytes {
                self.0.to_be_bytes()[($inner_bytes - $bytes)..]
                    .try_into()
                    .unwrap()
            }

            fn to_le_bytes(&self) -> Self::Bytes {
                self.0.to_le_bytes()[..$bytes].try_into().unwrap()
            }
        }

        impl FromBytes for $name {
            type Bytes = [u8; $bytes];

            fn from_be_bytes(bytes: &Self::Bytes) -> Self {
                let mut inner = 0;

                for b in bytes {
                    inner <<= 8;
                    inner |= *b as $inner
                }

                $name(inner)
            }

            fn from_le_bytes(bytes: &Self::Bytes) -> Self {
                let mut inner = 0;

                for b in bytes.iter().rev() {
                    inner <<= 8;
                    inner |= *b as $inner
                }

                $name(inner)
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $name((self.0 + rhs.0) & $mask)
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                $name((self.0 * rhs.0) & $mask)
            }
        }

        impl Zero for $name {
            fn zero() -> Self {
                $name(0)
            }

            fn is_zero(&self) -> bool {
                self.0 == 0
            }
        }

        impl One for $name {
            fn one() -> Self {
                $name(1)
            }
        }

        impl BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                $name((self.0 ^ rhs.0) & $mask)
            }
        }

        impl BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = *self ^ rhs;
            }
        }

        impl BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                $name((self.0 & rhs.0) & $mask)
            }
        }

        impl BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = *self & rhs;
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                $name((self.0 | rhs.0) & $mask)
            }
        }

        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = *self | rhs;
            }
        }

        paste! {
                /// Use bytes to fill the target with the type. Panics if target cannot be not exactly filled. Big-endian.
                pub fn [<fill_ $low_name s_be>]<T: AsRef<[u8]>>(target: &mut [$name], bytes: T) {
                    for (elem, chunk) in target.iter_mut().zip_eq(bytes.as_ref().chunks_exact($bytes)) {
                        *elem = <$name>::from_be_bytes(chunk.try_into().unwrap());
                    }
                }

                /// Use bytes to make an array filled with the type. Panics if the array cannot be exactly filled. Big-endian.
                pub fn [<make_ $low_name s_be>]<const N: usize>(bytes: &[u8]) -> [$name; N] {
                    let mut out = [<$name>::zero(); N];
                    for (elem, chunk) in out.iter_mut().zip_eq(bytes.chunks_exact($bytes)) {
                        *elem = <$name>::from_be_bytes(chunk.try_into().unwrap());
                    }
                    out
                }

                /// Take a slice of the type and filled the target with bytes. Panics if the target cannot be exactly filled. Big-endian.
                pub fn [<$low_name s_to_bytes_be>]<T: AsRef<[$name]>, S: AsMut<[u8]>>(mut target: S, words: T) {
                    for (chunk, word) in target.as_mut().chunks_exact_mut($bytes).zip_eq(words.as_ref()) {
                        chunk.copy_from_slice(&word.to_be_bytes());
                    }
                }

                /// Use bytes to fill the target with the type. Panics if target cannot be not exactly filled. Little-endian.
                pub fn [<fill_ $low_name s_le>]<T: AsRef<[u8]>>(target: &mut [$name], bytes: T) {
                    for (elem, chunk) in target.iter_mut().zip_eq(bytes.as_ref().chunks_exact($bytes)) {
                        *elem = <$name>::from_le_bytes(chunk.try_into().unwrap());
                    }
                }

                /// Use bytes to make an array filled with the type. Panics if the array cannot be exactly filled. Little-endian.
                pub fn [<make_ $low_name s_le>]<const N: usize>(bytes: &[u8]) -> [$name; N] {
                    let mut out = [<$name>::zero(); N];
                    for (elem, chunk) in out.iter_mut().zip_eq(bytes.chunks_exact($bytes)) {
                        *elem = <$name>::from_le_bytes(chunk.try_into().unwrap());
                    }
                    out
                }

                /// Take a slice of the type and filled the target with bytes. Panics if the target cannot be exactly filled. Little-endian.
                pub fn [<$low_name s_to_bytes_le>]<T: AsRef<[$name]>, S: AsMut<[u8]>>(mut target: S, words: T) {
                    for (chunk, word) in target.as_mut().chunks_exact_mut($bytes).zip_eq(words.as_ref()) {
                        chunk.copy_from_slice(&word.to_le_bytes());
                    }
                }

        }
    };
}

new_word!(U24, u24, u32, 4, 0x00ffffff, 3, 24);
new_word!(U48, u48, u64, 8, 0x0000ffffffffffff, 6, 48);
new_word!(U96, u96, u128, 16, 0xffffffffffffffffffffffff, 12, 96);
