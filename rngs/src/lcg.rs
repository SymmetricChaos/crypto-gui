use crate::traits::ClassicRng;

pub struct Lcg32 {
    pub state: u32,
    pub modulus: u32,
    pub multiplier: u32,
    pub increment: u32,
}

impl Default for Lcg32 {
    fn default() -> Self {
        Self {
            state: 1257924810,
            modulus: 4294967295,
            multiplier: 1664525,
            increment: 1013904223,
        }
    }
}

impl ClassicRng for Lcg32 {
    fn next_u32(&mut self) -> u32 {
        let m = (self.state as u64 * self.multiplier as u64) % self.modulus as u64;
        self.state = ((m + self.increment as u64) % self.modulus as u64) as u32;
        self.state
    }
}

pub struct Lcg64 {
    pub state: u64,
    pub modulus: u64,
    pub multiplier: u64,
    pub increment: u64,
}

impl Default for Lcg64 {
    fn default() -> Self {
        Self {
            state: 1257924810,
            modulus: 4294967295,
            multiplier: 1664525,
            increment: 1013904223,
        }
    }
}

impl ClassicRng for Lcg64 {
    fn next_u32(&mut self) -> u32 {
        let m = (self.state as u128 * self.multiplier as u128) % self.modulus as u128;
        self.state = ((m + self.increment as u128) % self.modulus as u128) as u64;
        self.state as u32
    }

    fn next_u64(&mut self) -> u64 {
        let m = (self.state as u128 * self.multiplier as u128) % self.modulus as u128;
        self.state = ((m + self.increment as u128) % self.modulus as u128) as u64;
        self.state
    }
}
