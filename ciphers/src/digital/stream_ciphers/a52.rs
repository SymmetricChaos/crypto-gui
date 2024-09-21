use super::lfsr_copy::Lfsr32;
use crate::{Cipher, CipherError};
use utils::byte_formatting::{xor_into_bytes, ByteFormat};

fn majority(a: u32, b: u32, c: u32) -> u32 {
    (a & b) | (a & c) | (b & c)
}

#[derive(Debug, Clone)]
pub struct A52Rng {
    pub lfsrs: [Lfsr32; 4],
}

impl Default for A52Rng {
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

impl A52Rng {
    const LOADED: [u32; 4] = [15, 16, 18, 10];
    const MSB: [u32; 4] = [18, 21, 22, 16];

    pub fn ksa(&mut self, key: [u8; 8], frame_number: u32) {
        // Frame number limited to 22 bits
        assert!(frame_number < 0x00400000);

        // Zero out the registers
        for rng in self.lfsrs.iter_mut() {
            rng.register = 0
        }

        // Mix in the key bits one byte at a time, LSB first
        for i in 0..64 {
            self.step_all(false);
            let b = ((key[i / 8] >> (i & 7)) & 1) as u32;
            self.lfsrs[0].register ^= b;
            self.lfsrs[1].register ^= b;
            self.lfsrs[2].register ^= b;
            self.lfsrs[3].register ^= b;
        }

        // Mix in the frame bits LSB first
        for i in 0..22 {
            // For the last bit of the frame number several bits are loaded when the LFSRs are stepped
            self.step_all(i == 21);
            let b = (frame_number >> i) & 1;
            self.lfsrs[0].register ^= b;
            self.lfsrs[1].register ^= b;
            self.lfsrs[2].register ^= b;
            self.lfsrs[3].register ^= b;
        }

        // Mix for 99 steps with normal clocking
        for _ in 0..99 {
            self.next_bit();
        }
    }

    pub fn step_all(&mut self, load: bool) {
        for (i, lfsr) in self.lfsrs.iter_mut().enumerate() {
            lfsr.next_bit();
            if load {
                lfsr.register |= 1 << Self::LOADED[i];
            }
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
            }
        }
        self.lfsrs[3].next_bit();

        // Determine the output bit
        let mut out = 0;

        // XOR in the MSB
        out ^= self.lfsrs[0].get_bit(Self::MSB[0]);
        // XOR in the majority of three chosen bits, with one inverted
        out ^= majority(
            self.lfsrs[0].get_bit(15),
            self.lfsrs[0].get_bit(14) ^ 1,
            self.lfsrs[0].get_bit(12),
        );

        out ^= self.lfsrs[1].get_bit(Self::MSB[1]);
        out ^= majority(
            self.lfsrs[1].get_bit(16) ^ 1,
            self.lfsrs[1].get_bit(13),
            self.lfsrs[1].get_bit(9),
        );

        out ^= self.lfsrs[2].get_bit(Self::MSB[2]);
        out ^= majority(
            self.lfsrs[2].get_bit(18),
            self.lfsrs[2].get_bit(16),
            self.lfsrs[2].get_bit(13) ^ 1,
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

    pub fn keystream(&mut self, n_bytes: usize, key: [u8; 8], frame_number: u32) -> Vec<u8> {
        let mut bytes = vec![0; n_bytes];
        let mut f = frame_number;
        for i in 0..(n_bytes * 8) {
            if i % 114 == 0 {
                self.ksa(key, f);
                f += 1;
                f %= 0x00400000;
            }
            let b = self.next_bit();
            bytes[i / 8] |= (b << (7 - (i & 7))) as u8;
        }
        bytes
    }
}

pub struct A52 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,

    pub rng: A52Rng,
    pub key: [u8; 8],
    pub frame_number: u32,
}

impl Default for A52 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,

            rng: Default::default(),
            key: [0u8; 8],
            frame_number: 0u32,
        }
    }
}

impl A52 {
    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut rng = self.rng.clone();
        let mut keystream = rng.keystream(bytes.len(), self.key, self.frame_number);
        xor_into_bytes(&mut keystream, &bytes);
        keystream
    }
}

crate::impl_cipher_for_stream_cipher!(A52);

#[cfg(test)]
mod a52_tests {

    use super::*;

    #[test]
    fn test_ksa() {
        let mut cipher = A52Rng::default();
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
}
