use utils::byte_formatting::{fill_u64s_le, ByteFormat};

use crate::traits::ClassicHasher;

macro_rules! mix {
    ($a: ident, $b: ident, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r);
        $b ^= $a;
    };
}

// The number 240 encrypted with AES with an all zero key
const C240: u64 = 0x1BD11BDAA9FC1A22;

pub struct Skein256 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u64; 4],
    pub tweak: [u64; 2],
}

impl Default for Skein256 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: [0; 4],
            tweak: [0; 2],
        }
    }
}

impl Skein256 {
    const WORDS: usize = 4;
    const ROUNDS: usize = 72;

    fn key(&mut self, bytes: [u8; Self::WORDS * 8]) {
        fill_u64s_le(&mut self.key, &bytes);
    }

    fn with_key(mut self, bytes: [u8; Self::WORDS * 8]) -> Self {
        self.key(bytes);
        self
    }

    fn tweak(&mut self, bytes: [u8; 16]) {
        fill_u64s_le(&mut self.tweak, &bytes);
    }

    fn with_tweak(mut self, bytes: [u8; 16]) -> Self {
        self.tweak(bytes);
        self
    }

    fn eight_rounds(mut a: u64, mut b: u64, mut c: u64, mut d: u64) -> (u64, u64, u64, u64) {
        mix!(a, b, 14);
        mix!(c, d, 16);
        (b, d) = (d, b);

        mix!(a, b, 52);
        mix!(c, d, 57);
        (b, d) = (d, b);

        mix!(a, b, 23);
        mix!(c, d, 40);
        (b, d) = (d, b);

        mix!(a, b, 5);
        mix!(c, d, 37);
        (b, d) = (d, b);

        mix!(a, b, 25);
        mix!(c, d, 33);
        (b, d) = (d, b);

        mix!(a, b, 46);
        mix!(c, d, 12);
        (b, d) = (d, b);

        mix!(a, b, 58);
        mix!(c, d, 22);
        (b, d) = (d, b);

        mix!(a, b, 32);
        mix!(c, d, 32);
        (b, d) = (d, b);

        (a, b, c, d)
    }

    fn encrypt_block(mut a: u64, mut b: u64, mut c: u64, mut d: u64) -> (u64, u64, u64, u64) {
        // TODO: ROUND KEY

        // 8
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        // 16
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        // 24
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        // 32
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        // 40
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        // 48
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        // 56
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        // 64
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        // 72
        (a, b, c, d) = Self::eight_rounds(a, b, c, d);
        // TODO: ROUND KEY

        (a, b, c, d)
    }
}

impl ClassicHasher for Skein256 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    crate::hash_bytes_from_string! {}
}
