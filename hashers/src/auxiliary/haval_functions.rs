// Reference
// https://github.com/bitbandi/all-hash-python/blob/master/sph/haval.c

use std::ops::Shr;

use super::haval_arrays::{K2, K3, K4, K5, W2, W3, W4, W5};

// Boolean functions
// The negations are not in the HAVAL paper or the reference documentation. Optimization?
fn f1(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    ((x1) & ((x0) ^ (x4))) ^ ((x2) & (x5)) ^ ((x3) & (x6)) ^ (x0)
}

fn f2(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    ((x2) & (((x1) & !(x3)) ^ ((x4) & (x5)) ^ (x6) ^ (x0)))
        ^ ((x4) & ((x1) ^ (x5)))
        ^ ((x3 & (x5)) ^ (x0))
}

fn f3(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    ((x3) & (((x1) & (x2)) ^ (x6) ^ (x0))) ^ ((x1) & (x4)) ^ ((x2) & (x5)) ^ (x0)
}

fn f4(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    ((x3) & (((x1) & (x2)) ^ ((x4) | (x6)) ^ (x5)))
        ^ ((x4) & ((!(x2) & (x5)) ^ (x1) ^ (x6) ^ (x0)))
        ^ ((x2) & (x6))
        ^ (x0)
}

fn f5(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    ((x0) & !(((x1) & (x2) & (x3)) ^ (x5))) ^ ((x1) & (x4)) ^ ((x2) & (x5)) ^ ((x3) & (x6))
}

// Boolean functions with selectable permutation p
fn fp3(p: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    match p {
        1 => f1(x1, x0, x3, x5, x6, x2, x4),
        2 => f2(x4, x2, x1, x0, x5, x3, x6),
        3 => f3(x6, x1, x2, x3, x4, x5, x0),
        _ => unreachable!("invalid permutation"),
    }
}

fn fp4(p: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    match p {
        1 => f1(x2, x6, x1, x4, x5, x3, x0),
        2 => f2(x3, x5, x2, x0, x1, x6, x4),
        3 => f3(x1, x4, x3, x6, x0, x2, x5),
        4 => f4(x6, x4, x0, x5, x2, x1, x3),
        _ => unreachable!("invalid permutation"),
    }
}

fn fp5(p: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    match p {
        1 => f1(x3, x4, x1, x0, x5, x2, x6),
        2 => f2(x6, x2, x1, x0, x3, x4, x5),
        3 => f3(x2, x6, x0, x4, x3, x1, x5),
        4 => f4(x1, x5, x3, x2, x0, x4, x6),
        5 => f5(x2, x5, x0, x6, x4, x3, x1),
        _ => unreachable!("invalid permutation"),
    }
}

// Boolean functions with selectable round count n and permutation p
// example: if HAVAL is set to use 4 rounds then this always selects fp4 and passes the current permutation into the function
fn fp(n: u32, p: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
    match n {
        3 => fp3(p, x6, x5, x4, x3, x2, x1, x0),
        4 => fp4(p, x6, x5, x4, x3, x2, x1, x0),
        5 => fp5(p, x6, x5, x4, x3, x2, x1, x0),
        _ => unreachable!("invalid round"),
    }
}

// Step
pub fn step(n: u32, p: u32, s: &mut [u32; 8], w: u32, c: u32) {
    // Implement the boolean function
    let t = fp(n, p, s[6], s[5], s[4], s[3], s[2], s[1], s[0]);
    // Update s[7] then move it to the front
    s[7] = t
        .rotate_right(7)
        .wrapping_add(s[7].rotate_right(11))
        .wrapping_add(w)
        .wrapping_add(c);
    s.rotate_right(1);
}

// Pass functions
pub fn h1(s: &mut [u32; 8], block: &[u32; 32], n: u32) {
    for i in 0..32 {
        step(n, 1, s, block[i], 0);
    }
}
pub fn h2(s: &mut [u32; 8], block: &[u32; 32], n: u32) {
    for i in 0..32 {
        step(n, 2, s, block[W2[i]], K2[i]);
    }
}
pub fn h3(s: &mut [u32; 8], block: &[u32; 32], n: u32) {
    for i in 0..32 {
        step(n, 3, s, block[W3[i]], K3[i]);
    }
}
pub fn h4(s: &mut [u32; 8], block: &[u32; 32], n: u32) {
    for i in 0..32 {
        step(n, 4, s, block[W4[i]], K4[i]);
    }
}
pub fn h5(s: &mut [u32; 8], block: &[u32; 32], n: u32) {
    for i in 0..32 {
        step(n, 5, s, block[W5[i]], K5[i]);
    }
}

// Finalization functions
fn mix_128(a: u32, b: u32, c: u32, d: u32, n: u32) -> u32 {
    (a & 0x000000FF | b & 0x0000FF00 | c & 0x00FF0000 | d & 0xFF000000).rotate_left(n)
}

