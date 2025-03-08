use strum::EnumIter;

use crate::traits::ClassicRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, strum::Display)]
pub enum Scrambler {
    None,
    Plus,
    Star,
}

pub struct Xorshift64 {
    pub state: u64,
    pub scrambler: Scrambler,
}

impl Default for Xorshift64 {
    fn default() -> Self {
        Self {
            state: 0x0BAD_5EED0BAD_5EED,
            scrambler: Scrambler::None,
        }
    }
}

impl ClassicRng for Xorshift64 {
    fn next_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        match self.scrambler {
            Scrambler::None => (self.state >> 32) as u32,
            Scrambler::Plus => (self.state >> 32).wrapping_add(self.state << 32) as u32,
            Scrambler::Star => (self.state >> 32).wrapping_mul(0x2545F4914F6CDD1) as u32,
        }
    }
}

pub struct Xorshift128 {
    pub state: [u32; 4],
    pub scrambler: Scrambler,
}

impl Default for Xorshift128 {
    fn default() -> Self {
        Self {
            state: [0x0BAD_5EED, 0x0BAD_5EED, 0x0BAD_5EED, 0x0BAD_5EED],
            scrambler: Scrambler::None,
        }
    }
}

impl ClassicRng for Xorshift128 {
    fn next_u32(&mut self) -> u32 {
        let x = self.state[0];
        let t = x ^ (x << 11);
        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[2] = self.state[3];
        let tw = self.state[3];
        self.state[3] = tw ^ (tw >> 19) ^ (t ^ (t >> 8));

        self.state[3]
    }
}
