use num::Zero;
use utils::bits::Bit;

use crate::traits::ClassicRng;

pub struct Lfsr {
    bits: Vec<Bit>,
    taps: Vec<bool>,
}

impl ClassicRng for Lfsr {
    // fn next(&mut self) -> u32 {
    //     let mut next_bit = Bit::zero();
    //     for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
    //         if *tap {
    //             next_bit ^= *bit;
    //         }
    //     }
    //     self.bits.pop();
    //     self.bits.insert(0, next_bit);

    //     bits_to_int_big_endian(&self.bits)
    // }

    fn step(&mut self) {
        let mut next_bit = Bit::zero();
        for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
            if *tap {
                next_bit ^= *bit;
            }
        }
        self.bits.pop();
        self.bits.insert(0, next_bit);
    }
}
