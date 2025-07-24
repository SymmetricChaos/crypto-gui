use crate::ClassicRng;

// 64 bit
const PHILOX_M2_64: u64 = 0xD2B74407B1CE6E93;
const PHILOX_M4_64: [u64; 2] = [0xD2E7470EE14C6C93, 0xCA5A826395121157];

// 32 bit
const PHILOX_M2_32: u32 = 0xD256D193;
const PHILOX_M4_32: [u32; 2] = [0xD2511F53, 0xCD9E8D57];

//64 bit
const PHILOX_W_64: [u64; 2] = [0x9E3779B97F4A7C15, 0xBB67AE8584CAA73B];

// 32 bit
const PHILOX_W_32: [u32; 2] = [0x9E3779B9, 0xBB67AE85];

#[inline]
fn mul32(a: u32, b: u32) -> (u32, u32) {
    let p = (a as u64).wrapping_mul(b as u64);
    ((p >> 32) as u32, (p as u32))
}

#[inline]
fn mul64(a: u64, b: u64) -> (u64, u64) {
    let p = (a as u128).wrapping_mul(b as u128);
    ((p >> 64) as u64, (p as u64))
}

#[derive(Debug)]
pub struct Philox2_32 {
    pub key: u32,
    pub ctr: [u32; 2],
    pub saved: [u32; 2],
    idx: usize,
    pub rounds: usize,
}

impl Default for Philox2_32 {
    fn default() -> Self {
        Self {
            key: 0,
            ctr: [0, 0],
            saved: [0, 0],
            idx: 0,
            rounds: 10,
        }
    }
}

impl Philox2_32 {
    fn next_round_key(key: &mut u32) {
        *key = key.wrapping_add(PHILOX_W_32[0])
    }

    fn round(key: u32, ctr: &mut [u32; 2]) {
        let (hi, lo) = mul32(PHILOX_M2_32, ctr[0]);
        ctr[0] = hi ^ ctr[1] ^ key;
        ctr[1] = lo;
    }

    pub fn array(&self) -> [u32; 2] {
        let mut key = self.key;
        let mut ctr = self.ctr;
        for _ in 0..(self.rounds - 1) {
            Self::round(key, &mut ctr);
            Self::next_round_key(&mut key);
        }
        Self::round(key, &mut ctr);
        ctr
    }
}

impl ClassicRng for Philox2_32 {
    fn next_u32(&mut self) -> u32 {
        if self.idx == 0 {
            self.saved = self.array();
            self.ctr[0] = self.ctr[0].wrapping_add(1);
            if self.ctr[0] == 0 {
                self.ctr[1] = self.ctr[1].wrapping_add(1);
            }
        }
        let out = self.saved[self.idx];
        self.idx = (self.idx + 1) % 2;
        out
    }
}

#[derive(Debug)]
pub struct Philox4_32 {
    pub key: [u32; 2],
    pub ctr: [u32; 4],
    pub saved: [u32; 4],
    idx: usize,
    pub rounds: usize,
}

impl Default for Philox4_32 {
    fn default() -> Self {
        Self {
            key: [0, 0],
            ctr: [0, 0, 0, 0],
            saved: [0, 0, 0, 0],
            idx: 0,
            rounds: 10,
        }
    }
}

impl Philox4_32 {
    fn next_round_key(key: &mut [u32; 2]) {
        key[0] = key[0].wrapping_add(PHILOX_W_32[0]);
        key[1] = key[1].wrapping_add(PHILOX_W_32[1]);
    }

    fn round(key: &[u32; 2], ctr: &mut [u32; 4]) {
        let (hi1, lo1) = mul32(PHILOX_M4_32[0], ctr[0]);
        let (hi2, lo2) = mul32(PHILOX_M4_32[1], ctr[2]);
        ctr[0] = hi2 ^ ctr[1] ^ key[0];
        ctr[1] = lo2;
        ctr[2] = hi1 ^ ctr[3] ^ key[1];
        ctr[3] = lo1;
    }

    pub fn array(&self) -> [u32; 4] {
        let mut key = self.key;
        let mut ctr = self.ctr;
        for _ in 0..(self.rounds - 1) {
            Self::round(&key, &mut ctr);
            Self::next_round_key(&mut key);
        }
        Self::round(&key, &mut ctr);
        ctr
    }
}

