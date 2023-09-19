use crate::traits::ClassicRng;

pub struct Lcg {
    pub state: u32,
    pub modulus: u32,
    pub multiplier: u32,
    pub increment: u32,
}

impl ClassicRng for Lcg {
    fn next(&mut self) -> u32 {
        // No overflows can happen here because the inputs are are u32 initially
        let m = (self.multiplier as u64 * self.state as u64) % self.modulus as u64;
        self.state = ((m + self.increment as u64) % self.modulus as u64) as u32;
        self.state
    }
}
