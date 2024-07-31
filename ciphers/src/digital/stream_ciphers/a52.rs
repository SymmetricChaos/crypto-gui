use super::lfsr_copy::Lfsr32;
use crate::Cipher;

fn majority(a: u32, b: u32, c: u32) -> u32 {
    (a & b) | (a & c) | (b & c)
}

pub struct A52 {
    pub lfsrs: [Lfsr32; 4],
}

impl Default for A52 {
    fn default() -> Self {
        Self {
            lfsrs: [
                Lfsr32::from_taps(0x072000), // 18, 17, 16, 13
                Lfsr32::from_taps(0x300000), // 21, 20
                Lfsr32::from_taps(0x700080), // 22, 21, 20, 7
                Lfsr32::from_taps(0x010800), // 16, 11
            ],
        }
    }
}

impl A52 {
    const LOADED: [u32; 4] = [15, 16, 18, 10];
    const MSB: [u32; 4] = [18, 21, 22, 16];
    const CHECK: [(u32, u32, u32); 3] = [(15, 14, 12), (16, 13, 9), (18, 16, 13)];

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
            self.lfsrs[3].register ^= b;
        }

        // Mix in the frame bits LSB first
        for i in 0..22 {
            self.step_all();
            let b = (frame_number >> i) & 1;
            self.lfsrs[0].register ^= b;
            self.lfsrs[1].register ^= b;
            self.lfsrs[2].register ^= b;
            self.lfsrs[3].register ^= b;
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

    // https://archive.ph/20130120032216/http://www.cryptodox.com/A5/2
    pub fn next_bit(&mut self) -> u32 {
        // The fourth (and shortest) LFSR controls the stepping of all the others
        let (a, b, c) = (
            self.lfsrs[3].get_bit(10),
            self.lfsrs[3].get_bit(3),
            self.lfsrs[3].get_bit(7),
        );

        // Calculate majority bit from the fourth register
        let m = majority(a, b, c);

        // Clock everything
        for (clock, idx) in [(a, 0), (b, 1), (c, 2)] {
            if clock == m {
                self.lfsrs[idx].next_bit();
                self.lfsrs[idx].register |= 1 << Self::LOADED[idx]; // Forcibly set the loaded bit
            }
        }
        self.lfsrs[3].next_bit();
        self.lfsrs[3].register |= 1 << 10;

        // Determine the output bit
        let mut out = 0;

        // XOR in the MSB
        out ^= self.lfsrs[0].get_bit(Self::MSB[0]);
        // XOR in the majority of three chosen bits, with one inverted
        let (x, y, z) = Self::CHECK[0];
        out ^= majority(
            self.lfsrs[0].get_bit(x),
            self.lfsrs[0].get_bit(y) ^ 1,
            self.lfsrs[0].get_bit(z),
        );

        out ^= self.lfsrs[1].get_bit(Self::MSB[1]);
        let (x, y, z) = Self::CHECK[1];
        out ^= majority(
            self.lfsrs[1].get_bit(x) ^ 1,
            self.lfsrs[1].get_bit(y),
            self.lfsrs[1].get_bit(z),
        );

        out ^= self.lfsrs[2].get_bit(Self::MSB[2]);
        let (x, y, z) = Self::CHECK[2];
        out ^= majority(
            self.lfsrs[2].get_bit(x),
            self.lfsrs[2].get_bit(y),
            self.lfsrs[2].get_bit(z) ^ 1,
        );

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

// impl Cipher for A52 {
//     fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
//         todo!()
//     }

//     fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
//         todo!()
//     }
// }

#[cfg(test)]
mod a52_tests {

    use super::*;

    #[test]
    fn test_ksa() {
        let mut cipher = A52::default();
        cipher.ksa([0x00, 0xfc, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], 0x21);
        let correct_bytes_ab: [u8; 15] = [
            0xf4, 0x51, 0x2c, 0xac, 0x13, 0x59, 0x37, 0x64, 0x46, 0x0b, 0x72, 0x2d, 0xad, 0xd5,
            0x00,
        ];
        let correct_bytes_ba: [u8; 15] = [
            0x48, 0x00, 0xd4, 0x32, 0x8e, 0x16, 0xa1, 0x4d, 0xcd, 0x7b, 0x97, 0x22, 0x26, 0x51,
            0x00,
        ];

        let (bytes_ab, bytes_ba) = cipher.burst_bytes();

        println!("\nA -> B");
        for (a, b) in correct_bytes_ab.into_iter().zip(bytes_ab.into_iter()) {
            println!("{:08b} {:02x} {:08b} {:02x}", a, a, b, b)
        }
        println!("\nB -> A");
        for (a, b) in correct_bytes_ba.into_iter().zip(bytes_ba.into_iter()) {
            println!("{:08b} {:02x} {:08b} {:02x}", a, a, b, b)
        }

        // assert_eq!(correct_bytes_ab, bytes_ab);
        // assert_eq!(correct_bytes_ba, bytes_ba);
    }
}
