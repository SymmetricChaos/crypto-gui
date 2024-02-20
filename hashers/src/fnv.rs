use crate::{errors::HasherError, traits::ClassicHasher};
use utils::byte_formatting::ByteFormat;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrimeSize {
    P32,
    P64,
    P128,
}

pub const P32: u32 = 16777619;
pub const P64: u64 = 1099511628211;
pub const P128: u128 = 309485009821345068724781371;

pub const O32: u32 = 2166136261;
pub const O64: u64 = 14695981039346656037;
pub const O128: u128 = 144066263297769815596495629667062367629;

pub struct Fnv {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub size: PrimeSize,
    pub alternate: bool,
}

impl Default for Fnv {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            size: PrimeSize::P64,
            alternate: true,
        }
    }
}

impl Fnv {
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
}

impl ClassicHasher for Fnv {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.size {
            PrimeSize::P32 => {
                let mut state = O32;
                for byte in bytes {
                    self.hash_byte_32(&mut state, *byte)
                }
                state.to_be_bytes().to_vec()
            }
            PrimeSize::P64 => {
                let mut state = O64;
                for byte in bytes {
                    self.hash_byte_64(&mut state, *byte)
                }
                state.to_be_bytes().to_vec()
            }
            PrimeSize::P128 => {
                let mut state = O128;
                for byte in bytes {
                    self.hash_byte_128(&mut state, *byte)
                }
                state.to_be_bytes().to_vec()
            }
        }
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod fxhash_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Fnv::default();

        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;

        hasher.size = PrimeSize::P32;
        assert_eq!("e40c292c", hasher.hash_bytes_from_string("a").unwrap());

        hasher.size = PrimeSize::P64;
        assert_eq!(
            "af63dc4c8601ec8c",
            hasher.hash_bytes_from_string("a").unwrap()
        );
    }
}
