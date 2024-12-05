use utils::{byte_formatting::ByteFormat, padding};

use crate::traits::ClassicHasher;

fn step_hash(h: [u8; 32], m: [u8; 32]) -> [u8; 32] {
    todo!()
}

#[derive(Debug, Clone)]
pub struct Gost {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub h1: [u8; 32],
}

impl Default for Gost {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            h1: [0; 32],
        }
    }
}

impl ClassicHasher for Gost {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Final block is padded with zeroes
        padding::zero_padding(&mut input, 32);

        let mut h = self.h1;

        let mut ctrl = [0; 32];

        // Take input in 256-bit blocks
        for block in input.chunks_exact(32) {
            for i in 0..32 {
                ctrl[i] += block[i]
            }
            h = step_hash(h, block.try_into().unwrap())
        }

        //h = step_hash(h, bytes.len())

        step_hash(h, ctrl).to_vec()
    }

    crate::hash_bytes_from_string! {}
}
