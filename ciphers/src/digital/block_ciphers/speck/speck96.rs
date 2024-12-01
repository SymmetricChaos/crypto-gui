use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};
use itertools::Itertools;
use num::traits::{FromBytes, ToBytes, WrappingAdd, WrappingSub};
use std::ops::{Add, BitXor, BitXorAssign, Sub};
use utils::byte_formatting::ByteFormat;

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

impl Add for U48 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        U48((self.0 + rhs.0) & 0x0000ffffffffffff)
    }
}

impl WrappingAdd for U48 {
    fn wrapping_add(&self, v: &Self) -> Self {
        U48((self.0 + v.0) & 0x0000ffffffffffff)
    }
}

impl Sub for U48 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        U48(self.0 - rhs.0)
    }
}

impl WrappingSub for U48 {
    fn wrapping_sub(&self, v: &Self) -> Self {
        U48(self.0.wrapping_sub(v.0) & 0x0000ffffffffffff)
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

macro_rules! speck96 {
    ($name:ident, $key_words:literal, $rounds:literal) => {
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

            pub fn ksa_64(&mut self, key: [u64; $key_words]) {
                self.generate_subkeys(key.map(|w| U48::new(w)));
            }

            pub fn with_key_64(mut self, key: [u64; $key_words]) -> Self {
                self.ksa_64(key);
                self
            }

            // For encryption this can be done on the fly for each round
            pub fn generate_subkeys(&mut self, key: [U48; $key_words]) {
                let mut subkeys = [U48::new(0); $rounds as usize];
                let mut k = key;
                for i in 0..$rounds {
                    subkeys[i as usize] = k[$key_words - 1];
                    let mut tc = k[$key_words - 2];
                    let mut td = k[$key_words - 1];
                    tc = tc.rotate_right(8);
                    tc = tc.wrapping_add(&td);
                    tc ^= U48::new(i);
                    td = td.rotate_left(3);
                    td ^= tc;
                    k[0..$key_words - 1].rotate_right(1);
                    k[0] = tc;
                    k[$key_words - 1] = td;
                }
                self.subkeys = subkeys;
            }
        }

        impl BlockCipher<12> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u48s_be::<2>(bytes);

                for k in self.subkeys {
                    x = x.rotate_right(8);
                    x = x.wrapping_add(&y);
                    x ^= k;
                    y = y.rotate_left(3);
                    y ^= x;
                }

                u48s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u48s_be::<2>(bytes);

                for k in self.subkeys.into_iter().rev() {
                    y ^= x;
                    y = y.rotate_right(3);
                    x ^= k;
                    x = x.wrapping_sub(&y);
                    x = x.rotate_left(8);
                }

                u48s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

speck96!(Speck96_96, 2, 28);
crate::impl_cipher_for_block_cipher!(Speck96_96, 12);
speck96!(Speck96_144, 3, 29);
crate::impl_cipher_for_block_cipher!(Speck96_144, 12);

crate::test_block_cipher!(
    test_96_96, Speck96_96::default().with_key([0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]),
    [0x65, 0x77, 0x6f, 0x68, 0x20, 0x2c, 0x65, 0x67, 0x61, 0x73, 0x75, 0x20],
    [0x9e, 0x4d, 0x09, 0xab, 0x71, 0x78, 0x62, 0xbd, 0xde, 0x8f, 0x79, 0xaa];
    test_96_144, Speck96_144::default().with_key([0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]),
    [0x65, 0x6d, 0x69, 0x74, 0x20, 0x6e, 0x69, 0x20, 0x2c, 0x72, 0x65, 0x76],
    [0x2b, 0xf3, 0x10, 0x72, 0x22, 0x8a, 0x7a, 0xe4, 0x40, 0x25, 0x2e, 0xe6];
);
