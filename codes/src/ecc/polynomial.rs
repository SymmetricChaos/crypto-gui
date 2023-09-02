use itertools::Itertools;
use num::Zero;
use utils::{
    bit_polynomial::BitPolynomial,
    bits::{bits_from_string, int_to_bits, Bit},
};

use crate::{errors::CodeError, traits::Code};

pub struct PolynomialCode {
    block_size: usize,
    generator: BitPolynomial,
}

impl Default for PolynomialCode {
    fn default() -> Self {
        Self {
            block_size: 5,
            generator: BitPolynomial::from_int_array([1, 1, 1]).unwrap(),
        }
    }
}

impl PolynomialCode {
    pub fn int_to_code(&self, n: u32) -> BitPolynomial {
        BitPolynomial::from(int_to_bits(n)) * &self.generator
    }

    pub fn data_size(&self) -> usize {
        self.block_size - self.generator.len() + 1
    }
}

impl Code for PolynomialCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_string(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();
        if bits.len() % self.data_size() != 0 {
            return Err(CodeError::Input(format!(
                "when encoding the input must have a length that is a multiple of {}",
                self.data_size()
            )));
        };

        let mut out = String::new();

        for chunk in &bits.into_iter().chunks(self.data_size()) {
            let poly = BitPolynomial::from(chunk.collect_vec());
            let code = poly * &self.generator;
            for bit in code.coef {
                out.push(bit.into())
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_string(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();
        if bits.len() % self.block_size != 0 {
            return Err(CodeError::Input(format!(
                "when decoding the input must have a length that is a multiple of {}",
                self.block_size
            )));
        };

        let mut out = String::new();

        for chunk in &bits.into_iter().chunks(self.block_size) {
            let poly = BitPolynomial::from(chunk.collect_vec());
            let (q, r) = poly.div_rem(&self.generator);
            if r.is_zero() {
                for bit in q.coef.iter() {
                    out.push(bit.into())
                }
                for _ in 0..(self.data_size() - q.coef.len()) {
                    out.push('0')
                }
            } else {
                out.push_str(&"?".repeat(self.data_size()))
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod polynomial_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = PolynomialCode::default();
        assert_eq!(code.encode("001").unwrap(), "00111");
    }

    #[test]
    fn test_decode() {
        let code = PolynomialCode::default();
        assert_eq!(code.decode("").unwrap(), "");
    }

    #[test]
    fn test_decode_with_err() {
        let code = PolynomialCode::default();
        assert_eq!(
            code.decode("").unwrap_err(),
            CodeError::input("invalid check digit")
        );
    }
}
