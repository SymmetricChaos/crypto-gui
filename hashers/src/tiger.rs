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
}

impl ClassicHasher for Tiger {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let a: u64 = 0x0123456789ABCDEF;
        let b: u64 = 0xFEDCBA9876543210;
        let c: u64 = 0xF096A5B4C3B2E187;
        todo!()
    }
    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod tiger_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Tiger::default();
        assert_eq!(
            "6d12a41e72e644f017b6f0e2f7b44c6285f06dd5d2c5b075",
            hasher
                .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
                .unwrap()
        );
    }
}
