use crate::traits::ClassicHasher;
use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_le};

#[derive(Clone)]
pub struct RipeMd128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub extended: bool,
}

impl Default for RipeMd128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            extended: false,
        }
    }
}

impl RipeMd128 {
    pub const K: [u32; 4] = [0x00000000, 0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC];

    pub const K_PRIME: [u32; 4] = [0x50A28BE6, 0x5C4DD124, 0x6D703EF3, 0x00000000];

    pub const PERM: [usize; 64] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9,
        5, 2, 14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8,
        12, 4, 13, 3, 7, 15, 14, 5, 6, 2,
    ];

    pub const PERM_PRIME: [usize; 64] = [
        5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8,
        12, 4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11,
        15, 0, 5, 12, 2, 13, 9, 7, 10, 14,
    ];

    pub const ROL: [u32; 64] = [
        11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12,
        15, 9, 11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, 11, 12, 14,
        15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12,
    ];

    pub const ROL_PRIME: [u32; 64] = [
        8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7, 12, 8, 9, 11, 7, 7,
        12, 7, 6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11,
        14, 14, 6, 14, 6, 9, 12, 9, 12, 5, 15, 8,
    ];

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    fn f(j: usize, x: u32, y: u32, z: u32) -> u32 {
        match j / 16 {
            0 => x ^ y ^ z,
            1 => (x & y) | (!x & z),
            2 => (x | !y) ^ z,
            3 => (x & z) | (y & !z),
            4 => x ^ (y | !z),
            _ => unreachable!(),
        }
    }

    pub fn left_chain(j: usize, s: &mut [u32; 4], block: [u32; 16]) {
        let t = (s[0]
            .wrapping_add(Self::f(j, s[1], s[2], s[3]))
            .wrapping_add(block[Self::PERM[j]])
            .wrapping_add(Self::K[j / 16]))
        .rotate_left(Self::ROL[j]);
        s[0] = s[3];
        s[3] = s[2];
        s[2] = s[1];
        s[1] = t;
    }

    pub fn right_chain(j: usize, s: &mut [u32; 4], block: [u32; 16]) {
        let t = (s[0]
            .wrapping_add(Self::f(63 - j, s[1], s[2], s[3]))
            .wrapping_add(block[Self::PERM_PRIME[j]])
            .wrapping_add(Self::K_PRIME[j / 16]))
        .rotate_left(Self::ROL_PRIME[j]);
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

        let mut state: [u32; 4] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];

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
    RipeMd128::default(), test_128_1, "",
    "cdf26213a150dc3ecb610f18f6b38b46";
    RipeMd128::default(), test_128_2, "a",
    "86be7afa339d0fc7cfc785e72f578d33";
    RipeMd128::default(), test_128_3, "abc",
    "c14a12199c66e4ba84636b0f69144c77";
    RipeMd128::default(), test_128_4, "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "3f45ef194732c2dbb2c4a2c769795fa3";

    // RipeMd::default(), test_256_1, "",
    // "02ba4c4e5f8ecd1877fc52d64d30e37a2d9774fb1e5d026380ae0168e3c5522d";
    // RipeMd::default(), test_256_2, "a",
    // "f9333e45d857f5d90a91bab70a1eba0cfb1be4b0783c9acfcd883a9134692925";
    // RipeMd::default(), test_256_3, "abc",
    // "afbd6e228b9d8cbbcef5ca2d03e6dba10ac0bc7dcbe4680e1e42d2e975459b65";
    // RipeMd::default(), test_256_4, "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    // "06fdcc7a409548aaf91368c06a6275b553e3f099bf0ea4edfd6778df89a890dd";
);
