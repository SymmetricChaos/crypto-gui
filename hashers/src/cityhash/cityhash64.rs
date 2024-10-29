// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use std::ops::BitXor;

use crate::traits::ClassicHasher;
use num::Integer;
use utils::byte_formatting::ByteFormat;

// 64-bit primes
const P0: u64 = 0xc3a5c85c97cb3127;
const P1: u64 = 0xb492b66fbe98f273;
const P2: u64 = 0x9ae16a3b2f90404f;

fn fetch_u64(bytes: &[u8], p: usize) -> u64 {
    u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap())
}

fn hash64_0_to_16(bytes: &[u8]) -> u64 {
    todo!()
}

fn hash64_17_to_32(bytes: &[u8]) -> u64 {
    todo!()
}

fn hash64_33_to_64(bytes: &[u8]) -> u64 {
    todo!()
}

fn hash64_16(a: u64, b: u64) -> u64 {
    todo!()
}

fn weak_hash_32_with_seeds(bytes: &[u8], mut a: u64, mut b: u64) -> (u64, u64) {
    let w = fetch_u64(bytes, 0);
    let x = fetch_u64(bytes, 8);
    let y = fetch_u64(bytes, 16);
    let z = fetch_u64(bytes, 24);

    a = a.wrapping_add(w);
    b = b.wrapping_add(a).wrapping_add(z).rotate_right(21);
    let c = a;
    a = a.wrapping_add(x).wrapping_add(y);
    b = a.rotate_right(44);

    (a.wrapping_add(z), b.wrapping_add(c))
}

fn shift_mix(a: u64) -> u64 {
    a ^ (a >> 47)
}

fn hash64_65(bytes: &[u8]) -> u64 {
    let l = bytes.len();

    let mut x = fetch_u64(bytes, l - 40);
    let mut y = fetch_u64(bytes, l - 16).wrapping_add(fetch_u64(bytes, l - 56));
    let mut z = hash64_16(
        fetch_u64(bytes, l - 48).wrapping_add(l as u64),
        fetch_u64(bytes, l - 24),
    );

    let (mut v1, mut v2) = weak_hash_32_with_seeds(&bytes[l - 64..], l as u64, z);
    let (mut w1, mut w2) = weak_hash_32_with_seeds(&bytes[l - 32..], y.wrapping_add(P1), x);

    x = x.wrapping_add(P1).wrapping_add(fetch_u64(bytes, 0));

    for block in bytes.chunks_exact(64) {
        x = x
            .wrapping_add(y)
            .wrapping_add(v1)
            .wrapping_add(fetch_u64(block, 8))
            .rotate_right(37)
            .wrapping_mul(P1);
        y = y
            .wrapping_add(v2)
            .wrapping_add(fetch_u64(block, 48))
            .rotate_right(42)
            .wrapping_mul(P1);

        x ^= w2;

        y = y.wrapping_add(v1).wrapping_add(fetch_u64(block, 40));

        z = z.wrapping_add(w1).rotate_right(33).wrapping_mul(P1);

        (v1, v2) = weak_hash_32_with_seeds(block, v2.wrapping_mul(P1), x.wrapping_add(w1));
        (w1, w2) = weak_hash_32_with_seeds(
            &block[32..],
            z.wrapping_add(w2),
            y.wrapping_add(fetch_u64(block, 16)),
        );

        (z, x) = (x, z);
    }

    hash64_16(
        hash64_16(v1, w1)
            .wrapping_add(shift_mix(y).wrapping_mul(P1))
            .wrapping_add(z),
        hash64_16(v2, w2).wrapping_add(x),
    )
}

pub struct CityHash64 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for CityHash64 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl CityHash64 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for CityHash64 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            0..=16 => hash64_0_to_16(bytes),
            17..=32 => hash64_17_to_32(bytes),
            33..=64 => hash64_33_to_64(bytes),
            _ => hash64_65(bytes),
        }
        .to_be_bytes()
        .to_vec()
    }

    crate::hash_bytes_from_string! {}
}
