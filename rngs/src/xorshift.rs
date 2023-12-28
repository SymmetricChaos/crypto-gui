use crate::traits::ClassicRng;

pub struct Xorshift32 {
    pub state: u32,
}

impl Default for Xorshift32 {
    fn default() -> Self {
        Self { state: 0x0BAD_5EED }
    }
}

impl ClassicRng for Xorshift32 {
    fn next_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }
}

pub struct Xorshift128 {
    pub state: [u32; 4],
}

impl Default for Xorshift128 {
    fn default() -> Self {
        Self {
            state: [0x0BAD_5EED, 0x0BAD_5EED, 0x0BAD_5EED, 0x0BAD_5EED],
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
