use itertools::Itertools;
use num::Zero;
use utils::bits::{bits_from_str, bits_to_u32_ltr, bits_to_u32_rtl, bools_from_str, Bit};

use crate::traits::ClassicRng;

pub struct Lfsr {
    pub bits: Vec<Bit>,
    pub taps: Vec<bool>,
    pub big_endian: bool,
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
        Self {
            bits: bit_vec,
            taps: tap_vec,
            big_endian: true,
        }
    }

    /// Construct from a vector of tap positions. The bits will be set to have Bit::One at index 0 and Bit::Zero elsewhere
    pub fn from_tap_positions(taps: Vec<usize>) -> Self {
        let l = taps.iter().max().unwrap() + 1;
        let mut tap_vec = vec![false; l];
        for t in taps {
            tap_vec[t] = true
        }
        let mut bit_vec = vec![Bit::Zero; l];
        bit_vec[0] = Bit::One;
        Self {
            bits: bit_vec,
            taps: tap_vec,
            big_endian: true,
        }
    }

    pub fn next_bit(&mut self) -> Bit {
        let mut next_bit = Bit::zero();
        for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
            if *tap {
                next_bit ^= *bit;
            }
        }
        self.bits.pop();
        self.bits.insert(0, next_bit);
        next_bit
    }

    pub fn peek_next_bit(&self) -> Bit {
        let mut next_bit = Bit::zero();
        for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
            if *tap {
                next_bit ^= *bit;
            }
        }
        next_bit
    }
}

impl ClassicRng for Lfsr {
    fn next_u32(&mut self) -> u32 {
        let mut output_bits = Vec::with_capacity(32);
        for _ in 0..32 {
            output_bits.push(self.next_bit())
        }

        match self.big_endian {
            true => bits_to_u32_ltr(&output_bits),
            false => bits_to_u32_rtl(&output_bits),
        }
    }
}
#[cfg(test)]
mod vic_tests {

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
        let mut rng = Lfsr::from_tap_positions(vec![2, 4]);
        rng.set_bits_from_str("00001");
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
}
