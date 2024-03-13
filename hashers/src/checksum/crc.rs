use crate::{errors::HasherError, traits::ClassicHasher};

use utils::{
    bit_polynomial::BitPolynomial,
    bits::bit_vec_from_bytes,
    byte_formatting::{ByteFormat, ByteFormatError},
};

pub enum CrcAlgorithm {
    Crc32,
}

impl CrcAlgorithm {
    pub fn bits(&self) -> usize {
        match self {
            CrcAlgorithm::Crc32 => 32,
        }
    }
}

pub struct CyclicRedundancyCheckHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub generator: BitPolynomial,
    pub mode: CrcAlgorithm,
}

impl Default for CyclicRedundancyCheckHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            generator: BitPolynomial::from_str("1101").unwrap(),
            mode: CrcAlgorithm::Crc32,
        }
    }
}

impl CyclicRedundancyCheckHash {
    pub fn check_bits(&self) -> usize {
        self.generator.degree()
    }

    pub fn set_generator_from_hex(&mut self, text: &str) -> Result<(), ByteFormatError> {
        let bytes = ByteFormat::Hex.text_to_bytes(text)?;
        self.generator = BitPolynomial::from_bytes(&bytes);
        Ok(())
    }
}

impl ClassicHasher for CyclicRedundancyCheckHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let bits = bit_vec_from_bytes(bytes);

        let poly = BitPolynomial::from(bits);
        let (_, r) = poly.div_rem(&self.generator);

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

    #[test]
    fn test() {
        let mut hasher = CyclicRedundancyCheckHash::default();
    }
}
