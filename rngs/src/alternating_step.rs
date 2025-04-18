use crate::{lfsr::Lfsr, ClassicRng};
use utils::bits::{bits_to_u32_upper, bits_to_u64_upper, Bit};

pub struct AlternatingStep {
    pub lfsrs: [Lfsr; 3],
    pub lb0: Bit,
    pub lb1: Bit,
    pub ltr: bool,
}

impl Default for AlternatingStep {
    fn default() -> Self {
        Self {
            lfsrs: [Default::default(), Default::default(), Default::default()],
            lb0: Bit::Zero,
            lb1: Bit::Zero,
            ltr: true,
        }
    }
}

impl AlternatingStep {
    pub fn next_bit(&mut self) -> Bit {
        match self.lfsrs[0].next_bit() {
            Bit::Zero => self.lb0 = self.lfsrs[1].next_bit(),
            Bit::One => self.lb1 = self.lfsrs[2].next_bit(),
        }
        self.lb0 ^ self.lb1
    }
}

impl ClassicRng for AlternatingStep {
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
