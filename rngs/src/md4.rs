use std::ops::Not;

use num::{BigUint, One, Zero};

use crate::traits::ClassicRng;

fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}
pub struct Md4 {
    ctr: BigUint,
}

impl Default for Md4 {
    fn default() -> Self {
        Self {
            ctr: BigUint::zero(),
        }
    }
}

impl Md4 {
    pub fn f(x: u32, y: u32, z: u32) -> u32 {
        x.wrapping_mul(y) | x.not().wrapping_mul(z)
    }

    pub fn g(x: u32, y: u32, z: u32) -> u32 {
        x.wrapping_mul(y) | x.wrapping_mul(z) | y.wrapping_mul(z)
    }

    pub fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    pub fn r1(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
        *a = (*a + Self::f(b, c, d) + i).rotate_left(s)
    }

    pub fn r2(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
        *a = (*a + Self::g(b, c, d) + i + 0x5A827999).rotate_left(s)
    }

    pub fn r3(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
        *a = (*a + Self::h(b, c, d) + i + 0x6ED9EBA1).rotate_left(s)
    }

    pub fn hash(k: &[u8]) -> u128 {
        let mut state = k.to_vec();
        // Length in bits before padding
        let b_len = (state.len() * 8) as u64;
        // Step 1. Append padding bits (here bytes)
        // push a byte with a leading 1 to the bytes
        state.push(0x80);
        // push zeros until the length is 448 mod 512
        while (state.len() % 512) != 448 {
            state.push(0)
        }
        // Step 2. Append length
        for b in b_len.to_le_bytes() {
            state.push(b)
        }
        // Step 3. Initialize MD buffer
        let mut a = 0x67452301_u32;
        let mut b = 0xefcdab89_u32;
        let mut c = 0x98badcfe_u32;
        let mut d = 0x10325476_u32;
        // Step 4. Process message in 16-word blocks
        for block in state.chunks_exact(64) {
            let ta = a;
            let tb = b;
            let tc = c;
            let td = d;

            let mut x = [0u32; 16];
            for i in 0..16 {
                x[i] = as_u32_le(&block[(i * 4)..(i * 4 + 4)]);
            }
            Self::r1(&mut a, b, c, d, x[0], 3);
            Self::r1(&mut d, a, b, c, x[1], 7);
            Self::r1(&mut c, d, a, b, x[2], 11);
            Self::r1(&mut b, c, d, a, x[3], 19);
            Self::r1(&mut a, b, c, d, x[4], 3);
            Self::r1(&mut d, a, b, c, x[5], 7);
            Self::r1(&mut c, d, a, b, x[6], 11);
            Self::r1(&mut b, c, d, a, x[7], 19);
            Self::r1(&mut a, b, c, d, x[8], 3);
            Self::r1(&mut d, a, b, c, x[9], 7);
            Self::r1(&mut c, d, a, b, x[10], 11);
            Self::r1(&mut b, c, d, a, x[11], 19);
            Self::r1(&mut a, b, c, d, x[12], 3);
            Self::r1(&mut d, a, b, c, x[13], 7);
            Self::r1(&mut c, d, a, b, x[14], 11);
            Self::r1(&mut b, c, d, a, x[15], 19);

            Self::r2(&mut a, b, c, d, x[0], 3);
            Self::r2(&mut d, a, b, c, x[4], 5);
            Self::r2(&mut c, d, a, b, x[8], 9);
            Self::r2(&mut b, c, d, a, x[12], 13);
            Self::r2(&mut a, b, c, d, x[1], 3);
            Self::r2(&mut d, a, b, c, x[5], 5);
            Self::r2(&mut c, d, a, b, x[9], 9);
            Self::r2(&mut b, c, d, a, x[13], 13);
            Self::r2(&mut a, b, c, d, x[2], 3);
            Self::r2(&mut d, a, b, c, x[6], 5);
            Self::r2(&mut c, d, a, b, x[10], 9);
            Self::r2(&mut b, c, d, a, x[14], 13);
            Self::r2(&mut a, b, c, d, x[3], 3);
            Self::r2(&mut d, a, b, c, x[7], 5);
            Self::r2(&mut c, d, a, b, x[11], 9);
            Self::r2(&mut b, c, d, a, x[15], 13);

            Self::r3(&mut a, b, c, d, x[0], 3);
            Self::r3(&mut d, a, b, c, x[8], 9);
            Self::r3(&mut c, d, a, b, x[4], 11);
            Self::r3(&mut b, c, d, a, x[12], 15);
            Self::r3(&mut a, b, c, d, x[2], 3);
            Self::r3(&mut d, a, b, c, x[10], 9);
            Self::r3(&mut c, d, a, b, x[6], 11);
            Self::r3(&mut b, c, d, a, x[14], 15);
            Self::r3(&mut a, b, c, d, x[1], 3);
            Self::r3(&mut d, a, b, c, x[9], 9);
            Self::r3(&mut c, d, a, b, x[5], 11);
            Self::r3(&mut b, c, d, a, x[13], 15);
            Self::r3(&mut a, b, c, d, x[3], 3);
            Self::r3(&mut d, a, b, c, x[11], 9);
            Self::r3(&mut c, d, a, b, x[7], 11);
            Self::r3(&mut b, c, d, a, x[15], 15);

            a = a.wrapping_add(ta);
            b = b.wrapping_add(tb);
            c = c.wrapping_add(tc);
            d = d.wrapping_add(td);
        }

        let mut out = 0;
        out += (a as u128) << 96;
        out += (b as u128) << 64;
        out += (c as u128) << 32;
        out += d as u128;
        out
    }
}

impl ClassicRng for Md4 {
    fn next_u32(&mut self) -> u32 {
        self.ctr += BigUint::one();
        (Self::hash(&self.ctr.to_bytes_le()) >> 96) as u32
    }
}

// #[cfg(test)]
// mod md4_tests {
//     use super::*;

//     #[test]
//     fn test_suite() {

//     }
// }
