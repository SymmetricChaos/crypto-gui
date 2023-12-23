use num::Zero;
use utils::bits::{bits_from_str, bits_to_int_big_endian, bits_to_int_little_endian, Bit};

use crate::traits::ClassicRng;

pub struct Lfsr {
    pub bits: Vec<Bit>,
    pub taps: Vec<bool>,
    pub big_endian: bool,
}

impl Default for Lfsr {
    fn default() -> Self {
        Self {
            bits: bits_from_str("0110111100000001").unwrap().collect(),
            taps: vec![
                false, false, false, false, false, false, false, false, false, false, true, false,
                true, true, false, true,
            ],
            big_endian: true,
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
    fn next_u32(&mut self) -> u32 {
        let mut next_bit = Bit::zero();
        for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
            if *tap {
                next_bit ^= *bit;
            }
        }
        self.bits.pop();
        self.bits.insert(0, next_bit);
        match self.big_endian {
            true => bits_to_int_big_endian(&self.bits),
            false => bits_to_int_little_endian(&self.bits),
        }
    }
}
