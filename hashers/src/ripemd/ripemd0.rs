use crate::traits::ClassicHasher;
use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_le};

// Similiar but not identical to the strengthened versions
pub const PERM: [usize; 48] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5,
    14, 2, 11, 8, 3, 10, 2, 4, 9, 15, 8, 1, 14, 7, 0, 6, 11, 13, 5, 12,
];

// Similiar but not identical to the strengthened versions
pub const ROL: [u32; 48] = [
    11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15,
    9, 7, 11, 13, 12, 11, 13, 14, 7, 14, 9, 13, 15, 6, 8, 13, 6, 12, 5, 7, 5,
];

// Selectable boolean function
fn f(j: usize, x: u32, y: u32, z: u32) -> u32 {
    match j / 16 {
        0 => (x & y) | (!x & z),
        1 => (x & y) | (x & z) | (y & z),
        2 => x ^ y ^ z,
        _ => unreachable!(),
    }
}

fn ff(j: usize, s: &mut [u32; 4], block: [u32; 16], k: u32) {
    s[0] = (s[0]
        .wrapping_add(f(j, s[1], s[2], s[3]))
        .wrapping_add(block[PERM[j]].wrapping_add(k)))
    .rotate_left(ROL[j]);
    s.rotate_right(1);
}

// The name RIPEMD-0 is not used in the literature
// It is just convenient to distinguish it from the
// selectable RIPEMD function
pub struct RipeMd0 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for RipeMd0 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl RipeMd0 {
    pub const K: [u32; 4] = [0x50a28be6, 0x5a827999, 0x6ed9eba1, 0x5c4dd124];

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn compress(state: &mut [u32; 4], block: [u32; 16]) {
        let mut l = state.clone();
        let mut r = state.clone();

        for j in 0..16 {
            ff(j, &mut l, block, 0);
            ff(j, &mut r, block, Self::K[0]);
        }
        for j in 0..16 {
            ff(j + 16, &mut l, block, Self::K[1]);
            ff(j + 16, &mut r, block, 0);
        }
        for j in 0..16 {
            ff(j + 32, &mut l, block, Self::K[2]);
            ff(j + 32, &mut r, block, Self::K[3]);
        }

        let t = state[1].wrapping_add(l[2]).wrapping_add(r[3]);
        state[1] = state[2].wrapping_add(l[3]).wrapping_add(r[0]);
        state[2] = state[3].wrapping_add(l[0]).wrapping_add(r[1]);
        state[3] = state[0].wrapping_add(l[1]).wrapping_add(r[2]);
        state[0] = t;
    }
}

impl ClassicHasher for RipeMd0 {
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
    test_0_1, RipeMd0::default(),"",
    "9f73aa9b372a9dacfb86a6108852e2d9";
    test_0_2, RipeMd0::default(), "a",
    "486f74f790bc95ef7963cd2382b4bbc9";
    test_0_3, RipeMd0::default(), "abc",
    "3f14bad4c2f9b0ea805e5485d3d6882d";
    test_0_4, RipeMd0::default(), "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "dfd6b45f60fe79bbbde87c6bfc6580a5";
);
