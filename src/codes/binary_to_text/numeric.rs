use crate::codes::Code;
use crate::errors::Error;
use itertools::Itertools;
use num::Integer;
use std::fs::read;
use std::path::PathBuf;

use super::{bytes_to_hex, BinaryToText, BinaryToTextMode};

// Make it possible to encode an aribtrary file
pub struct Numeric {
    pub file: Option<PathBuf>,
    pub mode: BinaryToTextMode,
    pub fixed_width: bool,
    pub radix: u32,
}

impl Default for Numeric {
    fn default() -> Self {
        Self {
            file: None,
            mode: BinaryToTextMode::Utf8,
            fixed_width: false,
            radix: 10,
        }
    }
}

impl Numeric {
    pub fn chars_codes(&self) -> impl Iterator<Item = (String, String)> + '_ {
        (0..255u8).map(|x| (format!("{x}"), self.byte_to_number(&x)))
    }

    pub fn byte_to_number(&self, byte: &u8) -> String {
        // Built ins
        match self.radix {
            2 => return format!("{:b}", byte),
            8 => return format!("{:o}", byte),
            10 => return format!("{}", byte),
            16 => return format!("{:X}", byte),
            _ => (),
        }
        // Handle zero
        if byte == &0 {
            String::from("0")
        } else {
            let mut b = *byte;
            let mut s = Vec::new();
            let divisor = self.radix as u8;
            while b != 0 {
                let (q, r) = b.div_rem(&divisor);
                if r < 10 {
                    s.push(r + 48) // shift to start of ASCII numbers
                } else {
                    s.push(r + 55) // shift to start of ASCII uppercase letters
                }
                b = q;
            }
            String::from_utf8(s.into_iter().rev().collect_vec()).unwrap()
        }
    }

    pub fn number_to_byte(&self, number: &str) -> Result<u8, Error> {
        u8::from_str_radix(number, self.radix).map_err(|e| Error::Input(e.to_string()))
    }

    pub fn encode_file(&self) -> Result<String, Error> {
        if self.file.is_none() {
            return Err(Error::input("no file stored"));
        }
        let bytes = &read(self.file.as_ref().unwrap()).unwrap()[..];
        self.encode_bytes(bytes)
    }
}

impl BinaryToText for Numeric {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, Error> {
        Ok(bytes.iter().map(|b| self.byte_to_number(b)).join(" "))
    }
}

impl Code for Numeric {
    fn encode(&self, text: &str) -> Result<String, Error> {
        match self.mode {
            BinaryToTextMode::Hex => self.encode_hex(text),
            BinaryToTextMode::Utf8 => self.encode_utf8(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut bytes = Vec::new();
        for s in text.split(" ") {
            if s.is_empty() {
                continue;
            } else {
                let b = self.number_to_byte(s)?;
                bytes.push(b)
            }
        }
        match self.mode {
            BinaryToTextMode::Hex => bytes_to_hex(&bytes),
            BinaryToTextMode::Utf8 => {
                String::from_utf8(bytes).map_err(|e| Error::Input(e.to_string()))
            }
        }
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod numeric_tests {
    use super::*;

    const TESTS_10: [(&'static str, &'static str); 2] =
        [("<=>", "60 61 62"), ("tHe", "116 72 101")];
    const TESTS_36: [(&'static str, &'static str); 2] = [("<=>", "1O 1P 1Q"), ("tHe", "38 20 2T")];

    #[test]
    fn encode_test() {
        let mut code = Numeric::default();
        for (ptext, ctext) in TESTS_10 {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
        code.radix = 36;
        for (ptext, ctext) in TESTS_36 {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
    }

    #[test]
    fn decode_test() {
        let mut code = Numeric::default();
        for (ptext, ctext) in TESTS_10 {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
        code.radix = 36;
        for (ptext, ctext) in TESTS_36 {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
    }
}
