use super::XoshiroScrambler;
use crate::traits::ClassicRng;

pub struct Xoshiro128 {
    pub state: [u32; 4],
    pub scrambler: XoshiroScrambler,
}

impl Default for Xoshiro128 {
    fn default() -> Self {
        Self {
            state: [0x0BAD_5EED, 0x0BAD_5EED, 0x0BAD_5EED, 0x0BAD_5EED],
            scrambler: XoshiroScrambler::PlusPlus,
        }
    }
}

impl Xoshiro128 {
    const JUMP: [u32; 4] = [0x8764000b, 0xf542d2d3, 0x6fa035c3, 0x77f2db5b];
    const LONG_JUMP: [u32; 4] = [0xb523952e, 0x0b6f099f, 0xccf5a0ef, 0x1c580662];

    fn step(&mut self) {
        let t = self.state[1] << 9;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(11);
    }

    pub fn next_u64(&mut self) -> u64 {
        let x = u64::from(self.next_u32());
        let y = u64::from(self.next_u32());
        (y << 32) | x
    }

    // Jumps forward by 2^64 steps
    pub fn jump(&mut self) {
        let mut s = [0; 4];

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
        for n in 0..4 {
            self.state[n] = s[n];
        }
    }

    // Jumps forward by 2^96 steps
    pub fn long_jump(&mut self) {
        let mut s = [0; 4];
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
        for n in 0..4 {
            self.state[n] = s[n];
        }
    }
}

impl ClassicRng for Xoshiro128 {
    fn next_u32(&mut self) -> u32 {
        let out = match self.scrambler {
            XoshiroScrambler::PlusPlus => self.state[0]
                .wrapping_add(self.state[3])
                .rotate_left(7)
                .wrapping_add(self.state[0]),
            XoshiroScrambler::StarStar => {
                self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9)
            }
            XoshiroScrambler::Plus => self.state[0].wrapping_add(self.state[3]),
        };
        self.step();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro128plus.rs.html
    #[test]
    fn reference_plus() {
        let mut rng = Xoshiro128 {
            state: [1, 2, 3, 4],
            scrambler: XoshiroScrambler::Plus,
        };
        let expected = [
            5, 12295, 25178119, 27286542, 39879690, 1140358681, 3276312097, 4110231701, 399823256,
            2144435200,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u32(), e);
        }
    }

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro128plusplus.rs.html
    #[test]
    fn reference_plusplus() {
        let mut rng = Xoshiro128 {
            state: [1, 2, 3, 4],
            scrambler: XoshiroScrambler::PlusPlus,
        };
        let expected = [
            641, 1573767, 3222811527, 3517856514, 836907274, 4247214768, 3867114732, 1355841295,
            495546011, 621204420,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u32(), e);
        }
    }

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro128starstar.rs.html
    #[test]
    fn reference_starstar() {
        let mut rng = Xoshiro128 {
            state: [1, 2, 3, 4],
            scrambler: XoshiroScrambler::StarStar,
        };
        let expected = [
            11520, 0, 5927040, 70819200, 2031721883, 1637235492, 1287239034, 3734860849,
            3729100597, 4258142804,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u32(), e);
        }
    }
}
