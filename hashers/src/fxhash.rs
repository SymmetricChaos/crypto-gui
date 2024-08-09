use crate::{errors::HasherError, traits::ClassicHasher};
use std::ops::BitXor;
use utils::byte_formatting::ByteFormat;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WordSize {
    W32,
    W64,
}

pub struct FxHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub word_size: WordSize,
}

impl Default for FxHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            word_size: WordSize::W64,
        }
    }
}

impl FxHash {
    pub fn hash_word_64(state: &mut u64, word: u64) {
        *state = state
            .rotate_left(5)
            .bitxor(word)
            .wrapping_mul(0x517cc1b727220a95);
    }

    pub fn hash_word_32(state: &mut u32, word: u32) {
        *state = state.rotate_left(5).bitxor(word).wrapping_mul(0x27220a95);
    }
}

impl ClassicHasher for FxHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut bytes = bytes;
        if self.word_size == WordSize::W64 {
            let mut hash = 0_u64;
            while bytes.len() >= 8 {
                let n = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
                Self::hash_word_64(&mut hash, n);
                bytes = bytes.split_at(8).1;
            }

            if bytes.len() >= 4 {
                let n = u64::from_be_bytes(bytes[0..4].try_into().unwrap());
                Self::hash_word_64(&mut hash, n);
                bytes = bytes.split_at(4).1;
            }

            for byte in bytes {
                Self::hash_word_64(&mut hash, *byte as u64);
            }
            hash.to_be_bytes().to_vec()
        } else {
            let mut hash = 0_u32;

            if bytes.len() >= 4 {
                let n = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
                Self::hash_word_32(&mut hash, n);
                bytes = bytes.split_at(4).1;
            }

            for byte in bytes {
                Self::hash_word_32(&mut hash, *byte as u32);
            }
            hash.to_be_bytes().to_vec()
        }
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod fxhash_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = FxHash::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        // assert_eq!(
        //     "",
        //     hasher
        //         .hash_bytes_from_string("")
        //         .unwrap()
        // );
    }
}
