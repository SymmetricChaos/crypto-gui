use utils::bits::{bits_to_u8, u32_to_bits, u64_to_bits, Bit};

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
    pub fn ksa(&mut self, key: u64, frame_number: u32) {
        // Frame number must be limited to 22 bits
        assert!(frame_number < 0x003fffff);

        // TODO: Need to check the exact order the bits are used

        // Mix in the key bits
        let key_bits = u64_to_bits(key);
        for key_bit in key_bits.into_iter().rev() {
            self.step_all();
            self.lfsrs[0].bits[0] = key_bit;
            self.lfsrs[1].bits[0] = key_bit;
            self.lfsrs[2].bits[0] = key_bit;
        }

        // Mix in the frame bits, this is essentially a nonce
        let frame_bits = u32_to_bits(frame_number);
        for frame_bit in frame_bits.into_iter().rev().take(22) {
            self.step_all();
            self.lfsrs[0].bits[0] = frame_bit;
            self.lfsrs[1].bits[0] = frame_bit;
            self.lfsrs[2].bits[0] = frame_bit;
        }

        // Mix for 100 steps with normal clocking
        for _ in 0..100 {
            self.next_bit();
        }
    }

    pub fn step_all(&mut self) {
        for lfsr in self.lfsrs.iter_mut() {
            lfsr.next_bit();
        }
    }

    pub fn next_bit(&mut self) -> Bit {
        let clock_bits = [
            self.lfsrs[0].bits[8],
            self.lfsrs[1].bits[10],
            self.lfsrs[2].bits[10],
        ];

        // Calculate majority bit
        let n = (clock_bits[0] & clock_bits[1])
            ^ (clock_bits[0] & clock_bits[2])
            ^ (clock_bits[1] & clock_bits[2]);

        let mut out = Bit::Zero;
        for (i, b) in clock_bits.into_iter().enumerate() {
            if b == n {
                self.lfsrs[i].next_bit();
                // XIR together the most significant bits
                out ^= *self.lfsrs[i].bits.last().unwrap();
            }
        }

        out
    }

    // Produce 114 bits of keystream
    pub fn burst(&mut self) -> [Bit; 114] {
        let mut arr = [Bit::Zero; 114];
        for i in 0..114 {
            arr[i] = self.next_bit();
        }
        arr
    }

    // Produce 15 bytes of keystream but with the last six bits always 0 because only 114 bits are produced
    pub fn burst_bytes(&mut self) -> [u8; 15] {
        let mut bytes = [0u8; 15];
        let bits = self.burst();

        for (i, seq) in bits.chunks(8).enumerate() {
            bytes[i] = bits_to_u8(seq)
        }

        bytes
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

#[cfg(test)]
mod a51_tests {

    use utils::bits::bit_string;

    use super::*;

    #[test]
    fn test_ksa() {
        // expected output is 0x534EAA582FE8151AB6E1855A728C00
        let mut cipher = A51::default();
        cipher.ksa(0x1223456789ABCDEF, 0x134);
        println!("{:02x?}", cipher.burst_bytes());
    }
}
