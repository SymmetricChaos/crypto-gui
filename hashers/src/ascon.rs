use utils::{
    byte_formatting::{u64s_to_bytes_be, ByteFormat},
    padding::bit_padding,
};

use crate::traits::ClassicHasher;

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

#[derive(Debug, Clone)]
pub struct AsconHashState {
    state: [u64; 5],
}

// Shortcut indexing
impl std::ops::Index<usize> for AsconHashState {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.state[index]
    }
}

impl std::ops::IndexMut<usize> for AsconHashState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.state[index]
    }
}

// Default to precomputed state
impl Default for AsconHashState {
    fn default() -> Self {
        Self {
            state: [
                0xee9398aadb67f03d,
                0x8bb21831c60f1002,
                0xb48a92db98d5da62,
                0x43189921b8f8e3e8,
                0x348fa5c9d525e140,
            ],
        }
    }
}

impl AsconHashState {
    const A: usize = 12; // initialization rounds

    pub fn rounds_a(&mut self) {
        for i in 0..Self::A {
            self.transform(i);
        }
    }

    pub fn transform(&mut self, i: usize) {
        // round constant
        self[2] ^= C[i];
        // substitution
        self.sbox();
        // linear diffusion
        self.linear_diffusor();
    }

    // The sbox works across words
    // It effectively take the nth bit of each word, interprets it as a 5-bit word, then substitutes it
    pub fn sbox(&mut self) {
        self[0] ^= self[4];
        self[4] ^= self[3];
        self[2] ^= self[1];

        let mut t = self.state.clone();
        for i in 0..5 {
            t[i] ^= !self[(i + 1) % 5] & self[(i + 2) % 5];
        }

        t[1] ^= t[0];
        t[0] ^= t[4];
        t[3] ^= t[2];
        t[2] = !t[2];

        self.state = t;
    }

    // This diffuses bits within each word of state
    pub fn linear_diffusor(&mut self) {
        for i in 0..5 {
            self[i] ^= self[i].rotate_right(ROTS[i].0) ^ self[i].rotate_right(ROTS[i].1);
        }
    }
}

pub struct AsconHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for AsconHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for AsconHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = AsconHashState::default();

        todo!()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod ascon_tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let mut state = AsconHashState::default();
        state.state = [0x00400c0000000100, 0, 0, 0, 0];
        state.rounds_a();
        assert_eq!(
            [
                0xee9398aadb67f03d,
                0x8bb21831c60f1002,
                0xb48a92db98d5da62,
                0x43189921b8f8e3e8,
                0x348fa5c9d525e140
            ],
            state.state
        )
    }
}