impl ClassicRng for Philox4_32 {
    fn next_u32(&mut self) -> u32 {
        if self.idx == 0 {
            self.saved = self.array();
            self.ctr[0] = self.ctr[0].wrapping_add(1);
            if self.ctr[0] == 0 {
                self.ctr[1] = self.ctr[1].wrapping_add(1);
                if self.ctr[1] == 0 {
                    self.ctr[2] = self.ctr[2].wrapping_add(1);
                    if self.ctr[2] == 0 {
                        self.ctr[3] = self.ctr[3].wrapping_add(1);
                    }
                }
            }
        }

        let out = self.saved[self.idx];
        self.idx = (self.idx + 1) % 4;
        out
    }
}

#[derive(Debug)]
pub struct Philox2_64 {
    pub key: u64,
    pub ctr: [u64; 2],
    pub saved: [u64; 2],
    idx: usize,
    pub rounds: usize,
}

impl Default for Philox2_64 {
    fn default() -> Self {
        Self {
            key: 0,
            ctr: [0, 0],
            saved: [0, 0],
            idx: 0,
            rounds: 10,
        }
    }
}

impl Philox2_64 {
    fn next_round_key(key: &mut u64) {
        *key = key.wrapping_add(PHILOX_W_64[0])
    }

    fn round(key: u64, ctr: &mut [u64; 2]) {
        let (hi, lo) = mul64(PHILOX_M2_64, ctr[0]);
        ctr[0] = hi ^ ctr[1] ^ key;
        ctr[1] = lo;
    }

    pub fn array(&self) -> [u64; 2] {
        let mut key = self.key;
        let mut ctr = self.ctr;
        for _ in 0..(self.rounds - 1) {
            Self::round(key, &mut ctr);
            Self::next_round_key(&mut key);
        }
        Self::round(key, &mut ctr);
        ctr
    }
}

impl ClassicRng for Philox2_64 {
    /// The 64-bit Philox is meant to produce 64-bit random numbers and this methods ignores the upper bits.
    /// To make use of all the bits for smaller values extract them from .next_u64() or from .array()
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        if self.idx == 0 {
            self.saved = self.array();
            self.ctr[0] = self.ctr[0].wrapping_add(1);
            if self.ctr[0] == 0 {
                self.ctr[1] = self.ctr[1].wrapping_add(1);
            }
        }
        let out = self.saved[self.idx];
        self.idx = (self.idx + 1) % 2;
        out
    }
}

#[derive(Debug)]
pub struct Philox4_64 {
    pub key: [u64; 2],
    pub ctr: [u64; 4],
    pub saved: [u64; 4],
    idx: usize,
    pub rounds: usize,
}

impl Default for Philox4_64 {
    fn default() -> Self {
        Self {
            key: [0, 0],
            ctr: [0, 0, 0, 0],
            saved: [0, 0, 0, 0],
            idx: 0,
            rounds: 10,
        }
    }
}

impl Philox4_64 {
    fn next_round_key(key: &mut [u64; 2]) {
        key[0] = key[0].wrapping_add(PHILOX_W_64[0]);
        key[1] = key[1].wrapping_add(PHILOX_W_64[1]);
    }

    fn round(key: &[u64; 2], ctr: &mut [u64; 4]) {
        let (hi1, lo1) = mul64(PHILOX_M4_64[0], ctr[0]);
        let (hi2, lo2) = mul64(PHILOX_M4_64[1], ctr[2]);
        ctr[0] = hi2 ^ ctr[1] ^ key[0];
        ctr[1] = lo2;
        ctr[2] = hi1 ^ ctr[3] ^ key[1];
        ctr[3] = lo1;
    }

    pub fn array(&self) -> [u64; 4] {
        let mut key = self.key;
        let mut ctr = self.ctr;
        for _ in 0..(self.rounds - 1) {
            Self::round(&key, &mut ctr);
            Self::next_round_key(&mut key);
        }
        Self::round(&key, &mut ctr);
        ctr
    }
}

