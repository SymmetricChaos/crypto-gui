use utils::{bit_polynomial::BitPolynomial, bits::Bit};

use crate::{errors::CodeError, traits::Code};

pub struct PolynomialCode {
    degree_prod: usize,
    generator: Vec<u8>,
}

impl Default for PolynomialCode {
    fn default() -> Self {
        Self {
            degree_prod: 5,
            generator: vec![1, 1, 1],
        }
    }
}

impl PolynomialCode {
    pub fn is_divisible(&self, poly: BitPolynomial) -> bool {
        let (_, r) = poly.div_rem(BitPolynomial::from_int_vec(&self.generator).unwrap());
        r.coef.into_iter().sum::<Bit>() == Bit::Zero
    }

    pub fn int_to_binary(&self, n: usize) {}
}

impl Code for PolynomialCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
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
