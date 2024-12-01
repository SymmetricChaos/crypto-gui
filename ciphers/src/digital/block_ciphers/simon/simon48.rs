use std::ops::{BitAnd, BitXor, BitXorAssign};

use itertools::Itertools;
use num::traits::{FromBytes, ToBytes};
use utils::byte_formatting::ByteFormat;

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

use super::select_z_bit;

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

impl BitAnd for U24 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        U24(self.0 & rhs.0)
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

macro_rules! simon48 {
    ($name:ident, $key_words:literal, $rounds:literal, $z_string:literal) => {
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
                    subkeys: [U24::default(); $rounds],
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
                self.generate_subkeys(key);
            }

            pub fn with_key(mut self, bytes: [u8; $key_words * 3]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn ksa_16(&mut self, key: [u32; $key_words]) {
                self.generate_subkeys(key.map(|w| U24::new(w)));
            }

            pub fn with_key_16(mut self, key: [u32; $key_words]) -> Self {
                self.ksa_16(key);
                self
            }

            pub fn generate_subkeys(&mut self, key: [U24; $key_words]) {
                let mut subkeys = [U24::new(0); $rounds as usize];

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
                        ^ U24::new(!3)
                        ^ t
                        ^ U24::new(select_z_bit($z_string, bit_idx) as u32);
                }

                self.subkeys = subkeys;
            }
        }

        impl BlockCipher<6> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u24s_be::<2>(bytes);

                for k in self.subkeys {
                    let t = y;
                    // L_i+1 = R_i
                    y = x;

                    // R_i+1 = L_i xor f(R_i)
                    x = t ^ super::round!(y, k);
                }

                u24s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u24s_be::<2>(bytes);

                for k in self.subkeys.into_iter().rev() {
                    let t = x;
                    // L_i+1 = R_i
                    x = y;

                    // R_i+1 = L_i xor f(R_i)
                    y = t ^ super::round!(y, k);
                }

                u24s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

simon48!(Simon48_72, 3, 36, 0);
crate::impl_cipher_for_block_cipher!(Simon48_72, 6);
simon48!(Simon48_96, 4, 36, 1);
crate::impl_cipher_for_block_cipher!(Simon48_96, 6);

#[cfg(test)]
mod simon48_tests {
    use super::*;

    #[test]
    #[ignore = ""]
    fn check_operations() {
        let a = U24::new(!0);
        println!("{:08x?}", a);
        let b = U24::new(0x23232323);
        println!("{:08x?}", b);
        let c = a ^ b;
        println!("{:08x?}", c);
        let d = a & b;
        println!("{:08x?}", d);
        let e = U24::from_be_bytes(&[0x01, 0x23, 0x45]);
        println!("{:08x?}", e);
        let f = U24::from_le_bytes(&[0x01, 0x23, 0x45]);
        println!("{:08x?}", f);
        println!("{:02x?}", U24::new(0x012345).to_be_bytes());
        println!("{:02x?}", U24::new(0x012345).to_le_bytes());
        println!("{:08x?}", U24::new(0x012345).rotate_left(8));
        println!("{:08x?}", U24::new(0x012345).rotate_right(8));
    }
}

crate::test_block_cipher!(
    test_48_72, Simon48_72::default().with_key([0x12, 0x11, 0x10, 0x0a, 0x09, 0x08, 0x02, 0x01, 0x00]),
    [0x61, 0x20, 0x67, 0x6e, 0x69, 0x6c],
    [0xda, 0xe5, 0xac, 0x29, 0x2c, 0xac];
    test_48_96, Simon48_96::default().with_key([0x1a, 0x19, 0x18, 0x12, 0x11, 0x10, 0x0a, 0x09, 0x08, 0x02, 0x01, 0x00]),
    [0x72, 0x69, 0x63, 0x20, 0x64, 0x6e],
    [0x6e, 0x06, 0xa5, 0xac, 0xf1, 0x56];
);
