use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

// 64-bit primes
const P0: u64 = 0xc3a5c85c97cb3127;
const P1: u64 = 0xb492b66fbe98f273;
const P2: u64 = 0x9ae16a3b2f90404f;

// 32-bit constants
const C0: u32 = 0xcc9e2d51;
const C1: u32 = 0x1b873593;
const C2: u32 = 0xe6546b64;
const C3: u32 = 0x85ebca6b;
const C4: u32 = 0xc2b2ae35;

// Functions taken from Murmur3
fn final_mix(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(C3);
    x ^= x >> 13;
    x = x.wrapping_mul(C4);
    x ^= x >> 16;
    x
}

fn compress(mut x: u32, mut y: u32) -> u32 {
    x = x.wrapping_mul(C0);
    x = x.rotate_left(17);
    x = x.wrapping_mul(C1);
    y ^= x;
    y = y.rotate_left(19);
    y.wrapping_mul(5).wrapping_add(C2)
}

macro_rules! permute3 {
    ($a: ident, $b: ident, $c: ident) => {
        std::mem::swap(&mut $a, &mut $b);
        std::mem::swap(&mut $a, &mut $c);
    };
}

fn fetch_u32(bytes: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap())
}

fn hash32_0_to_4(bytes: &[u8]) -> u32 {
    let l = bytes.len() as u32;
    let mut b: u32 = 0;
    let mut c: u32 = 9;
    for byte in bytes {
        b = b.wrapping_mul(C0).wrapping_add(*byte as u32);
        c ^= b;
    }
    final_mix(compress(b, compress(l, c)))
}

fn hash32_5_to_12(bytes: &[u8]) -> u32 {
    let l = bytes.len();
    let mut a = bytes.len() as u32;
    let mut b = a.wrapping_mul(5);
    let mut c: u32 = 9;
    let d: u32 = b;
    a = a.wrapping_add(fetch_u32(bytes, 0));
    b = b.wrapping_add(fetch_u32(bytes, l - 4));
    c = c.wrapping_add(fetch_u32(bytes, (l >> 1) & 4));
    final_mix(compress(c, compress(b, compress(a, d))))
}

fn hash32_13_to_24(bytes: &[u8]) -> u32 {
    let l = bytes.len();
    let a = fetch_u32(bytes, (l >> 1) - 4);
    let b = fetch_u32(bytes, 4);
    let c = fetch_u32(bytes, l - 8);
    let d = fetch_u32(bytes, l >> 1);
    let e = fetch_u32(bytes, 0);
    let f = fetch_u32(bytes, l - 4);
    let h = bytes.len() as u32;
    final_mix(compress(
        f,
        compress(e, compress(d, compress(c, compress(b, compress(a, h))))),
    ))
}

fn hash32_25(bytes: &[u8]) -> u32 {
    todo!()
}

pub struct CityHash32 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for CityHash32 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for CityHash32 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            0..=4 => hash32_0_to_4(bytes).to_le_bytes().to_vec(),
            5..=12 => hash32_5_to_12(bytes).to_le_bytes().to_vec(),
            13..=24 => hash32_13_to_24(bytes).to_le_bytes().to_vec(),
            _ => hash32_25(bytes).to_le_bytes().to_vec(),
        }
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod ghash_tests {
    use super::*;

    #[test]
    fn test_fetching_does_not_panic() {
        let mut v = Vec::new();
        for i in 0..=4 {
            hash32_0_to_4(&v);
            v.push(i);
        }
        for i in 5..=12 {
            hash32_5_to_12(&v);
            v.push(i);
        }
        for i in 13..=24 {
            hash32_13_to_24(&v);
            v.push(i);
        }
    }
}
