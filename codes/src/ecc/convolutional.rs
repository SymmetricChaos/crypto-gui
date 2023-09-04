use crate::{errors::CodeError, traits::Code};

use utils::{
    bit_polynomial::BitPolynomial,
    bits::{bits_from_str, Bit},
};

pub struct ConvolutionalCode {
    pub constraint_length: usize,
    pub generators: Vec<BitPolynomial>,
}

impl Default for ConvolutionalCode {
    fn default() -> Self {
        Self {
            constraint_length: 3,
            generators: vec![
                BitPolynomial::from_str("111").unwrap(),
                BitPolynomial::from_str("101").unwrap(),
            ],
        }
    }
}

impl ConvolutionalCode {
    fn validate(&self) -> Result<(), CodeError> {
        Ok(())
    }
}

impl Code for ConvolutionalCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;

        let mut bits: Vec<Bit> = bits_from_str(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        for _ in 0..self.constraint_length - 1 {
            bits.insert(0, Bit::Zero)
        }

        let mut out = String::new();

        for window in bits.windows(self.constraint_length) {
            let c = BitPolynomial::from(window);
            println!("{window:?}");
            for poly in self.generators.iter() {
                let p: Bit = c.bitwise_and(poly).coef.iter().copied().sum();
                out.push(p.to_char())
            }
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let bits: Vec<Bit> = bits_from_str(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();
        todo!()
    }
}

#[cfg(test)]
mod crc_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = ConvolutionalCode::default();
        assert_eq!(code.encode("101100").unwrap(), "111000010111");
    }

    #[test]
    fn test_decode() {
        let mut code = ConvolutionalCode::default();
        assert_eq!(code.decode("111000010111").unwrap(), "101100");
    }

    // #[test]
    // fn test_decode_with_err() {
    //     let mut code = ConvolutionalCode::default();
    //     assert_eq!(code.decode("01110111001011001").unwrap(), "");
    // }
}
