use bimap::BiMap;
use lazy_static::lazy_static;
use regex::Regex;
use utils::functions::bimap_from_iter;

use crate::{errors::CodeError, traits::Code};

const GUARD: &'static str = "101"; // Stard and End guard pattern
const MIDDLE: &'static str = "01010";

lazy_static! {
    pub static ref UPCA_PATTERN: Regex = Regex::new(r"^101[01]{42}01010[01]{42}101$").unwrap();
    pub static ref UPCA_DIGITS: Regex = Regex::new(r"^[0-9]{12}$").unwrap();
    pub static ref UPCA_LEFT: BiMap<char, &'static str> =
        bimap_from_iter("0123456789".chars().zip([
            "0001101", "0011001", "0010011", "0111101", "0100011", "0110001", "0101111", "0111011",
            "0110111", "0001011"
        ]));
    pub static ref UPCA_RIGHT: BiMap<char, &'static str> =
        bimap_from_iter("0123456789".chars().zip([
            "1110010", "1100110", "1101100", "1000010", "1011100", "1001110", "1010000", "1000100",
            "1001000", "1110100"
        ]));
}

pub struct Upc {}

impl Default for Upc {
    fn default() -> Self {
        Self {}
    }
}

impl Upc {
    fn check_digit(text: &str) -> String {
        todo!()
    }
}

impl Code for Upc {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if !UPCA_DIGITS.is_match(text) {
            return Err(CodeError::input("a UPC-A code must have exactly 12 digits"));
        }

        let mut out = String::with_capacity(95);

        out.push_str(GUARD);

        for c in text.chars().take(6) {
            out.push_str(
                UPCA_LEFT
                    .get_by_left(&c)
                    .ok_or_else(|| CodeError::invalid_input_char(c))?,
            )
        }

        out.push_str(MIDDLE);

        for c in text.chars().skip(6).take(6) {
            out.push_str(
                UPCA_RIGHT
                    .get_by_left(&c)
                    .ok_or_else(|| CodeError::invalid_input_char(c))?,
            )
        }

        out.push_str(GUARD);
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        // Ignore quiet area on the ends
        let trimmed = text.trim_matches('0');

        if !UPCA_PATTERN.is_match(trimmed) {
            return Err(CodeError::input("not structured as a UPC-A code"));
        }

        let mut out = String::new();
        // Left side
        for i in 0..6 {
            let start = 3 + i * 7;
            let end = start + 7;
            let group = &trimmed[start..end];
            let digit = UPCA_LEFT
                .get_by_right(group)
                .ok_or_else(|| CodeError::invalid_input_group(group))?;
            out.push(*digit);
        }

        // Right side
        for i in 0..6 {
            let start = 50 + i * 7;
            let end = start + 7;
            let group = &trimmed[start..end];
            let digit = UPCA_RIGHT
                .get_by_right(group)
                .ok_or_else(|| CodeError::invalid_input_group(group))?;
            out.push(*digit);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod upc_tests {
    use super::*;

    #[test]
    fn encode() {
        let code = Upc::default();
        println!("{}", code.encode("012345678912").unwrap())
    }

    #[test]
    fn decode() {
        let code = Upc::default();
        println!("{}", code.decode("10100011010011001001001101111010100011011000101010101000010001001001000111010011001101101100101").unwrap())
    }
}
