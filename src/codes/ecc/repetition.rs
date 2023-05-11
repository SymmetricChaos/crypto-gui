use super::bits_from_bitstring;
use crate::{codes::Code, errors::Error};

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
    fn encode(&self, text: &str) -> Result<String, Error> {
        let zeroes = "0".repeat(self.block_size);
        let ones = "1".repeat(self.block_size);
        let mut out = String::new();
        for bit in bits_from_bitstring(text) {
            match bit? {
                0 => out.push_str(&zeroes),
                1 => out.push_str(&ones),
                _ => unreachable!("bits_from_bitstring should filter out everything but 0s and 1s"),
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        let mut zeroes = 0;
        let mut ones = 0;
        let mut ctr = 0;
        for bit in bits_from_bitstring(text) {
            match bit? {
                0 => zeroes += 1,
                1 => ones += 1,
                _ => unreachable!("bits_from_bitstring should filter out everything but 0s and 1s"),
            }
            ctr += 1;
            if ctr == self.block_size {
                match zeroes.cmp(&ones) {
                    std::cmp::Ordering::Less => out.push('1'),
                    std::cmp::Ordering::Equal => out.push('�'),
                    std::cmp::Ordering::Greater => out.push('0'),
                }
                zeroes = 0;
                ones = 0;
                ctr = 0;
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
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
        assert_eq!(code.decode("110100110000").unwrap(), "1�0");
    }
}
