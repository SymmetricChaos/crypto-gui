use crate::traits::StatefulHasher;
use utils::byte_formatting::make_u64s_le;

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

const BLOCK_LEN_32: usize = 4;
pub struct Murmur3_32 {
    state: u32,
    buffer: Vec<u8>,
    bytes_taken: u32,
}

impl Murmur3_32 {
    pub fn init(seed: &[u8]) -> Self {
        Self {
            state: u32::from_le_bytes(seed.try_into().expect("seed must be exactly four bytes")),
            buffer: Vec::with_capacity(BLOCK_LEN_32),
            bytes_taken: 0,
        }
    }

    /// Initialize with state set to zero
    pub fn init_zero() -> Self {
        Self {
            state: 0,
            buffer: Vec::with_capacity(BLOCK_LEN_32),
            bytes_taken: 0,
        }
    }
}

impl StatefulHasher for Murmur3_32 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let chunks = self.buffer.chunks_exact(BLOCK_LEN_32);
        let rem = chunks.remainder().to_vec();
        // For each full chunk mix it into the state and then mix the state
        for chunk in chunks {
            self.bytes_taken += 4;
            let k = u32::from_le_bytes(chunk.try_into().unwrap());
            self.state ^= block_mix(k);
            self.state = state_mix(self.state);
        }
        self.buffer = rem;
    }

    // fn update(&mut self, mut bytes: &[u8]) {
    //     crate::compression_routine!(self.buffer, bytes, BLOCK_LEN_32, {
    //         self.bytes_taken += 4;
    //         let k = u32::from_le_bytes(self.buffer.clone().try_into().unwrap());
    //         self.state ^= block_mix(k);
    //         self.state = state_mix(self.state);
    //     });
    // }

    fn finalize(mut self) -> Vec<u8> {
        self.bytes_taken += self.buffer.len() as u32;
        // Load any bytes in the remainder and mix them
        let mut k = 0_u32;
        for byte in self.buffer {
            k = k << 8;
            k |= byte as u32;
        }
        self.state ^= block_mix(k);
        // XOR in the length in bytes
        self.state ^= self.bytes_taken;

        self.state = final_mix(self.state);
        self.state.to_be_bytes().to_vec()
    }

    crate::stateful_hash_helpers!();
}

const BLOCK_LEN_128: usize = 16;

pub struct Murmur3_128 {
    state: [u64; 2],
    buffer: Vec<u8>,
    bytes_taken: u64,
}

impl Murmur3_128 {
    pub fn init(seed: &[u8]) -> Self {
        let s = u32::from_le_bytes(seed.try_into().expect("seed must be exactly four bytes"));
        Self {
            state: [s as u64; 2],
            buffer: Vec::with_capacity(BLOCK_LEN_128),
            bytes_taken: 0,
        }
    }

    /// Initialize with state set to zero
    pub fn init_zero() -> Self {
        Self {
            state: [0; 2],
            buffer: Vec::with_capacity(BLOCK_LEN_128),
            bytes_taken: 0,
        }
    }
}

impl StatefulHasher for Murmur3_128 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let chunks = self.buffer.chunks_exact(BLOCK_LEN_128);
        let rem = chunks.remainder().to_vec();
        // For each full chunk mix it into the state and then mix the state
        for chunk in chunks {
            self.bytes_taken += 16;
            let k = make_u64s_le::<2>(chunk);

            self.state[0] ^= block_mix_64_0(k[0]);
            self.state[0] = self.state[0].rotate_left(27);
            self.state[0] = self.state[0].wrapping_add(self.state[1]);
            self.state[0] = self.state[0].wrapping_mul(5).wrapping_add(0x52dce729);

            self.state[1] ^= block_mix_64_1(k[1]);
            self.state[1] = self.state[1].rotate_left(31);
            self.state[1] = self.state[1].wrapping_add(self.state[0]);
            self.state[1] = self.state[1].wrapping_mul(5).wrapping_add(0x38495ab5);
        }
        self.buffer = rem;
    }

    fn finalize(mut self) -> Vec<u8> {
        self.bytes_taken += self.buffer.len() as u64;

        // Load any bytes in the remainder and mix them
        let mut k = 0_u64;
        for byte in self.buffer.iter().skip(8).take(8).rev() {
            k = k << 8;
            k |= *byte as u64;
        }
        self.state[1] ^= block_mix_64_1(k);

        k = 0;
        for byte in self.buffer.iter().take(8).rev() {
            k = k << 8;
            k |= *byte as u64;
        }
        self.state[0] ^= block_mix_64_0(k);

        // XOR in the length in bytes
        self.state[0] ^= self.bytes_taken;
        self.state[1] ^= self.bytes_taken;

        // Final mix
        self.state[0] = self.state[0].wrapping_add(self.state[1]);
        self.state[1] = self.state[1].wrapping_add(self.state[0]);
        self.state[0] = final_mix_64(self.state[0]);
        self.state[1] = final_mix_64(self.state[1]);
        self.state[0] = self.state[0].wrapping_add(self.state[1]);
        self.state[1] = self.state[1].wrapping_add(self.state[0]);

        self.state
            .into_iter()
            .flat_map(|w| w.to_be_bytes())
            .collect()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1, Murmur3_32::init_zero(), b"",
    "00000000";
    test2, Murmur3_32::init(&[1,0,0,0]), b"",
    "514e28b7";
    test3, Murmur3_32::init_zero(), b"hello",
    "248bfa47";
    test4, Murmur3_32::init(&[1,0,0,0]), b"hello",
    "bb4abcad";
    test5, Murmur3_32::init_zero(), b"The quick brown fox jumps over the lazy dog.",
    "d5c48bfc";
    test6, Murmur3_32::init(&[1,0,0,0]), b"The quick brown fox jumps over the lazy dog.",
    "846f6a36";

    test7, Murmur3_128::init_zero(), b"",
    "00000000000000000000000000000000";
    test8, Murmur3_128::init(&[1,0,0,0]), b"",
    "4610abe56eff5cb551622daa78f83583";
    test9, Murmur3_128::init_zero(), b"hello",
    "cbd8a7b341bd9b025b1e906a48ae1d19";
    test10, Murmur3_128::init(&[1,0,0,0]), b"hello",
    "a78ddff5adae8d10128900ef20900135";
    test11, Murmur3_128::init_zero(), b"The quick brown fox jumps over the lazy dog.",
    "cd99481f9ee902c9695da1a38987b6e7";
    test12, Murmur3_128::init(&[1,0,0,0]), b"The quick brown fox jumps over the lazy dog.",
    "fb3325171f9744daaaf8b92a5f722952";
);
