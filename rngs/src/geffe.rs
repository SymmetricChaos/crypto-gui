use crate::{lfsr::Lfsr, ClassicRng};
use utils::bits::{bits_to_u32_ltr, bits_to_u32_rtl, Bit};

pub struct Geffe {
    pub rngs: [Lfsr; 3],
    pub ltr: bool,
}

impl Default for Geffe {
    fn default() -> Self {
        Self {
            rngs: [Default::default(), Default::default(), Default::default()],
            ltr: true,
        }
    }
}

impl Geffe {
    pub fn next_bit(&mut self) -> Bit {
        let (a, b, c) = (
            self.rngs[0].next_bit(),
            self.rngs[1].next_bit(),
            self.rngs[2].next_bit(),
        );
        (a & b) ^ (!a & c)
    }

    pub fn peek_next_bit(&self) -> Bit {
        let (a, b, c) = (
            self.rngs[0].peek_next_bit(),
            self.rngs[1].peek_next_bit(),
            self.rngs[2].peek_next_bit(),
        );
        (a & b) ^ (!a & c)
    }
}

impl ClassicRng for Geffe {
    fn next_u32(&mut self) -> u32 {
        let mut output_bits = Vec::with_capacity(32);
        for _ in 0..32 {
            output_bits.push(self.next_bit())
        }

        match self.ltr {
            true => bits_to_u32_ltr(&output_bits),
            false => bits_to_u32_rtl(&output_bits),
        }
    }
}
