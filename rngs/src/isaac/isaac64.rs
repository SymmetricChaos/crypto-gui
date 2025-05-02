use crate::ClassicRng;
use std::num::Wrapping;

const SIZE: usize = 256;

// not the same as the other ISAAC function
macro_rules! mix(
    ($a:expr) => (
    {
        $a[0] -= $a[4]; $a[5] ^= $a[7] >> 9; $a[7] += $a[0];
        $a[1] -= $a[5]; $a[6] ^= $a[0] << 9; $a[0] += $a[1];
        $a[2] -= $a[6]; $a[7] ^= $a[1] >> 23; $a[1] += $a[2];
        $a[3] -= $a[7]; $a[0] ^= $a[2] << 15; $a[2] += $a[3];
        $a[4] -= $a[0]; $a[1] ^= $a[3] >> 14; $a[3] += $a[4];
        $a[5] -= $a[1]; $a[2] ^= $a[4] << 20; $a[4] += $a[5];
        $a[6] -= $a[2]; $a[3] ^= $a[5] >> 17; $a[5] += $a[6];
        $a[7] -= $a[3]; $a[4] ^= $a[6] << 14; $a[6] += $a[7];
    } );
);

pub struct Isaac64 {
    array: [Wrapping<u64>; SIZE],
    a: Wrapping<u64>,
    b: Wrapping<u64>,
    c: Wrapping<u64>,
    rand_rsl: [Wrapping<u64>; SIZE], // effectively the output state (I do not know why it is called this)
    ctr: usize,                      // point to the current position in rand_rsl
}

impl Default for Isaac64 {
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

impl Isaac64 {
    fn isaac(&mut self) {
        self.c += Wrapping(1);
        self.b += self.c;
        for i in 0..SIZE {
            let x = self.array[i];
            match i % 4 {
                0 => self.a ^= self.a << 21,
                1 => self.a ^= self.a >> 5,
                2 => self.a ^= self.a << 12,
                3 => self.a ^= self.a >> 33,
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
        let mut arr = [Wrapping(0x9e3779b97f4a7c13_u64); 8];

        for _ in 0..4 {
            mix!(arr)
        }

        for i in (0..SIZE).step_by(8) {
            if extra_pass {
                for j in 0..8 {
                    arr[j] += self.rand_rsl[i + j];
                }
            }
            mix!(arr);
            for j in 0..8 {
                self.array[i + j] = arr[j]
            }
        }

        if extra_pass {
            for i in (0..SIZE).step_by(8) {
                for j in 0..8 {
                    arr[j] += self.array[i + j];
                }
                mix!(arr);
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
            self.rand_rsl[i] = Wrapping(u64::from(seed[i]));
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

    pub fn next_u64(&mut self) -> u64 {
        if self.ctr >= SIZE {
            self.isaac();
        }
        let n = self.rand_rsl[self.ctr].0;
        self.ctr += 1;
        n
    }
}

impl ClassicRng for Isaac64 {
    fn next_u32(&mut self) -> u32 {
        if self.ctr >= SIZE {
            self.isaac();
        }
        let n = self.rand_rsl[self.ctr].0;
        self.ctr += 1;
        n as u32
    }

    fn next_u64(&mut self) -> u64 {
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
        let mut rng = Isaac64::init_with_seed(key, true);
        let enc = msg.iter().map(|&b| (rng.next_ascii() ^ b)).collect_vec();
        println!("{:02x?}", enc);
    }
}
