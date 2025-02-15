use super::auxiliary::tiger_arrays::{T1, T2, T3, T4};
use crate::traits::StatefulHasher;
use std::num::Wrapping;
use utils::byte_formatting::{fill_u64s_le, make_u64s_le};

const BLOCK_LEN: usize = 64;

pub fn round(
    a: &mut Wrapping<u64>,
    b: &mut Wrapping<u64>,
    c: &mut Wrapping<u64>,
    x: &Wrapping<u64>,
    mul: Wrapping<u64>,
) {
    *c ^= *x;
    // c.0 refers to the u64 inside the Wrapping<u64>
    let idxs = c.0.to_le_bytes().map(|b| b as usize);
    *a -= T1[idxs[0]] ^ T2[idxs[2]] ^ T3[idxs[4]] ^ T4[idxs[6]];
    *b += T4[idxs[1]] ^ T3[idxs[3]] ^ T2[idxs[5]] ^ T1[idxs[7]];
    *b *= mul;
}

pub fn pass(
    a: &mut Wrapping<u64>,
    b: &mut Wrapping<u64>,
    c: &mut Wrapping<u64>,
    x: &[Wrapping<u64>; 8],
    mul: Wrapping<u64>,
) {
    round(a, b, c, &x[0], mul);
    round(b, c, a, &x[1], mul);
    round(c, a, b, &x[2], mul);
    round(a, b, c, &x[3], mul);
    round(b, c, a, &x[4], mul);
    round(c, a, b, &x[5], mul);
    round(a, b, c, &x[6], mul);
    round(b, c, a, &x[7], mul);
}

pub fn key_schedule(x: &mut [Wrapping<u64>; 8]) {
    x[0] -= x[7] ^ Wrapping(0xA5A5A5A5A5A5A5A5);
    x[1] ^= x[0];
    x[2] += x[1];
    x[3] -= x[2] ^ ((!x[1]) << 19);
    x[4] ^= x[3];
    x[5] += x[4];
    x[6] -= x[5] ^ ((!x[4]) >> 23);
    x[7] ^= x[6];
    x[0] += x[7];
    x[1] -= x[0] ^ ((!x[7]) << 19);
    x[2] ^= x[1];
    x[3] += x[2];
    x[4] -= x[3] ^ ((!x[2]) >> 23);
    x[5] ^= x[4];
    x[6] += x[5];
    x[7] -= x[6] ^ Wrapping(0x0123456789ABCDEF);
}

pub fn compress(state: &mut [Wrapping<u64>; 3], x: &mut [Wrapping<u64>; 8]) {
    let [mut a, mut b, mut c] = state;
    pass(&mut a, &mut b, &mut c, x, Wrapping(5));
    key_schedule(x);
    pass(&mut c, &mut a, &mut b, x, Wrapping(7));
    key_schedule(x);
    pass(&mut b, &mut c, &mut a, x, Wrapping(9));
    state[0] ^= a;
    state[1] = b - state[1];
    state[2] += c;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TigerVersion {
    One,
    Two,
}

pub struct Tiger {
    version: TigerVersion,
    buffer: Vec<u8>,
    state: [Wrapping<u64>; 3],
    bits_taken: u64,
}

impl Tiger {
    pub fn init(variant: TigerVersion) -> Self {
        Self {
            version: variant,
            buffer: Vec::with_capacity(BLOCK_LEN),
            state: [
                Wrapping(0x0123456789ABCDEF),
                Wrapping(0xFEDCBA9876543210),
                Wrapping(0xF096A5B4C3B2E187),
            ],
            bits_taken: 0,
        }
    }

    pub fn init_v1() -> Self {
        println!("init");
        Self::init(TigerVersion::One)
    }

    pub fn init_v2() -> Self {
        Self::init(TigerVersion::Two)
    }
}

impl StatefulHasher for Tiger {
    fn update(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            if self.buffer.len() == BLOCK_LEN {
                self.bits_taken += 512;
                let x = make_u64s_le(&self.buffer);
                let mut x = x.map(|n| Wrapping(n));
                compress(&mut self.state, &mut x);
                self.buffer.clear();
            }
            crate::take_bytes!(self.buffer, bytes, BLOCK_LEN);
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        self.bits_taken += self.buffer.len() as u64 * 8;
        // This first padding byte is the only difference between V1 and V2
        match self.version {
            TigerVersion::One => self.buffer.push(0x01),
            TigerVersion::Two => self.buffer.push(0x80),
        }
        while (self.buffer.len() % 64) != 56 {
            self.buffer.push(0x00)
        }
        self.buffer.extend(self.bits_taken.to_le_bytes());

        for chunk in self.buffer.chunks_exact(64) {
            let mut x = [0; 8];
            fill_u64s_le(&mut x, &chunk);
            let mut x = x.map(|n| Wrapping(n));
            compress(&mut self.state, &mut x)
        }

        let mut out = vec![0; 24];
        for (offset, word) in self.state.iter().enumerate() {
            // Confirmed that this is little endian from reference implementations
            for (i, byte) in word.0.to_le_bytes().iter().enumerate() {
                out[i + offset * 8] = *byte
            }
        }
        out
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_v1_pangram, Tiger::init_v1(), b"The quick brown fox jumps over the lazy dog",
    "6d12a41e72e644f017b6f0e2f7b44c6285f06dd5d2c5b075";
    test_v1_empty, Tiger::init_v1(), b"",
    "3293ac630c13f0245f92bbb1766e16167a4e58492dde73f3";
    test_v1_a, Tiger::init_v1(), b"a",
    "77befbef2e7ef8ab2ec8f93bf587a7fc613e247f5f247809";
    test_v1_long, Tiger::init_v1(), b"This input is long enough to force the compression function to be called multiple times.",
    "9f94af52c7dfd86af83ea99b65b0912695ab32b8a93f2e9c";
    test_v2_pangram, Tiger::init_v2(), b"The quick brown fox jumps over the lazy dog",
    "976abff8062a2e9dcea3a1ace966ed9c19cb85558b4976d8";
    test_v2_empty, Tiger::init_v2(), b"",
    "4441be75f6018773c206c22745374b924aa8313fef919f41";
);
