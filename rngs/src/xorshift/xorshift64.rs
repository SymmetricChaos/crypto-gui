use super::XorshiftScrambler;
use crate::traits::ClassicRng;

pub struct Xorshift64 {
    pub state: u64,
    pub scrambler: XorshiftScrambler,
}

impl Default for Xorshift64 {
    fn default() -> Self {
        Self {
            state: 0x0BAD_5EED0BAD_5EED,
            scrambler: XorshiftScrambler::None,
        }
    }
}

impl ClassicRng for Xorshift64 {
    fn next_u32(&mut self) -> u32 {
        crate::xorshift_lrl!(self.state, 13, 7, 17);
        match self.scrambler {
            XorshiftScrambler::None => (self.state >> 32) as u32,
            XorshiftScrambler::Plus => (self.state >> 32).wrapping_add(self.state << 32) as u32,
            XorshiftScrambler::Star => (self.state >> 32).wrapping_mul(0x2545F4914F6CDD1) as u32,
        }
    }
}
