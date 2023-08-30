use crate::{errors::CodeError, traits::Code};

use utils::{
    bit_polynomial::BitPolynomial,
    bits::{bits_from_string, Bit},
};

pub struct CyclicRedundancyCheck {
    pub block_size: usize,
    pub generator: BitPolynomial,
}

impl Default for CyclicRedundancyCheck {
    fn default() -> Self {
        Self {
            block_size: 4,
            generator: BitPolynomial::from_str("1101").unwrap(),
        }
    }
}

impl CyclicRedundancyCheck {
    pub fn check_bits(&self) -> usize {
        self.generator.degree()
    }

    fn validate(&self) -> Result<(), CodeError> {
        if self.block_size < 2 {
            return Err(CodeError::state("block size must be greater than 1"));
        }
        if self.generator.len() < 2 {
            return Err(CodeError::state(
                "generator polynomial must be greater than degree 1",
            ));
        }
        Ok(())
    }
}

impl Code for CyclicRedundancyCheck {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let bits: Vec<Bit> = bits_from_string(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        if bits.len() % self.block_size != 0 {
            return Err(CodeError::Input(format!(
                "when encoding the input must have a length that is a multiple of {}, the block size",
                self.block_size
            )));
        };

        let mut out = String::new();

        for chunk in bits.chunks_exact(self.block_size) {
            let poly = BitPolynomial::from(chunk.to_vec());
            let (_, r) = poly.div_rem(&self.generator);
            out.push_str(&poly.to_string());
            out.push_str(&r.to_string());
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let bits: Vec<Bit> = bits_from_string(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        if bits.len() % (self.block_size + self.check_bits()) != 0 {
            return Err(CodeError::Input(format!(
                    "when decoding the input must have a length that is a multiple of {}, the block size plus the number of check bits",
                    self.block_size + self.check_bits()
                )));
        };

        // for chunk in bits.chunks_exact(self.block_size + self.check_bits()) {
        //     let v = chunk.to_vec();
        //     let poly = BitPolynomial::from(v[0..self.block_size]);
        //     let (_, r) = poly.div_rem(&self.generator);

        //     out.push_str(&poly.to_string());
        // }

        todo!()
    }
}

#[cfg(test)]
mod crc_tests {
    use super::*;

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
