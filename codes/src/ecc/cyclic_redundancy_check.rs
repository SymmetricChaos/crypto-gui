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
            let mut poly = BitPolynomial::from(chunk.to_vec());
            poly.increase_degree(self.check_bits());
            let (_, r) = poly.div_rem(&self.generator);
            poly.decrease_degree(self.check_bits());
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

        let mut out = String::new();

        for chunk in bits.chunks_exact(self.block_size + self.check_bits()) {
            let check = BitPolynomial::from(&chunk[self.block_size..]);
            let mut poly = BitPolynomial::from(&chunk[0..self.block_size]);
            poly.increase_degree(self.check_bits());
            let (_, r) = poly.div_rem(&self.generator);
            poly.decrease_degree(self.check_bits());
            if r == check {
                out.push_str(&poly.to_string());
            } else {
                out.push_str(&"?".repeat(self.block_size))
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod crc_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut code = CyclicRedundancyCheck::default();
        code.block_size = 14;
        assert_eq!(code.encode("00110111001011").unwrap(), "00110111001011001");
    }

    #[test]
    fn test_decode() {
        let mut code = CyclicRedundancyCheck::default();
        code.block_size = 14;
        assert_eq!(code.decode("00110111001011001").unwrap(), "00110111001011");
    }

    #[test]
    fn test_decode_with_err() {
        let mut code = CyclicRedundancyCheck::default();
        code.block_size = 14;
        assert_eq!(code.decode("01110111001011001").unwrap(), "??????????????");
    }
}
