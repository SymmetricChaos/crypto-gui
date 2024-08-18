use std::collections::VecDeque;

use crate::traits::ClassicHasher;
use utils::{byte_formatting::ByteFormat, padding::zero_padding};

/// 960 bits of √6
const MD6_Q: [u64; 15] = [
    0x7311c2812425cfa0,
    0x6432286434aac8e7,
    0xb60450e9ef68b7c1,
    0xe8fb23908d9f06f1,
    0xdd2e76cba691e5bf,
    0x0cd0d63b2c30bc41,
    0x1f8ccf6823058f8a,
    0x54e5ed5b88e3775d,
    0x4ad12aae0a6d6031,
    0x3e7f16bb88222e0d,
    0x8af8671d3fb50c2c,
    0x995ad1178bd25c31,
    0xc878c1dd04c4b633,
    0x3b72066c7a1552ac,
    0x0d6f3522631effcb,
];

const TAPS: [usize; 5] = [17, 18, 21, 31, 67];

const RSHIFT: [u32; 16] = [10, 5, 13, 10, 11, 12, 2, 7, 14, 15, 7, 13, 11, 7, 6, 12];
const LSHIFT: [u32; 16] = [11, 24, 9, 16, 15, 9, 27, 15, 6, 2, 29, 8, 15, 5, 31, 9];

#[derive(Debug, Clone)]
pub struct Md6 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub output_length: u32,  // output length in bits
    pub key: Vec<u8>,        // key of up to 64 bytes
    pub mode: u32, // mode of operation parameter, if less than 27 some processing is done sequentially with lower memory overhead
    pub rounds: Option<u32>, // Rounds can be specified manually or derived from the output length
}

impl Default for Md6 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            output_length: 256,
            key: Vec::new(),
            mode: 64,
            rounds: None,
        }
    }
}

impl Md6 {
    pub fn md6_224() -> Self {
        Self {
            output_length: 224,
            ..Default::default()
        }
    }

    pub fn md6_256() -> Self {
        Self {
            output_length: 256,
            ..Default::default()
        }
    }

    pub fn md6_384() -> Self {
        Self {
            output_length: 384,
            ..Default::default()
        }
    }

    pub fn md6_512() -> Self {
        Self {
            output_length: 512,
            ..Default::default()
        }
    }

    /// Rounds can be a specified parameter or derived from the number of output bits with a minimum of 80 rounds for a keyed hash
    pub fn n_rounds(&self) -> u32 {
        if let Some(n) = self.rounds {
            return n;
        }
        if self.key.is_empty() {
            40 + self.output_length / 4
        } else {
            // If a key is given the minimum number of rounds is 80
            80.max(40 + self.output_length / 4)
        }
    }

    /// Key used in compression function derived by padding user key with 0x00
    pub fn key(&self) -> [u64; 8] {
        let mut out = [0u64; 8];

        for (i, byte) in self.key.iter().enumerate() {
            out[i / 8] |= (*byte as u64) << (7 - (i % 8)) * 8
        }

        out
    }

    pub fn next_round_key(round_key: u64) -> u64 {
        round_key.rotate_left(1) ^ (0x7311c2812425cfa0 & round_key)
    }

    pub fn par(m: &[u8], k: &[u64; 8]) {}

    pub fn seq(m: &[u8], k: &[u64; 8]) -> [u64; 16] {
        let mut c = [0_u64; 16];

        c
    }

    pub fn compress(&self, input: &[u64; 89]) -> [u64; 16] {
        let [t0, t1, t2, t3, t4] = TAPS;
        let n = 89;
        let mut a = VecDeque::from(input.to_vec());
        let mut round_key: u64 = 0x0123456789abcdef;

        for _round in 0..self.n_rounds() {
            for step in 0..16 {
                let mut x = round_key ^ a[0] ^ a[n - t0];
                x ^= (a[n - t1] & a[n - t2]) ^ (a[n - t3] & a[n - t4]);
                x ^= x >> RSHIFT[step];
                x ^= x << LSHIFT[step];

                a.pop_front();
                a.push_back(x);
            }
            round_key = Md6::next_round_key(round_key);
        }

        a.make_contiguous()[73..].try_into().unwrap()
    }
}

impl ClassicHasher for Md6 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(self.output_length <= 512);
        assert!(self.key.len() <= 64);
        assert!(self.mode <= 64);

        let k = self.key();
        let level = 0;
        let mut input = bytes.to_vec();
        zero_padding(&mut input, 512);

        todo!()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod md6_tests {
    use super::*;

    #[test]
    fn test_key() {
        let mut hasher = Md6::default();
        hasher.key = vec![
            0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        ];
        assert_eq!(
            [0x0a0b0c0d0e0f1a1b, 0x1c1d1e0000000000, 0, 0, 0, 0, 0, 0],
            hasher.key()
        );
    }

    #[test]
    fn test_round_keys() {
        let mut n = 0x0123456789abcdef;

        let first_ten = [
            0x0123456789abcdef,
            0x0347cace1376567e,
            0x058e571c26c8eadc,
            0x0a1cec3869911f38,
            0x16291870f3233150,
            0x3e5330e1c66763a0,
            0x4eb7614288eb84e0,
            0xdf7f828511f68d60,
            0xedee878b23c997e1,
            0xbadd8d976792a863,
        ];

        for i in 0..10 {
            assert_eq!(first_ten[i], n);
            n = Md6::next_round_key(n);
        }
    }

    #[test]
    fn test_abc_compression() {
        let mut hasher = Md6::default();
        hasher.rounds = Some(5);
        let input: [u64; 89] = [
            0x7311c2812425cfa0,
            0x6432286434aac8e7,
            0xb60450e9ef68b7c1,
            0xe8fb23908d9f06f1,
            0xdd2e76cba691e5bf,
            0x0cd0d63b2c30bc41,
            0x1f8ccf6823058f8a,
            0x54e5ed5b88e3775d,
            0x4ad12aae0a6d6031,
            0x3e7f16bb88222e0d,
            0x8af8671d3fb50c2c,
            0x995ad1178bd25c31,
            0xc878c1dd04c4b633,
            0x3b72066c7a1552ac,
            0x0d6f3522631effcb,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0x0100000000000000,
            0x00054010fe800100,
            0x6162630000000000,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let output: [u64; 16] = [
            0x2d1abe0601b2e6b0,
            0x61d59fd2b7310353,
            0xea7da28dec708ec7,
            0xa63a99a574e40155,
            0x290b4fabe80104c4,
            0x8c6a3503cf881a99,
            0xe370e23d1b700cc5,
            0x4492e78e3fe42f13,
            0xdf6c91b7eaf3f088,
            0xaab3e19a8f63b80a,
            0xd987bdcbda2e934f,
            0xaeae805de12b0d24,
            0x8854c14dc284f840,
            0xed71ad7ba542855c,
            0xe189633e48c797a5,
            0x5121a746be48cec8,
        ];

        assert_eq!(output, hasher.compress(&input));
    }

    #[test]
    fn test_suite() {
        let mut hasher = Md6::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        assert_eq!("", hasher.hash_bytes_from_string("").unwrap());
        assert_eq!("", hasher.hash_bytes_from_string("a").unwrap());
    }
}
