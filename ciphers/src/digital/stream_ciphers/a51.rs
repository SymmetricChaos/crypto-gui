use super::lfsr_copy::Lfsr32;
use crate::Cipher;

pub struct A51 {
    pub lfsrs: [Lfsr32; 3],
}

impl Default for A51 {
    fn default() -> Self {
        Self {
            // These are one off from wikipedia example due to indexing difference
            lfsrs: [
                Lfsr32::from_taps(0x072000),
                Lfsr32::from_taps(0x300000),
                Lfsr32::from_taps(0x700080),
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
        for i in 0..64 {
            self.step_all();
            self.lfsrs[0].register ^= ((key[i / 8] >> (i & 7)) & 1) as u32;
            self.lfsrs[1].register ^= ((key[i / 8] >> (i & 7)) & 1) as u32;
            self.lfsrs[2].register ^= ((key[i / 8] >> (i & 7)) & 1) as u32;
        }

        // Mix in the frame bits LSB first, this is essentially a nonce
        for i in 0..22 {
            self.step_all();
            self.lfsrs[0].register ^= (frame_number >> i) & 1;
            self.lfsrs[1].register ^= (frame_number >> i) & 1;
            self.lfsrs[2].register ^= (frame_number >> i) & 1;
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
        let n = (a & b) | (a & c) | (b & c);

        let mut out = 0;
        if a == n {
            self.lfsrs[0].next_bit();
            out ^= self.lfsrs[0].get_bit(18);
        }
        if b == n {
            self.lfsrs[1].next_bit();
            out ^= self.lfsrs[1].get_bit(21);
        }
        if c == n {
            self.lfsrs[2].next_bit();
            out ^= self.lfsrs[2].get_bit(22);
        }

        out
    }

    // Produce 15 bytes of keystream but with the last six bits always 0 because only 114 bits are produced
    pub fn burst_bytes(&mut self) -> [u8; 15] {
        let mut bytes = [0u8; 15];

        for i in 0..114 {
            bytes[i / 8] |= (self.next_bit() << (7 - (i & 7))) as u8;
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
