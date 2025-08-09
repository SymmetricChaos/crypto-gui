use crate::SimpleRng;

pub struct WyRand {
    pub state: u64,
}

impl Default for WyRand {
    fn default() -> Self {
        Self { state: 1234567890 }
    }
}

impl SimpleRng for WyRand {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x2d358dccaa6c78a5);
        let t = (self.state as u128).wrapping_mul((self.state ^ 0x8bb84b93962eacc9) as u128);
        ((t >> 64) ^ t) as u64
    }
}
