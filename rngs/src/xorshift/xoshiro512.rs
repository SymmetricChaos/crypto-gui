use crate::traits::ClassicRng;

use super::Scrambler;

pub struct Xoshiro512 {
    pub state: [u64; 8],
    pub scrambler: Scrambler,
}

impl Default for Xoshiro512 {
    fn default() -> Self {
        Self {
            state: [0, 0, 0, 0, 0, 0, 0, 0],
            scrambler: Scrambler::PlusPlus,
        }
    }
}

impl Xoshiro512 {
    const JUMP: [u64; 8] = [
        0x33ed89b6e7a353f9,
        0x760083d7955323be,
        0x2837f2fbb5f22fae,
        0x4b8c5674d309511c,
        0xb11ac47a7ba28c25,
        0xf1be7667092bcc1c,
        0x53851efdb6df0aaf,
        0x1ebbc8b23eaf25db,
    ];
    const LONG_JUMP: [u64; 8] = [
        0x11467fef8f921d28,
        0xa2a819f2e79c8ea8,
        0xa8299fc284b3959a,
        0xb4d347340ca63ee1,
        0x1cb0940bedbff6ce,
        0xd956c5c4fa1f8e17,
        0x915e38fd4eda93bc,
        0x5b3ccdfa5d7daca5,
    ];

    fn step(&mut self) {
        let t = self.state[1] << 11;
        self.state[2] ^= self.state[0];
        self.state[5] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[7] ^= self.state[3];
        self.state[3] ^= self.state[4];
        self.state[4] ^= self.state[5];
        self.state[0] ^= self.state[6];
        self.state[6] ^= self.state[7];
        self.state[6] ^= t;
        self.state[7] = self.state[7].rotate_left(21);
    }

    pub fn next_u64(&mut self) -> u64 {
        let out = match self.scrambler {
            Scrambler::PlusPlus => (self.state[0].wrapping_add(self.state[2]))
                .rotate_left(17)
                .wrapping_add(self.state[2]),
            Scrambler::StarStar => (self.state[1].wrapping_mul(5))
                .rotate_left(7)
                .wrapping_mul(9),
            Scrambler::Plus => self.state[0].wrapping_add(self.state[2]),
        };
        self.step();
        out
    }

    // Jumps forward by 2^256 steps
    pub fn jump(&mut self) {
        let mut s = [0; 8];
        for j in Self::JUMP {
            for b in 0..64 {
                if j & (1 << b) != 0 {
                    for n in 0..4 {
                        s[n] ^= self.state[n]
                    }
                }
                self.step()
            }
        }
        for n in 0..8 {
            self.state[n] = s[n];
        }
    }

    // Jumps forward by 2^384 steps
    pub fn long_jump(&mut self) {
        let mut s = [0; 8];
        for j in Self::LONG_JUMP {
            for b in 0..64 {
                if j & (1 << b) != 0 {
                    for n in 0..4 {
                        s[n] ^= self.state[n]
                    }
                }
                self.step()
            }
        }
        for n in 0..8 {
            self.state[n] = s[n];
        }
    }
}

impl ClassicRng for Xoshiro512 {
    fn next_u32(&mut self) -> u32 {
        let out = (self.next_u64() >> 32) as u32;
        self.step();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro512plus.rs.html
    #[test]
    fn reference_plus() {
        let mut rng = Xoshiro512 {
            state: [1, 2, 3, 4, 5, 6, 7, 8],
            scrambler: Scrambler::Plus,
        };
        let expected = [
            4,
            8,
            4113,
            25169936,
            52776585412635,
            57174648719367,
            9223482039571869716,
            9331471677901559830,
            9340533895746033672,
            14078399799840753678,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro512plusplus.rs.html
    #[test]
    fn reference_plusplus() {
        let mut rng = Xoshiro512 {
            state: [1, 2, 3, 4, 5, 6, 7, 8],
            scrambler: Scrambler::PlusPlus,
        };
        let expected = [
            524291,
            1048578,
            539099140,
            3299073855497,
            6917532603230064654,
            7494048333530275843,
            14418333309547923463,
            10960079161595355914,
            18279570946505382726,
            10209173166699159237,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro512starstar.rs.html
    #[test]
    fn reference_starstart() {
        let mut rng = Xoshiro512 {
            state: [1, 2, 3, 4, 5, 6, 7, 8],
            scrambler: Scrambler::StarStar,
        };
        let expected = [
            11520,
            0,
            23040,
            23667840,
            144955163520,
            303992986974289920,
            25332796375735680,
            296904390158016,
            13911081092387501979,
            15304787717237593024,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
