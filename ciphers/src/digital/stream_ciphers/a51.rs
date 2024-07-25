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
    fn step(&mut self) {
        let clock_bits = [
            self.lfsrs[0].bits[9],
            self.lfsrs[1].bits[11],
            self.lfsrs[2].bits[11],
        ];
        // Hilariously long winded way to find majority bit
        // If the sum is 2 or 3 then 1 is the majority
        // If the sum is 0 or 1 then 0 is the majority
        let n = clock_bits.into_iter().fold(0u8, |acc, b| acc + b);

        if n < 2 {
            // Majority 0
            for (i, b) in clock_bits.into_iter().enumerate() {
                if b == Bit::Zero {
                    self.lfsrs[i].next_bit();
                }
            }
        } else {
            // Majority 1
            for (i, b) in clock_bits.into_iter().enumerate() {
                if b == Bit::One {
                    self.lfsrs[i].next_bit();
                }
            }
        }
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
