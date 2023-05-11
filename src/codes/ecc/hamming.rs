use itertools::Itertools;

use super::bits_from_bitstring;
use crate::{codes::Code, errors::Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HammingCodeVariant {
    ThreeOne,
    FourSeven,
    FifteenEleven,
}

pub struct HammingCode {
    pub variant: HammingCodeVariant,
}

impl HammingCode {
    fn encode_3_1(text: &str) -> Result<String, Error> {
        let mut out = String::new();
        for bit in bits_from_bitstring(text) {
            match bit? {
                0 => out.push_str("000"),
                1 => out.push_str("111"),
                _ => unreachable!("bits_from_bitstring should filter out everything but 0s and 1s"),
            }
        }
        Ok(out)
    }

    fn decode_3_1(text: &str) -> Result<String, Error> {
        let mut out = String::new();
        for (a, b, c) in bits_from_bitstring(text).tuples() {
            match (a?, b?, c?) {
                (0, 0, 1) => out.push('0'),
                (0, 0, 0) => out.push('0'),
                (0, 1, 0) => out.push('0'),
                (1, 0, 0) => out.push('0'),
                (1, 1, 1) => out.push('1'),
                (1, 1, 0) => out.push('1'),
                (1, 0, 1) => out.push('1'),
                (0, 1, 1) => out.push('1'),
                _ => unreachable!("bits_from_bitstring should filter out everything but 0s and 1s"),
            }
        }
        Ok(out)
    }
}

impl Default for HammingCode {
    fn default() -> Self {
        Self {
            variant: HammingCodeVariant::FourSeven,
        }
    }
}

impl Code for HammingCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        match self.variant {
            HammingCodeVariant::ThreeOne => Self::encode_3_1(text),
            HammingCodeVariant::FourSeven => todo!(),
            HammingCodeVariant::FifteenEleven => todo!(),
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        match self.variant {
            HammingCodeVariant::ThreeOne => Self::decode_3_1(text),
            HammingCodeVariant::FourSeven => todo!(),
            HammingCodeVariant::FifteenEleven => todo!(),
        }
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod hamming_tests {
    use super::*;

    #[test]
    fn test_encode_3_1() {
        assert_eq!(HammingCode::encode_3_1("1011").unwrap(), "111000111111");
    }

    #[test]
    fn test_decode_3_1() {
        assert_eq!(HammingCode::decode_3_1("111000111111").unwrap(), "1011");
    }
}
