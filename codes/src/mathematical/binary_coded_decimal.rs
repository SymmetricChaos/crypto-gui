use crate::traits::Code;
use itertools::Itertools;
use paste::paste;
use utils::byte_formatting::ByteFormat;

const ARR: [u8; 10] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];
const POS: u8 = 0xC;
const NEG: u8 = 0xD;
const R32MIN: i32 = -9999999;
const R32MAX: i32 = 9999999;
const R64MIN: i64 = -999999999999999;
const R64MAX: i64 = 999999999999999;
const R128MIN: i128 = -9999999999999999999999999999999;
const R128MAX: i128 = 9999999999999999999999999999999;
const U32MAX: u32 = 99999999;
const U64MAX: u64 = 9999999999999999;
const U128MAX: u128 = 99999999999999999999999999999999;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordWidth {
    W32,
    W64,
    W128,
}

macro_rules! encode_decode_signed {
    ($t: ty, $u: ty, $bytes: literal, $min: ident, $max: ident) => {
        paste! {
            pub fn [<encode_ $t>](&self, values: &[$t]) -> Result<Vec<$u>, utils::errors::GeneralError> {
                let mut out = Vec::with_capacity(values.len());
                for &value in values {
                    if value < $min || value > $max {
                        return Err(utils::errors::GeneralError::input(
                            format!("the allowed range is {}..={}", $min, $max)
                        ));
                    }
                    let t = value.abs() as $u;
                    let mut word = 0 as $u;
                    let ten = 10 as $u;
                    for i in (0..($bytes-1)).rev() {
                        word |= (t / (ten.pow(i))) % 10;
                        word <<= 4;
                    }
                    if value.is_negative() {
                        word |= NEG as $u;
                    } else {
                        word |= POS as $u;
                    }
                    out.push(word);
                }
                Ok(out)
            }

            pub fn [<decode_ $u>](&self, values: &[$u]) -> Result<Vec<$t>, utils::errors::GeneralError> {
                let mut out = Vec::with_capacity(values.len());
                for value in values {
                    let negative = if value & 0xF == NEG as $u {
                        true
                    } else if value & 0xF == POS as $u {
                        false
                    } else {
                        return Err(utils::errors::GeneralError::input(format!(
                            "invalid sign nibble in value {:08x?}, only 0xC and 0xD are allowed",
                            value
                        )));
                    };
                    let mut n = 0;
                    for i in (1..$bytes).rev() {
                        let nibble = ((value >> (4 * i)) & 0xF) as u8;
                        if ARR.contains(&nibble) {
                            n *= 10;
                            n += nibble as $t;
                        } else {
                            return Err(utils::errors::GeneralError::input(format!(
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
        }
    };
}

macro_rules! encode_decode_unsigned {
    ($t: ty, $bytes: literal, $max: ident) => {
        paste! {
            pub fn [<encode_ $t _usigned>](&self, values: &[$t]) -> Result<Vec<$t>, utils::errors::GeneralError> {
                let mut out = Vec::with_capacity(values.len());
                for &value in values {
                    if value > $max {
                        return Err(utils::errors::GeneralError::input(
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

            pub fn [<decode_ $t _usigned>](&self, values: &[$t]) -> Result<Vec<$t>, utils::errors::GeneralError> {
                let mut out = Vec::with_capacity(values.len());
                for value in values {
                    let mut n = 0;
                    for i in (0..$bytes).rev() {
                        let nibble = ((value >> (4 * i)) & 0xF) as u8;
                        if ARR.contains(&nibble) {
                            n *= 10;
                            n += nibble as $t;
                        } else {
                            return Err(utils::errors::GeneralError::input(format!(
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

pub struct BinaryCodedDecimal {
    pub formatting: ByteFormat,
    pub width: WordWidth,
    pub signed: bool,
}

impl Default for BinaryCodedDecimal {
    fn default() -> Self {
        Self {
            formatting: ByteFormat::Hex,
            width: WordWidth::W32,
            signed: true,
        }
    }
}

impl BinaryCodedDecimal {
    pub fn signed() -> Self {
        Self {
            formatting: ByteFormat::Hex,
            width: WordWidth::W32,
            signed: true,
        }
    }

    pub fn unsigned() -> Self {
        Self {
            formatting: ByteFormat::Hex,
            width: WordWidth::W32,
            signed: false,
        }
    }

    encode_decode_signed!(i32, u32, 8, R32MIN, R32MAX);
    encode_decode_signed!(i64, u64, 16, R64MIN, R64MAX);
    encode_decode_signed!(i128, u128, 32, R128MIN, R128MAX);

    encode_decode_unsigned!(u32, 8, U32MAX);
    encode_decode_unsigned!(u64, 16, U64MAX);
    encode_decode_unsigned!(u128, 32, U128MAX);
}

impl Code for BinaryCodedDecimal {
    fn encode(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        if self.signed {
            match self.width {
                WordWidth::W32 => {
                    let mut values = Vec::new();
                    for s in text.split(",").map(|s| s.trim()) {
                        let n = i32::from_str_radix(s, 10)
                            .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                        values.push(n);
                    }
                    Ok(self
                        .formatting
                        .u32_slice_to_text_be(&self.encode_i32(&values)?))
                }
                WordWidth::W64 => {
                    let mut values = Vec::new();
                    for value in text.split(",").map(|s| s.trim()) {
                        let n = i64::from_str_radix(value, 10)
                            .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                        values.push(n);
                    }
                    Ok(self
                        .formatting
                        .u64_slice_to_text_be(&self.encode_i64(&values)?))
                }
                WordWidth::W128 => {
                    let mut values = Vec::new();
                    for value in text.split(",").map(|s| s.trim()) {
                        let n = i128::from_str_radix(value.trim(), 10)
                            .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                        values.push(n);
                    }
                    Ok(self
                        .formatting
                        .u128_slice_to_text_be(&self.encode_i128(&values)?))
                }
            }
        } else {
            match self.width {
                WordWidth::W32 => {
                    let mut values = Vec::new();
                    for value in text.split(",").map(|s| s.trim()) {
                        let n = u32::from_str_radix(value.trim(), 10)
                            .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                        values.push(n);
                    }
                    Ok(self
                        .formatting
                        .u32_slice_to_text_be(&self.encode_u32_usigned(&values)?))
                }
                WordWidth::W64 => {
                    let mut values = Vec::new();
                    for value in text.split(",").map(|s| s.trim()) {
                        let n = u64::from_str_radix(value.trim(), 10)
                            .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                        values.push(n);
                    }
                    Ok(self
                        .formatting
                        .u64_slice_to_text_be(&self.encode_u64_usigned(&values)?))
                }
                WordWidth::W128 => {
                    let mut values = Vec::new();
                    for value in text.split(",").map(|s| s.trim()) {
                        let n = u128::from_str_radix(value.trim(), 10)
                            .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                        values.push(n);
                    }
                    Ok(self
                        .formatting
                        .u128_slice_to_text_be(&self.encode_u128_usigned(&values)?))
                }
            }
        }
    }

    fn decode(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        if self.signed {
            match self.width {
                WordWidth::W32 => {
                    let values = self
                        .formatting
                        .text_to_u32_be(text)
                        .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
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
                        .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
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
                        .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                    Ok(self
                        .decode_u128(&values)?
                        .into_iter()
                        .map(|n| n.to_string())
                        .join(", "))
                }
            }
        } else {
            match self.width {
                WordWidth::W32 => {
                    let values = self
                        .formatting
                        .text_to_u32_be(text)
                        .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                    Ok(self
                        .decode_u32_usigned(&values)?
                        .into_iter()
                        .map(|n| n.to_string())
                        .join(", "))
                }
                WordWidth::W64 => {
                    let values = self
                        .formatting
                        .text_to_u64_be(text)
                        .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                    Ok(self
                        .decode_u64_usigned(&values)?
                        .into_iter()
                        .map(|n| n.to_string())
                        .join(", "))
                }
                WordWidth::W128 => {
                    let values = self
                        .formatting
                        .text_to_u128_be(text)
                        .map_err(|e| utils::errors::GeneralError::input(e.to_string()))?;
                    Ok(self
                        .decode_u128_usigned(&values)?
                        .into_iter()
                        .map(|n| n.to_string())
                        .join(", "))
                }
            }
        }
    }
}

#[cfg(test)]
mod bcd_tests {
    use super::*;

    const PTEXT32: &'static str = "12345, -9876543";
    const ENCODEDTEXT32: &'static str = "0012345c9876543d";

    const PTEXT64: &'static str = "1234567890, -876543211000, 567567567567567";
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
        assert_eq!(ENCODEDTEXT32, code.encode(PTEXT32).unwrap());
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
        assert_eq!(PTEXT32, code.decode(ENCODEDTEXT32).unwrap())
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
        assert_eq!(ENCODEDTEXT64, code.encode(PTEXT64).unwrap());
    }

    #[test]
    fn decode_test_64() {
        let mut code = BinaryCodedDecimal::default();
        code.width = WordWidth::W64;
        assert_eq!(PTEXT64, code.decode(ENCODEDTEXT64).unwrap())
    }
}

#[cfg(test)]
mod bcd_unsigned_tests {
    use super::*;

    const PTEXT32: &'static str = "12345, 9876543";
    const ENCODEDTEXT32: &'static str = "0001234509876543";

    const PTEXT64: &'static str = "1234567890, 876543211000, 567567567567567";
    const ENCODEDTEXT64: &'static str = "000000123456789000008765432110000567567567567567";

    #[test]
    fn test_words_32() {
        assert_eq!(
            vec![0x00012345_u32, 0x09876543],
            BinaryCodedDecimal::unsigned()
                .encode_u32_usigned(&[12345_u32, 9876543])
                .unwrap()
        );
        assert_eq!(
            vec![12345_u32, 9876543],
            BinaryCodedDecimal::unsigned()
                .decode_u32_usigned(&[0x00012345_u32, 0x09876543])
                .unwrap()
        );
    }

    #[test]
    fn encode_test_32() {
        let code = BinaryCodedDecimal::unsigned();
        assert_eq!(ENCODEDTEXT32, code.encode(PTEXT32).unwrap());
    }

    #[test]
    fn decode_test_32() {
        let code = BinaryCodedDecimal::unsigned();
        assert_eq!(PTEXT32, code.decode(ENCODEDTEXT32).unwrap())
    }

    #[test]
    fn test_words_64() {
        assert_eq!(
            vec![
                0x00000001234567890_u64,
                0x0000876543211000,
                0x0567567567567567
            ],
            BinaryCodedDecimal::unsigned()
                .encode_u64_usigned(&[1234567890_u64, 876543211000, 567567567567567])
                .unwrap()
        );
        assert_eq!(
            vec![1234567890_u64, 876543211000, 567567567567567],
            BinaryCodedDecimal::unsigned()
                .decode_u64_usigned(&[
                    0x0000001234567890_u64,
                    0x0000876543211000,
                    0x0567567567567567
                ])
                .unwrap()
        );
    }

    #[test]
    fn encode_test_64() {
        let mut code = BinaryCodedDecimal::unsigned();
        code.width = WordWidth::W64;
        assert_eq!(ENCODEDTEXT64, code.encode(PTEXT64).unwrap());
    }

    #[test]
    fn decode_test_64() {
        let mut code = BinaryCodedDecimal::unsigned();
        code.width = WordWidth::W64;
        assert_eq!(PTEXT64, code.decode(ENCODEDTEXT64).unwrap())
    }
}
