use crate::{lfsr::Lfsr, SimpleRng};
use utils::bits::Bit;

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

impl SimpleRng for Geffe {
    fn next_u32(&mut self) -> u32 {
        let mut out = 0;
        for _ in 0..32 {
            out <<= 1;
            out |= self.next_bit() as u32;
        }
        out
    }

    fn next_u64(&mut self) -> u64 {
        let mut out = 0;
        for _ in 0..64 {
            out <<= 1;
            out |= self.next_bit() as u64;
        }
        out
    }
}
