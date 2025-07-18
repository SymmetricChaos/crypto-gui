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

#[derive(Debug, Default)]
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

    pub fn next_ctr(&mut self) -> [u32; 2] {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr
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

#[derive(Debug, Default)]
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

    pub fn next_ctr(&mut self) -> [u32; 4] {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr
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

#[derive(Debug, Default)]
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

    pub fn next_ctr(&mut self) -> [u64; 2] {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr
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

#[derive(Debug, Default)]
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

    pub fn next_ctr(&mut self) -> [u64; 4] {
        for _ in 0..9 {
            self.round();
            self.bumpkey();
        }
        self.round();
        self.ctr
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sequence2_32() {
        let mut rng = Philox2_32::default();
        rng.key = 0;
        rng.ctr = [0, 0];
        assert_eq!([0xff1dae59, 0x6cd10df2], rng.next_ctr());
        rng.key = 0xffffffff;
        rng.ctr = [0xffffffff, 0xffffffff];
        assert_eq!([0x2c3f628b, 0xab4fd7ad], rng.next_ctr());
        rng.key = 0x13198a2e;
        rng.ctr = [0x243f6a88, 0x85a308d3];
        assert_eq!([0xdd7ce038, 0xf62a4c12], rng.next_ctr());
    }

    #[test]
    fn sequence4_32() {
        let mut rng = Philox4_32::default();
        rng.key = [0, 0];
        rng.ctr = [0, 0, 0, 0];

        assert_eq!(
            [0x6627e8d5, 0xe169c58d, 0xbc57ac4c, 0x9b00dbd8],
            rng.next_ctr()
        );
        rng.key = [0xffffffff, 0xffffffff];
        rng.ctr = [0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff];
        assert_eq!(
            [0x408f276d, 0x41c83b0e, 0xa20bc7c6, 0x6d5451fd],
            rng.next_ctr()
        );
        rng.key = [0x243f6a88, 0x85a308d3];
        rng.ctr = [0x13198a2e, 0x03707344, 0xa4093822, 0x299f31d0];
        assert_eq!(
            [0xd16cfe09, 0x94fdcceb, 0x5001e420, 0x24126ea1],
            rng.next_ctr()
        );
    }
}
