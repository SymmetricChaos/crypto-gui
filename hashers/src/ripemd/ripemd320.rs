use super::ripemd160::RipeMd160;
use crate::traits::ClassicHasher;
use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_le};

#[derive(Clone)]
pub struct RipeMd320 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for RipeMd320 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl RipeMd320 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn compress(state_l: &mut [u32; 5], state_r: &mut [u32; 5], block: [u32; 16]) {
        let mut l = state_l.clone();
        let mut r = state_r.clone();

        for j in 0..80 {
            // Exact same round functions as RIPEMD-160
            RipeMd160::left_chain(j, &mut l, block);
            RipeMd160::right_chain(j, &mut r, block);
            // While in RIPEMD-160 the l and r arrays and mixed together
            // at the end of the compression function that is not possible
            // here since both are needed for output. Instead at the end of
            // each 16 step round a word is swapped between them.
            if j == 15 {
                std::mem::swap(&mut l[1], &mut r[1])
            }
            if j == 31 {
                std::mem::swap(&mut l[3], &mut r[3])
            }
            if j == 47 {
                std::mem::swap(&mut l[0], &mut r[0])
            }
            if j == 63 {
                std::mem::swap(&mut l[2], &mut r[2])
            }
            if j == 79 {
                std::mem::swap(&mut l[4], &mut r[4])
            }
        }

        for i in 0..5 {
            state_l[i] = state_l[i].wrapping_add(l[i]);
            state_r[i] = state_r[i].wrapping_add(r[i]);
        }
    }
}

impl ClassicHasher for RipeMd320 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        md_strengthening_64_le(&mut input, 64);

        let mut state_l: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];
        let mut state_r: [u32; 5] = [0x76543210, 0xfedcba98, 0x89abcdef, 0x01234567, 0x3c2d1e0f];

        for chunk in input.chunks_exact(64) {
            let mut block = [0u32; 16];
            for (elem, b) in block.iter_mut().zip(chunk.chunks_exact(4)) {
                *elem = u32::from_le_bytes(b.try_into().unwrap());
            }
            Self::compress(&mut state_l, &mut state_r, block)
        }

        let mut out = Vec::with_capacity(40);
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

    RipeMd320::default(), test_320_1, "",
    "22d65d5661536cdc75c1fdf5c6de7b41b9f27325ebc61e8557177d705a0ec880151c3a32a00899b8";
    RipeMd320::default(), test_320_2, "a",
    "ce78850638f92658a5a585097579926dda667a5716562cfcf6fbe77f63542f99b04705d6970dff5d";
    RipeMd320::default(), test_320_3, "abc",
    "de4c01b3054f8930a79d09ae738e92301e5a17085beffdc1b8d116713e74f82fa942d64cdbc4682d";
    RipeMd320::default(), test_320_4, "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "557888af5f6d8ed62ab66945c6d2a0a47ecd5341e915eb8fea1d0524955f825dc717e4a008ab2d42";
);
