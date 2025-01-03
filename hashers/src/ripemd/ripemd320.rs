use super::ripemd160::RipeMd160;
use crate::traits::StatefulHasher;

#[derive(Clone)]
pub struct RipeMd320 {
    state_l: [u32; 5],
    state_r: [u32; 5],
    buffer: Vec<u8>,
    bits_taken: u64,
}

impl RipeMd320 {
    pub fn init() -> Self {
        Self {
            state_l: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0],
            state_r: [0x76543210, 0xfedcba98, 0x89abcdef, 0x01234567, 0x3c2d1e0f],
            buffer: Vec::new(),
            bits_taken: 0,
        }
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

impl StatefulHasher for RipeMd320 {
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
    test_320_1, RipeMd320::init(), b"",
    "22d65d5661536cdc75c1fdf5c6de7b41b9f27325ebc61e8557177d705a0ec880151c3a32a00899b8";
    test_320_2, RipeMd320::init(), b"a",
    "ce78850638f92658a5a585097579926dda667a5716562cfcf6fbe77f63542f99b04705d6970dff5d";
    test_320_3, RipeMd320::init(), b"abc",
    "de4c01b3054f8930a79d09ae738e92301e5a17085beffdc1b8d116713e74f82fa942d64cdbc4682d";
    test_320_4, RipeMd320::init(), b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "557888af5f6d8ed62ab66945c6d2a0a47ecd5341e915eb8fea1d0524955f825dc717e4a008ab2d42";
);
