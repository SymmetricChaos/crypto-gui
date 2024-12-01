use std::ops::{BitAnd, BitXor, BitXorAssign};

use itertools::Itertools;
use num::traits::{FromBytes, ToBytes};
use utils::byte_formatting::ByteFormat;

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

use super::select_z_bit;

// 96 bit value stored in the lower 96 bits of a u128
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U96(u128);

impl U96 {
    pub fn new(n: u128) -> U96 {
        U96(n & 0xffffffffffffffffffffffff)
    }
}

impl ToBytes for U96 {
    type Bytes = [u8; 12];

    fn to_be_bytes(&self) -> Self::Bytes {
        self.0.to_be_bytes()[4..].try_into().unwrap()
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        self.0.to_le_bytes()[..12].try_into().unwrap()
    }
}

impl Default for U96 {
    fn default() -> Self {
        Self(0)
    }
}

// 48 bit value stored in the lower 48 bits of a u64
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U48(u64);

impl Default for U48 {
    fn default() -> Self {
        Self(0)
    }
}

impl U48 {
    pub fn new(n: u64) -> U48 {
        U48(n & 0x0000ffffffffffff)
    }

    pub fn rotate_left(mut self, d: u32) -> U48 {
        self.0 = ((self.0 << d) | (self.0 >> (48 - d))) & 0x0000ffffffffffff;
        self
    }

    pub fn rotate_right(mut self, d: u32) -> U48 {
        self.0 = ((self.0 >> d) | (self.0 << (48 - d))) & 0x0000ffffffffffff;
        self
    }
}

impl ToBytes for U48 {
    type Bytes = [u8; 6];

    fn to_be_bytes(&self) -> Self::Bytes {
        self.0.to_be_bytes()[2..].try_into().unwrap()
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        self.0.to_le_bytes()[..6].try_into().unwrap()
    }
}

impl FromBytes for U48 {
    type Bytes = [u8; 6];

    fn from_be_bytes(bytes: &Self::Bytes) -> Self {
        let mut inner = 0_u64;

        for b in bytes {
            inner <<= 8;
            inner |= *b as u64
        }

        U48(inner)
    }

    fn from_le_bytes(bytes: &Self::Bytes) -> Self {
        let mut inner = 0_u64;

        for b in bytes.iter().rev() {
            inner <<= 8;
            inner |= *b as u64
        }

        U48(inner)
    }
}

impl BitXor for U48 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        U48(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for U48 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl BitAnd for U48 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        U48(self.0 & rhs.0)
    }
}

pub fn make_u48s_be<const N: usize>(bytes: &[u8]) -> [U48; N] {
    let mut out = [U48::new(0); N];
    for (elem, chunk) in out.iter_mut().zip_eq(bytes.chunks_exact(6)) {
        *elem = U48::from_be_bytes(chunk.try_into().unwrap());
    }
    out
}

pub fn u48s_to_bytes_be<T: AsRef<[U48]>, S: AsMut<[u8]>>(mut target: S, words: T) {
    for (chunk, word) in target.as_mut().chunks_exact_mut(6).zip_eq(words.as_ref()) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
}
macro_rules! simon96 {
    ($name:ident, $key_words:literal, $rounds:literal, $z_string:literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: U96,
            pub subkeys: [U48; $rounds],
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Hex,
                    output_format: ByteFormat::Hex,
                    mode: Default::default(),
                    padding: Default::default(),
                    iv: Default::default(),
                    subkeys: [U48::new(0); $rounds],
                }
            }
        }

        impl $name {
            pub fn input(mut self, input: utils::byte_formatting::ByteFormat) -> Self {
                self.input_format = input;
                self
            }

            pub fn output(mut self, output: utils::byte_formatting::ByteFormat) -> Self {
                self.output_format = output;
                self
            }

            pub fn padding(
                mut self,
                padding: crate::digital::block_ciphers::block_cipher::BCPadding,
            ) -> Self {
                self.padding = padding;
                self
            }

            pub fn mode(
                mut self,
                mode: crate::digital::block_ciphers::block_cipher::BCMode,
            ) -> Self {
                self.mode = mode;
                self
            }

            pub fn iv(mut self, iv: u128) -> Self {
                self.iv = U96::new(iv);
                self
            }

            pub fn ksa(&mut self, bytes: [u8; $key_words * 6]) {
                let key = make_u48s_be::<$key_words>(&bytes);
                self.generate_subkeys(key);
            }

            pub fn with_key(mut self, bytes: [u8; $key_words * 6]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn ksa_16(&mut self, key: [u64; $key_words]) {
                self.generate_subkeys(key.map(|w| U48::new(w)));
            }

            pub fn with_key_16(mut self, key: [u64; $key_words]) -> Self {
                self.ksa_16(key);
                self
            }

            pub fn generate_subkeys(&mut self, key: [U48; $key_words]) {
                let mut subkeys = [U48::new(0); $rounds as usize];

                // First four subkeys are just the key itself
                for i in 0..$key_words {
                    subkeys[$key_words - i - 1] = key[i]
                }

                for i in $key_words..$rounds as usize {
                    let mut t = subkeys[i - 1].rotate_right(3);
                    if $key_words == 4 {
                        t ^= subkeys[i - 3];
                    }
                    t ^= t.rotate_right(1);
                    let bit_idx = (i - $key_words) % 62;

                    subkeys[i] = (subkeys[i - $key_words])
                        ^ U48::new(!3)
                        ^ t
                        ^ U48::new(select_z_bit($z_string, bit_idx) as u64);
                }

                self.subkeys = subkeys;
            }
        }

        impl BlockCipher<12> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u48s_be::<2>(bytes);

                for k in self.subkeys {
                    let t = y;
                    // L_i+1 = R_i
                    y = x;

                    // R_i+1 = L_i xor f(R_i)
                    x = t ^ super::round!(x, k);
                }

                u48s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u48s_be::<2>(bytes);

                for k in self.subkeys.into_iter().rev() {
                    let t = x;
                    // L_i+1 = R_i
                    x = y;

                    // R_i+1 = L_i xor f(R_i)
                    y = t ^ super::round!(y, k);
                }

                u48s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

simon96!(Simon96_96, 2, 52, 2);
crate::impl_cipher_for_block_cipher!(Simon96_96, 12);
simon96!(Simon96_144, 3, 54, 3);
crate::impl_cipher_for_block_cipher!(Simon96_144, 12);

crate::test_block_cipher!(
    test_96_96, Simon96_96::default().with_key([0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]),
    [0x20, 0x72, 0x61, 0x6c, 0x6c, 0x69, 0x70, 0x20, 0x65, 0x68, 0x74, 0x20],
    [0x60, 0x28, 0x07, 0xa4, 0x62, 0xb4, 0x69, 0x06, 0x3d, 0x8f, 0xf0, 0x82];
    test_96_144, Simon96_144::default().with_key([0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]),
    [0x74, 0x61, 0x68, 0x74, 0x20, 0x74, 0x73, 0x75, 0x64, 0x20, 0x66, 0x6f],
    [0xec, 0xad, 0x1c, 0x6c, 0x45, 0x1e, 0x3f, 0x59, 0xc5, 0xdb, 0x1a, 0xe9];
);
