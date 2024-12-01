use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};
use itertools::Itertools;
use num::traits::{FromBytes, ToBytes, WrappingAdd, WrappingSub};
use std::ops::{Add, BitXor, BitXorAssign, Sub};
use utils::byte_formatting::ByteFormat;

// 48 bit value stored in the lower 48 bits of a u64
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U48(u64);

impl U48 {
    pub fn new(n: u64) -> U48 {
        U48(n & 0x0000ffffffffffff)
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

impl Default for U48 {
    fn default() -> Self {
        Self(0)
    }
}

// 24 bit value stored in the lower 24 bits of a u32
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U24(u32);

impl Default for U24 {
    fn default() -> Self {
        Self(0)
    }
}

impl U24 {
    pub fn new(n: u32) -> U24 {
        U24(n & 0x00ffffff)
    }

    pub fn rotate_left(mut self, d: u32) -> U24 {
        self.0 = ((self.0 << d) | (self.0 >> (24 - d))) & 0x00ffffff;
        self
    }

    pub fn rotate_right(mut self, d: u32) -> U24 {
        self.0 = ((self.0 >> d) | (self.0 << (24 - d))) & 0x00ffffff;
        self
    }
}

impl ToBytes for U24 {
    type Bytes = [u8; 3];

    fn to_be_bytes(&self) -> Self::Bytes {
        self.0.to_be_bytes()[1..].try_into().unwrap()
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        self.0.to_le_bytes()[..3].try_into().unwrap()
    }
}

impl FromBytes for U24 {
    type Bytes = [u8; 3];

    fn from_be_bytes(bytes: &Self::Bytes) -> Self {
        let mut inner = 0_u32;

        for b in bytes {
            inner <<= 8;
            inner |= *b as u32
        }

        U24(inner)
    }

    fn from_le_bytes(bytes: &Self::Bytes) -> Self {
        let mut inner = 0_u32;

        for b in bytes.iter().rev() {
            inner <<= 8;
            inner |= *b as u32
        }

        U24(inner)
    }
}

impl BitXor for U24 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        U24(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for U24 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Add for U24 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        U24((self.0 + rhs.0) & 0x00ffffff)
    }
}

impl WrappingAdd for U24 {
    fn wrapping_add(&self, v: &Self) -> Self {
        U24((self.0 + v.0) & 0x00ffffff)
    }
}

impl Sub for U24 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        U24(self.0 - rhs.0)
    }
}

impl WrappingSub for U24 {
    fn wrapping_sub(&self, v: &Self) -> Self {
        U24(self.0.wrapping_sub(v.0) & 0x00ffffff)
    }
}

pub fn make_u24s_be<const N: usize>(bytes: &[u8]) -> [U24; N] {
    let mut out = [U24::new(0); N];
    for (elem, chunk) in out.iter_mut().zip_eq(bytes.chunks_exact(3)) {
        *elem = U24::from_be_bytes(chunk.try_into().unwrap());
    }
    out
}

pub fn u24s_to_bytes_be<T: AsRef<[U24]>, S: AsMut<[u8]>>(mut target: S, words: T) {
    for (chunk, word) in target.as_mut().chunks_exact_mut(3).zip_eq(words.as_ref()) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
}

macro_rules! speck48 {
    ($name:ident, $key_words:literal, $rounds:literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: U48,
            pub subkeys: [U24; $rounds],
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Hex,
                    output_format: ByteFormat::Hex,
                    mode: Default::default(),
                    padding: Default::default(),
                    iv: Default::default(),
                    subkeys: [U24::new(0); $rounds],
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

            pub fn iv(mut self, iv: u64) -> Self {
                self.iv = U48::new(iv);
                self
            }

            pub fn ksa(&mut self, bytes: [u8; $key_words * 3]) {
                let key = make_u24s_be::<$key_words>(&bytes);
                self.generate_subkeys(key)
            }

            pub fn with_key(mut self, bytes: [u8; $key_words * 3]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn ksa_32(&mut self, key: [u32; $key_words]) {
                self.generate_subkeys(key.map(|w| U24::new(w)));
            }

            pub fn with_key_32(mut self, key: [u32; $key_words]) -> Self {
                self.ksa_32(key);
                self
            }

            // For encryption this can be done on the fly for each round
            pub fn generate_subkeys(&mut self, key: [U24; $key_words]) {
                let mut subkeys = [U24::new(0); $rounds as usize];
                let mut k = key;
                // let [mut a, mut b, mut c, mut d] = self.key;
                for i in 0..$rounds {
                    subkeys[i as usize] = k[$key_words - 1];
                    let mut tc = k[$key_words - 2];
                    let mut td = k[$key_words - 1];
                    tc = tc.rotate_right(8);
                    tc = tc.wrapping_add(&td);
                    tc ^= U24::new(i);
                    td = td.rotate_left(3);
                    td ^= tc;
                    k[0..$key_words - 1].rotate_right(1);
                    k[0] = tc;
                    k[$key_words - 1] = td;
                }
                self.subkeys = subkeys;
            }
        }

        impl BlockCipher<6> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u24s_be::<2>(bytes);

                for k in self.subkeys {
                    x = x.rotate_right(8);
                    x = x.wrapping_add(&y);
                    x ^= k;
                    y = y.rotate_left(3);
                    y ^= x;
                }

                u24s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u24s_be::<2>(bytes);

                for k in self.subkeys.into_iter().rev() {
                    y ^= x;
                    y = y.rotate_right(3);
                    x ^= k;
                    x = x.wrapping_sub(&y);
                    x = x.rotate_left(8);
                }

                u24s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

speck48!(Speck48_72, 3, 22);
crate::impl_cipher_for_block_cipher!(Speck48_72, 6);
speck48!(Speck48_96, 4, 23);
crate::impl_cipher_for_block_cipher!(Speck48_96, 6);

crate::test_block_cipher!(
    test_64_96, Speck48_72::default().with_key([0x12, 0x11, 0x10, 0x0a, 0x09, 0x08, 0x02, 0x01, 0x00]),
    [0x20, 0x79, 0x6c, 0x6c, 0x61, 0x72],
    [0xc0, 0x49, 0xa5, 0x38, 0x5a, 0xdc];
    test_64_128, Speck48_96::default().with_key([0x1a, 0x19, 0x18, 0x12, 0x11, 0x10, 0x0a, 0x09, 0x08, 0x02, 0x01, 0x00]),
    [0x6d, 0x20, 0x73, 0x69, 0x68, 0x74],
    [0x73, 0x5e, 0x10, 0xb6, 0x44, 0x5d];
);
