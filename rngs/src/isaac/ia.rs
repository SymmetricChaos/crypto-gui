use crate::ClassicRng;
use std::num::Wrapping;

// https://burtleburtle.net/bob/rand/isaac.html

const ALPHA: u32 = 8;
const SIZE: usize = 1 << ALPHA;
const MASK: u32 = (SIZE - 1) as u32;

pub struct Ia {
    pub array: [Wrapping<u32>; SIZE],
    pub b: Wrapping<u32>,
    pub rand_rsl: [Wrapping<u32>; SIZE], // effectively the output state (I do not know why it is called this)
    pub ctr: usize,                      // point to the current position in rand_rsl
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
            self.array[i] = y;
            t = self.array[((y.0 >> ALPHA) & MASK) as usize] + x;
            self.rand_rsl[i] = t;
        }
        self.b = t;
    }

    // Same initialization method used by ISAAC
    fn init(&mut self, extra_pass: bool) {
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

        self.ia();
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
