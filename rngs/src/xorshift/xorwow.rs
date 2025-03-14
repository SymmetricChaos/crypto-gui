use num::Integer;

use crate::traits::ClassicRng;

pub struct Xorwow {
    pub state: [u32; 5],
    pub ctr: u32,
    pub weyl: u32,
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
            weyl: 362437, // any odd constant will produce a Weyl sequence
        }
    }
}

impl ClassicRng for Xorwow {
    fn next_u32(&mut self) -> u32 {
        assert!(self.weyl.is_odd());

        let t = self.state[0] ^ (self.state[0] >> 2);

        // 32-bit rotation of the whole state
        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[2] = self.state[3];
        self.state[3] = self.state[4];

        self.state[4] ^= (self.state[4] << 4) ^ (t ^ (t << 1));

        // Marsaglia also notes that XOR can replace addition here
        self.ctr = self.ctr.wrapping_add(self.weyl);

        self.state[4].wrapping_add(self.ctr)
    }
}