pub fn finalize_128(s: &[u32; 8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(128);
    out.extend(
        s[0].wrapping_add(mix_128(s[7], s[4], s[5], s[6], 24))
            .to_le_bytes(),
    );
    out.extend(
        s[1].wrapping_add(mix_128(s[6], s[7], s[4], s[5], 16))
            .to_le_bytes(),
    );
    out.extend(
        s[2].wrapping_add(mix_128(s[5], s[6], s[7], s[4], 8))
            .to_le_bytes(),
    );
    out.extend(
        s[3].wrapping_add(mix_128(s[4], s[5], s[6], s[7], 0))
            .to_le_bytes(),
    );
    out
}

fn mix_160_0(a: u32, b: u32, c: u32) -> u32 {
    (a & 0x01F80000 | b & 0xFE000000 | c & 0x0000003F).rotate_left(13)
}

fn mix_160_1(a: u32, b: u32, c: u32) -> u32 {
    (a & 0xFE000000 | b & 0x0000003F | c & 0x00000FC0).rotate_left(7)
}

fn mix_160_2(a: u32, b: u32, c: u32) -> u32 {
    a & 0x0000003F | b & 0x00000FC0 | c & 0x0007F000
}

fn mix_160_3(a: u32, b: u32, c: u32) -> u32 {
    (a & 0x00000FC0 | b & 0x0007F000 | c & 0x01F80000).shr(6)
}

fn mix_160_4(a: u32, b: u32, c: u32) -> u32 {
    (a & 0x0007F000 | b & 0x01F80000 | c & 0xFE000000).shr(12)
}

pub fn finalize_160(s: &[u32; 8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(160);
    out.extend(s[0].wrapping_add(mix_160_0(s[5], s[6], s[7])).to_le_bytes());
    out.extend(s[1].wrapping_add(mix_160_1(s[5], s[6], s[7])).to_le_bytes());
    out.extend(s[2].wrapping_add(mix_160_2(s[5], s[6], s[7])).to_le_bytes());
    out.extend(s[3].wrapping_add(mix_160_3(s[5], s[6], s[7])).to_le_bytes());
    out.extend(s[4].wrapping_add(mix_160_4(s[5], s[6], s[7])).to_le_bytes());
    out
}

fn mix_192_0(a: u32, b: u32) -> u32 {
    (a & 0xFC000000 | b & 0x0000001F).rotate_left(6)
}

fn mix_192_1(a: u32, b: u32) -> u32 {
    a & 0x0000001F | b & 0x000003E0
}

fn mix_192_2(a: u32, b: u32) -> u32 {
    (a & 0x000003E0 | b & 0x0000FC00).shr(5)
}

fn mix_192_3(a: u32, b: u32) -> u32 {
    (a & 0x0000FC00 | b & 0x001F0000).shr(10)
}

fn mix_192_4(a: u32, b: u32) -> u32 {
    (a & 0x001F0000 | b & 0x03E00000).shr(16)
}

fn mix_192_5(a: u32, b: u32) -> u32 {
    (a & 0x03E00000 | b & 0xFC000000).shr(21)
}

pub fn finalize_192(s: &[u32; 8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(192);
    out.extend(s[0].wrapping_add(mix_192_0(s[6], s[7])).to_le_bytes());
    out.extend(s[1].wrapping_add(mix_192_1(s[6], s[7])).to_le_bytes());
    out.extend(s[2].wrapping_add(mix_192_2(s[6], s[7])).to_le_bytes());
    out.extend(s[3].wrapping_add(mix_192_3(s[6], s[7])).to_le_bytes());
    out.extend(s[4].wrapping_add(mix_192_4(s[6], s[7])).to_le_bytes());
    out.extend(s[5].wrapping_add(mix_192_5(s[6], s[7])).to_le_bytes());
    out
}

pub fn finalize_224(s: &[u32; 8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(224);
    out.extend(s[0].wrapping_add((s[7] >> 27) & 0x1f).to_le_bytes());
    out.extend(s[1].wrapping_add((s[7] >> 22) & 0x1f).to_le_bytes());
    out.extend(s[2].wrapping_add((s[7] >> 18) & 0x0f).to_le_bytes());
    out.extend(s[3].wrapping_add((s[7] >> 13) & 0x1f).to_le_bytes());
    out.extend(s[4].wrapping_add((s[7] >> 9) & 0x0f).to_le_bytes());
    out.extend(s[5].wrapping_add((s[7] >> 4) & 0x1f).to_le_bytes());
    out.extend(s[6].wrapping_add((s[7] >> 0) & 0x0f).to_le_bytes());
    out
}

pub fn finalize_256(s: &[u32; 8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(256);
    for i in 0..8 {
        out.extend(s[i].to_le_bytes());
    }
    out
}

// Padding function
pub fn haval_padding(bytes: &mut Vec<u8>, hash_len: u8, rounds: u8) {
    // Length in bits before padding
    let b_len = (bytes.len() as u64).wrapping_mul(8);
    // push a byte with a leading 1 to the bytes
    bytes.push(0x01);
    // push zeros until the length is ten bytes less than the block size.
    while (bytes.len() % 128 as usize) != (128 - 10) as usize {
        bytes.push(0)
    }

    let hash_len_bits = (hash_len as u16) * 8;

    // version number, number of rounds, hash length
    // Unclear why the HAVAL document
    bytes.push(((hash_len_bits & 0x3) as u8) << 6 | rounds << 3 | 0x001);
    bytes.push((hash_len_bits >> 2) as u8);

    // Append the eight bytes of length
    for b in b_len.to_le_bytes() {
        bytes.push(b)
    }
}
