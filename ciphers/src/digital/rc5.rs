use std::cmp::max;

use crate::{Cipher, CipherError};

const P32: u32 = 0xB7E15163;
// const P64: u64 = 0xB7E151628AED2A6B;
const Q32: u32 = 0x9E3779B9;
// const Q64: u64 = 0x9E3779B97F4A7C15;

pub struct Rc5 {
    rounds: usize,
    state: Vec<u32>,
}

impl Default for Rc5 {
    fn default() -> Self {
        Self {
            rounds: 12,
            state: Vec::new(),
        }
    }
}

impl Rc5 {
    pub fn ksa_32(&self, key: &[u8]) {
        let b = key.len();
        let u = 4; // Bytes in a word
        let c = max(b.div_ceil(u), 1);
        let mut l = vec![0_u32; c];
        for i in (0..b).rev() {
            l[i / u] = (l[i / u].rotate_left(8)).wrapping_add(key[i] as u32)
        }

        let t = 2 * (self.rounds + 1);
        let mut s = vec![0; t];
        s[0] = P32;
        for i in 1..t {
            s[i] = s[i - 1].wrapping_add(Q32)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        for _ in 0..(3 * max(t, c)) {
            s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
            a = s[i];
            l[j] = (l[j].wrapping_add(a).wrapping_add(b)).rotate_left(a.wrapping_add(b));
            b = l[j];
            i = (i + 1) % t;
            j = (j + 1) % c;
        }
    }

    // pub fn ksa_64(&self, key: &[u8]) {
    //     let b = key.len();
    //     let u = 8; // Bytes in a word
    //     let c = max(b.div_ceil(u), 1);
    //     let mut l = vec![0_u64; c];
    //     for i in (0..b).rev() {
    //         l[i / u] = (l[i / u].rotate_left(8)).wrapping_add(key[i] as u64)
    //     }

    //     let t = 2 * (self.rounds + 1);
    //     let mut s = vec![0; t];
    //     s[0] = P64;
    //     for i in 1..t {
    //         s[i] = s[i - 1].wrapping_add(Q64)
    //     }

    //     let mut i = 0;
    //     let mut j = 0;
    //     let mut a = 0;
    //     let mut b = 0;
    //     for _ in 0..(3 * max(t, c)) {
    //         s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
    //         a = s[i];
    //         l[j] = (l[j].wrapping_add(a).wrapping_add(b)).rotate_left(a.wrapping_add(b));
    //         b = l[j];
    //         i = (i + 1) % t;
    //         j = (j + 1) % c;
    //     }
    // }

    pub fn encrypt_block_32(&self, bytes: &[u8]) {}

    pub fn encrypt_block_64(&self, bytes: &[u8]) {}
}

impl Cipher for Rc5 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}

#[cfg(test)]
mod rc5_tests {

    use super::*;
}
