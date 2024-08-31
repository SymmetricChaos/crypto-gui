use super::ripemd128::RipeMd128;
use crate::traits::ClassicHasher;
use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_le};

#[derive(Clone)]
pub struct RipeMd256 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for RipeMd256 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl RipeMd256 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
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

impl ClassicHasher for RipeMd256 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        md_strengthening_64_le(&mut input, 64);

        let mut state_l: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
        let mut state_r: [u32; 4] = [0x76543210, 0xfedcba98, 0x89abcdef, 0x01234567];

        for chunk in input.chunks_exact(64) {
            let mut block = [0u32; 16];
            utils::byte_formatting::fill_u32s_le(&mut block, &chunk);
            Self::compress(&mut state_l, &mut state_r, block)
        }

        let mut out = Vec::with_capacity(32);
        for word in state_l {
            out.extend(word.to_le_bytes())
        }
        for word in state_r {
            out.extend(word.to_le_bytes())
        }
        out
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(

    RipeMd256::default(), test_256_1, "",
    "02ba4c4e5f8ecd1877fc52d64d30e37a2d9774fb1e5d026380ae0168e3c5522d";
    RipeMd256::default(), test_256_2, "a",
    "f9333e45d857f5d90a91bab70a1eba0cfb1be4b0783c9acfcd883a9134692925";
    RipeMd256::default(), test_256_3, "abc",
    "afbd6e228b9d8cbbcef5ca2d03e6dba10ac0bc7dcbe4680e1e42d2e975459b65";
    RipeMd256::default(), test_256_4, "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "06fdcc7a409548aaf91368c06a6275b553e3f099bf0ea4edfd6778df89a890dd";
);
