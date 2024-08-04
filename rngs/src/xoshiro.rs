use strum::{Display, EnumIter};

use crate::traits::ClassicRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum Scrambler {
    PlusPlus,
    StarStar,
}

pub struct Xoshiro256 {
    pub state: [u64; 4],
    pub scrambler: Scrambler,
}

impl Default for Xoshiro256 {
    fn default() -> Self {
        Self {
            state: [0, 0, 0, 0],
            scrambler: Scrambler::PlusPlus,
        }
    }
}

impl Xoshiro256 {
    const JUMP: [u64; 4] = [
        0x180ec6d33cfd0aba,
        0xd5a61266f0c9392c,
        0xa9582618e03fc9aa,
        0x39abdc4529b1661c,
    ];
    const LONG_JUMP: [u64; 4] = [
        0x76e15d3efefdcbbf,
        0xc5004e441c522fb3,
        0x77710069854ee241,
        0x39109bb02acbe635,
    ];

    pub fn step(&mut self) {
        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);
    }

    pub fn output(&mut self) -> u64 {
        match self.scrambler {
            Scrambler::PlusPlus => (self.state[0].wrapping_add(self.state[3]))
                .rotate_left(23)
                .wrapping_add(self.state[0]),
            Scrambler::StarStar => (self.state[1].wrapping_mul(5))
                .rotate_left(7)
                .wrapping_mul(9),
        }
    }

    // Jumps forward by 2^128 steps
    pub fn jump(&mut self) {
        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        for i in 0..4 {
            for b in 0..64 {
                if Self::JUMP[i] & (1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                    s2 ^= self.state[2];
                    s3 ^= self.state[3];
                }
                self.step()
            }
        }
        self.state[0] = s0;
        self.state[1] = s1;
        self.state[2] = s2;
        self.state[3] = s3;
    }

    // Jumps forward by 2^192 steps
    pub fn long_jump(&mut self) {
        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        for i in 0..4 {
            for b in 0..64 {
                if Self::LONG_JUMP[i] & (1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                    s2 ^= self.state[2];
                    s3 ^= self.state[3];
                }
                self.step()
            }
        }
        self.state[0] = s0;
        self.state[1] = s1;
        self.state[2] = s2;
        self.state[3] = s3;
    }
}

impl ClassicRng for Xoshiro256 {
    fn next_u32(&mut self) -> u32 {
        let out = (self.output() >> 32) as u32;
        self.step();
        out
    }
}
