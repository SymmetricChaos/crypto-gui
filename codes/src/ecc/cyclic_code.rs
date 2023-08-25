use crate::{errors::CodeError, traits::Code};
use std::collections::VecDeque;
use utils::bits::{bits_from_string, Bit};

pub struct CyclicCode {
    pub generator_word: Vec<Bit>,
}

impl Default for CyclicCode {
    fn default() -> Self {
        Self {
            generator_word: vec![Bit::One, Bit::One, Bit::Zero, Bit::One],
        }
    }
}

impl CyclicCode {
    pub fn codes(&self) -> Vec<Vec<Bit>> {
        let mut out = Vec::with_capacity(self.generator_word.len());
        let mut word = VecDeque::from(self.generator_word.clone());
        for _ in 0..self.generator_word.len() {
            let v = Vec::from(word.clone());
            if !out.contains(&v) {
                out.push(v);
            }
            word.rotate_right(1);
        }
        out
    }
}

impl Code for CyclicCode {
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
mod parity_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = CyclicCode::default();
        assert_eq!(code.encode("").unwrap(), "");
    }

    #[test]
    fn test_decode() {
        let code = CyclicCode::default();
        assert_eq!(code.decode("").unwrap(), "");
    }

    #[test]
    fn test_decode_with_err() {
        let code = CyclicCode::default();
        assert_eq!(code.decode("").unwrap(), "");
    }
}
