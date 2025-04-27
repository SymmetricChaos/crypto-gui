use std::ops::{Index, IndexMut};

// pub mod skein1024;
pub mod skein256;
// pub mod skein512;

// The number 240 encrypted with AES with an all zero key
const C240: u64 = 0x1BD11BDAA9FC1A22;
pub const SCHEMA_VERSION: u64 = 0x0000000133414853; // schema string "SHA3" and version number 1
pub const TREE_INFO: u64 = 0x0000000000000000; // only sequential hashing is supported so this is all zero

/// Processing the first block
const FIRST: u64 = 1 << 62;
/// Processing the last block
const FINAL: u64 = 1 << 63;
/// Processing the configuration block
const CFG: u64 = 4 << 56;
/// Processing a message block
const MSG: u64 = 48 << 56;
/// Processing an output block
const OUT: u64 = 63 << 56;
/// Length of the Config string
const CFG_LEN: u64 = 4 * 8;

#[derive(Debug, Copy, Clone)]
pub struct Tweak([u64; 2]);

impl Tweak {
    pub fn new() -> Self {
        Self([0; 2])
    }

    pub fn blank_with_flags(flags: u64) -> Self {
        Self([0, flags])
    }

    // Increment the 96-bit counter
    pub fn increment(&mut self, n: u64) {
        match self[0].overflowing_add(n) {
            (x, false) => self[0] = x,
            (x, true) => {
                self[0] = x;
                self[1] = (self[1].wrapping_add(1)) & 0x00000000FFFFFFFF;
            }
        }
    }

    pub fn extended(&self) -> [u64; 3] {
        [self[0], self[1], self[0] ^ self[1]]
    }
}

