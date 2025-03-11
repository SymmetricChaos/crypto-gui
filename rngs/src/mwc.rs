use std::num::Wrapping;

use crate::ClassicRng;

// const A1: u128 = 0xffebb71d94fcdaf9;

pub struct MultiplyWithCarry128 {
    a: Wrapping<u128>,
    x: Wrapping<u128>,
    c: Wrapping<u128>,
}

impl Default for MultiplyWithCarry128 {
    fn default() -> Self {
        Self {
            a: Wrapping(0xffebb71d94fcdaf9),
            x: Wrapping(0x0BAD_5EED),
            c: Wrapping(1),
        }
    }
}

impl MultiplyWithCarry128 {
    pub fn from_u64(seed: u64) -> Self {
        Self {
            a: Wrapping(0xffebb71d94fcdaf9),
            x: Wrapping(seed as u128),
            c: Wrapping(1),
        }
    }
}

impl ClassicRng for MultiplyWithCarry128 {
    fn next_u32(&mut self) -> u32 {
        let out = self.x;
        let t = self.a * self.x + self.c;
        self.x = t;
        self.c = t >> 64;
        out.0 as u32
    }

    fn next_u64(&mut self) -> u64 {
        let out = self.x;
        let t = self.a * self.x + self.c;
        self.x = t;
        self.c = t >> 64;
        out.0 as u64
    }
}
