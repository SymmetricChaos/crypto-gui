use utils::{
    byte_formatting::{fill_u32s_le, ByteFormat},
    padding::md_strengthening_64_le,
};

use crate::traits::ClassicHasher;

#[derive(Debug, Clone)]
pub struct Md4 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Md4 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Md4 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }

    pub fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (x & z) | (y & z)
    }

    pub fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    pub fn r1(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
        *a = (a.wrapping_add(Self::f(b, c, d)).wrapping_add(i)).rotate_left(s)
    }

    pub fn r2(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
        *a = (a
            .wrapping_add(Self::g(b, c, d))
            .wrapping_add(i)
            .wrapping_add(0x5A827999))
        .rotate_left(s)
    }

    pub fn r3(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
        *a = (a
            .wrapping_add(Self::h(b, c, d))
            .wrapping_add(i)
            .wrapping_add(0x6ED9EBA1))
        .rotate_left(s)
    }
}

impl ClassicHasher for Md4 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();
        md_strengthening_64_le(&mut input, 64);

        // Step 3. Initialize MD buffer
        let mut a = 0x67452301_u32;
        let mut b = 0xefcdab89_u32;
        let mut c = 0x98badcfe_u32;
        let mut d = 0x10325476_u32;
        // Step 4. Process message in 16-word blocks
        for block in input.chunks_exact(64) {
            let ta = a;
            let tb = b;
            let tc = c;
            let td = d;

            let mut x = [0u32; 16];
            fill_u32s_le(&mut x, &block);

            // Round 1
            for i in [0, 4, 8, 12] {
                Self::r1(&mut a, b, c, d, x[i], 3);
                Self::r1(&mut d, a, b, c, x[i + 1], 7);
                Self::r1(&mut c, d, a, b, x[i + 2], 11);
                Self::r1(&mut b, c, d, a, x[i + 3], 19);
            }

            // Round 2
            for i in [0, 1, 2, 3] {
                Self::r2(&mut a, b, c, d, x[i], 3);
                Self::r2(&mut d, a, b, c, x[i + 4], 5);
                Self::r2(&mut c, d, a, b, x[i + 8], 9);
                Self::r2(&mut b, c, d, a, x[i + 12], 13);
            }

            // Round 3
            for i in [0, 2, 1, 3] {
                Self::r3(&mut a, b, c, d, x[i], 3);
                Self::r3(&mut d, a, b, c, x[i + 8], 9);
                Self::r3(&mut c, d, a, b, x[i + 4], 11);
                Self::r3(&mut b, c, d, a, x[i + 12], 15);
            }

            a = a.wrapping_add(ta);
            b = b.wrapping_add(tb);
            c = c.wrapping_add(tc);
            d = d.wrapping_add(td);
        }

        let mut out = Vec::with_capacity(16);
        for word in [a, b, c, d] {
            out.extend(word.to_le_bytes())
        }
        out
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1, Md4::default(), "", "31d6cfe0d16ae931b73c59d7e0c089c0";
    test2, Md4::default(), "a","bde52cb31de33e46245e05fbdbd6fb24";
    test3, Md4::default(), "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "043f8582f241db351ce627e153e7f0e4";
    test4, Md4::default(), "12345678901234567890123456789012345678901234567890123456789012345678901234567890","e33b4ddc9c38f2199c3e7b164fcc0536";
);
