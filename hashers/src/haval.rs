use crate::{
    auxiliary::haval_functions::{
        finalize_128, finalize_160, finalize_192, finalize_224, finalize_256, h1, h2, h3, h4, h5,
        haval_padding,
    },
    traits::ClassicHasher,
};

use super::auxiliary::haval_arrays::D;
use utils::byte_formatting::ByteFormat;

pub struct Haval {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub rounds: u32,
    pub hash_len: u32,
}

impl Default for Haval {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rounds: 5,
            hash_len: 32,
        }
    }
}

impl Haval {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn rounds(mut self, rounds: u32) -> Self {
        assert!(
            rounds == 3 || rounds == 4 || rounds == 5,
            "rounds must be 3, 4, or 5"
        );
        self.rounds = rounds;
        self
    }

    pub fn hash_len(mut self, hash_len: u32) -> Self {
        assert!(
            self.hash_len % 4 == 0 && self.hash_len >= 16 && self.hash_len <= 32,
            "output length is in bytes and must be 16, 20, 24, 28, or 32"
        );
        self.hash_len = hash_len;
        self
    }

    pub fn compress(&self, state: &mut [u32; 8], block: &[u32; 32]) {
        // Save the state before compressing
        let saved_state = state.clone();
        // Mix for the specified number of rounds
        h1(state, block, self.rounds);
        h2(state, block, self.rounds);
        h3(state, block, self.rounds);
        if self.rounds > 3 {
            h4(state, block, self.rounds);
        }
        if self.rounds > 4 {
            h5(state, block, self.rounds);
        }
        // Add the saved state to the current state
        for (current_word, saved_word) in state.iter_mut().zip(saved_state.iter()) {
            *current_word = current_word.wrapping_add(*saved_word)
        }
    }
}

impl ClassicHasher for Haval {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(
            self.rounds == 3 || self.rounds == 4 || self.rounds == 5,
            "rounds must be 3, 4, or 5"
        );
        assert!(
            self.hash_len % 4 == 0 && self.hash_len >= 16 && self.hash_len <= 32,
            "output length is in bytes and must be 16, 20, 24, 28, or 32"
        );

        let mut input = bytes.to_vec();
        haval_padding(&mut input, self.hash_len as u8, self.rounds as u8);

        let mut state = D;

        for block in input.chunks_exact(128) {
            let mut x = [0u32; 32];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
                *elem = u32::from_le_bytes(chunk.try_into().unwrap());
            }
            self.compress(&mut state, &x)
        }

        if self.hash_len == 32 {
            finalize_256(&state)
        } else if self.hash_len == 28 {
            finalize_224(&state)
        } else if self.hash_len == 24 {
            finalize_192(&state)
        } else if self.hash_len == 20 {
            finalize_160(&state)
        } else if self.hash_len == 16 {
            finalize_128(&state)
        } else {
            unreachable!("output length is in bytes and must be 16, 20, 24, 28, or 32")
        }
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod haval_tests {
    use super::*;

    #[test]
    fn test_haval_256_5() {
        let hasher = Haval::default().rounds(5).hash_len(32);
        assert_eq!(
            "be417bb4dd5cfb76c7126f4f8eeb1553a449039307b1a3cd451dbfdc0fbbe330",
            hasher.hash_bytes_from_string("").unwrap()
        );
    }

    #[test]
    fn test_haval_128_3() {
        let hasher = Haval::default().rounds(3).hash_len(16);
        assert_eq!(
            "c68f39913f901f3ddf44c707357a7d70",
            hasher.hash_bytes_from_string("").unwrap()
        );
    }
}
