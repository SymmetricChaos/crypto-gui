use crate::traits::ClassicRng;

pub struct Pcg {
    pub state: u64,
    pub modulus: u64,
    pub multiplier: u64,
    pub increment: u64,
}

impl Default for Pcg {
    fn default() -> Self {
        Self {
            state: 1257924810,
            modulus: 4294967295,
            multiplier: 1664525,
            increment: 1013904223,
        }
    }
}

impl Pcg {
    pub fn pcg_rs(&self) -> u32 {
        (self.state >> (29 - (self.state >> 61))) as u32
    }

    pub fn pcg_rr(&self) -> u32 {
        (self.state.rotate_left(29 - (self.state >> 61) as u32)) as u32
    }

    pub fn pcg_xsh_rr(&self) -> u32 {
        u64::rotate_left(
            (self.state ^ (self.state >> 18)) >> 27,
            (self.state >> 59) as u32,
        ) as u32
    }

    pub fn pcg_xsh_rs(&self) -> u32 {
        ((self.state ^ (self.state >> 22)) >> (22 + (self.state >> 61))) as u32
    }
}

impl ClassicRng for Pcg {
    fn step(&mut self) {
        // No overflows can happen here because the inputs are are u32 initially
        let m = (self.multiplier as u128 * self.state as u128) % self.modulus as u128;
        self.state = ((m + self.increment as u128) % self.modulus as u128) as u64;
    }
}
