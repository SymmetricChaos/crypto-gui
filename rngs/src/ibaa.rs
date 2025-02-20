use crate::ClassicRng;
use std::num::Wrapping;

// https://burtleburtle.net/bob/rand/isaac.html

const ALPHA: u32 = 8;
const SIZE: usize = 1 << ALPHA;
const MASK: u32 = (SIZE - 1) as u32;

pub struct Ibaa {
    array: [Wrapping<u32>; SIZE],
    a: Wrapping<u32>,
    b: Wrapping<u32>,
    rand_rsl: [Wrapping<u32>; SIZE], // effectively the output state (I do not know why it is called this)
    ctr: usize,                      // point to the current position in rand_rsl
}

impl Default for Ibaa {
    fn default() -> Self {
        Self {
            array: [Wrapping(0); SIZE],
            a: Wrapping(0),
            b: Wrapping(0),
            rand_rsl: [Wrapping(0); SIZE],
            ctr: 0,
        }
    }
}

impl Ibaa {
    fn ibaa(&mut self) {
        let mut ta = self.a;
        let mut tb = self.b;

        for i in 0..SIZE {
            let x = self.array[i];
            ta = Wrapping(ta.0.rotate_left(19)) + self.array[(i + SIZE / 2) & 0xff];
            let y = self.array[(x.0 & MASK) as usize] + ta + tb;
            self.array[i] = y;
            tb = self.array[((y.0 >> ALPHA) & MASK) as usize] + x;
            self.rand_rsl[i] = tb;
        }
        self.a = ta;
        self.b = tb;
    }
}

impl ClassicRng for Ibaa {
    fn next_u32(&mut self) -> u32 {
        if self.ctr >= SIZE {
            self.ibaa();
        }
        let n = self.rand_rsl[self.ctr].0;
        self.ctr += 1;
        n
    }
}
