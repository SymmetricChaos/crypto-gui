use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

pub struct Sha1 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Sha1 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Sha1 {}

impl ClassicHasher for Sha1 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Padding and appending length is identical to MD4 and MD5
        // Length in bits before padding
        let b_len = (input.len().wrapping_mul(8)) as u64;

        // Step 1.Padding
        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length in bits is 448 mod 512
        // equivalently until the length in bytes is 56 mod 64
        while (input.len() % 64) != 56 {
            input.push(0)
        }

        // Step 2. Append length
        for b in b_len.to_be_bytes() {
            input.push(b)
        }

        // Step 3. Initialize variables
        let mut a = 0x67452301_u32;
        let mut b = 0xefcdab89_u32;
        let mut c = 0x98badcfe_u32;
        let mut d = 0x10325476_u32;
        let mut e = 0xc3d2e1f0_u32;

        // Step 4. Process message in 16-word blocks
        for block in input.chunks_exact(64) {
            let mut ta = a;
            let mut tb = b;
            let mut tc = c;
            let mut td = d;
            let mut te = e;

            let mut x = [0u32; 80];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)).take(16) {
                *elem = u32::from_be_bytes(chunk.try_into().unwrap());
            }

            // Extend the 16 words to 80 words
            for i in 16..80 {
                x[i] = (x[i - 3] ^ x[i - 8] ^ x[i - 14] ^ x[i - 16]).rotate_left(1)
            }

            for i in 0..80 {
                let mut f = 0;
                let mut g = 0;
                if i < 20 {
                    f = (tb & tc) | (!tb & td);
                    g = 0x5a827999;
                }
                if i >= 20 && i < 40 {
                    f = tb ^ tc ^ td;
                    g = 0x6ed9eba1;
                }
                if i >= 40 && i < 60 {
                    f = (tb & tc) | (tb & td) | (tc & td);
                    g = 0x8f1bbcdc;
                }
                if i >= 60 {
                    f = tb ^ tc ^ td;
                    g = 0xca62c1d6;
                }

                let t = ta
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(te)
                    .wrapping_add(g)
                    .wrapping_add(x[i]);
                te = td;
                td = tc;
                tc = tb.rotate_left(30);
                tb = ta;
                ta = t;
            }
            a = a.wrapping_add(ta);
            b = b.wrapping_add(tb);
            c = c.wrapping_add(tc);
            d = d.wrapping_add(td);
            e = e.wrapping_add(te);
        }

        let mut out = vec![0; 20];
        for (offset, word) in [a, b, c, d, e].iter().enumerate() {
            for (i, byte) in word.to_be_bytes().iter().enumerate() {
                out[i + offset * 4] = *byte
            }
        }
        out
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.bytes_to_text(&out))
    }
}

#[cfg(test)]
mod sha1_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Sha1::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        assert_eq!(
            "da39a3ee5e6b4b0d3255bfef95601890afd80709",
            hasher.hash_bytes_from_string("").unwrap()
        );
        assert_eq!(
            "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12",
            hasher
                .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
                .unwrap()
        );
        assert_eq!(
            "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3",
            hasher
                .hash_bytes_from_string("The quick brown fox jumps over the lazy cog")
                .unwrap()
        );
    }
}
