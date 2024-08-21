use super::tiger_arrays::{T1, T2, T3, T4};
use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

pub enum TigerVersion {
    One,
    Two,
}

pub struct Tiger {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub version: TigerVersion,
}

impl Default for Tiger {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            version: TigerVersion::One,
        }
    }
}

impl Tiger {
    pub fn round(a: &mut u64, b: &mut u64, c: &mut u64, x: &u64, mul: u64) {
        let idxs = [
            (*c >> 0 * 8) & 0xff,
            (*c >> 2 * 8) & 0xff,
            (*c >> 4 * 8) & 0xff,
            (*c >> 6 * 8) & 0xff,
            (*c >> 1 * 8) & 0xff,
            (*c >> 3 * 8) & 0xff,
            (*c >> 5 * 8) & 0xff,
            (*c >> 7 * 8) & 0xff,
        ];
        *c ^= x;
        *a = a.wrapping_sub(
            T1[idxs[0] as usize]
                ^ T2[idxs[1] as usize]
                ^ T3[idxs[2] as usize]
                ^ T4[idxs[3] as usize],
        );
        *b = b.wrapping_add(
            T4[idxs[4] as usize]
                ^ T3[idxs[5] as usize]
                ^ T2[idxs[6] as usize]
                ^ T1[idxs[7] as usize],
        );
        *b = b.wrapping_mul(mul);
    }

    pub fn pass(a: &mut u64, b: &mut u64, c: &mut u64, x: &[u64; 8], mul: u64) {
        Tiger::round(a, b, c, &x[0], mul);
        Tiger::round(b, c, a, &x[1], mul);
        Tiger::round(c, a, b, &x[2], mul);
        Tiger::round(a, b, c, &x[3], mul);
        Tiger::round(b, c, a, &x[4], mul);
        Tiger::round(c, a, b, &x[5], mul);
        Tiger::round(a, b, c, &x[6], mul);
        Tiger::round(b, c, a, &x[7], mul);
    }

    pub fn key_schedule(x: &mut [u64; 8]) {
        x[0] = x[0].wrapping_sub(x[7] ^ 0xA5A5A5A5A5A5A5A5);
        x[1] ^= x[0];
        x[2] = x[2].wrapping_add(x[1]);
        x[3] = x[3].wrapping_sub(x[2] ^ (!x[1] << 19));
        x[4] ^= x[3];
        x[5] = x[5].wrapping_add(x[4]);
        x[6] = x[6].wrapping_sub(x[5] ^ (!x[4] >> 23));
        x[7] ^= x[6];
        x[0] = x[0].wrapping_add(x[7]);
        x[1] = x[1].wrapping_sub(x[0] ^ (!x[7] << 19));
        x[2] ^= x[1];
        x[3] = x[3].wrapping_add(x[2]);
        x[4] = x[4].wrapping_sub(x[3] ^ (!x[2] >> 23));
        x[5] ^= x[4];
        x[6] = x[6].wrapping_add(x[5]);
        x[7] = x[7].wrapping_sub(x[6] ^ 0x0123456789ABCDEF);
    }

    pub fn compress(a: &mut u64, b: &mut u64, c: &mut u64, x: &mut [u64; 8]) {
        let aa = a.clone();
        let bb = b.clone();
        let cc = c.clone();
        Tiger::pass(a, b, c, x, 5);
        Tiger::key_schedule(x);
        Tiger::pass(c, a, b, x, 7);
        Tiger::key_schedule(x);
        Tiger::pass(b, c, a, x, 9);
        *a ^= aa;
        *b = b.wrapping_sub(bb);
        *c = c.wrapping_add(cc);
    }
}

impl ClassicHasher for Tiger {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // First padding byte is the only difference between V1 and V2
        let b_len = (input.len().wrapping_mul(8)) as u64;
        match self.version {
            TigerVersion::One => input.push(0x01),
            TigerVersion::Two => input.push(0x80),
        }
        while (input.len() % 64 as usize) != 56 {
            input.push(0)
        }
        for b in b_len.to_le_bytes() {
            input.push(b)
        }

        let mut a: u64 = 0x0123456789ABCDEF;
        let mut b: u64 = 0xFEDCBA9876543210;
        let mut c: u64 = 0xF096A5B4C3B2E187;

        todo!()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod tiger_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let hasher = Tiger::default();
        assert_eq!(
            "6d12a41e72e644f017b6f0e2f7b44c6285f06dd5d2c5b075",
            hasher
                .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
                .unwrap()
        );
    }
}
