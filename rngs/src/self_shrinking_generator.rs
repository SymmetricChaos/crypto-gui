use crate::{lfsr::Lfsr, SimpleRng};
use utils::bits::{
    bits_to_u32_upper, bits_to_u64_upper,
    Bit::{self, One, Zero},
};

pub struct SelfShrinkingGenerator {
    pub a: Lfsr,
    pub ltr: bool,
    pub outputs: [Option<Bit>; 4],
}

impl Default for SelfShrinkingGenerator {
    fn default() -> Self {
        Self {
            a: Lfsr::default(),
            ltr: true,
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

impl SimpleRng for SelfShrinkingGenerator {
    fn next_u32(&mut self) -> u32 {
        let mut output_bits = Vec::with_capacity(32);
        for _ in 0..32 {
            output_bits.push(self.next_bit())
        }
        if !self.ltr {
            output_bits.reverse();
        }
        bits_to_u32_upper(&output_bits)
    }

    fn next_u64(&mut self) -> u64 {
        let mut output_bits = Vec::with_capacity(64);
        for _ in 0..64 {
            output_bits.push(self.next_bit())
        }
        if !self.ltr {
            output_bits.reverse();
        }
        bits_to_u64_upper(&output_bits)
    }
}
