use utils::byte_formatting::{fill_u64s_le, ByteFormat};

use crate::traits::ClassicHasher;

macro_rules! mix {
    ($a: ident, $b: ident, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r);
        $b ^= $a;
    };
}

fn four_rounds(
    mut a: u64,
    mut b: u64,
    mut c: u64,
    mut d: u64,
    keys: [u64; 4],
) -> (u64, u64, u64, u64) {
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

    a = a.wrapping_add(keys[0]);
    b = b.wrapping_add(keys[1]);
    c = c.wrapping_add(keys[2]);
    d = d.wrapping_add(keys[3]);

    (a, b, c, d)
}

// The number 240 encrypted with AES with an all zero key
const C240: u64 = 0x1BD11BDAA9FC1A22;

pub struct Skein256 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u64; Self::WORDS],
    pub tweak: [u64; 2],
}

impl Default for Skein256 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: [0; Self::WORDS],
            tweak: [0; 2],
        }
    }
}

impl Skein256 {
    const WORDS: usize = 4;
    const ROUNDS: usize = 72;

    pub fn key(&mut self, bytes: [u8; Self::WORDS * 8]) {
        fill_u64s_le(&mut self.key, &bytes);
    }

    pub fn with_key(mut self, bytes: [u8; Self::WORDS * 8]) -> Self {
        self.key(bytes);
        self
    }

    fn ksa(&self) -> [[u64; 4]; 19] {
        // XOR together all the key words and the C240 constant
        let knw = self.key.into_iter().fold(C240, |acc, w| acc ^ w);
        let t2 = self.tweak[0] ^ self.tweak[1];
        todo!()
    }

    fn encrypt_block(
        mut a: u64,
        mut b: u64,
        mut c: u64,
        mut d: u64,
        round_keys: [[u64; 4]; 19],
    ) -> (u64, u64, u64, u64) {
        // First round key
        a = a.wrapping_add(round_keys[0][0]);
        b = b.wrapping_add(round_keys[0][1]);
        c = c.wrapping_add(round_keys[0][2]);
        d = d.wrapping_add(round_keys[0][3]);

        for i in 1..(Self::ROUNDS / 4 + 1) {
            (a, b, c, d) = four_rounds(a, b, c, d, round_keys[i]);
        }

        (a, b, c, d)
    }

    // Unique Block Iteration
    // Incorporates the tweak information for each block to make each block and each mode unique
    fn ubi() {}
}

impl ClassicHasher for Skein256 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    crate::hash_bytes_from_string! {}
}
