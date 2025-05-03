use super::{f, PERM, PERM_PRIME, ROL, ROL_PRIME};
use crate::traits::StatefulHasher;

const BLOCK_LEN: usize = 64;

#[derive(Clone)]
pub struct RipeMd160 {
    state: [u32; 5],
    buffer: Vec<u8>,
    bits_taken: u64,
}

impl RipeMd160 {
    pub const K: [u32; 5] = [0x00000000, 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xa953fd4e];

    pub const K_PRIME: [u32; 5] = [0x50a28be6, 0x5c4dd124, 0x6d703ef3, 0x7a6d76e9, 0x00000000];

    pub fn init() -> Self {
        Self {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0],
            buffer: Vec::new(),
            bits_taken: 0,
        }
    }

    pub(super) fn left_chain(j: usize, s: &mut [u32; 5], block: [u32; 16]) {
        let t = (s[0]
            .wrapping_add(f(j, s[1], s[2], s[3]))
            .wrapping_add(block[PERM[j]])
            .wrapping_add(Self::K[j / 16]))
        .rotate_left(ROL[j])
        .wrapping_add(s[4]);
        s[0] = s[4];
        s[4] = s[3];
        s[3] = s[2].rotate_left(10);
        s[2] = s[1];
        s[1] = t;
    }

    pub(super) fn right_chain(j: usize, s: &mut [u32; 5], block: [u32; 16]) {
        let t = (s[0]
            .wrapping_add(f(79 - j, s[1], s[2], s[3]))
            .wrapping_add(block[PERM_PRIME[j]])
            .wrapping_add(Self::K_PRIME[j / 16]))
        .rotate_left(ROL_PRIME[j])
        .wrapping_add(s[4]);
        s[0] = s[4];
        s[4] = s[3];
        s[3] = s[2].rotate_left(10);
        s[2] = s[1];
        s[1] = t;
    }

    pub fn compress(state: &mut [u32; 5], block: [u32; 16]) {
        let mut l = state.clone();
        let mut r = state.clone();

        for j in 0..80 {
            Self::left_chain(j, &mut l, block);
            Self::right_chain(j, &mut r, block);
        }

        let t = state[1].wrapping_add(l[2]).wrapping_add(r[3]);
        state[1] = state[2].wrapping_add(l[3]).wrapping_add(r[4]);
        state[2] = state[3].wrapping_add(l[4]).wrapping_add(r[0]);
        state[3] = state[4].wrapping_add(l[0]).wrapping_add(r[1]);
        state[4] = state[0].wrapping_add(l[1]).wrapping_add(r[2]);
        state[0] = t;
    }
}

impl StatefulHasher for RipeMd160 {
    fn update(&mut self, mut bytes: &[u8]) {
        let mut block = [0u32; 16];
        crate::compression_routine!(self.buffer, bytes, BLOCK_LEN, {
            self.bits_taken += 512;
            utils::byte_formatting::fill_u32s_le(&mut block, &self.buffer);
            Self::compress(&mut self.state, block)
        });
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
            Self::compress(&mut self.state, block)
        }

        let mut out = Vec::with_capacity(16);
        for word in self.state {
            out.extend(word.to_le_bytes())
        }
        out
    }
}

crate::stateful_hash_tests!(
    test_160_1, RipeMd160::init(), b"",
    "9c1185a5c5e9fc54612808977ee8f548b2258d31";
    test_160_2, RipeMd160::init(), b"a",
    "0bdc9d2d256b3ee9daae347be6f4dc835a467ffe";
    test_160_3, RipeMd160::init(), b"abc",
    "8eb208f7e05d987a9b044a8e98c6b087f15a0bfc";
    test_160_4, RipeMd160::init(), b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "9b752e45573d4b39f4dbd3323cab82bf63326bfb";
);
