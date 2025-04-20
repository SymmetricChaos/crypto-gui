use std::num::Wrapping as W;
use std::ops::{Index, IndexMut};
use utils::byte_formatting::make_u64s_le;

// pub mod skein1024;
pub mod skein256;
// pub mod skein512;

// The number 240 encrypted with AES with an all zero key
const C240: u64 = 0x1BD11BDAA9FC1A22;
pub const SCHEMA_VERSION: u64 = 0x0000000133414853; // schema string "SHA3" and version number 1
pub const TREE_INFO: u64 = 0x0000000000000000; // only sequential hashing is supported so this is all zero

#[derive(Debug, Copy, Clone)]
pub struct Tweak([W<u64>; 2]);

impl Tweak {
    pub fn new() -> Self {
        Tweak([W(0); 2])
    }

    // Increment the 96-bit counter
    pub fn increment(&mut self, n: u64) {
        match self[0].0.overflowing_add(n) {
            (x, false) => self[0] = W(x),
            (x, true) => {
                self[0] = W(x);
                self[1] = W((self[1].0 + 1) & 0x00000000FFFFFFFF);
            }
        }
    }

    pub fn extended(&self) -> [W<u64>; 3] {
        [self[0], self[1], W(self[0].0 ^ self[1].0)]
    }
}

