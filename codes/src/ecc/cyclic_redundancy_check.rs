use crate::{errors::CodeError, traits::Code};

use utils::{
    bit_polynomial::BitPolynomial,
    bits::{bits_from_string, Bit},
};

pub struct CyclicRedundancyCheck {
    pub block_size: usize,
    pub generator: BitPolynomial,
}

impl CyclicRedundancyCheck {}

impl Default for CyclicRedundancyCheck {
    fn default() -> Self {
        Self {
            block_size: 4,
            generator: BitPolynomial::from_int_array([1, 1, 0, 1]).unwrap(),
        }
    }
}

impl Code for CyclicRedundancyCheck {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_string(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_string(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        todo!()
    }
}

#[cfg(test)]
mod crc_tests {
    use super::*;

    #[test]
    fn division() {
        let a = BitPolynomial::from_str("11010011101100000").unwrap();
        let b = BitPolynomial::from_str("1101").unwrap();
        let (q, r) = a.div_rem(&b);
        println!("{q} {r}")
    }

    #[test]
    fn test_encode() {
        let code = CyclicRedundancyCheck::default();
        assert_eq!(code.encode("").unwrap(), "");
    }

    #[test]
    fn test_decode() {
        let code = CyclicRedundancyCheck::default();
        assert_eq!(code.decode("").unwrap(), "");
    }

    #[test]
    fn test_decode_with_err() {
        let code = CyclicRedundancyCheck::default();
        assert_eq!(code.decode("").unwrap(), "");
    }
}
