use crate::ClassicRng;

const A1: u128 = 0xffebb71d94fcdaf9;

pub struct MultiplyWithCarry128 {
    a: u128,
    x: u128,
    c: u128,
}

impl Default for MultiplyWithCarry128 {
    fn default() -> Self {
        Self {
            a: 0xffebb71d94fcdaf9,
            x: 0x0BAD_5EED,
            c: 1,
        }
    }
}

impl MultiplyWithCarry128 {}

impl ClassicRng for MultiplyWithCarry128 {
    fn next_u32(&mut self) -> u32 {
        let out = self.x;
        let t = self.a * self.x + self.c;
        self.x = t;
        self.c = t >> 64;
        out as u32
    }
}