impl ClassicRng for Philox4_64 {
    /// The 64-bit Philox is meant to produce 64-bit random numbers and this methods ignores the upper bits.
    /// To make use of all the bits for smaller values extract them from .next_u64() or from .array()
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        if self.idx == 0 {
            self.saved = self.array();
            self.ctr[0] = self.ctr[0].wrapping_add(1);
            if self.ctr[0] == 0 {
                self.ctr[1] = self.ctr[1].wrapping_add(1);
                if self.ctr[1] == 0 {
                    self.ctr[2] = self.ctr[2].wrapping_add(1);
                    if self.ctr[2] == 0 {
                        self.ctr[3] = self.ctr[3].wrapping_add(1);
                    }
                }
            }
        }
        let out = self.saved[self.idx];
        self.idx = (self.idx + 1) % 4;
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sequence2_32() {
        let mut rng = Philox2_32::default();

        rng.key = 0;
        rng.ctr = [0, 0];
        assert_eq!([0xff1dae59, 0x6cd10df2], rng.array());

        rng.key = 0xffffffff;
        rng.ctr = [0xffffffff, 0xffffffff];
        assert_eq!([0x2c3f628b, 0xab4fd7ad], rng.array());

        rng.key = 0x13198a2e;
        rng.ctr = [0x243f6a88, 0x85a308d3];
        assert_eq!([0xdd7ce038, 0xf62a4c12], rng.array());
    }

    #[test]
    fn sequence4_32() {
        let mut rng = Philox4_32::default();

        rng.key = [0, 0];
        rng.ctr = [0, 0, 0, 0];
        assert_eq!(
            [0x6627e8d5, 0xe169c58d, 0xbc57ac4c, 0x9b00dbd8],
            rng.array()
        );

        rng.key = [0xffffffff, 0xffffffff];
        rng.ctr = [0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff];
        assert_eq!(
            [0x408f276d, 0x41c83b0e, 0xa20bc7c6, 0x6d5451fd],
            rng.array()
        );

        rng.key = [0xa4093822, 0x299f31d0];
        rng.ctr = [0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344];
        assert_eq!(
            [0xd16cfe09, 0x94fdcceb, 0x5001e420, 0x24126ea1],
            rng.array()
        );
    }

    #[test]
    fn sequence2_64() {
        let mut rng = Philox2_64::default();

        rng.key = 0;
        rng.ctr = [0, 0];
        assert_eq!([0xca00a0459843d731, 0x66c24222c9a845b5], rng.array());

        rng.key = 0xffffffffffffffff;
        rng.ctr = [0xffffffffffffffff, 0xffffffffffffffff];
        assert_eq!([0x65b021d60cd8310f, 0x4d02f3222f86df20], rng.array());

        rng.key = 0xa4093822299f31d0;
        rng.ctr = [0x243f6a8885a308d3, 0x13198a2e03707344];
        assert_eq!([0x0a5e742c2997341c, 0xb0f883d38000de5d], rng.array());
    }

    #[test]
    fn sequence4_64() {
        let mut rng = Philox4_64::default();

        rng.key = [0, 0];
        rng.ctr = [0, 0, 0, 0];
        assert_eq!(
            [
                0x16554d9eca36314c,
                0xdb20fe9d672d0fdc,
                0xd7e772cee186176b,
                0x7e68b68aec7ba23b
            ],
            rng.array()
        );

        rng.key = [0xffffffffffffffff, 0xffffffffffffffff];
        rng.ctr = [
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ];
        assert_eq!(
            [
                0x87b092c3013fe90b,
                0x438c3c67be8d0224,
                0x9cc7d7c69cd777b6,
                0xa09caebf594f0ba0
            ],
            rng.array()
        );

        rng.key = [0x452821e638d01377, 0xbe5466cf34e90c6c];
        rng.ctr = [
            0x243f6a8885a308d3,
            0x13198a2e03707344,
            0xa4093822299f31d0,
            0x082efa98ec4e6c89,
        ];
        assert_eq!(
            [
                0xa528f45403e61d95,
                0x38c72dbd566e9788,
                0xa5a1610e72fd18b5,
                0x57bd43b5e52b7fe6
            ],
            rng.array()
        );
    }
}
