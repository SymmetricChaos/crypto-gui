use crate::traits::ClassicHasher;
use crypto_bigint::{ArrayEncoding, U256};
use utils::byte_formatting::ByteFormat;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FnvSize {
    P32,
    P64,
    P128,
    P256,
}

pub const P32: u32 = 16777619;
pub const P64: u64 = 1099511628211;
pub const P128: u128 = 309485009821345068724781371;
pub const P256: U256 =
    U256::from_be_hex("0000000000000000000001000000000000000000000000000000000000000163");

pub const O32: u32 = 2166136261;
pub const O64: u64 = 14695981039346656037;
pub const O128: u128 = 144066263297769815596495629667062367629;
pub const O256: U256 =
    U256::from_be_hex("DD268DBCAAC550362D98C384C4E576CCC8B1536847B6BBB31023B4C8CAEE0535");

pub struct Fnv {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub size: FnvSize,
    pub alternate: bool,
    pub zero_basis: bool,
}

impl Default for Fnv {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            size: FnvSize::P64,
            alternate: true,
            zero_basis: false,
        }
    }
}

impl Fnv {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn size(mut self, size: FnvSize) -> Self {
        self.size = size;
        self
    }

    pub fn alternate(mut self, alternate: bool) -> Self {
        self.alternate = alternate;
        self
    }

    pub fn zero_basis(mut self, zero_basis: bool) -> Self {
        self.zero_basis = zero_basis;
        self
    }

    pub fn hash_byte_32(&self, state: &mut u32, byte: u8) {
        if self.alternate {
            *state ^= byte as u32;
            *state = state.wrapping_mul(P32)
        } else {
            *state = state.wrapping_mul(P32);
            *state ^= byte as u32;
        }
    }

    pub fn hash_byte_64(&self, state: &mut u64, byte: u8) {
        if self.alternate {
            *state ^= byte as u64;
            *state = state.wrapping_mul(P64)
        } else {
            *state = state.wrapping_mul(P64);
            *state ^= byte as u64;
        }
    }

    pub fn hash_byte_128(&self, state: &mut u128, byte: u8) {
        if self.alternate {
            *state ^= byte as u128;
            *state = state.wrapping_mul(P128)
        } else {
            *state = state.wrapping_mul(P128);
            *state ^= byte as u128;
        }
    }

    pub fn hash_byte_256(&self, state: &mut U256, byte: u8) {
        if self.alternate {
            *state ^= U256::from_u8(byte);
            *state = state.wrapping_mul(&P256)
        } else {
            *state = state.wrapping_mul(&P256);
            *state ^= U256::from_u8(byte);
        }
    }
}

impl ClassicHasher for Fnv {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.size {
            FnvSize::P32 => {
                let mut state = match self.zero_basis {
                    true => 0,
                    false => O32,
                };
                for byte in bytes {
                    self.hash_byte_32(&mut state, *byte)
                }
                state.to_be_bytes().to_vec()
            }
            FnvSize::P64 => {
                let mut state = match self.zero_basis {
                    true => 0,
                    false => O64,
                };
                for byte in bytes {
                    self.hash_byte_64(&mut state, *byte)
                }
                state.to_be_bytes().to_vec()
            }
            FnvSize::P128 => {
                let mut state = match self.zero_basis {
                    true => 0,
                    false => O128,
                };
                for byte in bytes {
                    self.hash_byte_128(&mut state, *byte)
                }
                state.to_be_bytes().to_vec()
            }
            FnvSize::P256 => {
                let mut state = match self.zero_basis {
                    true => U256::ZERO,
                    false => O256,
                };
                for byte in bytes {
                    self.hash_byte_256(&mut state, *byte)
                }
                state.to_be_byte_array().to_vec()
            }
        }
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod fnvhash_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Fnv::default();

        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;

        hasher.size = FnvSize::P32;
        assert_eq!("e40c292c", hasher.hash_bytes_from_string("a").unwrap());

        hasher.size = FnvSize::P64;
        assert_eq!(
            "af63dc4c8601ec8c",
            hasher.hash_bytes_from_string("a").unwrap()
        );
    }
}
