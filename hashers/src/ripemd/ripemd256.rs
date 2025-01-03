use super::ripemd128::RipeMd128;
use crate::traits::StatefulHasher;

#[derive(Clone)]
pub struct RipeMd256 {
    state_l: [u32; 4],
    state_r: [u32; 4],
    buffer: Vec<u8>,
    bits_taken: u64,
}

impl RipeMd256 {
    pub fn init() -> Self {
        Self {
            state_l: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            state_r: [0x76543210, 0xfedcba98, 0x89abcdef, 0x01234567],
            buffer: Vec::new(),
            bits_taken: 0,
        }
    }

    pub fn compress(state_l: &mut [u32; 4], state_r: &mut [u32; 4], block: [u32; 16]) {
        let mut l = state_l.clone();
        let mut r = state_r.clone();
        for j in 0..64 {
            // Exact same round functions as RIPEMD-128
            RipeMd128::left_chain(j, &mut l, block);
            RipeMd128::right_chain(j, &mut r, block);
            // While in RIPEMD-128 the l and r arrays and mixed together
            // at the end of the compression function that is not possible
            // here since both are needed for output. Instead at the end of
            // each 16 step round a word is swapped between them.
            if j == 15 {
                std::mem::swap(&mut l[0], &mut r[0])
            }
            if j == 31 {
                std::mem::swap(&mut l[1], &mut r[1])
            }
            if j == 47 {
                std::mem::swap(&mut l[2], &mut r[2])
            }
            if j == 63 {
                std::mem::swap(&mut l[3], &mut r[3])
            }
        }
        for i in 0..4 {
            state_l[i] = state_l[i].wrapping_add(l[i]);
            state_r[i] = state_r[i].wrapping_add(r[i]);
        }
    }
}

impl StatefulHasher for RipeMd256 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let chunks = self.buffer.chunks_exact(64);
        let rem = chunks.remainder().to_vec();
        let mut block = [0u32; 16];
        for chunk in chunks {
            self.bits_taken += 512;
            utils::byte_formatting::fill_u32s_le(&mut block, &chunk);
            Self::compress(&mut self.state_l, &mut self.state_r, block)
        }
        self.buffer = rem;
    }

    fn finalize(mut self) -> Vec<u8> {
        // MD Padding
        self.bits_taken += self.buffer.len() as u64 * 8;
        self.buffer.push(0x80);
        while (self.buffer.len() % 64) != 56 {
            self.buffer.push(0)
        }
        self.buffer.extend(self.bits_taken.to_le_bytes());

        let chunks = self.buffer.chunks_exact(64);
        let mut block = [0u32; 16];
        for chunk in chunks {
            self.bits_taken += 512;
            utils::byte_formatting::fill_u32s_le(&mut block, &chunk);
            Self::compress(&mut self.state_l, &mut self.state_r, block)
        }

        let mut out = Vec::with_capacity(32);
        for word in self.state_l {
            out.extend(word.to_le_bytes())
        }
        for word in self.state_r {
            out.extend(word.to_le_bytes())
        }
        out
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_256_1, RipeMd256::init(), b"",
    "02ba4c4e5f8ecd1877fc52d64d30e37a2d9774fb1e5d026380ae0168e3c5522d";
    test_256_2, RipeMd256::init(), b"a",
    "f9333e45d857f5d90a91bab70a1eba0cfb1be4b0783c9acfcd883a9134692925";
    test_256_3, RipeMd256::init(), b"abc",
    "afbd6e228b9d8cbbcef5ca2d03e6dba10ac0bc7dcbe4680e1e42d2e975459b65";
    test_256_4, RipeMd256::init(), b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "06fdcc7a409548aaf91368c06a6275b553e3f099bf0ea4edfd6778df89a890dd";
);
