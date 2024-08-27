use crate::traits::ClassicHasher;
use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_le};

#[derive(Clone)]
pub struct RipeMd160 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub extended: bool,
}

impl Default for RipeMd160 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            extended: false,
        }
    }
}

impl RipeMd160 {
    pub const K: [u32; 5] = [0x00000000, 0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xA953FD4E];

    pub const K_PRIME: [u32; 5] = [0x50A28BE6, 0x5C4DD124, 0x6D703EF3, 0x7A6D76E9, 0x00000000];

    pub const PERM: [usize; 80] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9,
        5, 2, 14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8,
        12, 4, 13, 3, 7, 15, 14, 5, 6, 2, 4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
    ];

    pub const PERM_PRIME: [usize; 80] = [
        5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8,
        12, 4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11,
        15, 0, 5, 12, 2, 13, 9, 7, 10, 14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
    ];

    pub const ROL: [u32; 80] = [
        11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12,
        15, 9, 11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, 11, 12, 14,
        15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11,
        8, 5, 6,
    ];

    pub const ROL_PRIME: [u32; 80] = [
        8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7, 12, 8, 9, 11, 7, 7,
        12, 7, 6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11,
        14, 14, 6, 14, 6, 9, 12, 9, 12, 5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13,
        11, 11,
    ];

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn f(j: usize, x: u32, y: u32, z: u32) -> u32 {
        match j / 16 {
            0 => x ^ y ^ z,
            1 => (x & y) | (!x & z),
            2 => (x | !y) ^ z,
            3 => (x & z) | (y & !z),
            4 => x ^ (y | !z),
            _ => unreachable!(),
        }
    }

    pub fn left_chain(j: usize, s: &mut [u32; 5], block: [u32; 16]) {
        let t = (s[0]
            .wrapping_add(Self::f(j, s[1], s[2], s[3]))
            .wrapping_add(block[Self::PERM[j]])
            .wrapping_add(Self::K[j / 16]))
        .rotate_left(Self::ROL[j])
        .wrapping_add(s[4]);
        s[0] = s[4];
        s[4] = s[3];
        s[3] = s[2].rotate_left(10);
        s[2] = s[1];
        s[1] = t;
    }

    pub fn right_chain(j: usize, s: &mut [u32; 5], block: [u32; 16]) {
        let t = (s[0]
            .wrapping_add(Self::f(79 - j, s[1], s[2], s[3]))
            .wrapping_add(block[Self::PERM_PRIME[j]])
            .wrapping_add(Self::K_PRIME[j / 16]))
        .rotate_left(Self::ROL_PRIME[j])
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

        let mut state: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];

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

    // RipeMd::default(), test_320_1, "",
    // "22d65d5661536cdc75c1fdf5c6de7b41b9f27325ebc61e8557177d705a0ec880151c3a32a00899b8";
    // RipeMd::default(), test_320_2, "a",
    // "ce78850638f92658a5a585097579926dda667a5716562cfcf6fbe77f63542f99b04705d6970dff5d";
    // RipeMd::default(), test_320_3, "abc",
    // "de4c01b3054f8930a79d09ae738e92301e5a17085beffdc1b8d116713e74f82fa942d64cdbc4682d";
    // RipeMd::default(), test_320_4, "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    // "557888af5f6d8ed62ab66945c6d2a0a47ecd5341e915eb8fea1d0524955f825dc717e4a008ab2d42";
);
