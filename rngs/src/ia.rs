use crate::ClassicRng;
use std::num::Wrapping;

const ALPHA: u32 = 8;
const SIZE: usize = 1 << ALPHA;
const MASK: u32 = (SIZE - 1) as u32;

pub struct Ia {
    array: [Wrapping<u32>; SIZE],
    b: Wrapping<u32>,
    rand_rsl: [Wrapping<u32>; SIZE], // effectively the output state (I do not know why it is called this)
    ctr: usize,                      // point to the current position in rand_rsl
}

impl Default for Ia {
    fn default() -> Self {
        Self {
            array: [Wrapping(0); SIZE],
            b: Wrapping(0),
            rand_rsl: [Wrapping(0); SIZE],
            ctr: 0,
        }
    }
}

impl Ia {
    fn ia(&mut self) {
        let mut t = self.b;
        for i in 0..SIZE {
            let x = self.array[i];
            let y = self.array[(x.0 & MASK) as usize] + t;
            t = self.array[((y.0 >> ALPHA) & MASK) as usize] + x;
            self.rand_rsl[i] = t;
        }
        self.b = t;
    }
}

impl ClassicRng for Ia {
    fn next_u32(&mut self) -> u32 {
        if self.ctr >= SIZE {
            self.ia();
        }
        let n = self.rand_rsl[self.ctr].0;
        self.ctr += 1;
        n
    }
}
