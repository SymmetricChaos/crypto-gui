use itertools::Itertools;
use num::Zero;
use utils::bits::{bits_from_str, bits_to_u32_upper, bits_to_u64_upper, bools_from_str, Bit};

use crate::traits::SimpleRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LfsrMode {
    Fibonncci,
    Galois,
}

pub struct Lfsr {
    pub bits: Vec<Bit>,
    pub taps: Vec<bool>,
    pub ltr: bool, // disabled
    pub mode: LfsrMode,
}

impl Default for Lfsr {
    fn default() -> Self {
        Self::from_strings("0110111100000001", "0000000000101101")
    }
}

impl Lfsr {
    pub fn set_bits_from_str(&mut self, bits: &str) {
        self.bits = bits_from_str(bits).unwrap().collect();
    }

    pub fn set_taps_from_str(&mut self, taps: &str) {
        self.taps = bools_from_str(taps).unwrap().collect();
    }

    /// Construct from two strings, specifying the active bits and the active taps
    pub fn from_strings(bits: &str, taps: &str) -> Self {
        let bit_vec = bits_from_str(bits).unwrap().collect_vec();
        let tap_vec = bools_from_str(taps).unwrap().collect_vec();
        assert_eq!(bit_vec.len(), tap_vec.len());
        assert!(bits.len() != 0);
        Self {
            bits: bit_vec,
            taps: tap_vec,
            ltr: true,
            mode: LfsrMode::Fibonncci,
        }
    }

    /// Construct from a vector of tap positions. The bits will be set to 00...01
    /// Taps are given by the usual (1-indexed) ordering. So a vector of [9, 11] taps the 9th and 11th bits counted from the left.
    pub fn from_tap_positions(taps: &[usize]) -> Self {
        assert!(!taps.contains(&0));
        let len = *taps.iter().max().unwrap();
        let mut tap_vec = vec![false; len];
        for t in taps {
            tap_vec[t - 1] = true
        }
        let mut bit_vec = vec![Bit::Zero; len];
        bit_vec[len - 1] = Bit::One;
        Self {
            bits: bit_vec,
            taps: tap_vec,
            ltr: true,
            mode: LfsrMode::Fibonncci,
        }
    }

    pub fn next_bit(&mut self) -> Bit {
        let next_bit = match self.mode {
            LfsrMode::Fibonncci => {
                let mut next_bit = Bit::zero();
                for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
                    if *tap {
                        next_bit ^= *bit;
                    }
                }
                next_bit
            }
            LfsrMode::Galois => {
                let next_bit = *self.bits.last().unwrap();
                for (bit, tap) in self.bits.iter_mut().zip(self.taps.iter()) {
                    if *tap {
                        *bit ^= next_bit;
                    }
                }
                next_bit
            }
        };
        self.bits.pop();
        self.bits.insert(0, next_bit);
        next_bit
    }

    pub fn peek_next_bit(&self) -> Bit {
        match self.mode {
            LfsrMode::Fibonncci => {
                let mut next_bit = Bit::zero();
                for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
                    if *tap {
                        next_bit ^= *bit;
                    }
                }
                next_bit
            }
            LfsrMode::Galois => *self.bits.last().unwrap(),
        }
    }
}

impl SimpleRng for Lfsr {
    fn next_u32(&mut self) -> u32 {
        let mut output_bits = Vec::with_capacity(32);
        for _ in 0..32 {
            output_bits.push(self.next_bit())
        }
        if !self.ltr {
            output_bits.reverse();
        }
        bits_to_u32_upper(&output_bits)
    }

    fn next_u64(&mut self) -> u64 {
        let mut output_bits = Vec::with_capacity(64);
        for _ in 0..64 {
            output_bits.push(self.next_bit())
        }
        if !self.ltr {
            output_bits.reverse();
        }
        bits_to_u64_upper(&output_bits)
    }
}

#[cfg(test)]
mod lfsr_tests {

    use utils::bits::bit_string;

    use super::*;

    #[test]
    fn small_test() {
        let mut rng = Lfsr::from_strings("00001", "00101");
        let test_vals = [
            "00001", "10000", "01000", "00100", "10010", "01001", "10100", "11010", "01101",
            "00110", "10011", "11001", "11100", "11110", "11111", "01111", "00111", "00011",
        ];
        for (i, test) in test_vals.into_iter().enumerate() {
            assert_eq!(test, bit_string(&rng.bits), "{}", i);
            rng.next_bit();
        }
    }

    #[test]
    fn small_test_alt_positions() {
        let mut rng = Lfsr::from_tap_positions(&[3, 5]);
        for (i, test) in [
            "00001", "10000", "01000", "00100", "10010", "01001", "10100", "11010", "01101",
            "00110", "10011", "11001", "11100", "11110", "11111", "01111", "00111", "00011",
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(test, bit_string(&rng.bits), "{}", i);
            rng.next_bit();
        }
    }

    #[test]
    fn cycle_length_tests() {
        let mut rng = Lfsr::from_strings("00001", "01111");
        let mut states = Vec::new();
        while !states.contains(&bit_string(&rng.bits)) {
            states.push(bit_string(&rng.bits));
            rng.next_bit();
        }
        // Five bits should give (2^5)-1 = 31 states
        assert_eq!(31, states.len());

        let mut rng = Lfsr::from_tap_positions(&[9, 11]);
        rng.set_bits_from_str("00000000001");
        let mut states = Vec::new();
        while !states.contains(&bit_string(&rng.bits)) {
            states.push(bit_string(&rng.bits));
            rng.next_bit();
        }
        // Eleven bits should give (2^11)-1 = 2047 states
        assert_eq!(2047, states.len())
    }

    #[test]
    fn galois_test() {
        let mut rng = Lfsr::from_strings(
            "00000000000000000000000000000001",
            "01000110000000000000000000000000",
        );
        rng.mode = LfsrMode::Galois;

        rng.set_bits_from_str("00000000000000000000000010100011");
        for (i, test) in [
            "10100011000000000000000001010001",
            "11110010100000000000000000101000",
            "01111001010000000000000000010100",
            "00111100101000000000000000001010",
            "00011110010100000000000000000101",
            "10101100001010000000000000000010",
        ]
        .into_iter()
        .enumerate()
        {
            rng.next_bit();
            // println!("{}", bit_string(&rng.bits));
            assert_eq!(test, bit_string(&rng.bits), "{}", i);
        }
    }
}
