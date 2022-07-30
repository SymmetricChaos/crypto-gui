// Yes this can be done entirely with bitshifts and bitmasks but this
// implementation is easier to read for curious people and easier to make a UI
// for
pub struct FibLsfr<const N: usize> {
    pub bits: [u8; N],   // represent the bits of the state
    pub taps: [bool; N], // represent which bits are tapped
}

impl Default for FibLsfr<8_usize> {
    fn default() -> FibLsfr<8_usize> {
        FibLsfr {
            bits: [1, 0, 1, 0, 1, 1, 0, 0],
            // Taps are at 8, 6, 5, 3
            taps: [false, false, true, false, true, true, false, true],
        }
    }
}

impl Default for FibLsfr<16_usize> {
    fn default() -> FibLsfr<16_usize> {
        FibLsfr {
            bits: [1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1],
            // Taps are at 16, 14, 13, 11
            taps: [
                false, false, false, false, false, false, false, false, false, false, true, false,
                true, true, false, true,
            ],
        }
    }
}

impl Default for FibLsfr<32_usize> {
    fn default() -> FibLsfr<32_usize> {
        FibLsfr {
            bits: [
                1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0,
                0, 0, 0, 1,
            ],
            // Taps are at 31, 30, 27, 25, 23, 22, 21
            taps: [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, true, true, true, false,
                true, false, true, false, false, true, true, false,
            ],
        }
    }
}

impl<const N: usize> FibLsfr<N> {
    pub fn step(&mut self) -> u8 {
        let mut bit = 0;
        for (n, t) in self.taps.iter().enumerate() {
            if *t {
                bit ^= self.bits[n]
            }
        }
        for pos in (0..(N - 1)).rev() {
            self.bits[pos + 1] = self.bits[pos]
        }
        self.bits[0] = bit;
        bit
    }

    // fn make_u8(&mut self) -> u8 {
    //     let mut out = 0;
    //     for _ in 0..8 {
    //         out <<= 1;
    //         out ^= self.step();
    //     }
    //     out
    // }

    // fn make_u16(&mut self) -> u16 {
    //     let mut out = 0;
    //     for _ in 0..16 {
    //         out <<= 1;
    //         out ^= self.step() as u16;
    //     }
    //     out
    // }

    // fn make_u32(&mut self) -> u32 {
    //     let mut out = 0;
    //     for _ in 0..32 {
    //         out <<= 1;
    //         out ^= self.step() as u32;
    //     }
    //     out
    // }

    // fn make_i8(&mut self) -> i8 {
    //     let mut out = 0;
    //     for _ in 0..8 {
    //         out <<= 1;
    //         out ^= self.step() as i8;
    //     }
    //     out
    // }

    // fn make_i16(&mut self) -> i16 {
    //     let mut out = 0;
    //     for _ in 0..16 {
    //         out <<= 1;
    //         out ^= self.step() as i16;
    //     }
    //     out
    // }

    // fn make_i32(&mut self) -> i32 {
    //     let mut out = 0;
    //     for _ in 0..32 {
    //         out <<= 1;
    //         out ^= self.step() as i32;
    //     }
    //     out
    // }
}
