use utils::bits::{
    bits_to_u32_ltr, bits_to_u32_rtl,
    Bit::{self, One, Zero},
};

use crate::{lfsr::Lfsr, ClassicRng};

pub struct SelfShrinkingGenerator {
    pub a: Lfsr,
    pub big_endian: bool,
    pub outputs: [Option<Bit>; 4],
}

impl Default for SelfShrinkingGenerator {
    fn default() -> Self {
        Self {
            a: Lfsr::default(),
            big_endian: true,
            outputs: [None, None, Some(One), Some(Zero)],
        }
    }
}

impl SelfShrinkingGenerator {
    pub fn step(&mut self) -> Option<Bit> {
        let pair = (self.a.next_bit(), self.a.next_bit());
        match pair {
            (Zero, Zero) => self.outputs[0],
            (Zero, One) => self.outputs[1],
            (One, Zero) => self.outputs[2],
            (One, One) => self.outputs[3],
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

impl ClassicRng for SelfShrinkingGenerator {
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
