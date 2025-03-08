use crate::traits::ClassicRng;

use super::XorshiftScrambler;

pub struct Xorshift128 {
    pub state: [u32; 4],
    pub scrambler: XorshiftScrambler,
}

impl Default for Xorshift128 {
    fn default() -> Self {
        Self {
            state: [0x0BAD_5EED, 0x0BAD_5EED, 0x0BAD_5EED, 0x0BAD_5EED],
            scrambler: XorshiftScrambler::None,
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
