use itertools::Itertools;
use utils::bits::{bits_from_str, bits_to_u32_ltr, bits_to_u32_rtl, bools_from_str, Bit};

use crate::traits::ClassicRng;

pub struct Glfsr {
    pub bits: Vec<Bit>,
    pub taps: Vec<bool>,
    pub big_endian: bool,
}

impl Default for Glfsr {
    fn default() -> Self {
        Self::from_strings("0110111100000001", "0000000000101101")
    }
}

impl Glfsr {
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

    /// Construct from a vector of tap positions. The bits will be set to have Bit::One at the rightmost position and Bit::Zero elsewhere
    pub fn from_tap_positions(taps: Vec<usize>, len: usize) -> Self {
        let mut tap_vec = vec![false; len];
        for t in taps {
            tap_vec[t] = true
        }
        let mut bit_vec = vec![Bit::Zero; len];
        bit_vec[len - 1] = Bit::One;
        Self {
            bits: bit_vec,
            taps: tap_vec,
            big_endian: true,
        }
    }

    pub fn next_bit(&mut self) -> Bit {
        let next_bit = *self.bits.last().unwrap();
        for (bit, tap) in self.bits.iter_mut().zip(self.taps.iter()) {
            if *tap {
                *bit ^= next_bit;
            }
        }
        self.bits.pop();
        self.bits.insert(0, next_bit);
        next_bit
    }
}

impl ClassicRng for Glfsr {
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
mod glfsr_tests {

    use utils::bits::bit_string;

    use super::*;

    #[test]
    fn small_test() {
        let mut rng = Glfsr::from_strings(
            "00000000000000000000000000000001",
            "01000110000000000000000000000000",
        );

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
