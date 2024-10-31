// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use std::ops::BitXor;

use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

// 32-bit constants from Murmur3
const C0: u32 = 0xcc9e2d51;
const C1: u32 = 0x1b873593;
const C2: u32 = 0xe6546b64;
const C3: u32 = 0x85ebca6b;
const C4: u32 = 0xc2b2ae35;

// Function taken from Murmur3
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
    x = x.rotate_right(17);
    x = x.wrapping_mul(C1);
    y ^= x;
    y = y.rotate_right(19);
    y.wrapping_mul(5).wrapping_add(C2)
}

fn fetch_u32(bytes: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap())
}

fn hash32_0_to_4(bytes: &[u8]) -> u32 {
    let l = bytes.len() as u32;
    let mut b: u32 = 0;
    let mut c: u32 = 9;
    for byte in bytes {
        b = b.wrapping_mul(C0).wrapping_add(*byte as i8 as u32); // yes really, conversion to i8 then to u32 is the intended transformation
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
    let l = bytes.len();

    let mut h = l as u32;
    let mut g = C0.wrapping_mul(h);
    let mut f = g;

    let a0 = fetch_u32(bytes, l - 4)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    let a1 = fetch_u32(bytes, l - 8)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    let a2 = fetch_u32(bytes, l - 16)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    let a3 = fetch_u32(bytes, l - 12)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    let a4 = fetch_u32(bytes, l - 20)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);

    h = h
        .bitxor(a0)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);
    h = h
        .bitxor(a2)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);
    g = g
        .bitxor(a1)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);
    g = g
        .bitxor(a3)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);
    f = f
        .wrapping_add(a4)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);

    let mut offset = 0;

    for _ in 0..((l - 1) / 20) {
        let a0 = fetch_u32(bytes, offset)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a1 = fetch_u32(bytes, offset + 4);
        let a2 = fetch_u32(bytes, offset + 8)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a3 = fetch_u32(bytes, offset + 12)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a4 = fetch_u32(bytes, offset + 16);

        h = h
            .bitxor(a0)
            .rotate_right(18)
            .wrapping_mul(5)
            .wrapping_add(C2);

        f = f.wrapping_add(a1).rotate_right(19).wrapping_mul(C0);

        g = g
            .wrapping_add(a2)
            .rotate_right(18)
            .wrapping_mul(5)
            .wrapping_add(C2);

        h ^= a3.wrapping_add(a1);
        h = h.rotate_right(19).wrapping_mul(5).wrapping_add(C2);

        g = g.bitxor(a4).swap_bytes().wrapping_mul(5);

        h = h.wrapping_add(a4.wrapping_mul(5)).swap_bytes();

        f = f.wrapping_add(a0);

        (f, g, h) = (g, h, f);

        offset += 20
    }

    g = g
        .rotate_right(11)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C0);
    f = f
        .rotate_right(11)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C0);

    h.wrapping_add(g)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2)
        .rotate_right(17)
        .wrapping_mul(C0)
        .wrapping_add(f)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2)
        .rotate_right(17)
        .wrapping_mul(C0)
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

impl CityHash32 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for CityHash32 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            0..=4 => hash32_0_to_4(bytes),
            5..=12 => hash32_5_to_12(bytes),
            13..=24 => hash32_13_to_24(bytes),
            _ => hash32_25(bytes),
        }
        .to_be_bytes()
        .to_vec()
    }

    crate::hash_bytes_from_string! {}
}
