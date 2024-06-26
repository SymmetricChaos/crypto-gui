use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

#[derive(Debug, Clone)]
pub struct Md4 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Md4 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Md4 {
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
        // Length in bits before padding
        let b_len = (input.len().wrapping_mul(8)) as u64;
        // Step 1. Append padding bits (here bytes)
        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length is 448 mod 512
        while (input.len() % 64) != 56 {
            input.push(0)
        }
        // Step 2. Append length
        for b in b_len.to_le_bytes() {
            input.push(b)
        }
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
            for (o, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
                *o = u32::from_le_bytes(chunk.try_into().unwrap());
            }

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

        let mut out = vec![0; 16];
        for (offset, word) in [a, b, c, d].iter().enumerate() {
            for (i, byte) in word.to_le_bytes().iter().enumerate() {
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
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod md4_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Md4::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        assert_eq!(
            "31d6cfe0d16ae931b73c59d7e0c089c0",
            hasher.hash_bytes_from_string("").unwrap()
        );
        assert_eq!(
            "bde52cb31de33e46245e05fbdbd6fb24",
            hasher.hash_bytes_from_string("a").unwrap()
        );
    }
}
