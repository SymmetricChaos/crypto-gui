use num::Zero;
use utils::bits::{bits_from_str, bits_to_u32_be, bits_to_u32_le, Bit};

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
        self.bits.pop();
        self.bits.insert(0, next_bit);
        next_bit
    }

    pub fn peek_next_bit(&self) -> Bit {
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
        let mut output_bits = Vec::with_capacity(32);
        for _ in 0..32 {
            output_bits.push(self.next_bit())
        }

        match self.big_endian {
            true => bits_to_u32_be(&output_bits),
            false => bits_to_u32_le(&output_bits),
        }
    }
}
