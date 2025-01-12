use utils::byte_formatting::ByteFormat;

use crate::{errors::CodeError, traits::Code};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BcdVariant {
    V8421,
    V7421,
    Aiken,
    Excess3,
    Gray,
}

impl BcdVariant {
    fn array(&self) -> [u8; 10] {
        match self {
            BcdVariant::V8421 => [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9],
            BcdVariant::V7421 => [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x8, 0x9, 0xA],
            BcdVariant::Aiken => [0x0, 0x1, 0x2, 0x3, 0x4, 0xB, 0xC, 0xD, 0xE, 0xF],
            BcdVariant::Excess3 => [0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC],
            BcdVariant::Gray => [0x0, 0x1, 0x3, 0x2, 0x7, 0x6, 0x4, 0x5, 0xC, 0xD],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordWidth {
    W32,
    W64,
}

fn digit_to_nibble(digit: char, arr: &[u8; 10]) -> Result<u8, CodeError> {
    match digit {
        '0'..='9' => Ok(arr[(digit as u32 - 48) as usize]),
        _ => return Err(CodeError::invalid_input_char(digit)),
    }
}

pub struct BinaryCodedDecimal {
    variant: BcdVariant,
    formatting: ByteFormat,
    signed: bool,
    width: WordWidth,
}

impl Default for BinaryCodedDecimal {
    fn default() -> Self {
        Self {
            variant: BcdVariant::V8421,
            signed: true,
            formatting: ByteFormat::Hex,
            width: WordWidth::W32,
        }
    }
}

impl BinaryCodedDecimal {
    fn range(&self) -> (i64, i64) {
        match (self.signed, self.width) {
            (true, WordWidth::W32) => (-9999999, 9999999),
            (true, WordWidth::W64) => (-999999999999999, 999999999999999),
            (false, WordWidth::W32) => (0, 99999999),
            (false, WordWidth::W64) => (0, 9999999999999999),
        }
    }

    fn encode_signed_to_u32(&self, text: &str) -> Result<Vec<u32>, CodeError> {
        let arr = self.variant.array();
        let mut words = Vec::new();
        for number in text.split(',').map(|s| s.trim()) {
            if number.is_empty() {
                continue;
            }
            let negative = number.chars().next().unwrap() == '-';
            if negative && number.chars().count() > 8 {
                return Err(CodeError::invalid_input_group(number));
            }
            if !negative && number.chars().count() > 7 {
                return Err(CodeError::invalid_input_group(number));
            }
            let mut word: u32 = 0;
            if negative {
                for digit in number.chars().skip(1) {
                    word <<= 4;
                    word |= digit_to_nibble(digit, &arr)? as u32;
                }
            } else {
                for digit in number.chars() {
                    word <<= 4;
                    word |= digit_to_nibble(digit, &arr)? as u32;
                }
            }
            if negative {
                word <<= 4;
                word |= 0xD
            } else {
                word <<= 4;
                word |= 0xC
            }
            words.push(word);
        }

        Ok(words)
    }

    fn encode_signed_to_u64(&self, text: &str) -> Result<Vec<u64>, CodeError> {
        let arr = self.variant.array();
        let mut words = Vec::new();
        for number in text.split(',').map(|s| s.trim()) {
            if number.is_empty() {
                continue;
            }
            let negative = number.chars().next().unwrap() == '-';
            if negative && number.chars().count() > 16 {
                return Err(CodeError::invalid_input_group(number));
            }
            if !negative && number.chars().count() > 15 {
                return Err(CodeError::invalid_input_group(number));
            }
            let mut word: u64 = 0;
            if negative {
                for digit in number.chars().skip(1) {
                    word <<= 4;
                    word |= digit_to_nibble(digit, &arr)? as u64;
                }
            } else {
                for digit in number.chars() {
                    word <<= 4;
                    word |= digit_to_nibble(digit, &arr)? as u64;
                }
            }
            if negative {
                word <<= 4;
                word |= 0xD
            } else {
                word <<= 4;
                word |= 0xC
            }
            words.push(word);
        }

        Ok(words)
    }
}

impl Code for BinaryCodedDecimal {
    fn encode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod bcd_tests {
    use super::*;

    const PLAINTEXT: &'static str = "45541, -2321111";
    const ENCODEDTEXT: &'static str = "";

    #[test]
    fn encode_test() {
        let code = BinaryCodedDecimal::default();
        assert_eq!(
            vec![0x0045541c, 0x2321111d],
            code.encode_signed_to_u32(PLAINTEXT).unwrap(),
        );
    }

    #[test]
    fn decode_test() {
        let code = BinaryCodedDecimal::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
