use crate::traits::ClassicRng;

pub struct HaltonSequence {
    pub base: u32,
    pub num: u32,
    pub den: u32,
}

impl Default for HaltonSequence {
    fn default() -> Self {
        Self {
            base: 3,
            num: 0,
            den: 1,
        }
    }
}

impl HaltonSequence {}

impl ClassicRng for HaltonSequence {
    fn step(&mut self) {
        let x = self.den - self.num;

        if x == 1 {
            self.num = 1;
            self.den *= self.base;
        } else {
            let mut y = self.den / self.base;
            while x <= y {
                y /= self.base;
            }
            self.num = (self.base + 1) * y - x;
        }
    }
}
