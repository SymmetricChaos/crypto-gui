use utils::{
    byte_formatting::ByteFormat,
    padding::{self, zero_padding},
};

use crate::traits::ClassicHasher;

fn a(y: [u64; 4]) -> [u64; 4] {
    [y[0] ^ y[1], y[3], y[2], y[1]]
}

fn p() {}

fn e() {}

fn compress(h: [u8; 32], m: [u8; 32]) -> [u8; 32] {
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
        zero_padding(&mut input, 32);

        let mut h = self.h1;
        let mut ctrl = [0; 32];

        // Take input in 256-bit blocks
        for block in input.chunks_exact(32) {
            for i in 0..32 {
                ctrl[i] ^= block[i]
            }
            h = compress(h, block.try_into().unwrap())
        }

        // Compress in the length of the input
        let mut l = [0; 32];
        for (i, b) in (bytes.len() as u64).to_be_bytes().iter().enumerate() {
            l[i + 23] = *b
        }
        h = compress(h, l);

        // Compress in the check value
        compress(h, ctrl).to_vec()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1, Gost::default(),
    "The quick brown fox jumps over the lazy dog",
    "77b7fa410c9ac58a25f49bca7d0468c9296529315eaca76bd1a10f376d1f4294";

    test2, Gost::default(),
    "This is message, length=32 bytes",
    "b1c466d37519b82e8319819ff32595e047a28cb6f83eff1c6916a815a637fffa";

    test3, Gost::default(),
    "Suppose the original message has length = 50 bytes",
    "471aba57a60a770d3a76130635c1fbea4ef14de51f78b4ae57dd893b62f55208";
);
