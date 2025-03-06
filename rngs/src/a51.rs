use strum::{Display, EnumIter};

use crate::{lfsr_l, lfsr_small::get_bit32, ClassicRng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum ReKeyRule {
    #[strum(to_string = "114-bits")]
    K114,
    #[strum(to_string = "228-bits")]
    K228,
    #[strum(to_string = "Never")]
    KNever,
}

impl ReKeyRule {
    pub fn rekey(&self, n: usize) -> bool {
        match self {
            ReKeyRule::K114 => n % 114 == 0,
            ReKeyRule::K228 => n % 228 == 0,
            ReKeyRule::KNever => false,
        }
    }
}

lfsr_l!(rng0, u32, 19; 18, 17, 14); // 0x072000
lfsr_l!(rng1, u32, 22; 21); // 0x300000
lfsr_l!(rng2, u32, 23; 22, 21, 8); // 0x700080

#[derive(Debug, Clone)]
pub struct A51Rng {
    pub lfsrs: [u32; 3],
    pub key: [u8; 8],
    pub frame_number: u32,
    pub rekey: ReKeyRule,
}

impl Default for A51Rng {
    fn default() -> Self {
        let mut out = Self {
            lfsrs: [1, 1, 1],
            key: [0, 0, 0, 0, 0, 0, 0, 1], // avoid starting with an empty array
            frame_number: 0,
            rekey: ReKeyRule::K114,
        };
        out.ksa();
        out
    }
}

impl A51Rng {
    pub fn ksa(&mut self) {
        // Frame number limited to 22 bits
        assert!(self.frame_number < 0x00400000);

        // Zero out the registers
        for rng in self.lfsrs.iter_mut() {
            *rng = 0
        }

        // Mix in the key bits one byte at a time, LSB first
        for i in 0..64 {
            self.step_all();
            let b = ((self.key[i / 8] >> (i & 7)) & 1) as u32;
            self.lfsrs[0] ^= b;
            self.lfsrs[1] ^= b;
            self.lfsrs[2] ^= b;
        }

        // Mix in the frame bits LSB first
        for i in 0..22 {
            self.step_all();
            let b = (self.frame_number >> i) & 1;
            self.lfsrs[0] ^= b;
            self.lfsrs[1] ^= b;
            self.lfsrs[2] ^= b;
        }

        // Mix for 100 steps with normal clocking
        for _ in 0..100 {
            self.next_bit();
        }
    }

    fn step_lfsr(&mut self, n: usize) {
        match n {
            0 => self.lfsrs[0] = rng0(self.lfsrs[0]),
            1 => self.lfsrs[1] = rng1(self.lfsrs[1]),
            2 => self.lfsrs[2] = rng2(self.lfsrs[2]),
            _ => unreachable!("there are only three lfsrs"),
        }
    }

    pub fn step_all(&mut self) {
        self.step_lfsr(0);
        self.step_lfsr(1);
        self.step_lfsr(2);
    }

    // https://cryptome.org/jya/a51-pi.htm
    pub fn next_bit(&mut self) -> u32 {
        let (a, b, c) = (
            get_bit32(self.lfsrs[0], 8),
            get_bit32(self.lfsrs[1], 10),
            get_bit32(self.lfsrs[2], 10),
        );

        // Calculate majority bit
        let majority = (a & b) | (a & c) | (b & c);

        let mut out = 0;
        for (clock, idx, msb) in [(a, 0, 18), (b, 1, 21), (c, 2, 22)] {
            if clock == majority {
                self.step_lfsr(idx);
            }
            out ^= get_bit32(self.lfsrs[idx], msb);
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

    pub fn keystream(&mut self, n_bytes: usize) -> Vec<u8> {
        let mut bytes = vec![0; n_bytes];
        for i in 0..(n_bytes * 8) {
            if self.rekey.rekey(i) {
                self.ksa();
                self.frame_number += 1;
                self.frame_number %= 0x00400000;
            }
            let b = self.next_bit();
            bytes[i / 8] |= (b << (7 - (i & 7))) as u8;
        }
        bytes
    }
}

impl ClassicRng for A51Rng {
    fn next_u32(&mut self) -> u32 {
        u32::from_be_bytes(self.keystream(4).try_into().unwrap())
    }
}

#[cfg(test)]
mod a51rng_tests {

    use super::*;

    #[test]
    fn test_ksa() {
        let mut cipher = A51Rng::default();
        cipher.key = [0x12, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        cipher.frame_number = 0x134;
        cipher.ksa();
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

    // #[test]
    // fn test_masks() {
    //     let rng = A51Rng::default();
    //     assert_eq!(rng.lfsrs[0].mask, 0x07FFFF);
    //     assert_eq!(rng.lfsrs[1].mask, 0x3FFFFF);
    //     assert_eq!(rng.lfsrs[2].mask, 0x7FFFFF);
    // }

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
