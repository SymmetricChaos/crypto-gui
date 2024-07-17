use crate::lfsr::Lfsr;

pub struct ShrinkingGenerator {
    a: Lfsr,
    s: Lfsr,
    big_endian: bool,
}

impl Default for ShrinkingGenerator {
    fn default() -> Self {
        Self {
            a: Lfsr::default(),
            s: Lfsr::default(),
            big_endian: true,
        }
    }
}

impl ShrinkingGenerator {
    pub fn next_bit(&mut self) -> Bit {
        loop {
            let a_bit = self.a.next_bit();
            let s_bit = self.s.next_bit();
            if s_bit == Bit::One {
                return a_bit;
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
            true => bits_to_u32_be(&output_bits),
            false => bits_to_u32_le(&output_bits),
        }
    }
}
