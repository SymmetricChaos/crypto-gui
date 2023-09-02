use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use num::One;
use utils::bits::{bits_from_string, Bit};

pub struct ParityBit {
    pub block_size: usize,
    pub position: usize,
    pub parity: Bit,
}

impl ParityBit {}

impl Default for ParityBit {
    fn default() -> Self {
        Self {
            block_size: 4,
            position: 4,
            parity: Bit::Zero,
        }
    }
}

impl Code for ParityBit {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_string(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        if bits.len() % self.block_size != 0 {
            return Err(CodeError::Input(format!(
                "the input must have a length that is a multiple of {}",
                self.block_size
            )));
        };

        let mut parity = self.parity;

        let mut buffer = Vec::with_capacity(self.block_size);
        let mut out = String::new();
        for chunk in &bits.into_iter().chunks(self.block_size) {
            for bit in chunk {
                buffer.push(bit);
                parity ^= bit;
            }

            buffer
                .iter()
                .take(self.position)
                .for_each(|b| out.push(char::from(b)));
            out.push(char::from(parity));
            buffer
                .iter()
                .skip(self.position)
                .for_each(|b| out.push(char::from(b)));

            buffer.clear();
            parity = self.parity;
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_string(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        if bits.len() % (self.block_size + 1) != 0 {
            return Err(CodeError::Input(format!(
                "the input must have a length that is a multiple of {}",
                self.block_size + 1
            )));
        };

        let mut parity = self.parity;

        let mut out = String::new();
        let mut buffer = String::new();
        for chunk in &bits.into_iter().chunks(self.block_size + 1) {
            for bit in chunk {
                buffer.push(char::from(bit));
                parity ^= bit;
            }

            if self.parity.is_one() {
                if self.parity != parity {
                    out.push_str(&buffer[..self.position]);
                    out.push_str(&buffer[self.position + 1..]);
                } else {
                    out.push_str(&"?".repeat(self.block_size))
                }
            } else {
                if self.parity == parity {
                    out.push_str(&buffer[..self.position]);
                    out.push_str(&buffer[self.position + 1..]);
                } else {
                    out.push_str(&"?".repeat(self.block_size))
                }
            }

            buffer.clear();
            parity = self.parity;
        }
        Ok(out)
    }
}

#[cfg(test)]
mod parity_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = ParityBit::default();
        assert_eq!(code.encode("111010010000").unwrap(), "111011001000000");
    }

    #[test]
    fn test_encode_inv() {
        let mut code = ParityBit::default();
        code.parity = Bit::One;
        assert_eq!(code.encode("111010010000").unwrap(), "111001001100001");
    }

    #[test]
    fn test_decode() {
        let code = ParityBit::default();
        assert_eq!(code.decode("111011001000000").unwrap(), "111010010000");
    }

    #[test]
    fn test_decode_inv() {
        let mut code = ParityBit::default();
        code.parity = Bit::One;
        assert_eq!(code.decode("111001001100001").unwrap(), "111010010000");
    }

    #[test]
    fn test_decode_with_err() {
        let code = ParityBit::default();
        assert_eq!(code.decode("111001001000000").unwrap(), "????10010000");
    }

    #[test]
    fn test_decode_inv_with_err() {
        let mut code = ParityBit::default();
        code.parity = Bit::One;
        assert_eq!(code.decode("011001001100001").unwrap(), "????10010000");
    }
}
