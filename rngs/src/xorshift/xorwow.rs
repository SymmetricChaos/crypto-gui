use crate::traits::ClassicRng;

pub struct Xorwow {
    pub state: [u32; 5],
    pub ctr: u32,
}

impl Default for Xorwow {
    fn default() -> Self {
        Self {
            state: [
                0x0BAD_5EED,
                0x0BAD_5EED,
                0x0BAD_5EED,
                0x0BAD_5EED,
                0x0BAD_5EED,
            ],
            ctr: 0,
        }
    }
}

impl ClassicRng for Xorwow {
    fn next_u32(&mut self) -> u32 {
        let t = self.state[0] ^ (self.state[0] >> 2);

        // 32-bit rotation of the whole state
        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[3] = self.state[4];
        self.state[4] = self.state[5];

        self.state[5] ^= (self.state[5] << 4) ^ (t ^ (t << 1));

        // any odd constant will produce the necessary Weyl sequence
        // Marsaglia also notes that XOR can replace addition here
        self.ctr = self.ctr.wrapping_add(362437);

        self.state[5].wrapping_add(self.ctr)
    }
}
