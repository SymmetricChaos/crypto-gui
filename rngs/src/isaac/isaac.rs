use crate::SimpleRng;
use std::num::Wrapping;

const SIZE: usize = 256;

pub struct Isaac {
    pub array: [Wrapping<u32>; SIZE],
    pub a: Wrapping<u32>,
    pub b: Wrapping<u32>,
    pub c: Wrapping<u32>,
    pub rand_rsl: [Wrapping<u32>; SIZE], // effectively the output state (I do not know why it is called this)
    pub ctr: usize,                      // point to the current position in rand_rsl
}

impl Default for Isaac {
    fn default() -> Self {
        Self {
            array: [Wrapping(0); SIZE],
            a: Wrapping(0),
            b: Wrapping(0),
            c: Wrapping(0),
            rand_rsl: [Wrapping(0); SIZE],
            ctr: 0,
        }
    }
}

impl Isaac {
    fn isaac(&mut self) {
        self.c += Wrapping(1);
        self.b += self.c;
        for i in 0..SIZE {
            let x = self.array[i];
            match i % 4 {
                0 => self.a ^= self.a << 13,
                1 => self.a ^= self.a >> 6,
                2 => self.a ^= self.a << 2,
                3 => self.a ^= self.a >> 16,
                _ => unreachable!(),
            }
            self.a += self.array[(i + 128) % SIZE];
            self.array[i] = self.array[(x.0 as usize >> 2) % SIZE] + self.a + self.b;
            let y = self.array[i].0 as usize;
            self.b = self.array[(y >> 10) % SIZE] + x;
            self.rand_rsl[i] = self.b;
        }
        self.ctr = 0;
    }

    fn init(&mut self, extra_pass: bool) {
        self.a = Wrapping(0);
        self.b = Wrapping(0);
        self.c = Wrapping(0);

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

        self.isaac();
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

    pub fn next_ascii(&mut self) -> u8 {
        (self.next_u32() % 95 + 32) as u8
    }
}

impl SimpleRng for Isaac {
    fn next_u32(&mut self) -> u32 {
        if self.ctr >= SIZE {
            self.isaac();
        }
        let n = self.rand_rsl[self.ctr].0;
        self.ctr += 1;
        n
    }
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use super::*;

    #[test]
    fn rosetta_test() {
        let msg = b"a Top Secret secret";
        let key = b"this is my secret key";
        let mut rng = Isaac::init_with_seed(key, true);
        let enc = msg.iter().map(|&b| rng.next_ascii() ^ b).collect_vec();
        println!("{:02x?}", enc);
    }
}
