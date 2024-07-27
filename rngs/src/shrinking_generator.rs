use utils::bits::{bits_to_u32, Bit};

use crate::{lfsr::Lfsr, ClassicRng};

pub struct ShrinkingGenerator {
    pub a: Lfsr,
    pub s: Lfsr,
    pub ltr: bool,
}

impl Default for ShrinkingGenerator {
    fn default() -> Self {
        Self {
            a: Lfsr::from_tap_positions(&[32, 30, 26, 25]),
            s: Lfsr::from_tap_positions(&[29, 28, 27, 25]),
            ltr: true,
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
        if !self.ltr {
            output_bits.reverse();
        }
        bits_to_u32(&output_bits)
    }
}
