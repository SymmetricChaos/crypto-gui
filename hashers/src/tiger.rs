use std::num::Wrapping;

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
    pub fn round(
        a: &mut Wrapping<u64>,
        b: &mut Wrapping<u64>,
        c: &mut Wrapping<u64>,
        x: &Wrapping<u64>,
        mul: Wrapping<u64>,
    ) {
        let idxs = [
            (c.0 >> 0 * 8) & 0xff,
            (c.0 >> 2 * 8) & 0xff,
            (c.0 >> 4 * 8) & 0xff,
            (c.0 >> 6 * 8) & 0xff,
            (c.0 >> 1 * 8) & 0xff,
            (c.0 >> 3 * 8) & 0xff,
            (c.0 >> 5 * 8) & 0xff,
            (c.0 >> 7 * 8) & 0xff,
        ];
        *c ^= x;
        *a -= T1[idxs[0] as usize]
            ^ T2[idxs[1] as usize]
            ^ T3[idxs[2] as usize]
            ^ T4[idxs[3] as usize];
        *b += T4[idxs[4] as usize]
            ^ T3[idxs[5] as usize]
            ^ T2[idxs[6] as usize]
            ^ T1[idxs[7] as usize];
        *b *= mul;
    }

    pub fn pass(
        a: &mut Wrapping<u64>,
        b: &mut Wrapping<u64>,
        c: &mut Wrapping<u64>,
        x: &[Wrapping<u64>; 8],
        mul: Wrapping<u64>,
    ) {
        Tiger::round(a, b, c, &x[0], mul);
        Tiger::round(b, c, a, &x[1], mul);
        Tiger::round(c, a, b, &x[2], mul);
        Tiger::round(a, b, c, &x[3], mul);
        Tiger::round(b, c, a, &x[4], mul);
        Tiger::round(c, a, b, &x[5], mul);
        Tiger::round(a, b, c, &x[6], mul);
        Tiger::round(b, c, a, &x[7], mul);
    }

    pub fn key_schedule(x: &mut [Wrapping<u64>; 8]) {
        x[0] -= x[7] ^ Wrapping(0xA5A5A5A5A5A5A5A5);
        x[1] ^= x[0];
        x[2] += x[1];
        x[3] -= x[2] ^ (!x[1] << 19);
        x[4] ^= x[3];
        x[5] += x[4];
        x[6] -= x[5] ^ (!x[4] >> 23);
        x[7] ^= x[6];
        x[0] += x[7];
        x[1] -= x[0] ^ (!x[7] << 19);
        x[2] ^= x[1];
        x[3] += x[2];
        x[4] -= x[3] ^ (!x[2] >> 23);
        x[5] ^= x[4];
        x[6] += x[5];
        x[7] -= x[6] ^ Wrapping(0x0123456789ABCDEF);
    }

    pub fn compress(
        a: &mut Wrapping<u64>,
        b: &mut Wrapping<u64>,
        c: &mut Wrapping<u64>,
        x: &mut [Wrapping<u64>; 8],
    ) {
        let aa = a.clone();
        let bb = b.clone();
        let cc = c.clone();
        Tiger::pass(a, b, c, x, Wrapping(5));
        Tiger::key_schedule(x);
        Tiger::pass(c, a, b, x, Wrapping(7));
        Tiger::key_schedule(x);
        Tiger::pass(b, c, a, x, Wrapping(9));
        *a ^= aa;
        *b -= bb;
        *c += cc;
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

        let mut a = Wrapping(0x0123456789ABCDEF);
        let mut b = Wrapping(0xFEDCBA9876543210);
        let mut c = Wrapping(0xF096A5B4C3B2E187);

        for block in input.chunks_exact(64) {
            let mut x = [Wrapping(0u64); 8];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(8)) {
                *elem = Wrapping(u64::from_le_bytes(chunk.try_into().unwrap()));
            }
            Tiger::compress(&mut a, &mut b, &mut c, &mut x)
        }

        let mut out = vec![0; 24];
        for (offset, word) in [a, b, c].iter().enumerate() {
            for (i, byte) in word.0.to_le_bytes().iter().enumerate() {
                out[i + offset * 8] = *byte
            }
        }
        out
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod tiger_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Tiger::default();
        // assert_eq!(
        //     "6d12a41e72e644f017b6f0e2f7b44c6285f06dd5d2c5b075",
        //     hasher
        //         .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
        //         .unwrap()
        // );
        assert_eq!(
            "3293ac630c13f0245f92bbb1766e16167a4e58492dde73f3",
            hasher.hash_bytes_from_string("").unwrap()
        );

        // hasher.version = TigerVersion::Two;
        // assert_eq!(
        //     "976abff8062a2e9dcea3a1ace966ed9c19cb85558b4976d8",
        //     hasher
        //         .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
        //         .unwrap()
        // );
        // assert_eq!(
        //     "4441be75f6018773c206c22745374b924aa8313fef919f41",
        //     hasher.hash_bytes_from_string("").unwrap()
        // );
    }
}
