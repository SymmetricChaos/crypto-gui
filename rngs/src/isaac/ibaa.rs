use crate::SimpleRng;
use std::num::Wrapping;

// https://burtleburtle.net/bob/rand/isaac.html

const ALPHA: u32 = 8;
const SIZE: usize = 1 << ALPHA;
const MASK: u32 = (SIZE - 1) as u32;

pub struct Ibaa {
    pub array: [Wrapping<u32>; SIZE],
    pub a: Wrapping<u32>,
    pub b: Wrapping<u32>,
    pub rand_rsl: [Wrapping<u32>; SIZE], // effectively the output state (I do not know why it is called this)
    pub ctr: usize,                      // point to the current position in rand_rsl
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

    // Same initialization as ISAAC
    fn init(&mut self, extra_pass: bool) {
        self.a = Wrapping(0);
        self.b = Wrapping(0);

        // Golden Ratio
        let mut arr = [Wrapping(0x9e3779b9_u32); 8];

        for _ in 0..4 {
            crate::isaac_mix!(arr)
        }

        for i in (0..SIZE).step_by(8) {
            if extra_pass {
                for j in 0..8 {
                    arr[j] += self.rand_rsl[i + j];
                }
            }
            crate::isaac_mix!(arr);
            for j in 0..8 {
                self.array[i + j] = arr[j]
            }
        }

        if extra_pass {
            for i in (0..SIZE).step_by(8) {
                for j in 0..8 {
                    arr[j] += self.array[i + j];
                }
                crate::isaac_mix!(arr);
                for j in 0..8 {
                    self.array[i + j] = arr[j]
                }
            }
        }

        self.ibaa();
    }

    pub fn seed(&mut self, seed: &[u8], extra_pass: bool) {
        assert!(seed.len() <= SIZE, "seed cannot have more than 256 bytes");
        self.array = [Wrapping(0); SIZE];
        self.rand_rsl = [Wrapping(0); SIZE];
        for i in 0..seed.len() {
            self.rand_rsl[i] = Wrapping(u32::from(seed[i]));
        }
        self.init(extra_pass);
    }

    pub fn init_with_seed(seed: &[u8], extra_pass: bool) -> Self {
        let mut rng = Self::default();
        rng.seed(seed, extra_pass);
        rng
    }
}

impl SimpleRng for Ibaa {
    fn next_u32(&mut self) -> u32 {
        if self.ctr >= SIZE {
            self.ibaa();
        }
        let n = self.rand_rsl[self.ctr].0;
        self.ctr += 1;
        n
    }
}
