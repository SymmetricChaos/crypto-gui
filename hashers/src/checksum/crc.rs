use crate::{errors::HasherError, traits::ClassicHasher};

use num::Zero;
use utils::{
    bit_polynomial::BitPolynomial,
    bits::{bit_string, Bit},
    byte_formatting::ByteFormat,
};

pub enum CrcAlgorithm {
    Crc32,
    Crc32C,
    Crc32K,
}

impl CrcAlgorithm {
    pub fn bits(&self) -> usize {
        match self {
            CrcAlgorithm::Crc32 => 32,
            CrcAlgorithm::Crc32C => 32,
            CrcAlgorithm::Crc32K => 16,
        }
    }

    pub fn generator(&self) -> BitPolynomial {
        BitPolynomial::from_str(match self {
            CrcAlgorithm::Crc32 => "111011011011100010000011001000001",
            CrcAlgorithm::Crc32C => "100000101111011000111011011110001",
            CrcAlgorithm::Crc32K => "111010110011000111011000001011101",
        })
        .unwrap()
    }
}

// https://www.ghsi.de/pages/subpages/Online%20CRC%20Calculation/indexDetails.php?Polynom=111011011011100010000011001000001&Message=E100CAFE
pub struct CyclicRedundancyCheckHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: CrcAlgorithm,
}

impl Default for CyclicRedundancyCheckHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: CrcAlgorithm::Crc32,
        }
    }
}

impl CyclicRedundancyCheckHash {}

impl ClassicHasher for CyclicRedundancyCheckHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        // Convert the bytes to a vector of Bits and treat it as a polynomial
        let data = BitPolynomial::from_bytes_ltr(bytes).coef;
        let mut state = vec![Bit::zero(); 32];

        println!("data: {:?}", bit_string(&data));

        println!("init: {:?}", bit_string(&state));
        for data_bit in data {
            let inv = data_bit ^ state[31];
            state[31] = state[30] ^ inv;
            state[30] = state[29] ^ inv;
            state[29] = state[28];
            state[28] = state[27] ^ inv;
            state[27] = state[26] ^ inv;
            state[26] = state[25];
            state[25] = state[24] ^ inv;
            state[24] = state[23] ^ inv;
            state[23] = state[22];
            state[22] = state[21] ^ inv;
            state[21] = state[20] ^ inv;
            state[20] = state[19] ^ inv;
            state[19] = state[18];
            state[18] = state[17];
            state[17] = state[16];
            state[16] = state[15] ^ inv;
            state[15] = state[14];
            state[14] = state[13];
            state[13] = state[12];
            state[12] = state[11];
            state[11] = state[10];
            state[10] = state[09] ^ inv;
            state[09] = state[08] ^ inv;
            state[08] = state[07];
            state[07] = state[06];
            state[06] = state[05] ^ inv;
            state[05] = state[04];
            state[04] = state[03];
            state[03] = state[02];
            state[02] = state[01];
            state[01] = state[00];
            state[00] ^= inv;
            println!("{data_bit} {:?}", bit_string(&state));
        }

        // let s: String = r.bit_string().chars().rev().collect();
        // println!("{s}");

        // Convert the CRC syndrome into bytes for output
        // ByteFormat::Bit.text_to_bytes(&r.bit_string()).unwrap()
        todo!()
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod crc_hasher_tests {

    use super::*;

    // fn crc_bits_from_hex(hex: &str) {
    //     let mut from_hex =
    //         BitPolynomial::from_bytes_rtl(&ByteFormat::HexLe.text_to_bytes(hex).unwrap());
    //     from_hex.coef.push(utils::bits::Bit::One);
    //     println!("{}", from_hex.polynomial_string());
    //     println!("{}", from_hex);
    // }

    // #[test]
    // fn polynomial_bits() {
    //     for hex in ["04C11DB7", "1EDC6F41", "741B8CD7"] {
    //         crc_bits_from_hex(hex)
    //     }
    // }

    #[test]
    fn test() {
        let hasher = CyclicRedundancyCheckHash::default();

        hasher.hash_bytes_from_string("E100CAFE").unwrap();
        // assert_eq!(
        //     "ef1a85f0",
        //     hasher.hash_bytes_from_string("E100CAFE").unwrap()
        // );
    }
}
