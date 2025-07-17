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

pub struct Philox2_32 {
    pub key: u32,
    pub ctr: [u32; 2],
}

impl Philox2_32 {
    pub fn bumpkey(&mut self) {
        self.key = self.key.wrapping_add(PHILOX_W_32[0])
    }

    fn round(&mut self) {
        let (hi, lo) = mul32(PHILOX_M2_32, self.ctr[0]);
        self.ctr[0] = hi ^ self.ctr[1] ^ self.key;
        self.ctr[1] = lo;
    }
}

impl ClassicRng for Philox2_32 {
    fn next_u32(&mut self) -> u32 {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr[0]
    }
}

pub struct Philox4_32 {
    pub key: [u32; 2],
    pub ctr: [u32; 4],
}

impl Philox4_32 {
    pub fn bumpkey(&mut self) {
        self.key[0] = self.key[0].wrapping_add(PHILOX_W_32[0]);
        self.key[1] = self.key[1].wrapping_add(PHILOX_W_32[1]);
    }

    fn round(&mut self) {
        let (hi1, lo1) = mul32(PHILOX_M4_32[0], self.ctr[0]);
        let (hi2, lo2) = mul32(PHILOX_M4_32[1], self.ctr[1]);
        self.ctr[0] = hi2 ^ self.ctr[1] ^ self.key[0];
        self.ctr[1] = lo2;
        self.ctr[2] = hi1 ^ self.ctr[3] ^ self.key[1];
        self.ctr[3] = lo1;
    }
}

impl ClassicRng for Philox4_32 {
    fn next_u32(&mut self) -> u32 {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr[0]
    }
}

pub struct Philox2_64 {
    pub key: u64,
    pub ctr: [u64; 2],
}

impl Philox2_64 {
    pub fn bumpkey(&mut self) {
        self.key = self.key.wrapping_add(PHILOX_W_64[0])
    }

    fn round(&mut self) {
        let (hi, lo) = mul64(PHILOX_M2_64, self.ctr[0]);
        self.ctr[0] = hi ^ self.ctr[1] ^ self.key;
        self.ctr[1] = lo;
    }
}

impl ClassicRng for Philox2_64 {
    fn next_u32(&mut self) -> u32 {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr[0] as u32
    }

    fn next_u64(&mut self) -> u64 {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr[0]
    }
}

pub struct Philox4_64 {
    pub key: [u64; 2],
    pub ctr: [u64; 4],
}

impl Philox4_64 {
    pub fn bumpkey(&mut self) {
        self.key[0] = self.key[0].wrapping_add(PHILOX_W_64[0]);
        self.key[1] = self.key[1].wrapping_add(PHILOX_W_64[1]);
    }

    fn round(&mut self) {
        let (hi1, lo1) = mul64(PHILOX_M4_64[0], self.ctr[0]);
        let (hi2, lo2) = mul64(PHILOX_M4_64[1], self.ctr[1]);
        self.ctr[0] = hi2 ^ self.ctr[1] ^ self.key[0];
        self.ctr[1] = lo2;
        self.ctr[2] = hi1 ^ self.ctr[3] ^ self.key[1];
        self.ctr[3] = lo1;
    }
}

impl ClassicRng for Philox4_64 {
    fn next_u32(&mut self) -> u32 {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr[0] as u32
    }

    fn next_u64(&mut self) -> u64 {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr[0]
    }
}
