use utils::bits::Bit;

use super::lfsr_copy::Lfsr;
use crate::Cipher;

pub struct A51 {
    pub lfsrs: [Lfsr; 3],
}

impl Default for A51 {
    fn default() -> Self {
        Self {
            // These are one off from wikipedia example due to indexing difference
            lfsrs: [
                Lfsr::from_tap_positions(&[14, 17, 18, 19]),
                Lfsr::from_tap_positions(&[21, 22]),
                Lfsr::from_tap_positions(&[8, 21, 22, 23]),
            ],
        }
    }
}

impl A51 {
    fn step(&mut self) -> Bit {
        let clock_bits = [
            self.lfsrs[0].bits[9],
            self.lfsrs[1].bits[11],
            self.lfsrs[2].bits[11],
        ];

        // Calculate majority bit
        let n = (clock_bits[0] & clock_bits[1])
            | (clock_bits[0] & clock_bits[2])
            | (clock_bits[1] & clock_bits[2]);

        let mut out = Bit::Zero;
        for (i, b) in clock_bits.into_iter().enumerate() {
            if b == n {
                out ^= self.lfsrs[i].next_bit();
            }
        }

        out
    }
}

impl Cipher for A51 {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }
}
