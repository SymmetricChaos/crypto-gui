use itertools::Itertools;
use utils::{
    bit_polynomial::BitPolynomial,
    bits::{bits_from_bitstring, Bit},
};

use crate::{errors::CodeError, traits::Code};

pub struct PolynomialCode {
    block_size: usize,
    generator: Vec<u8>,
}

impl Default for PolynomialCode {
    fn default() -> Self {
        Self {
            block_size: 5,
            generator: vec![1, 1, 1],
        }
    }
}

impl PolynomialCode {
    pub fn is_divisible(&self, poly: &BitPolynomial) -> bool {
        let (_, r) = poly.div_rem(BitPolynomial::from_int_vec(&self.generator).unwrap());
        r.coef.into_iter().sum::<Bit>() == Bit::Zero
    }

    pub fn int_to_binary(&self, n: usize) {}

    pub fn data_size(&self) -> usize {
        self.block_size - self.generator.len()
    }
}

impl Code for PolynomialCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_bitstring(text)
            .map_err(|e| CodeError::input(e))?
            .collect();
        if bits.len() % self.data_size() != 0 {
            return Err(CodeError::Input(format!(
                "when encoding the input must have a length that is a multiple of {}",
                self.data_size()
            )));
        };

        let mut out = String::new();

        let gen_poly = BitPolynomial::from_int_vec(&self.generator)
            .map_err(|e| CodeError::State(e.to_string()))?;

        for chunk in &bits.into_iter().chunks(self.data_size()) {
            let poly = BitPolynomial::from(chunk.collect_vec());
            let code = poly * &gen_poly;
            for bit in code.coef {
                out.push(bit.into())
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_bitstring(text)
            .map_err(|e| CodeError::input(e))?
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
            if self.is_divisible(&poly) {
                for bit in poly.coef.into_iter().take(self.data_size()) {
                    out.push(bit.into())
                }
            } else {
                out.push_str(&"�".repeat(self.data_size()))
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
        assert_eq!(code.encode("236").unwrap(), "2363");
    }

    #[test]
    fn test_decode() {
        let code = PolynomialCode::default();
        assert_eq!(code.decode("2363").unwrap(), "236");
    }

    #[test]
    fn test_decode_with_err() {
        let code = PolynomialCode::default();
        assert_eq!(
            code.decode("2365").unwrap_err(),
            CodeError::input("invalid check digit")
        );
    }
}
