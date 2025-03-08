use super::XoshiroScrambler;
use crate::traits::ClassicRng;

pub struct Xoshiro256 {
    pub state: [u64; 4],
    pub scrambler: XoshiroScrambler,
}

impl Default for Xoshiro256 {
    fn default() -> Self {
        Self {
            state: [
                0x0BAD_5EED0BAD_5EED,
                0x0BAD_5EED0BAD_5EED,
                0x0BAD_5EED0BAD_5EED,
                0x0BAD_5EED0BAD_5EED,
            ],
            scrambler: XoshiroScrambler::PlusPlus,
        }
    }
}

impl Xoshiro256 {
    const JUMP: [u64; 4] = [
        0x180ec6d33cfd0aba,
        0xd5a61266f0c9392c,
        0xa9582618e03fc9aa,
        0x39abdc4529b1661c,
    ];
    const LONG_JUMP: [u64; 4] = [
        0x76e15d3efefdcbbf,
        0xc5004e441c522fb3,
        0x77710069854ee241,
        0x39109bb02acbe635,
    ];

    fn step(&mut self) {
        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);
    }

    pub fn next_u64(&mut self) -> u64 {
        let out = match self.scrambler {
            XoshiroScrambler::PlusPlus => (self.state[0].wrapping_add(self.state[3]))
                .rotate_left(23)
                .wrapping_add(self.state[0]),
            XoshiroScrambler::StarStar => (self.state[1].wrapping_mul(5))
                .rotate_left(7)
                .wrapping_mul(9),
            XoshiroScrambler::Plus => self.state[0].wrapping_add(self.state[3]),
        };
        self.step();
        out
    }

    // Jumps forward by 2^128 steps
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

    // Jumps forward by 2^192 steps
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

impl ClassicRng for Xoshiro256 {
    fn next_u32(&mut self) -> u32 {
        let out = (self.next_u64() >> 32) as u32;
        self.step();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro256plus.rs.html
    #[test]
    fn reference_plus() {
        let mut rng = Xoshiro256 {
            state: [1, 2, 3, 4],
            scrambler: XoshiroScrambler::Plus,
        };
        let expected = [
            5,
            211106232532999,
            211106635186183,
            9223759065350669058,
            9250833439874351877,
            13862484359527728515,
            2346507365006083650,
            1168864526675804870,
            34095955243042024,
            3466914240207415127,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro256plusplus.rs.html
    #[test]
    fn reference_plusplus() {
        let mut rng = Xoshiro256 {
            state: [1, 2, 3, 4],
            scrambler: XoshiroScrambler::PlusPlus,
        };
        let expected = [
            41943041,
            58720359,
            3588806011781223,
            3591011842654386,
            9228616714210784205,
            9973669472204895162,
            14011001112246962877,
            12406186145184390807,
            15849039046786891736,
            10450023813501588000,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }

    // https://docs.rs/rand_xoshiro/0.7.0/src/rand_xoshiro/xoshiro256starstar.rs.html
    #[test]
    fn reference_starstart() {
        let mut rng = Xoshiro256 {
            state: [1, 2, 3, 4],
            scrambler: XoshiroScrambler::StarStar,
        };
        let expected = [
            11520,
            0,
            1509978240,
            1215971899390074240,
            1216172134540287360,
            607988272756665600,
            16172922978634559625,
            8476171486693032832,
            10595114339597558777,
            2904607092377533576,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
