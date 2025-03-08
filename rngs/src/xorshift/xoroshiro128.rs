use super::XoshiroScrambler;
use crate::traits::ClassicRng;

pub struct Xoroshiro128 {
    pub state: [u64; 2],
    pub scrambler: XoshiroScrambler,
}

impl Default for Xoroshiro128 {
    fn default() -> Self {
        Self {
            state: [0x0BAD_5EED0BAD_5EED, 0x0BAD_5EED0BAD_5EED],
            scrambler: XoshiroScrambler::PlusPlus,
        }
    }
}

impl Xoroshiro128 {
    const JUMP: [u64; 2] = [0xdf900294d8f554a5, 0x170865df4b3201fc];
    const LONG_JUMP: [u64; 2] = [0xd2a98b26625eee7b, 0xdddf9b1090aa7ac1];

    fn step(&mut self) {
        if self.scrambler == XoshiroScrambler::PlusPlus {
            self.state[1] ^= self.state[0];
            self.state[0] = self.state[0].rotate_left(49) ^ self.state[1] ^ (self.state[1] << 21);
            self.state[1] = self.state[1].rotate_left(28);
        } else {
            self.state[1] ^= self.state[0];
            self.state[0] = self.state[0].rotate_left(24) ^ self.state[1] ^ (self.state[1] << 16);
            self.state[1] = self.state[1].rotate_left(37);
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let out = match self.scrambler {
            XoshiroScrambler::PlusPlus => (self.state[0].wrapping_add(self.state[1]))
                .rotate_left(17)
                .wrapping_add(self.state[0]),
            XoshiroScrambler::StarStar => (self.state[0].wrapping_mul(5))
                .rotate_left(7)
                .wrapping_mul(9),
            XoshiroScrambler::Plus => self.state[0].wrapping_add(self.state[1]),
        };

        self.step();
        out
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

impl ClassicRng for Xoroshiro128 {
    fn next_u32(&mut self) -> u32 {
        let out = (self.next_u64() >> 32) as u32;
        self.step();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoroshiro128plus.rs.html
    #[test]
    fn reference_plus() {
        let mut rng = Xoroshiro128 {
            state: [1, 2],
            scrambler: XoshiroScrambler::Plus,
        };
        let expected = [
            3,
            412333834243,
            2360170716294286339,
            9295852285959843169,
            2797080929874688578,
            6019711933173041966,
            3076529664176959358,
            3521761819100106140,
            7493067640054542992,
            920801338098114767,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro128plusplus.rs.html
    #[test]
    fn reference_plusplus() {
        let mut rng = Xoroshiro128 {
            state: [1, 2],
            scrambler: XoshiroScrambler::PlusPlus,
        };
        let expected = [
            393217,
            669327710093319,
            1732421326133921491,
            11394790081659126983,
            9555452776773192676,
            3586421180005889563,
            1691397964866707553,
            10735626796753111697,
            15216282715349408991,
            14247243556711267923,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro128starstar.rs.html
    #[test]
    fn reference_starstar() {
        let mut rng = Xoroshiro128 {
            state: [1, 2],
            scrambler: XoshiroScrambler::StarStar,
        };
        let expected = [
            5760,
            97769243520,
            9706862127477703552,
            9223447511460779954,
            8358291023205304566,
            15695619998649302768,
            8517900938696309774,
            16586480348202605369,
            6959129367028440372,
            16822147227405758281,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
