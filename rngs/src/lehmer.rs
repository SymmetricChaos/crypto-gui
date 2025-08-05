use crate::traits::ClassicRng;

pub struct Lehmer {
    pub state: u128,
    pub multiplier: u128,
}

impl Default for Lehmer {
    fn default() -> Self {
        Self {
            state: 1257924810,
            multiplier: 0xda942042e4dd58b5,
        }
    }
}

impl Lehmer {
    pub fn new(state: u64, multiplier: u64) -> Self {
        Self {
            state: state as u128,
            multiplier: multiplier as u128,
        }
    }
}

impl ClassicRng for Lehmer {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(self.multiplier);
        (self.state >> 64) as u64
    }
}