impl Index<usize> for Tweak {
    type Output = W<u64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Tweak {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! skein_subkey_add {
    ($a: expr, $b: expr, $c: expr, $d: expr, $k: expr, $t: expr, $r: expr) => {
        $a = $a + ($k[($r + 0) as usize % 5]);
        $b = $b + ($k[($r + 1) as usize % 5]) + ($t[($r + 0) as usize % 3]);
        $c = $c + ($k[($r + 2) as usize % 5]) + ($t[($r + 1) as usize % 3]);
        $d = $d + ($k[($r + 3) as usize % 5]) + W($r);
    };
}

macro_rules! skein_mix {
    ($a: expr, $b: expr, $r: literal) => {
        $a = $a + ($b);
        $b = W($b.0.rotate_left($r)) ^ $a;
    };
}

pub fn octo_round_256(w: &mut [W<u64>; 4], key: &[W<u64>; 5], tweak: &[W<u64>; 3], round: u64) {
    // pub const PERM_256: [usize; 4] = [0, 3, 2, 1];
    skein_subkey_add!(w[0], w[1], w[2], w[3], key, tweak, round);

    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    skein_subkey_add!(w[0], w[1], w[2], w[3], key, tweak, round + 1);

    skein_mix!(w[0], w[1], 25);
    skein_mix!(w[2], w[3], 33);

    skein_mix!(w[0], w[3], 46);
    skein_mix!(w[2], w[1], 12);

    skein_mix!(w[0], w[1], 58);
    skein_mix!(w[2], w[3], 22);

    skein_mix!(w[0], w[3], 32);
    skein_mix!(w[2], w[1], 32);
}

// pub fn octo_round_512(w: &mut [u64; 8], subkeys0: [u64; 8], subkeys1: [u64; 8]) {
//    // pub const PERM_512: [usize; 8] = [2, 1, 4, 7, 6, 5, 0, 3];
//     for i in 0..8 {
//         w[i] = w[i].wrapping_add(subkeys0[i])
//     }

//     skein_mix!(w[0], w[1], 46);
//     skein_mix!(w[2], w[3], 36);
//     skein_mix!(w[4], w[5], 19);
//     skein_mix!(w[6], w[7], 37);

//     skein_mix!(w[2], w[1], 33);
//     skein_mix!(w[4], w[7], 27);
//     skein_mix!(w[6], w[5], 14);
//     skein_mix!(w[0], w[3], 42);

//     skein_mix!(w[4], w[1], 17);
//     skein_mix!(w[6], w[3], 49);
//     skein_mix!(w[0], w[5], 36);
//     skein_mix!(w[2], w[7], 39);

//     skein_mix!(w[6], w[1], 44);
//     skein_mix!(w[0], w[7], 9);
//     skein_mix!(w[2], w[5], 54);
//     skein_mix!(w[4], w[3], 56);

//     for i in 0..8 {
//         w[i] = w[i].wrapping_add(subkeys1[i])
//     }

//     skein_mix!(w[0], w[1], 39);
//     skein_mix!(w[2], w[3], 30);
//     skein_mix!(w[4], w[5], 34);
//     skein_mix!(w[6], w[7], 24);

//     skein_mix!(w[2], w[1], 13);
//     skein_mix!(w[4], w[7], 50);
//     skein_mix!(w[6], w[5], 10);
//     skein_mix!(w[0], w[3], 17);

//     skein_mix!(w[4], w[1], 25);
//     skein_mix!(w[6], w[3], 29);
//     skein_mix!(w[0], w[5], 39);
//     skein_mix!(w[2], w[7], 43);

//     skein_mix!(w[6], w[1], 8);
//     skein_mix!(w[0], w[7], 35);
//     skein_mix!(w[2], w[5], 56);
//     skein_mix!(w[4], w[3], 22);
// }

// pub fn octo_round_1024(w: &mut [u64; 16], subkeys0: [u64; 16], subkeys1: [u64; 16]) {
//     // pub const PERM_1024: [usize; 16] = [0, 9, 2, 13, 6, 11, 4, 15, 10, 7, 12, 3, 14, 5, 8, 1];
//     for i in 0..16 {
//         w[i] = w[i].wrapping_add(subkeys0[i])
//     }

//     skein_mix!(w[0], w[1], 24);
//     skein_mix!(w[2], w[3], 13);
//     skein_mix!(w[4], w[5], 8);
//     skein_mix!(w[6], w[7], 47);
//     skein_mix!(w[8], w[9], 8);
//     skein_mix!(w[10], w[11], 17);
//     skein_mix!(w[12], w[13], 22);
//     skein_mix!(w[14], w[15], 37);

//     skein_mix!(w[0], w[9], 38);
//     skein_mix!(w[2], w[13], 19);
//     skein_mix!(w[6], w[11], 10);
//     skein_mix!(w[4], w[15], 55);
//     skein_mix!(w[10], w[7], 49);
//     skein_mix!(w[12], w[3], 18);
//     skein_mix!(w[14], w[5], 23);
//     skein_mix!(w[8], w[1], 52);

//     skein_mix!(w[0], w[7], 33);
//     skein_mix!(w[2], w[5], 4);
//     skein_mix!(w[4], w[3], 51);
//     skein_mix!(w[6], w[1], 13);
//     skein_mix!(w[12], w[15], 34);
//     skein_mix!(w[14], w[13], 41);
//     skein_mix!(w[8], w[11], 59);
//     skein_mix!(w[10], w[9], 17);

//     skein_mix!(w[0], w[15], 5);
//     skein_mix!(w[2], w[11], 20);
//     skein_mix!(w[6], w[13], 48);
//     skein_mix!(w[4], w[9], 41);
//     skein_mix!(w[14], w[1], 47);
//     skein_mix!(w[8], w[5], 28);
//     skein_mix!(w[10], w[3], 16);
//     skein_mix!(w[12], w[7], 25);

//     for i in 0..16 {
//         w[i] = w[i].wrapping_add(subkeys1[i])
//     }

//     skein_mix!(w[0], w[1], 41);
//     skein_mix!(w[2], w[3], 9);
//     skein_mix!(w[4], w[5], 37);
//     skein_mix!(w[6], w[7], 31);
//     skein_mix!(w[8], w[9], 12);
//     skein_mix!(w[10], w[11], 47);
//     skein_mix!(w[12], w[13], 44);
//     skein_mix!(w[14], w[15], 30);

//     skein_mix!(w[0], w[9], 16);
//     skein_mix!(w[2], w[13], 34);
//     skein_mix!(w[6], w[11], 56);
//     skein_mix!(w[4], w[15], 51);
//     skein_mix!(w[10], w[7], 4);
//     skein_mix!(w[12], w[3], 53);
//     skein_mix!(w[14], w[5], 42);
//     skein_mix!(w[8], w[1], 41);

//     skein_mix!(w[0], w[7], 31);
//     skein_mix!(w[2], w[5], 44);
//     skein_mix!(w[4], w[3], 47);
//     skein_mix!(w[6], w[1], 46);
//     skein_mix!(w[12], w[15], 19);
//     skein_mix!(w[14], w[13], 42);
//     skein_mix!(w[8], w[11], 44);
//     skein_mix!(w[10], w[9], 25);

//     skein_mix!(w[0], w[15], 9);
//     skein_mix!(w[2], w[11], 48);
//     skein_mix!(w[6], w[13], 35);
//     skein_mix!(w[4], w[9], 52);
//     skein_mix!(w[14], w[1], 23);
//     skein_mix!(w[8], w[5], 31);
//     skein_mix!(w[10], w[3], 37);
//     skein_mix!(w[12], w[7], 20);
// }
