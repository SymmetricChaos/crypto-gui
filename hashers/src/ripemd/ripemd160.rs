use crate::traits::ClassicHasher;
use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_le};

use super::{f, PERM, PERM_PRIME, ROL, ROL_PRIME};

#[derive(Clone)]
pub struct RipeMd160 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for RipeMd160 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl RipeMd160 {
    pub const K: [u32; 5] = [0x00000000, 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xa953fd4e];

    pub const K_PRIME: [u32; 5] = [0x50a28be6, 0x5c4dd124, 0x6d703ef3, 0x7a6d76e9, 0x00000000];

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
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

impl ClassicHasher for RipeMd160 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        md_strengthening_64_le(&mut input, 64);

        let mut state: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

        for chunk in input.chunks_exact(64) {
            let mut block = [0u32; 16];
            for (elem, b) in block.iter_mut().zip(chunk.chunks_exact(4)) {
                *elem = u32::from_le_bytes(b.try_into().unwrap());
            }
            Self::compress(&mut state, block)
        }

        let mut out = Vec::with_capacity(20);
        for word in state {
            out.extend(word.to_le_bytes())
        }
        out
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    RipeMd160::default(), test_160_1, "",
    "9c1185a5c5e9fc54612808977ee8f548b2258d31";
    RipeMd160::default(), test_160_2, "a",
    "0bdc9d2d256b3ee9daae347be6f4dc835a467ffe";
    RipeMd160::default(), test_160_3, "abc",
    "8eb208f7e05d987a9b044a8e98c6b087f15a0bfc";
    RipeMd160::default(), test_160_4, "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "9b752e45573d4b39f4dbd3323cab82bf63326bfb";

);
