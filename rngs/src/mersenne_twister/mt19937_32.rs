use std::cmp::max;

use crate::traits::ClassicRng;

pub const N: usize = 624;
pub const M: usize = 397;
pub const A: u32 = 0x9908b0df;
pub const UPPER_MASK: u32 = 0x80000000;
pub const LOWER_MASK: u32 = 0x7fffffff;

pub struct Mt19937_32 {
    pub index: usize,
    pub arr: [u32; N],
}

impl Default for Mt19937_32 {
    fn default() -> Self {
        Self::from_u32(5489)
    }
}

impl Mt19937_32 {
    pub fn ksa_default(&mut self) {
        self.ksa_from_u32(5489)
    }

    pub fn from_u32(key: u32) -> Self {
        let mut arr = [0u32; N];
        let index = N;
        arr[0] = key; // default key
        for i in 1..N {
            arr[i] = 1812433253_u32
                .wrapping_mul(arr[i - 1] ^ (arr[i - 1] >> 30))
                .wrapping_add(i as u32)
        }
        Self { index, arr }
    }

    pub fn ksa_from_u32(&mut self, key: u32) {
        self.arr = [0u32; N];
        self.index = N;
        self.arr[0] = key;
        for i in 1..N {
            self.arr[i] = 1812433253_u32
                .wrapping_mul(self.arr[i - 1] ^ (self.arr[i - 1] >> 30))
                .wrapping_add(i as u32)
        }
    }

    pub fn from_array(key: &[u32]) -> Self {
        let mut rng = Self::from_u32(19650218u32);
        let mut i = 1;
        let mut j = 0;
        for _ in 0..max(N, key.len()) {
            rng.arr[i] = (rng.arr[i]
                ^ ((rng.arr[i - 1] ^ (rng.arr[i - 1] >> 30)).wrapping_mul(1664525)))
            .wrapping_add(key[j])
            .wrapping_add(j as u32);
            i += 1;
            if i >= N {
                rng.arr[0] = rng.arr[N - 1];
                i = 1;
            }
            j += 1;
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..N - 1 {
            rng.arr[i] = (rng.arr[i]
                ^ ((rng.arr[i - 1] ^ (rng.arr[i - 1] >> 30)).wrapping_mul(1566083941)))
            .wrapping_sub(i as u32);
            i += 1;
            if i >= N {
                rng.arr[0] = rng.arr[N - 1];
                i = 1;
            }
        }
        rng.arr[0] = 1 << 31;
        rng
    }

    pub fn ksa_from_array(&mut self, key: &[u32]) {
        self.ksa_from_u32(19650218u32);
        let mut i = 1;
        let mut j = 0;
        for _ in 0..max(N, key.len()) {
            self.arr[i] = (self.arr[i]
                ^ ((self.arr[i - 1] ^ (self.arr[i - 1] >> 30)).wrapping_mul(1664525)))
            .wrapping_add(key[j])
            .wrapping_add(j as u32);
            i += 1;
            if i >= N {
                self.arr[0] = self.arr[N - 1];
                i = 1;
            }
            j += 1;
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..N - 1 {
            self.arr[i] = (self.arr[i]
                ^ ((self.arr[i - 1] ^ (self.arr[i - 1] >> 30)).wrapping_mul(1566083941)))
            .wrapping_sub(i as u32);
            i += 1;
            if i >= N {
                self.arr[0] = self.arr[N - 1];
                i = 1;
            }
        }
        self.arr[0] = 1 << 31;
    }

    pub fn twist(&mut self) {
        for i in 0..N - M {
            let x = (self.arr[i] & UPPER_MASK) | (self.arr[i + 1] & LOWER_MASK);
            self.arr[i] = self.arr[i + M] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A));
        }
        for i in N - M..N - 1 {
            let x = (self.arr[i] & UPPER_MASK) | (self.arr[i + 1] & LOWER_MASK);
            self.arr[i] = self.arr[i + M - N] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A));
        }
        let x = (self.arr[N - 1] & UPPER_MASK) | (self.arr[0] & LOWER_MASK);
        self.arr[N - 1] = self.arr[M - 1] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A));
        self.index = 0;
    }

    pub fn temper(mut x: u32) -> u32 {
        x ^= x >> 11;
        x ^= (x << 7) & 0x9d2c5680;
        x ^= (x << 15) & 0xefc60000;
        x ^= x >> 18;
        x
    }

    pub fn untemper(mut x: u32) -> u32 {
        x ^= x >> 18;

        x ^= (x << 15) & 0x2fc60000;
        x ^= (x << 15) & 0xc0000000;

        x ^= (x << 7) & 0x00001680;
        x ^= (x << 7) & 0x000c4000;
        x ^= (x << 7) & 0x0d200000;
        x ^= (x << 7) & 0x90000000;

        x ^= x >> 11;
        x ^= x >> 22;

        x
    }
}

impl ClassicRng for Mt19937_32 {
    fn next_u32(&mut self) -> u32 {
        // index should never be zero here
        // if it is set default value
        if self.index == 0 {
            self.ksa_default()
        }
        if self.index >= N {
            self.twist(); // this sets self.index to zero
        }
        let y = self.arr[self.index];
        self.index += 1;
        Self::temper(y)
    }
}
