use utils::byte_formatting::fill_u32s_le;

use super::auxiliary::haval_arrays::D;
use crate::{
    auxiliary::haval_functions::{
        finalize_128, finalize_160, finalize_192, finalize_224, finalize_256, h1, h2, h3, h4, h5,
        haval_padding,
    },
    traits::StatefulHasher,
};

pub fn compress(state: &mut [u32; 8], block: &[u32; 32], rounds: u32) {
    // Save the state before compressing
    let saved_state = state.clone();
    // Mix for the specified number of rounds
    h1(state, block, rounds);
    h2(state, block, rounds);
    h3(state, block, rounds);
    if rounds >= 4 {
        h4(state, block, rounds);
    }
    if rounds == 5 {
        h5(state, block, rounds);
    }
    // Add the saved state to the current state
    for (current_word, saved_word) in state.iter_mut().zip(saved_state.into_iter()) {
        *current_word = current_word.wrapping_add(saved_word)
    }
}

pub struct Haval {
    pub rounds: u32,
    pub hash_len: u32,
    pub state: [u32; 8],
    buffer: Vec<u8>,
}

impl Default for Haval {
    fn default() -> Self {
        Self {
            rounds: 5,
            hash_len: 32,
            state: D,
            buffer: Vec::new(),
        }
    }
}

impl Haval {
    pub fn init(hash_len: u32, rounds: u32) -> Self {
        assert!([3, 4, 5].contains(&rounds), "rounds must be 3, 4, or 5");
        assert!(
            [32, 28, 24, 20, 16].contains(&hash_len),
            "hash_len must be 16, 20, 24, 28, or 32"
        );
        Self {
            rounds,
            hash_len,
            state: D,
            buffer: Vec::new(),
        }
    }

    pub fn init_256(rounds: u32) -> Self {
        Self::init(32, rounds)
    }

    pub fn init_224(rounds: u32) -> Self {
        Self::init(28, rounds)
    }

    pub fn init_192(rounds: u32) -> Self {
        Self::init(24, rounds)
    }

    pub fn init_160(rounds: u32) -> Self {
        Self::init(20, rounds)
    }

    pub fn init_128(rounds: u32) -> Self {
        Self::init(16, rounds)
    }
}

impl StatefulHasher for Haval {
    fn update(&mut self, mut bytes: &[u8]) {
        let mut x = [0u32; 32];
        crate::compression_routine!(self.buffer, bytes, 1024, {
            fill_u32s_le(&mut x, &self.buffer);
            compress(&mut self.state, &x, self.rounds);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        haval_padding(&mut self.buffer, self.hash_len as u8, self.rounds as u8);

        let mut x = [0u32; 32];
        for block in self.buffer.chunks_exact(128) {
            fill_u32s_le(&mut x, &block);
            compress(&mut self.state, &x, self.rounds)
        }

        if self.hash_len == 32 {
            finalize_256(&self.state)
        } else if self.hash_len == 28 {
            finalize_224(&self.state)
        } else if self.hash_len == 24 {
            finalize_192(&self.state)
        } else if self.hash_len == 20 {
            finalize_160(&self.state)
        } else if self.hash_len == 16 {
            finalize_128(&self.state)
        } else {
            unreachable!("output length is in bytes and must be 16, 20, 24, 28, or 32")
        }
    }
    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    haval_256_5_empty, Haval::init_256(5), b"",
    "be417bb4dd5cfb76c7126f4f8eeb1553a449039307b1a3cd451dbfdc0fbbe330";

    haval_128_3_empty, Haval::init_128(3), b"",
    "c68f39913f901f3ddf44c707357a7d70";
);
