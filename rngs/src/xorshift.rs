use crate::traits::ClassicRng;

pub struct Xorshift {
    pub state: u32,
}

impl Default for Xorshift {
    fn default() -> Self {
        Self { state: 1257924810 }
    }
}

impl ClassicRng for Xorshift {
    fn next_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }
}
