use utils::{byte_formatting::ByteFormat, padding::md_strengthening_64_be};

use crate::traits::ClassicHasher;

#[derive(Debug, Clone)]
pub struct Sha1 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub rot: bool,
}

impl Default for Sha1 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rot: true,
        }
    }
}

impl Sha1 {
    pub fn sha0() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rot: false,
        }
    }
}

impl ClassicHasher for Sha1 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        md_strengthening_64_be(&mut input, 64);

        // Initialize variables
        let mut h0 = 0x67452301_u32;
        let mut h1 = 0xefcdab89_u32;
        let mut h2 = 0x98badcfe_u32;
        let mut h3 = 0x10325476_u32;
        let mut h4 = 0xc3d2e1f0_u32;

        // Process message in 64-byte (512-bit) blocks
        for block in input.chunks_exact(64) {
            let mut a = h0;
            let mut b = h1;
            let mut c = h2;
            let mut d = h3;
            let mut e = h4;

            // Extract 16 words from the block and make them the first 16 values of the array
            let mut x = [0u32; 80];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)).take(16) {
                *elem = u32::from_be_bytes(chunk.try_into().unwrap());
            }

            // Extend the 16 words to 80 words
            for i in 16..80 {
                if self.rot {
                    x[i] = (x[i - 3] ^ x[i - 8] ^ x[i - 14] ^ x[i - 16]).rotate_left(1)
                } else {
                    x[i] = x[i - 3] ^ x[i - 8] ^ x[i - 14] ^ x[i - 16]
                }
            }

            // Apply 80 rounds of mixing
            for i in 0..80 {
                let mut f = 0;
                let mut g = 0;
                // Round functions and round constant are changed every 20 rounds
                if i < 20 {
                    f = (b & c) | (!b & d);
                    g = 0x5a827999;
                }
                if i >= 20 && i < 40 {
                    f = b ^ c ^ d;
                    g = 0x6ed9eba1;
                }
                if i >= 40 && i < 60 {
                    f = (b & c) | (b & d) | (c & d);
                    g = 0x8f1bbcdc;
                }
                if i >= 60 {
                    f = b ^ c ^ d;
                    g = 0xca62c1d6;
                }

                let t = a
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(g)
                    .wrapping_add(x[i]); // Each round a new word from the array x is added here
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = t;
            }
            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }

        let mut out = Vec::with_capacity(20);
        for word in [h0, h1, h2, h3, h4] {
            out.extend(word.to_be_bytes())
        }
        out
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod sha1_tests {
    use super::*;

    crate::basic_hash_tests!(
        Sha1::default(), test1, "",
        "da39a3ee5e6b4b0d3255bfef95601890afd80709";
        Sha1::default(), test2, "abc",
        "a9993e364706816aba3e25717850c26c9cd0d89d";
        Sha1::default(), test3, "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        "84983e441c3bd26ebaae4aa1f95129e5e54670f1";
        Sha1::default(), test4, "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
        "a49b2446a02c645bf419f995b67091253a04a259";
    );
}
