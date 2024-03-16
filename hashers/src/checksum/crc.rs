use crate::{errors::HasherError, traits::ClassicHasher};

use utils::{bit_polynomial::BitPolynomial, byte_formatting::ByteFormat};

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

pub struct CyclicRedundancyCheckHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    // pub generator: BitPolynomial,
    pub mode: CrcAlgorithm,
}

impl Default for CyclicRedundancyCheckHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            // generator: BitPolynomial::from_str("1101").unwrap(),
            mode: CrcAlgorithm::Crc32,
        }
    }
}

impl CyclicRedundancyCheckHash {
    // pub fn check_bits(&self) -> usize {
    //     self.generator.degree()
    // }

    // pub fn set_generator_from_hex(&mut self, text: &str) -> Result<(), ByteFormatError> {
    //     let bytes = ByteFormat::Hex.text_to_bytes(text)?;
    //     self.generator = BitPolynomial::from_bytes(&bytes);
    //     Ok(())
    // }
}

impl ClassicHasher for CyclicRedundancyCheckHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        // Convert the bytes to a vector of Bits and treat it as a polynomial
        let poly = BitPolynomial::from_bytes_rtl(bytes);

        // The remainder of the division is the CRC syndrome
        let (_, r) = poly.div_rem(&self.mode.generator());

        // Convert the CRC syndrome into bytes for output
        ByteFormat::Bit.text_to_bytes(&r.bit_string()).unwrap()
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
    use itertools::Itertools;

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
        let mut hasher = CyclicRedundancyCheckHash::default();
    }
}