impl Index<usize> for Tweak {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Tweak {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[inline]
fn subkey_add(state: &mut [u64], key: &[u64]) {
    for (s, k) in state.iter_mut().zip(key.iter()) {
        *s = s.wrapping_add(*k)
    }
}

macro_rules! skein_mix {
    ($a: expr, $b: expr, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r) ^ $a;
    };
}

#[inline]
pub fn octo_round_256(w: &mut [u64; 4], subkey: &[[u64; 4]]) {
    subkey_add(w, &subkey[0]);

    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    subkey_add(w, &subkey[1]);

    skein_mix!(w[0], w[1], 25);
    skein_mix!(w[2], w[3], 33);

    skein_mix!(w[0], w[3], 46);
    skein_mix!(w[2], w[1], 12);

    skein_mix!(w[0], w[1], 58);
    skein_mix!(w[2], w[3], 22);

    skein_mix!(w[0], w[3], 32);
    skein_mix!(w[2], w[1], 32);
}

#[inline]
pub fn octo_round_512(w: &mut [u64; 8], subkey: &[[u64; 8]]) {
    subkey_add(w, &subkey[0]);

    skein_mix!(w[0], w[1], 46);
    skein_mix!(w[2], w[3], 36);
    skein_mix!(w[4], w[5], 19);
    skein_mix!(w[6], w[7], 37);

    skein_mix!(w[2], w[1], 33);
    skein_mix!(w[4], w[7], 27);
    skein_mix!(w[6], w[5], 14);
    skein_mix!(w[0], w[3], 42);

    skein_mix!(w[4], w[1], 17);
    skein_mix!(w[6], w[3], 49);
    skein_mix!(w[0], w[5], 36);
    skein_mix!(w[2], w[7], 39);

    skein_mix!(w[6], w[1], 44);
    skein_mix!(w[0], w[7], 9);
    skein_mix!(w[2], w[5], 54);
    skein_mix!(w[4], w[3], 56);

    subkey_add(w, &subkey[1]);

    skein_mix!(w[0], w[1], 39);
    skein_mix!(w[2], w[3], 30);
    skein_mix!(w[4], w[5], 34);
    skein_mix!(w[6], w[7], 24);

    skein_mix!(w[2], w[1], 13);
    skein_mix!(w[4], w[7], 50);
    skein_mix!(w[6], w[5], 10);
    skein_mix!(w[0], w[3], 17);

    skein_mix!(w[4], w[1], 25);
    skein_mix!(w[6], w[3], 29);
    skein_mix!(w[0], w[5], 39);
    skein_mix!(w[2], w[7], 43);

    skein_mix!(w[6], w[1], 8);
    skein_mix!(w[0], w[7], 35);
    skein_mix!(w[2], w[5], 56);
    skein_mix!(w[4], w[3], 22);
}

#[inline]
pub fn octo_round_1024(w: &mut [u64; 16], subkey: &[[u64; 16]]) {
    subkey_add(w, &subkey[0]);

    skein_mix!(w[0], w[1], 24);
    skein_mix!(w[2], w[3], 13);
    skein_mix!(w[4], w[5], 8);
    skein_mix!(w[6], w[7], 47);
    skein_mix!(w[8], w[9], 8);
    skein_mix!(w[10], w[11], 17);
    skein_mix!(w[12], w[13], 22);
    skein_mix!(w[14], w[15], 37);

    skein_mix!(w[0], w[9], 38);
    skein_mix!(w[2], w[13], 19);
    skein_mix!(w[6], w[11], 10);
    skein_mix!(w[4], w[15], 55);
    skein_mix!(w[10], w[7], 49);
    skein_mix!(w[12], w[3], 18);
    skein_mix!(w[14], w[5], 23);
    skein_mix!(w[8], w[1], 52);

    skein_mix!(w[0], w[7], 33);
    skein_mix!(w[2], w[5], 4);
    skein_mix!(w[4], w[3], 51);
    skein_mix!(w[6], w[1], 13);
    skein_mix!(w[12], w[15], 34);
    skein_mix!(w[14], w[13], 41);
    skein_mix!(w[8], w[11], 59);
    skein_mix!(w[10], w[9], 17);

    skein_mix!(w[0], w[15], 5);
    skein_mix!(w[2], w[11], 20);
    skein_mix!(w[6], w[13], 48);
    skein_mix!(w[4], w[9], 41);
    skein_mix!(w[14], w[1], 47);
    skein_mix!(w[8], w[5], 28);
    skein_mix!(w[10], w[3], 16);
    skein_mix!(w[12], w[7], 25);

    subkey_add(w, &subkey[1]);

    skein_mix!(w[0], w[1], 41);
    skein_mix!(w[2], w[3], 9);
    skein_mix!(w[4], w[5], 37);
    skein_mix!(w[6], w[7], 31);
    skein_mix!(w[8], w[9], 12);
    skein_mix!(w[10], w[11], 47);
    skein_mix!(w[12], w[13], 44);
    skein_mix!(w[14], w[15], 30);

    skein_mix!(w[0], w[9], 16);
    skein_mix!(w[2], w[13], 34);
    skein_mix!(w[6], w[11], 56);
    skein_mix!(w[4], w[15], 51);
    skein_mix!(w[10], w[7], 4);
    skein_mix!(w[12], w[3], 53);
    skein_mix!(w[14], w[5], 42);
    skein_mix!(w[8], w[1], 41);

    skein_mix!(w[0], w[7], 31);
    skein_mix!(w[2], w[5], 44);
    skein_mix!(w[4], w[3], 47);
    skein_mix!(w[6], w[1], 46);
    skein_mix!(w[12], w[15], 19);
    skein_mix!(w[14], w[13], 42);
    skein_mix!(w[8], w[11], 44);
    skein_mix!(w[10], w[9], 25);

    skein_mix!(w[0], w[15], 9);
    skein_mix!(w[2], w[11], 48);
    skein_mix!(w[6], w[13], 35);
    skein_mix!(w[4], w[9], 52);
    skein_mix!(w[14], w[1], 23);
    skein_mix!(w[8], w[5], 31);
    skein_mix!(w[10], w[3], 37);
    skein_mix!(w[12], w[7], 20);
}
