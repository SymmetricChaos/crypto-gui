use crate::SimpleRng;
use std::num::Wrapping;

pub struct IsaacPlus {
    array: [Wrapping<u32>; 256],
    a: Wrapping<u32>,
    b: Wrapping<u32>,
    c: Wrapping<u32>,
    rand_rsl: [Wrapping<u32>; 256], // effectively the output state (I do not know why it is called this)
    ctr: usize,                     // point to the current position in rand_rsl
}

impl Default for IsaacPlus {
    fn default() -> Self {
        Self {
            array: [Wrapping(0); 256],
            a: Wrapping(0),
            b: Wrapping(0),
            c: Wrapping(0),
            rand_rsl: [Wrapping(0); 256],
            ctr: 0,
        }
    }
}

impl IsaacPlus {
    // Modification by Aumasson, the rotations in the match are implied by the paper
    fn isaac(&mut self) {
        self.c += Wrapping(1);
        self.b += self.c;
        for i in 0..256 {
            let x = self.array[i];
            match i % 4 {
                0 => self.a ^= Wrapping(self.a.0.rotate_left(13)),
                1 => self.a ^= Wrapping(self.a.0.rotate_right(6)),
                2 => self.a ^= Wrapping(self.a.0.rotate_left(2)),
                3 => self.a ^= Wrapping(self.a.0.rotate_right(16)),
                _ => unreachable!(),
            }
            self.a += self.array[(i + 128) % 256];
            self.array[i] = (self.a ^ self.b) + self.array[(x.0.rotate_right(2) as usize) % 256];
            let y = self.array[i].0 as usize;
            self.b = x + self.a ^ self.array[y.rotate_right(10) % 256];
            self.rand_rsl[i] = self.b;
        }
        self.ctr = 0;
    }

    fn init(&mut self, extra_pass: bool) {
        self.a = Wrapping(0);
        self.b = Wrapping(0);
        self.c = Wrapping(0);

        // Golden Ratio
        let mut arr = [Wrapping(0x9e37_79b9u32); 8];

        for _ in 0..4 {
            crate::isaac_mix!(arr)
        }

        for i in (0..256).step_by(8) {
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
            for i in (0..256).step_by(8) {
                for j in 0..8 {
                    arr[j] += self.array[i + j];
                }
                crate::isaac_mix!(arr);
                for j in 0..8 {
                    self.array[i + j] = arr[j]
                }
            }
        }

        self.isaac();
    }

    pub fn seed(&mut self, seed: &[u8], extra_pass: bool) {
        assert!(seed.len() <= 256, "seed cannot have more than 256 bytes");
        self.array = [Wrapping(0); 256];
        self.rand_rsl = [Wrapping(0); 256];
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

    pub fn next_ascii(&mut self) -> u8 {
        (self.next_u32() % 95 + 32) as u8
    }
}

impl SimpleRng for IsaacPlus {
    fn next_u32(&mut self) -> u32 {
        if self.ctr > 255 {
            self.isaac();
        }
        let n = self.rand_rsl[self.ctr].0;
        self.ctr += 1;
        n
    }
}
