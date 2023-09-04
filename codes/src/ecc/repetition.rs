use crate::{errors::CodeError, traits::Code};
use utils::bits::{bits_from_str, Bit};

pub struct Repetition {
    pub block_size: usize,
}

impl Repetition {}

impl Default for Repetition {
    fn default() -> Self {
        Self { block_size: 3 }
    }
}

impl Code for Repetition {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let zeroes = "0".repeat(self.block_size);
        let ones = "1".repeat(self.block_size);
        let mut out = String::new();
        for bit in bits_from_str(text).map_err(|e| CodeError::input(&e.to_string()))? {
            match bit {
                Bit::Zero => out.push_str(&zeroes),
                Bit::One => out.push_str(&ones),
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        let mut zeroes = 0;
        let mut ones = 0;
        let mut ctr = 0;
        for bit in bits_from_str(text).map_err(|e| CodeError::input(&e.to_string()))? {
            match bit {
                Bit::Zero => zeroes += 1,
                Bit::One => ones += 1,
            }
            ctr += 1;
            if ctr == self.block_size {
                match zeroes.cmp(&ones) {
                    std::cmp::Ordering::Less => out.push('1'),
                    std::cmp::Ordering::Equal => out.push('?'),
                    std::cmp::Ordering::Greater => out.push('0'),
                }
                zeroes = 0;
                ones = 0;
                ctr = 0;
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod repetition_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = Repetition::default();
        assert_eq!(code.encode("1011").unwrap(), "111000111111");
    }

    #[test]
    fn test_decode() {
        let code = Repetition::default();
        assert_eq!(code.decode("111000111111").unwrap(), "1011");
    }

    #[test]
    fn test_decode_with_err() {
        let mut code = Repetition::default();
        code.block_size = 4;
        assert_eq!(code.decode("110100110000").unwrap(), "1?0");
    }
}
