use utils::bits::{bits_to_u8, u32_to_bits, u8_to_bits, Bit};

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
    pub fn ksa(&mut self, key: [u8; 8], frame_number: u32) {
        // Frame number must be limited to 22 bits
        assert!(frame_number < 0x003fffff);

        // TODO: Need to check the exact order the bits are used

        // Mix in the key bits one byte at a time, LSB first
        for key_byte in key.into_iter() {
            // println!("{:08b}", key_byte);
            let key_bits = u8_to_bits(key_byte);
            // println!("{:?}", key_bits);
            for key_bit in key_bits.into_iter().rev() {
                // println!("{:?}", key_bit);
                self.step_all();
                self.lfsrs[0].bits[0] = key_bit;
                self.lfsrs[1].bits[0] = key_bit;
                self.lfsrs[2].bits[0] = key_bit;
            }
        }

        // Mix in the frame bits LSB first, this is essentially a nonce
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

    // https://cryptome.org/jya/a51-pi.htm
    pub fn next_bit(&mut self) -> Bit {
        let (a, b, c) = (
            self.lfsrs[0].bits[8],
            self.lfsrs[1].bits[10],
            self.lfsrs[2].bits[10],
        );

        // Calculate majority bit
        let n = (a & b) | (a & c) | (b & c);

        let mut out = Bit::Zero;
        for (i, b) in [a, b, c].into_iter().enumerate() {
            if b == n {
                self.lfsrs[i].next_bit();
                // XOR together the most significant bits of each LFSR
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
        let mut bits = self.burst();

        for (i, seq) in bits.chunks_mut(8).enumerate() {
            // seq.reverse();
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

    use super::*;

    #[test]
    fn test_ksa() {
        let mut cipher = A51::default();
        cipher.ksa([0x12, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF], 0x134);
        let correct_bytes: [u8; 15] = [
            0x53, 0x4E, 0xAA, 0x58, 0x2F, 0xE8, 0x15, 0x1A, 0xB6, 0xE1, 0x85, 0x5A, 0x72, 0x8C,
            0x00,
        ];
        let bytes = cipher.burst_bytes();
        println!("{:02x?}", correct_bytes);
        println!("{:02x?}", bytes);
        // assert_eq!(correct_bytes, bytes);
    }

    #[test]
    fn test_majority() {
        for a in [Bit::Zero, Bit::One] {
            for b in [Bit::Zero, Bit::One] {
                for c in [Bit::Zero, Bit::One] {
                    println!("{} {} {}: {}", a, b, c, (a & b) | (a & c) | (b & c))
                }
            }
        }
    }
}
