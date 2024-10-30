// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use crate::traits::ClassicHasher;
use num::Integer;
use std::ops::BitXor;
use utils::byte_formatting::ByteFormat;

// 64-bit primes
const P0: u64 = 0xc3a5c85c97cb3127;
const P1: u64 = 0xb492b66fbe98f273;
const P2: u64 = 0x9ae16a3b2f90404f;
const P3: u64 = 0x9ddfea08eb382d69;

fn fetch_u32(bytes: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap())
}

fn fetch_u64(bytes: &[u8], p: usize) -> u64 {
    u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap())
}

fn shift_mix(a: u64) -> u64 {
    a ^ (a >> 47)
}

fn hash64_0_to_16(bytes: &[u8]) -> u64 {
    let l = bytes.len();
    match l {
        0 => P2,
        1..=3 => {
            let a = bytes[0] as u32;
            let b = bytes[l >> 1] as u32;
            let c = bytes[l - 1] as u32;
            let y = a.wrapping_add(b << 8) as u64;
            let z = (l as u32).wrapping_add(c << 2) as u64;
            shift_mix(y.wrapping_mul(P2) ^ z.wrapping_mul(P0)).wrapping_mul(P2)
        }
        4..=7 => {
            let mul = P2.wrapping_add((l as u64) * 2); // no change that l will overflow so no point in wrapping mul, but should make no difference
            let a = fetch_u32(bytes, 0) as u64;
            let u = (l as u64).wrapping_add(a << 3);
            let v = fetch_u32(bytes, l - 4) as u64;
            hash64_16_mul(u, v, mul)
        }
        _ => {
            let mul = P2.wrapping_add((l as u64) * 2);
            let a = fetch_u64(bytes, 0).wrapping_add(P2);
            let b = fetch_u64(bytes, l - 8);
            let u = b.rotate_right(37).wrapping_mul(mul).wrapping_add(a);
            let v = a.rotate_right(25).wrapping_add(b).wrapping_mul(mul);
            hash64_16_mul(u, v, mul)
        }
    }
}

fn hash64_17_to_32(bytes: &[u8]) -> u64 {
    let l = bytes.len();
    let mul = P2.wrapping_add((l as u64) * 2);
    let a = fetch_u64(bytes, 0).wrapping_mul(P1);
    let b = fetch_u64(bytes, 8);
    let c = fetch_u64(bytes, l - 8).wrapping_mul(mul);
    let d = fetch_u64(bytes, l - 16).wrapping_mul(P2);
    let u = a
        .wrapping_add(b)
        .rotate_right(43)
        .wrapping_add(c.rotate_right(30))
        .wrapping_add(d);
    let v = a
        .wrapping_add(b.wrapping_add(P2).rotate_right(18))
        .wrapping_add(c);
    hash64_16_mul(u, v, mul)
}

fn hash64_33_to_64(bytes: &[u8]) -> u64 {
    let l = bytes.len();
    let mul = P2.wrapping_add((l as u64) * 2);
    let mut a = fetch_u64(bytes, 0).wrapping_mul(P2);
    let mut b = fetch_u64(bytes, 8);
    let c = fetch_u64(bytes, l - 24);
    let d = fetch_u64(bytes, l - 32);
    let e = fetch_u64(bytes, 16).wrapping_mul(P2);
    let f = fetch_u64(bytes, 24).wrapping_mul(9);
    let g = fetch_u64(bytes, l - 8);
    let h = fetch_u64(bytes, l - 16).wrapping_mul(mul);
    let u = a
        .wrapping_add(g)
        .rotate_right(43)
        .wrapping_add(b.rotate_right(30).wrapping_add(c).wrapping_mul(9));
    let v = a.wrapping_add(g).bitxor(d).wrapping_add(f).wrapping_add(1);
    let w = u
        .wrapping_add(v)
        .wrapping_mul(mul)
        .swap_bytes()
        .wrapping_add(h);
    let x = e.wrapping_add(f).rotate_right(42).wrapping_add(c);
    let y = v
        .wrapping_add(w)
        .wrapping_mul(mul)
        .swap_bytes()
        .wrapping_add(g)
        .wrapping_mul(mul);
    let z = e.wrapping_add(f).wrapping_add(c);
    a = x
        .wrapping_add(z)
        .wrapping_mul(mul)
        .wrapping_add(y)
        .swap_bytes()
        .wrapping_add(b);
    b = shift_mix(
        z.wrapping_add(a)
            .wrapping_mul(mul)
            .wrapping_add(d)
            .wrapping_add(h),
    )
    .wrapping_mul(mul);
    b.wrapping_add(x)
}

fn hash64_16_mul(u: u64, v: u64, mul: u64) -> u64 {
    let mut a = (u ^ v).wrapping_mul(mul);
    a ^= a >> 47;
    let mut b = (v ^ a).wrapping_mul(mul);
    b ^= b >> 47;
    b.wrapping_mul(mul)
}

// TODO: assure that x_high and x_low are the high and low bits of a u128
fn hash64_16(x_high: u64, x_low: u64) -> u64 {
    let mut a = x_high.bitxor(x_low).wrapping_mul(P3);
    a ^= a >> 47;
    let b = x_high.bitxor(a).wrapping_mul(P3);
    b.bitxor(b >> 47).wrapping_mul(P3)
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

fn hash64_65(bytes: &[u8]) -> u64 {
    let l = bytes.len();

    let mut x = fetch_u64(bytes, l - 40);
    let mut y = fetch_u64(bytes, l - 16).wrapping_add(fetch_u64(bytes, l - 56));
    let mut z = hash64_16(
        fetch_u64(bytes, l - 48).wrapping_add(l as u64),
        fetch_u64(bytes, l - 24),
    );

    let (mut v1, mut v2) = weak_hash_128_with_seeds(&bytes[l - 64..], l as u64, z);
    let (mut w1, mut w2) = weak_hash_128_with_seeds(&bytes[l - 32..], y.wrapping_add(P1), x);

    x = x.wrapping_mul(P1).wrapping_add(fetch_u64(bytes, 0));

    let mut n = l.prev_multiple_of(&64);
    let mut offset = 0;
    loop {
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
        if n == 0 {
            break;
        }
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
    pub seeds: Option<[u64; 2]>,
}

impl Default for CityHash64 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            seeds: None,
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

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seeds = Some([P2, seed]);
        self
    }

    pub fn with_seeds(mut self, seed0: u64, seed1: u64) -> Self {
        self.seeds = Some([seed0, seed1]);
        self
    }
}

impl ClassicHasher for CityHash64 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        if let Some([s0, s1]) = self.seeds {
            let h = match bytes.len() {
                0..=16 => hash64_0_to_16(bytes),
                17..=32 => hash64_17_to_32(bytes),
                33..=64 => hash64_33_to_64(bytes),
                _ => hash64_65(bytes),
            };
            hash64_16(h.wrapping_sub(s0), s1).to_be_bytes().to_vec()
        } else {
            match bytes.len() {
                0..=16 => hash64_0_to_16(bytes),
                17..=32 => hash64_17_to_32(bytes),
                33..=64 => hash64_33_to_64(bytes),
                _ => hash64_65(bytes),
            }
            .to_be_bytes()
            .to_vec()
        }
    }

    crate::hash_bytes_from_string! {}
}
