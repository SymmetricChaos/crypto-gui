use crate::traits::ClassicHasher;
use utils::byte_formatting::{make_u64s_le, ByteFormat};

fn final_mix(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x85ebca6b);
    x ^= x >> 13;
    x = x.wrapping_mul(0xc2b2ae35);
    x ^= x >> 16;
    x
}

fn final_mix_64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

fn block_mix(mut x: u32) -> u32 {
    x = x.wrapping_mul(0xcc9e2d51);
    x = x.rotate_left(15);
    x = x.wrapping_mul(0x1b873593);
    x
}

fn block_mix_64_0(mut x: u64) -> u64 {
    x = x.wrapping_mul(0x87c37b91114253d5);
    x = x.rotate_left(31);
    x = x.wrapping_mul(0x4cf5ad432745937f);
    x
}

fn block_mix_64_1(mut x: u64) -> u64 {
    x = x.wrapping_mul(0x4cf5ad432745937f);
    x = x.rotate_left(33);
    x = x.wrapping_mul(0x87c37b91114253d5);
    x
}

fn state_mix(mut x: u32) -> u32 {
    x = x.rotate_left(13);
    x = x.wrapping_mul(5).wrapping_add(0xe6546b64);
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

    pub fn with_seed(mut self, bytes: [u8; 4]) -> Self {
        self.seed = u32::from_le_bytes(bytes);
        self
    }

    pub fn with_seed_u32(mut self, seed: u32) -> Self {
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
            state ^= block_mix(k);
            state = state_mix(state);
        }
        // Load any bytes in the remainder and mix them
        let mut k = 0_u32;
        for byte in rem {
            k = k << 8;
            k |= *byte as u32;
        }
        state ^= block_mix(k);

        // XOR in the length in bytes
        state ^= bytes.len() as u32;

        state = final_mix(state);
        state.to_be_bytes().to_vec()
    }

    crate::hash_bytes_from_string! {}
}

pub struct Murmur3_128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub seed: u32,
}

impl Default for Murmur3_128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            seed: 0,
        }
    }
}

impl Murmur3_128 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn with_seed(mut self, bytes: [u8; 4]) -> Self {
        self.seed = u32::from_le_bytes(bytes);
        self
    }

    pub fn with_seed_u32(mut self, seed: u32) -> Self {
        self.seed = seed;
        self
    }
}

impl ClassicHasher for Murmur3_128 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = [self.seed as u64, self.seed as u64];

        // Divide the input into 32-bit chunks and save the remainder
        let chunks = bytes.chunks_exact(16);
        let rem = chunks.remainder();
        // For each full chunk mix it into the state and then mix the state
        for block in chunks {
            let k = make_u64s_le::<2>(block);

            state[0] ^= block_mix_64_0(k[0]);
            state[0] = state[0].rotate_left(27);
            state[0] = state[0].wrapping_add(state[1]);
            state[0] = state[0].wrapping_mul(5).wrapping_add(0x52dce729);

            state[1] ^= block_mix_64_1(k[1]);
            state[1] = state[1].rotate_left(31);
            state[1] = state[1].wrapping_add(state[0]);
            state[1] = state[1].wrapping_mul(5).wrapping_add(0x38495ab5);
        }

        // Load any bytes in the remainder and mix them

        let mut k = 0_u64;
        for byte in rem.into_iter().skip(8).take(8).rev() {
            k = k << 8;
            k |= *byte as u64;
        }
        state[1] ^= block_mix_64_1(k);

        k = 0;
        for byte in rem.into_iter().take(8).rev() {
            k = k << 8;
            k |= *byte as u64;
        }
        state[0] ^= block_mix_64_0(k);

        // XOR in the length in bytes
        state[0] ^= bytes.len() as u64;
        state[1] ^= bytes.len() as u64;

        // Final mix
        state[0] = state[0].wrapping_add(state[1]);
        state[1] = state[1].wrapping_add(state[0]);
        state[0] = final_mix_64(state[0]);
        state[1] = final_mix_64(state[1]);
        state[0] = state[0].wrapping_add(state[1]);
        state[1] = state[1].wrapping_add(state[0]);

        state.into_iter().flat_map(|w| w.to_be_bytes()).collect()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1, Murmur3_32::default(), "",
    "00000000";
    test2, Murmur3_32::default().with_seed_u32(0x01), "",
    "514e28b7";
    test3, Murmur3_32::default(), "hello",
    "248bfa47";
    test4, Murmur3_32::default().with_seed_u32(0x01), "hello",
    "bb4abcad";
    test5, Murmur3_32::default(), "The quick brown fox jumps over the lazy dog.",
    "d5c48bfc";
    test6, Murmur3_32::default().with_seed_u32(0x01), "The quick brown fox jumps over the lazy dog.",
    "846f6a36";

    test7, Murmur3_128::default(), "",
    "00000000000000000000000000000000";
    test8, Murmur3_128::default().with_seed_u32(0x01), "",
    "4610abe56eff5cb551622daa78f83583";
    test9, Murmur3_128::default(), "hello",
    "cbd8a7b341bd9b025b1e906a48ae1d19";
    test10, Murmur3_128::default().with_seed_u32(0x01), "hello",
    "a78ddff5adae8d10128900ef20900135";
    test11, Murmur3_128::default(), "The quick brown fox jumps over the lazy dog.",
    "cd99481f9ee902c9695da1a38987b6e7";
    test12, Murmur3_128::default().with_seed_u32(0x01), "The quick brown fox jumps over the lazy dog.",
    "fb3325171f9744daaaf8b92a5f722952";
);
