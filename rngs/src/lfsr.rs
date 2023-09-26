use num::Zero;
use utils::bits::{bits_from_str, Bit};

use crate::traits::ClassicRng;

pub struct Lfsr {
    pub bits: Vec<Bit>,
    pub taps: Vec<bool>,
}

impl Default for Lfsr {
    fn default() -> Self {
        Self {
            bits: bits_from_str("0110111100000001").unwrap().collect(),
            taps: vec![
                false, false, false, false, false, false, false, false, false, false, true, false,
                true, true, false, true,
            ],
        }
    }
}

impl Lfsr {
    pub fn next_bit(&mut self) -> Bit {
        let mut next_bit = Bit::zero();
        for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
            if *tap {
                next_bit ^= *bit;
            }
        }
        next_bit
    }
}

impl ClassicRng for Lfsr {
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
