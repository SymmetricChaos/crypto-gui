use utils::bits::{bits_to_u32_ltr, bits_to_u32_rtl, Bit};

use crate::{lfsr::Lfsr, ClassicRng};

pub struct ShrinkingGenerator {
    pub a: Lfsr,
    pub s: Lfsr,
    pub big_endian: bool,
}

impl Default for ShrinkingGenerator {
    fn default() -> Self {
        Self {
            a: Lfsr::from_tap_positions(vec![31, 13]),
            s: Lfsr::from_tap_positions(vec![24, 4, 3, 1]),
            big_endian: true,
        }
    }
}

impl ShrinkingGenerator {
    pub fn step(&mut self) -> Option<Bit> {
        let a_bit = self.a.next_bit();
        let s_bit = self.s.next_bit();
        if s_bit == Bit::One {
            Some(a_bit)
        } else {
            None
        }
    }

    pub fn next_bit(&mut self) -> Bit {
        loop {
            if let Some(bit) = self.step() {
                return bit;
            }
        }
    }
}

impl ClassicRng for ShrinkingGenerator {
    fn next_u32(&mut self) -> u32 {
        let mut output_bits = Vec::with_capacity(32);
        for _ in 0..32 {
            output_bits.push(self.next_bit())
        }

        match self.big_endian {
            true => bits_to_u32_ltr(&output_bits),
            false => bits_to_u32_rtl(&output_bits),
        }
    }
}
