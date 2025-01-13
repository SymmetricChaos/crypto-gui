use crate::{errors::CodeError, traits::Code};
use utils::byte_formatting::ByteFormat;

const ARR: [u8; 10] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum BcdVariant {
//     Simple,
//     V7421,
//     Aiken,
//     Excess3,
//     Gray,
// }

// impl BcdVariant {
//     fn array(&self) -> [u8; 10] {
//         match self {
//             BcdVariant::Simple => [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9],
//             BcdVariant::V7421 => [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x8, 0x9, 0xA],
//             BcdVariant::Aiken => [0x0, 0x1, 0x2, 0x3, 0x4, 0xB, 0xC, 0xD, 0xE, 0xF],
//             BcdVariant::Excess3 => [0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC],
//             BcdVariant::Gray => [0x0, 0x1, 0x3, 0x2, 0x7, 0x6, 0x4, 0x5, 0xC, 0xD],
//         }
//     }
// }

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
    // variant: BcdVariant,
    formatting: ByteFormat,
    width: WordWidth,
}

impl Default for BinaryCodedDecimal {
    fn default() -> Self {
        Self {
            // variant: BcdVariant::Simple,
            formatting: ByteFormat::Hex,
            width: WordWidth::W32,
        }
    }
}

impl BinaryCodedDecimal {
    // fn range(&self) -> (i64, i64) {
    //     match (self.signed, self.width) {
    //         (true, WordWidth::W32) => (-9999999, 9999999),
    //         (true, WordWidth::W64) => (-999999999999999, 999999999999999),
    //         (false, WordWidth::W32) => (0, 99999999),
    //         (false, WordWidth::W64) => (0, 9999999999999999),
    //     }
    // }

    fn encode_signed_to_u32(&self, text: &str) -> Result<Vec<u32>, CodeError> {
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
                    word |= digit_to_nibble(digit, &ARR)? as u32;
                }
            } else {
                for digit in number.chars() {
                    word <<= 4;
                    word |= digit_to_nibble(digit, &ARR)? as u32;
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

    fn decode_u32_to_signed(&self, values: &[u32]) -> Result<Vec<String>, CodeError> {
        let mut out = Vec::with_capacity(values.len());
        for value in values {
            let negative = if value & 0xF == 0xD {
                true
            } else if value & 0xF == 0xC {
                false
            } else {
                return Err(CodeError::Input(format!(
                    "invalid sign nibble in value {:08x?}, only 0xC and 0xD are allowed",
                    value
                )));
            };
            let mut n = 0;
            for i in (1..8).rev() {
                let nibble = ((value >> (4 * i)) & 0xF) as u8;
                if ARR.contains(&nibble) {
                    n *= 10;
                    n += nibble as i32;
                } else {
                    return Err(CodeError::Input(format!(
                        "invalid digit nibble `0x{:01x?}` in value {:08x?}",
                        nibble, value
                    )));
                }
            }
            if negative {
                n *= -1;
            }
            out.push(n.to_string());
        }

        Ok(out)
    }

    fn encode_signed_to_u64(&self, text: &str) -> Result<Vec<u64>, CodeError> {
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
                    word |= digit_to_nibble(digit, &ARR)? as u64;
                }
            } else {
                for digit in number.chars() {
                    word <<= 4;
                    word |= digit_to_nibble(digit, &ARR)? as u64;
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

    fn decode_u64_to_signed(&self, values: &[u64]) -> Result<Vec<String>, CodeError> {
        let mut out = Vec::with_capacity(values.len());
        // Most of this loop works only for the BcdVariant::Simple
        for value in values {
            let negative = if value & 0xF == 0xD {
                true
            } else if value & 0xF == 0xC {
                false
            } else {
                return Err(CodeError::Input(format!(
                    "invalid sign nibble in value {:016x?}, only 0xC and 0xD are allowed",
                    value
                )));
            };
            let mut n = 0;
            for i in (1..16).rev() {
                let nibble = ((value >> (4 * i)) & 0xF) as u8;
                if ARR.contains(&nibble) {
                    n *= 10;
                    n += nibble as i64;
                } else {
                    return Err(CodeError::Input(format!(
                        "invalid digit nibble `0x{:01x?}` in value {:016x?}",
                        nibble, value
                    )));
                }
            }
            if negative {
                n *= -1;
            }
            out.push(n.to_string());
        }

        Ok(out)
    }
}

impl Code for BinaryCodedDecimal {
    fn encode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        match self.width {
            WordWidth::W32 => {
                let v = self.encode_signed_to_u32(text)?;
                Ok(self.formatting.u32_slice_to_text_be(&v))
            }
            WordWidth::W64 => {
                let v = self.encode_signed_to_u64(text)?;
                Ok(self.formatting.u64_slice_to_text_be(&v))
            }
        }
    }

    fn decode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        match self.width {
            WordWidth::W32 => {
                let values = self
                    .formatting
                    .text_to_u32_be(text)
                    .map_err(|e| CodeError::Input(e.to_string()))?;
                Ok(self.decode_u32_to_signed(&values)?.join(", "))
            }
            WordWidth::W64 => {
                let values = self
                    .formatting
                    .text_to_u64_be(text)
                    .map_err(|e| CodeError::Input(e.to_string()))?;
                Ok(self.decode_u64_to_signed(&values)?.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod bcd_tests {
    use super::*;

    const PLAINTEXT32: &'static str = "12345, -9876543";
    const ENCODEDTEXT32_SIMPLE: &'static str = "0012345c9876543d";

    const PLAINTEXT64: &'static str = "1234567890, -876543211000";
    const ENCODEDTEXT64_SIMPLE: &'static str = "000001234567890c000876543211000d";

    #[test]
    fn encode_u32_simple_test() {
        let code = BinaryCodedDecimal::default();
        assert_eq!(ENCODEDTEXT32_SIMPLE, code.encode(PLAINTEXT32).unwrap())
    }

    #[test]
    fn decode_u32_simple_test() {
        let code = BinaryCodedDecimal::default();
        assert_eq!(PLAINTEXT32, code.decode(ENCODEDTEXT32_SIMPLE).unwrap())
    }

    #[test]
    fn encode_u64_simple_test() {
        let mut code = BinaryCodedDecimal::default();
        code.width = WordWidth::W64;
        assert_eq!(ENCODEDTEXT64_SIMPLE, code.encode(PLAINTEXT64).unwrap())
    }

    #[test]
    fn decode_u64_simple_test() {
        let mut code = BinaryCodedDecimal::default();
        code.width = WordWidth::W64;
        assert_eq!(PLAINTEXT64, code.decode(ENCODEDTEXT64_SIMPLE).unwrap())
    }
}
