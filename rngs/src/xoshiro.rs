use crate::traits::ClassicRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transform {
    PlusPlus,
    StarStar,
}

pub struct Xoshiro {
    pub state: [u64; 4],
    pub transform: Transform,
}

impl Default for Xoshiro {
    fn default() -> Self {
        Self {
            state: [0, 0, 0, 0],
            transform: Transform::PlusPlus,
        }
    }
}

impl Xoshiro {
    pub fn step(&mut self) {
        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);
    }

    pub fn transform(&mut self) -> u64 {
        match self.transform {
            Transform::PlusPlus => (self.state[0].wrapping_add(self.state[3]))
                .rotate_left(23)
                .wrapping_add(self.state[0]),
            Transform::StarStar => (self.state[1].wrapping_mul(5))
                .rotate_left(7)
                .wrapping_mul(9),
        }
    }
}

impl ClassicRng for Xoshiro {
    fn next_u32(&mut self) -> u32 {
        let out = self.transform() as u32;
        self.step();
        out
    }
}
