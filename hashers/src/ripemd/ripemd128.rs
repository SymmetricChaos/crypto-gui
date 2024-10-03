use crate::traits::ClassicHasher;
use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_le};

use super::{f, PERM, PERM_PRIME, ROL, ROL_PRIME};

#[derive(Clone)]
pub struct RipeMd128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for RipeMd128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl RipeMd128 {
    pub const K: [u32; 4] = [0x00000000, 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc];

    pub const K_PRIME: [u32; 4] = [0x50a28be6, 0x5c4dd124, 0x6d703ef3, 0x00000000];

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub(super) fn left_chain(j: usize, s: &mut [u32; 4], block: [u32; 16]) {
        let t = (s[0]
            .wrapping_add(f(j, s[1], s[2], s[3]))
            .wrapping_add(block[PERM[j]])
            .wrapping_add(Self::K[j / 16]))
        .rotate_left(ROL[j]);
        s[0] = s[3];
        s[3] = s[2];
        s[2] = s[1];
        s[1] = t;
    }

    pub(super) fn right_chain(j: usize, s: &mut [u32; 4], block: [u32; 16]) {
        let t = (s[0]
            .wrapping_add(f(63 - j, s[1], s[2], s[3]))
            .wrapping_add(block[PERM_PRIME[j]])
            .wrapping_add(Self::K_PRIME[j / 16]))
        .rotate_left(ROL_PRIME[j]);
        s[0] = s[3];
        s[3] = s[2];
        s[2] = s[1];
        s[1] = t;
    }

    pub fn compress(state: &mut [u32; 4], block: [u32; 16]) {
        let mut l = state.clone();
        let mut r = state.clone();

        for j in 0..64 {
            Self::left_chain(j, &mut l, block);
            Self::right_chain(j, &mut r, block);
        }

        let t = r[3].wrapping_add(state[1]).wrapping_add(l[2]);
        state[1] = state[2].wrapping_add(l[3]).wrapping_add(r[0]);
        state[2] = state[3].wrapping_add(l[0]).wrapping_add(r[1]);
        state[3] = state[0].wrapping_add(l[1]).wrapping_add(r[2]);
        state[0] = t;
    }
}

impl ClassicHasher for RipeMd128 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        md_strengthening_64_le(&mut input, 64);

        let mut state: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

        for chunk in input.chunks_exact(64) {
            let mut block = [0u32; 16];
            utils::byte_formatting::fill_u32s_le(&mut block, &chunk);
            Self::compress(&mut state, block)
        }

        let mut out = Vec::with_capacity(16);
        for word in state {
            out.extend(word.to_le_bytes())
        }
        out
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test_128_1, RipeMd128::default(), "",
    "cdf26213a150dc3ecb610f18f6b38b46";
    test_128_2, RipeMd128::default(), "a",
    "86be7afa339d0fc7cfc785e72f578d33";
    test_128_3, RipeMd128::default(), "abc",
    "c14a12199c66e4ba84636b0f69144c77";
    test_128_4, RipeMd128::default(), "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "3f45ef194732c2dbb2c4a2c769795fa3";
);
