use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use paste::paste;
use utils::byte_formatting::ByteFormat;

const ARR: [u8; 10] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];
const R32MAX: u32 = 99999999;
const R64MAX: u64 = 9999999999999999;
const R128MAX: u128 = 99999999999999999999999999999999;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordWidth {
    W32,
    W64,
    W128,
}

macro_rules! encode_decode_unsigned {
    ($t: ty, $bytes: literal, $max: ident) => {
        paste! {
            pub fn [<encode_ $t>](&self, values: &[$t]) -> Result<Vec<$t>, CodeError> {
                let mut out = Vec::with_capacity(values.len());
                for &value in values {
                    if value > $max {
                        return Err(CodeError::Input(
                            format!("the allowed range is 0..={}",  $max)
                        ));
                    }

                    let t = value;
                    let mut word = 0 as $t;
                    let ten = 10 as $t;
                    for i in (0..$bytes).rev() {
                        word <<= 4;
                        word |= (t / (ten.pow(i))) % 10;
                    }
                    out.push(word);
                }
                Ok(out)
            }

            pub fn [<decode_ $t>](&self, values: &[$t]) -> Result<Vec<$t>, CodeError> {
                let mut out = Vec::with_capacity(values.len());
                for value in values {
                    let mut n = 0;
                    for i in (0..$bytes).rev() {
                        let nibble = ((value >> (4 * i)) & 0xF) as u8;
                        if ARR.contains(&nibble) {
                            n *= 10;
                            n += nibble as $t;
                        } else {
                            return Err(CodeError::Input(format!(
                                "invalid digit nibble `0x{:01x?}` in value {:08x?}",
                                nibble, value
                            )));
                        }
                    }
                    out.push(n);
                }

                Ok(out)
            }
        }
    };
}

pub struct BinaryCodedDecimalUnsigned {
    pub formatting: ByteFormat,
    pub width: WordWidth,
}

impl Default for BinaryCodedDecimalUnsigned {
    fn default() -> Self {
        Self {
            formatting: ByteFormat::Hex,
            width: WordWidth::W32,
        }
    }
}

impl BinaryCodedDecimalUnsigned {
    encode_decode_unsigned!(u32, 8, R32MAX);
    encode_decode_unsigned!(u64, 16, R64MAX);
    encode_decode_unsigned!(u128, 32, R128MAX);
}

impl Code for BinaryCodedDecimalUnsigned {
    fn encode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        match self.width {
            WordWidth::W32 => {
                let mut values = Vec::new();
                for value in text.split(",").map(|s| s.trim()) {
                    let n = u32::from_str_radix(value.trim(), 10)
                        .map_err(|e| CodeError::Input(e.to_string()))?;
                    values.push(n);
                }
                Ok(self
                    .formatting
                    .u32_slice_to_text_be(&self.encode_u32(&values)?))
            }
            WordWidth::W64 => {
                let mut values = Vec::new();
                for value in text.split(",").map(|s| s.trim()) {
                    let n = u64::from_str_radix(value.trim(), 10)
                        .map_err(|e| CodeError::Input(e.to_string()))?;
                    values.push(n);
                }
                Ok(self
                    .formatting
                    .u64_slice_to_text_be(&self.encode_u64(&values)?))
            }
            WordWidth::W128 => {
                let mut values = Vec::new();
                for value in text.split(",").map(|s| s.trim()) {
                    let n = u128::from_str_radix(value.trim(), 10)
                        .map_err(|e| CodeError::Input(e.to_string()))?;
                    values.push(n);
                }
                Ok(self
                    .formatting
                    .u128_slice_to_text_be(&self.encode_u128(&values)?))
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
            WordWidth::W128 => {
                let values = self
                    .formatting
                    .text_to_u128_be(text)
                    .map_err(|e| CodeError::Input(e.to_string()))?;
                Ok(self
                    .decode_u128(&values)?
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

    const PLAINTEXT32: &'static str = "12345, 9876543";
    const ENCODEDTEXT32: &'static str = "0001234509876543";

    const PLAINTEXT64: &'static str = "1234567890, 876543211000, 567567567567567";
    const ENCODEDTEXT64: &'static str = "000000123456789000008765432110000567567567567567";

    #[test]
    fn test_words_32() {
        assert_eq!(
            vec![0x00012345_u32, 0x09876543],
            BinaryCodedDecimalUnsigned::default()
                .encode_u32(&[12345_u32, 9876543])
                .unwrap()
        );
        assert_eq!(
            vec![12345_u32, 9876543],
            BinaryCodedDecimalUnsigned::default()
                .decode_u32(&[0x00012345_u32, 0x09876543])
                .unwrap()
        );
    }

    #[test]
    fn encode_test_32() {
        let code = BinaryCodedDecimalUnsigned::default();
        assert_eq!(ENCODEDTEXT32, code.encode(PLAINTEXT32).unwrap());
    }

    #[test]
    fn decode_test_32() {
        let code = BinaryCodedDecimalUnsigned::default();
        assert_eq!(PLAINTEXT32, code.decode(ENCODEDTEXT32).unwrap())
    }

    #[test]
    fn test_words_64() {
        assert_eq!(
            vec![
                0x00000001234567890_u64,
                0x0000876543211000,
                0x0567567567567567
            ],
            BinaryCodedDecimalUnsigned::default()
                .encode_u64(&[1234567890_u64, 876543211000, 567567567567567])
                .unwrap()
        );
        assert_eq!(
            vec![1234567890_u64, 876543211000, 567567567567567],
            BinaryCodedDecimalUnsigned::default()
                .decode_u64(&[
                    0x0000001234567890_u64,
                    0x0000876543211000,
                    0x0567567567567567
                ])
                .unwrap()
        );
    }

    #[test]
    fn encode_test_64() {
        let mut code = BinaryCodedDecimalUnsigned::default();
        code.width = WordWidth::W64;
        assert_eq!(ENCODEDTEXT64, code.encode(PLAINTEXT64).unwrap());
    }

    #[test]
    fn decode_test_64() {
        let mut code = BinaryCodedDecimalUnsigned::default();
        code.width = WordWidth::W64;
        assert_eq!(PLAINTEXT64, code.decode(ENCODEDTEXT64).unwrap())
    }
}
