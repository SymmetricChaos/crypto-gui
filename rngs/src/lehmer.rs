use crate::traits::SimpleRng;

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
    pub fn new(state: u128, multiplier: u128) -> Self {
        Self { state, multiplier }
    }
}

impl SimpleRng for Lehmer {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(self.multiplier);
        (self.state >> 64) as u64
    }
}
