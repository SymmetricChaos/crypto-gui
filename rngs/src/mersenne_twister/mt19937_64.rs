use std::cmp::max;

use crate::traits::ClassicRng;

pub const N: usize = 312;
pub const M: usize = 156;
pub const A: u64 = 0xb5026f5aa96619e9;
pub const UPPER_MASK: u64 = 0xffffffff80000000;
pub const LOWER_MASK: u64 = 0x7fffffff;

pub struct Mt19937_64 {
    pub index: usize,
    pub arr: [u64; N],
}

impl Default for Mt19937_64 {
    fn default() -> Self {
        Self::from_u64(5489)
    }
}

impl Mt19937_64 {
    pub fn ksa_default(&mut self) {
        self.ksa_from_u64(5489)
    }

    pub fn from_u64(key: u64) -> Self {
        let mut arr = [0u64; N];
        let index = N;
        arr[0] = key;
        for i in 1..N {
            arr[i] = 6364136223846793005_u64
                .wrapping_mul(arr[i - 1] ^ (arr[i - 1] >> 62))
                .wrapping_add(i as u64)
        }
        Self { index, arr }
    }

    pub fn ksa_from_u64(&mut self, key: u64) {
        self.arr = [0u64; N];
        self.index = N;
        self.arr[0] = key;
        for i in 1..N {
            self.arr[i] = 6364136223846793005_u64
                .wrapping_mul(self.arr[i - 1] ^ (self.arr[i - 1] >> 62))
                .wrapping_add(i as u64)
        }
    }

    pub fn from_array(key: &[u64]) -> Self {
        let mut rng = Self::from_u64(19650218);
        let mut i = 1;
        let mut j = 0;
        for _ in 0..max(N, key.len()) {
            rng.arr[i] = (rng.arr[i]
                ^ ((rng.arr[i - 1] ^ (rng.arr[i - 1] >> 62)).wrapping_mul(3935559000370003845)))
            .wrapping_add(key[j])
            .wrapping_add(j as u64);
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
                ^ ((rng.arr[i - 1] ^ (rng.arr[i - 1] >> 62)).wrapping_mul(2862933555777941757)))
            .wrapping_sub(i as u64);
            i += 1;
            if i >= N {
                rng.arr[0] = rng.arr[N - 1];
                i = 1;
            }
        }
        rng.arr[0] = 1 << 63;
        rng
    }

    pub fn ksa_from_array(&mut self, key: &[u64]) {
        self.ksa_from_u64(19650218);
        let mut i = 1;
        let mut j = 0;
        for _ in 0..max(N, key.len()) {
            self.arr[i] = (self.arr[i]
                ^ ((self.arr[i - 1] ^ (self.arr[i - 1] >> 62)).wrapping_mul(3935559000370003845)))
            .wrapping_add(key[j])
            .wrapping_add(j as u64);
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
                ^ ((self.arr[i - 1] ^ (self.arr[i - 1] >> 62)).wrapping_mul(2862933555777941757)))
            .wrapping_sub(i as u64);
            i += 1;
            if i >= N {
                self.arr[0] = self.arr[N - 1];
                i = 1;
            }
        }
        self.arr[0] = 1 << 63;
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

    pub fn temper(mut x: u64) -> u64 {
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71d67fffeda60000;
        x ^= (x << 37) & 0xfff7eee000000000;
        x ^= x >> 43;
        x
    }

    pub fn untemper(mut x: u64) -> u64 {
        x ^= x >> 43;

        x ^= (x << 37) & 0xfff7eee000000000;

        x ^= (x << 17) & 0x00000003eda60000;
        x ^= (x << 17) & 0x00067ffc00000000;
        x ^= (x << 17) & 0x71d0000000000000;

        x ^= (x >> 29) & 0x0000000555555540;
        x ^= (x >> 29) & 0x0000000000000015;

        x
    }
}

impl ClassicRng for Mt19937_64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
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
