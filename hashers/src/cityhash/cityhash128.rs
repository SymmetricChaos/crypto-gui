// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use crate::traits::ClassicHasher;
use std::ops::BitXor;
use utils::byte_formatting::ByteFormat;

// 64-bit primes
const P0: u64 = 0xc3a5c85c97cb3127;
const P1: u64 = 0xb492b66fbe98f273;
// const P2: u64 = 0x9ae16a3b2f90404f;
const P3: u64 = 0x9ddfea08eb382d69;

fn fetch_u64(bytes: &[u8], p: usize) -> u64 {
    u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap())
}

fn shift_mix(a: u64) -> u64 {
    a ^ (a >> 47)
}

// Hash 128 bits down to 64 with a variable multiplier
fn hash128_64_mul(u: u64, v: u64, mul: u64) -> u64 {
    let a = u.bitxor(v).wrapping_mul(mul);
    let b = (v ^ shift_mix(a)).wrapping_mul(mul);
    shift_mix(b).wrapping_mul(mul)
}

// Hash 128 bits down to 64 with a fixed multiplier
fn hash128_64(u: u64, v: u64) -> u64 {
    hash128_64_mul(u, v, P3)
}

fn weak_hash_128_with_seeds(bytes: &[u8], mut a: u64, mut b: u64) -> (u64, u64) {
    let w = fetch_u64(bytes, 0);
    let x = fetch_u64(bytes, 8);
    let y = fetch_u64(bytes, 16);
    let z = fetch_u64(bytes, 24);

    a = a.wrapping_add(w);
    b = b.wrapping_add(a).wrapping_add(z).rotate_right(21);
    let c = a;
    a = a.wrapping_add(x).wrapping_add(y);
    b = b.wrapping_add(a.rotate_right(44));

    (a.wrapping_add(z), b.wrapping_add(c))
}

pub struct CityHash128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub seeds: [u64; 2],
}

impl Default for CityHash128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            seeds: [P0, P1],
        }
    }
}

impl CityHash128 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn with_seeds(mut self, seed0: u64, seed1: u64) -> Self {
        self.seeds = [seed0, seed1];
        self
    }
}

impl ClassicHasher for CityHash128 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let l = bytes.len();
        let mut x = self.seeds[0];
        let mut y = self.seeds[1];
        let mut z = (l as u64).wrapping_mul(P1);
        let mut v1 = y
            .bitxor(P1)
            .rotate_right(49)
            .wrapping_mul(P1)
            .wrapping_add(fetch_u64(bytes, 0));
        let mut v2 = v1
            .rotate_right(42)
            .wrapping_mul(P1)
            .wrapping_add(fetch_u64(bytes, 8));
        let mut w1 = y
            .wrapping_add(z)
            .rotate_right(35)
            .wrapping_mul(P1)
            .wrapping_add(x);
        let mut w2 = x
            .wrapping_add(fetch_u64(bytes, 88))
            .rotate_right(53)
            .wrapping_mul(P1);

        let mut n = l;
        let mut offset = 0;

        while n >= 128 {
            // Same action as CityHash64
            let block = &bytes[offset..];
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

            (v1, v2) = weak_hash_128_with_seeds(block, v2.wrapping_mul(P1), x.wrapping_add(w1));
            (w1, w2) = weak_hash_128_with_seeds(
                &block[32..],
                z.wrapping_add(w2),
                y.wrapping_add(fetch_u64(block, 16)),
            );

            (z, x) = (x, z);
            n -= 64;
            offset += 64;
        }

        x = v1.wrapping_add(z).rotate_right(49).wrapping_add(P0);
        y = y.wrapping_add(P0).wrapping_add(w2.rotate_right(37));
        z = z.wrapping_add(P0).wrapping_add(w1.rotate_right(27));
        w1 = w1.wrapping_add(9);
        v1 = v1.wrapping_mul(P0);

        let tail = &bytes[(l - 128)..];
        for pos in (0..l).step_by(32) {
            let offset = tail.len() - pos - 32;
            let block = &tail[offset..];

            y = x
                .wrapping_add(y)
                .rotate_right(42)
                .wrapping_mul(P0)
                .wrapping_add(v2);
            w1 = fetch_u64(block, 16);
            x = x.wrapping_mul(P0).wrapping_add(w1);
            z = z.wrapping_add(w2).wrapping_add(fetch_u64(bytes, 0));
            w2 = w2.wrapping_add(v1);
            (v1, v2) = weak_hash_128_with_seeds(bytes, v1.wrapping_add(z), v2);
            v1 = v1.wrapping_mul(P0);
        }

        x = hash128_64(x, v1);
        y = hash128_64(y.wrapping_add(z), w1);
        [
            hash128_64(x.wrapping_add(v2), w2).wrapping_add(y),
            hash128_64(x.wrapping_add(w2), y.wrapping_add(v2)),
        ]
        .into_iter()
        .flat_map(|w| w.to_be_bytes())
        .collect()
    }

    crate::hash_bytes_from_string! {}
}
