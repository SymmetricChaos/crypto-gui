use crate::{errors::CodeError, traits::Code};

use super::BinaryToText;
use itertools::Itertools;
use utils::{
    byte_formatting::ByteFormat,
    text_functions::{u8_to_string_with_radix, u8_to_string_with_radix_and_width},
};

// Make it possible to encode an aribtrary file
pub struct BytesAsNumbers {
    pub mode: ByteFormat,
    pub fixed_width: bool,
    pub radix: u32,
    pub width: usize,
    pub little_endian: bool,
}

impl Default for BytesAsNumbers {
    fn default() -> Self {
        Self {
            mode: ByteFormat::Utf8,
            fixed_width: false,
            radix: 10,
            width: 3,
            little_endian: true,
        }
    }
}

impl BytesAsNumbers {
    pub fn chars_codes(&self) -> impl Iterator<Item = (String, String)> + '_ {
        (0..=255u8).map(|x| (format!("{x: <3}"), self.byte_to_number(&x)))
    }

    pub fn set_width(&mut self) {
        self.width = 256.0_f32.log(self.radix as f32).ceil() as usize
    }

    pub fn byte_to_number(&self, byte: &u8) -> String {
        let s = if self.fixed_width {
            match self.radix {
                2 => return format!("{:08b}", byte),
                8 => return format!("{:03o}", byte),
                10 => return format!("{:03}", byte),
                16 => return format!("{:02X}", byte),
                r => u8_to_string_with_radix_and_width(byte, r as u8, self.width),
            }
        } else {
            match self.radix {
                2 => format!("{:b}", byte),
                8 => format!("{:o}", byte),
                10 => format!("{}", byte),
                16 => format!("{:X}", byte),
                r => u8_to_string_with_radix(byte, r as u8),
            }
        };
        if !self.little_endian {
            s.chars().rev().collect()
        } else {
            s
        }
    }

    pub fn number_to_byte(&self, number: &str) -> Result<u8, CodeError> {
        if !self.little_endian {
            u8::from_str_radix(&number.chars().rev().collect::<String>(), self.radix)
                .map_err(|e| CodeError::Input(e.to_string()))
        } else {
            u8::from_str_radix(number, self.radix).map_err(|e| CodeError::Input(e.to_string()))
        }
    }

    // pub fn encode_file(&self) -> Result<String, CodeError> {
    //     if self.file.is_none() {
    //         return Err(CodeError::input("no file stored"));
    //     }
    //     let bytes = &read(self.file.as_ref().unwrap()).unwrap()[..];
    //     self.encode_bytes(bytes)
    // }
}

impl BinaryToText for BytesAsNumbers {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
        Ok(bytes.iter().map(|b| self.byte_to_number(b)).join(" "))
    }
}

impl Code for BytesAsNumbers {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.mode {
            ByteFormat::Hex => self.encode_hex(text),
            ByteFormat::Utf8 => self.encode_utf8(text),
            ByteFormat::Base64 => self.encode_base64(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut bytes = Vec::new();
        for s in text.split(" ") {
            if s.is_empty() {
                continue;
            } else {
                let b = self.number_to_byte(s)?;
                bytes.push(b)
            }
        }
        Ok(self.mode.byte_slice_to_text(bytes))
    }
}

#[cfg(test)]
mod numeric_tests {
    use super::*;

    const TESTS_10: [(&'static str, &'static str); 2] =
        [("<=>", "60 61 62"), ("tHe", "116 72 101")];
    const TESTS_10_BE: [(&'static str, &'static str); 2] =
        [("<=>", "06 16 26"), ("tHe", "611 27 101")];
    const TESTS_36: [(&'static str, &'static str); 2] = [("<=>", "1O 1P 1Q"), ("tHe", "38 20 2T")];
    const TESTS_36_BE: [(&'static str, &'static str); 2] =
        [("<=>", "O1 P1 Q1"), ("tHe", "83 02 T2")];
    #[test]
    fn encode_test() {
        let mut code = BytesAsNumbers::default();
        for (ptext, ctext) in TESTS_10 {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
        code.radix = 36;
        for (ptext, ctext) in TESTS_36 {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
    }

    #[test]
    fn encode_test_bigendian() {
        let mut code = BytesAsNumbers::default();
        code.little_endian = false;
        for (ptext, ctext) in TESTS_10_BE {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
        code.radix = 36;
        for (ptext, ctext) in TESTS_36_BE {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
    }

    #[test]
    fn decode_test() {
        let mut code = BytesAsNumbers::default();
        for (ptext, ctext) in TESTS_10 {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
        code.radix = 36;
        for (ptext, ctext) in TESTS_36 {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
    }

    #[test]
    fn decode_test_bigendian() {
        let mut code = BytesAsNumbers::default();
        code.little_endian = false;
        for (ptext, ctext) in TESTS_10_BE {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
        code.radix = 36;
        for (ptext, ctext) in TESTS_36_BE {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
    }
}
