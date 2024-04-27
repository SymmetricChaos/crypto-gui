use crate::traits::ClassicRng;

pub struct MiddleSquareBinary {
    pub state: u32,
}

impl Default for MiddleSquareBinary {
    fn default() -> Self {
        Self { state: 123456 }
    }
}

impl ClassicRng for MiddleSquareBinary {
    fn next_u32(&mut self) -> u32 {
        let sq = self.state as u64 * self.state as u64;
        let mid = (sq >> 16) as u32;
        self.state = mid;
        mid
    }
}
