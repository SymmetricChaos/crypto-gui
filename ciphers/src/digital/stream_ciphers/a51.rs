use super::lfsr_copy::Lfsr32;
use crate::Cipher;

pub struct A51 {
    pub lfsrs: [Lfsr32; 3],
}

impl Default for A51 {
    fn default() -> Self {
        Self {
            lfsrs: [
                Lfsr32::from_taps(0x072000), // 18, 17, 16, 13
                Lfsr32::from_taps(0x300000), // 21, 20
                Lfsr32::from_taps(0x700080), // 22, 21, 20, 7
            ],
        }
    }
}

impl A51 {
    pub fn ksa(&mut self, key: [u8; 8], frame_number: u32) {
        // Frame number limited to 22 bits
        assert!(frame_number < 0x00400000);

        // Zero out the registers
        for rng in self.lfsrs.iter_mut() {
            rng.register = 0
        }

        // Mix in the key bits one byte at a time, LSB first
        for i in 0..64 {
            self.step_all();
            let b = ((key[i / 8] >> (i & 7)) & 1) as u32;
            self.lfsrs[0].register ^= b;
            self.lfsrs[1].register ^= b;
            self.lfsrs[2].register ^= b;
        }

        // Mix in the frame bits LSB first
        for i in 0..22 {
            self.step_all();
            let b = (frame_number >> i) & 1;
            self.lfsrs[0].register ^= b;
            self.lfsrs[1].register ^= b;
            self.lfsrs[2].register ^= b;
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
    pub fn next_bit(&mut self) -> u32 {
        let (a, b, c) = (
            self.lfsrs[0].get_bit(8),
            self.lfsrs[1].get_bit(10),
            self.lfsrs[2].get_bit(10),
        );

        // Calculate majority bit
        let majority = (a & b) | (a & c) | (b & c);

        let mut out = 0;
        for (clock, idx, msb) in [(a, 0, 18), (b, 1, 21), (c, 2, 22)] {
            if clock == majority {
                self.lfsrs[idx].next_bit();
            }
            out ^= self.lfsrs[idx].get_bit(msb);
        }

        out
    }

    // Produce the up and down keystreams. Each is 114 bits, stored in 15 bytes (with the lower 6 bits of the last byte always zero)
    pub fn burst_bytes(&mut self) -> ([u8; 15], [u8; 15]) {
        let mut bytes_ab = [0u8; 15];
        let mut bytes_ba = [0u8; 15];

        for i in 0..114 {
            let b = self.next_bit();
            bytes_ab[i / 8] |= (b << (7 - (i & 7))) as u8;
        }

        for i in 0..114 {
            let b = self.next_bit();
            bytes_ba[i / 8] |= (b << (7 - (i & 7))) as u8;
        }

        (bytes_ab, bytes_ba)
    }
}

// impl Cipher for A51 {
//     fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
//         todo!()
//     }

//     fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
//         todo!()
//     }
// }

#[cfg(test)]
mod a51_tests {

    use super::*;

    #[test]
    fn test_ksa() {
        let mut cipher = A51::default();
        cipher.ksa([0x12, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF], 0x134);
        let correct_bytes_ab: [u8; 15] = [
            0x53, 0x4E, 0xAA, 0x58, 0x2F, 0xE8, 0x15, 0x1A, 0xB6, 0xE1, 0x85, 0x5A, 0x72, 0x8C,
            0x00,
        ];
        let correct_bytes_ba: [u8; 15] = [
            0x24, 0xFD, 0x35, 0xA3, 0x5D, 0x5F, 0xB6, 0x52, 0x6D, 0x32, 0xF9, 0x06, 0xDF, 0x1A,
            0xC0,
        ];

        let (bytes_ab, bytes_ba) = cipher.burst_bytes();

        // println!("\nA -> B");
        // for (a, b) in correct_bytes_ab.into_iter().zip(bytes_ab.into_iter()) {
        //     println!("{:08b} {:02x} {:08b} {:02x}", a, a, b, b)
        // }
        // println!("\nB -> A");
        // for (a, b) in correct_bytes_ba.into_iter().zip(bytes_ba.into_iter()) {
        //     println!("{:08b} {:02x} {:08b} {:02x}", a, a, b, b)
        // }

        assert_eq!(correct_bytes_ab, bytes_ab);
        assert_eq!(correct_bytes_ba, bytes_ba);
    }

    #[test]
    fn test_masks() {
        let rng = A51::default();
        assert_eq!(rng.lfsrs[0].mask, 0x07FFFF);
        assert_eq!(rng.lfsrs[1].mask, 0x3FFFFF);
        assert_eq!(rng.lfsrs[2].mask, 0x7FFFFF);
    }

    // #[test]
    // fn test_majority() {
    //     for a in [0_u32, 1] {
    //         for b in [0_u32, 1] {
    //             for c in [0_u32, 1] {
    //                 println!("{} {} {}: {}", a, b, c, (a & b) | (a & c) | (b & c))
    //             }
    //         }
    //     }
    // }
}
