use utils::{
    byte_formatting::{fill_u32s_be, ByteFormat},
    padding::md_strengthening_64_be,
};

use crate::traits::ClassicHasher;

#[derive(Debug, Clone)]
pub struct Sha0 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Sha0 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Sha0 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    fn update(&self, state: &mut [u32; 5], block: &[u8]) {
        let mut v = state.clone();

        // Extract 16 words from the block and make them the first 16 values of the array
        let mut x = [0u32; 80];
        fill_u32s_be(&mut x[0..16], &block);

        // Extend the 16 words to 80 words
        for i in 16..80 {
            x[i] = x[i - 3] ^ x[i - 8] ^ x[i - 14] ^ x[i - 16]
        }

        // Apply 80 rounds of mixing
        for i in 0..80 {
            let mut f = 0;
            let mut g = 0;
            // Round functions and round constant are changed every 20 rounds
            if i < 20 {
                f = (v[1] & v[2]) | (!v[1] & v[3]);
                g = 0x5a827999;
            }
            if i >= 20 && i < 40 {
                f = v[1] ^ v[2] ^ v[3];
                g = 0x6ed9eba1;
            }
            if i >= 40 && i < 60 {
                f = (v[1] & v[2]) | (v[1] & v[3]) | (v[2] & v[3]);
                g = 0x8f1bbcdc;
            }
            if i >= 60 {
                f = v[1] ^ v[2] ^ v[3];
                g = 0xca62c1d6;
            }

            let t = v[0]
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(v[4])
                .wrapping_add(g)
                .wrapping_add(x[i]); // Each round a new word from the array x is added here
            v[4] = v[3];
            v[3] = v[2];
            v[2] = v[1].rotate_left(30);
            v[1] = v[0];
            v[0] = t;
        }

        for i in 0..5 {
            state[i] = state[i].wrapping_add(v[i]);
        }
    }
}

impl ClassicHasher for Sha0 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        md_strengthening_64_be(&mut input, 64);

        // Initialize variables
        let mut state: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

        // Process message in 64-byte (512-bit) blocks
        for block in input.chunks_exact(64) {
            self.update(&mut state, block)
        }

        let mut out = Vec::with_capacity(20);
        for word in state {
            out.extend(word.to_be_bytes())
        }
        out
    }

    crate::hash_bytes_from_string! {}
}
