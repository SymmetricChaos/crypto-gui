use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

const ARR: [u8; 10] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];
const POS: u8 = 0xC;
const NEG: u8 = 0xD;
const R32MIN: i32 = -9999999;
const R32MAX: i32 = 9999999;
const R64MIN: i64 = -999999999999999;
const R64MAX: i64 = 999999999999999;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordWidth {
    W32,
    W64,
}

pub struct BinaryCodedDecimal {
    pub formatting: ByteFormat,
    pub width: WordWidth,
}

impl Default for BinaryCodedDecimal {
    fn default() -> Self {
        Self {
            formatting: ByteFormat::Hex,
            width: WordWidth::W32,
        }
    }
}

impl BinaryCodedDecimal {
    pub fn encode_i32(&self, values: &[i32]) -> Result<Vec<u32>, CodeError> {
        let mut out = Vec::with_capacity(values.len());
        for &value in values {
            if value < R32MIN || value > R32MAX {
                return Err(CodeError::input(
                    "the range of 32-bit BCD is -9999999..=9999999",
                ));
            }
            let t = value.abs() as u32;
            let mut word = 0_u32;
            for i in (0..7).rev() {
                word |= (t / (10_u32.pow(i))) % 10;
                word <<= 4;
            }
            if value.is_negative() {
                word |= NEG as u32;
            } else {
                word |= POS as u32;
            }
            out.push(word);
        }
        Ok(out)
    }

    pub fn decode_u32(&self, values: &[u32]) -> Result<Vec<i32>, CodeError> {
        let mut out = Vec::with_capacity(values.len());
        for value in values {
            let negative = if value & 0xF == NEG as u32 {
                true
            } else if value & 0xF == POS as u32 {
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
            out.push(n);
        }

        Ok(out)
    }

    pub fn encode_i64(&self, values: &[i64]) -> Result<Vec<u64>, CodeError> {
        let mut out = Vec::with_capacity(values.len());
        for &value in values {
            if value < R64MIN || value > R64MAX {
                return Err(CodeError::input(
                    "the range of 64-bit BCD is -999999999999999..=999999999999999",
                ));
            }
            let t = value.abs() as u64;
            let mut word = 0_u64;
            for i in (0..15).rev() {
                word |= (t / (10_u64.pow(i))) % 10;
                word <<= 4;
            }
            if value.is_negative() {
                word |= NEG as u64;
            } else {
                word |= POS as u64;
            }
            out.push(word);
        }
        Ok(out)
    }

    fn decode_u64(&self, values: &[u64]) -> Result<Vec<i64>, CodeError> {
        let mut out = Vec::with_capacity(values.len());
        // Most of this loop works only for the BcdVariant::Simple
        for value in values {
            let negative = if value & 0xF == NEG as u64 {
                true
            } else if value & 0xF == POS as u64 {
                false
            } else {
                return Err(CodeError::Input(format!(
                    "invalid sign nibble in value {:016X?}, only 0xC and 0xD are allowed",
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
                        "invalid digit nibble `0x{:01X?}` in value {:016X?}",
                        nibble, value
                    )));
                }
            }
            if negative {
                n *= -1;
            }
            out.push(n);
        }

        Ok(out)
    }
}

impl Code for BinaryCodedDecimal {
    fn encode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        match self.width {
            WordWidth::W32 => {
                let mut values = Vec::new();
                for value in text.split(",").map(|s| s.trim()) {
                    let n = i32::from_str_radix(value.trim(), 10)
                        .map_err(|e| CodeError::Input(e.to_string()))?;
                    values.push(n);
                }
                Ok(self
                    .formatting
                    .u32_slice_to_text_be(&self.encode_i32(&values)?))
            }
            WordWidth::W64 => {
                let mut values = Vec::new();
                for value in text.split(",").map(|s| s.trim()) {
                    let n = i64::from_str_radix(value.trim(), 10)
                        .map_err(|e| CodeError::Input(e.to_string()))?;
                    values.push(n);
                }
                Ok(self
                    .formatting
                    .u64_slice_to_text_be(&self.encode_i64(&values)?))
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
                Ok(self
                    .decode_u32(&values)?
                    .into_iter()
                    .map(|n| n.to_string())
                    .join(", "))
            }
            WordWidth::W64 => {
                let values = self
                    .formatting
                    .text_to_u64_be(text)
                    .map_err(|e| CodeError::Input(e.to_string()))?;
                Ok(self
                    .decode_u64(&values)?
                    .into_iter()
                    .map(|n| n.to_string())
                    .join(", "))
            }
        }
    }
}

#[cfg(test)]
mod bcd_tests {
    use super::*;

    const PLAINTEXT32: &'static str = "12345, -9876543";
    const ENCODEDTEXT32: &'static str = "0012345c9876543d";

    const PLAINTEXT64: &'static str = "1234567890, -876543211000, 567567567567567";
    const ENCODEDTEXT64: &'static str = "000001234567890c000876543211000d567567567567567c";

    #[test]
    fn test_i32_and_u32() {
        assert_eq!(
            vec![0x0012345c_u32, 0x9876543d],
            BinaryCodedDecimal::default()
                .encode_i32(&[12345_i32, -9876543])
                .unwrap()
        );

        assert_eq!(
            vec![12345_i32, -9876543],
            BinaryCodedDecimal::default()
                .decode_u32(&[0x0012345c_u32, 0x9876543d])
                .unwrap()
        );
    }

    #[test]
    fn encode_test_32() {
        let code = BinaryCodedDecimal::default();
        assert_eq!(ENCODEDTEXT32, code.encode(PLAINTEXT32).unwrap());
    }

    #[test]
    fn encode_i32() {
        assert_eq!(
            vec![0x0012345c_u32, 0x9876543d],
            BinaryCodedDecimal::default()
                .encode_i32(&[12345_i32, -9876543])
                .unwrap()
        );
    }

    #[test]
    fn decode_test_32() {
        let code = BinaryCodedDecimal::default();
        assert_eq!(PLAINTEXT32, code.decode(ENCODEDTEXT32).unwrap())
    }

    #[test]
    fn test_i64_and_u64() {
        assert_eq!(
            vec![
                0x000001234567890c_u64,
                0x000876543211000d,
                0x567567567567567c
            ],
            BinaryCodedDecimal::default()
                .encode_i64(&[1234567890_i64, -876543211000, 567567567567567])
                .unwrap()
        );
        assert_eq!(
            vec![1234567890_i64, -876543211000, 567567567567567],
            BinaryCodedDecimal::default()
                .decode_u64(&[
                    0x000001234567890c_u64,
                    0x000876543211000d,
                    0x567567567567567c
                ])
                .unwrap()
        );
    }

    #[test]
    fn encode_test_64() {
        let mut code = BinaryCodedDecimal::default();
        code.width = WordWidth::W64;
        assert_eq!(ENCODEDTEXT64, code.encode(PLAINTEXT64).unwrap());
    }

    #[test]
    fn decode_test_64() {
        let mut code = BinaryCodedDecimal::default();
        code.width = WordWidth::W64;
        assert_eq!(PLAINTEXT64, code.decode(ENCODEDTEXT64).unwrap())
    }
}
