use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

// Final mix of the state
fn fmix(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x85ebca6b);
    x ^= x >> 13;
    x = x.wrapping_mul(0xc2b2ae35);
    x ^= x >> 16;
    x
}

// Mixing of the block
fn bmix(mut x: u32) -> u32 {
    x = x.wrapping_mul(0xcc9e2d51);
    x = x.rotate_left(15);
    x = x.wrapping_mul(0x1b873593);
    x
}

pub struct Murmur3_32 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub seed: u32,
}

impl Default for Murmur3_32 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            seed: 0,
        }
    }
}

impl Murmur3_32 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self
    }
}

impl ClassicHasher for Murmur3_32 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = self.seed;

        // Divide the input into 32-bit chunks and save the remainder
        let chunks = bytes.chunks_exact(4);
        let rem = chunks.remainder();
        // For each full chunk mix it into the state and then mix the state
        for block in chunks {
            let k = u32::from_le_bytes(block.try_into().unwrap());
            state ^= bmix(k);
            state = state.rotate_left(13);
            state = state.wrapping_mul(5).wrapping_add(0xe6546b64);
        }
        // Load any bytes in the remainder and mix them
        let mut final_block = 0_u32;
        for byte in rem {
            final_block = final_block << 8;
            final_block |= *byte as u32;
        }
        state ^= bmix(final_block);

        // XOR in the length in bytes
        state ^= bytes.len() as u32;
        // Final mix
        state = fmix(state);
        state.to_be_bytes().to_vec()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1, Murmur3_32::default(), "",
    "00000000";
    test2, Murmur3_32::default().with_seed(0xffffffff), "",
    "81f16f39";
    test3, Murmur3_32::default(), "test",
    "ba6bd213";
    test4, Murmur3_32::default().with_seed(0x9747b28c), "test",
    "704b81dc";
);
