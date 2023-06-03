use lazy_static::lazy_static;
use regex::Regex;

use crate::{errors::CodeError, traits::Code};

const GUARD: &'static str = "101"; // Stard and End guard pattern
const MIDDLE: &'static str = "01010";

lazy_static! {
    pub static ref UPCA_PATTERN: Regex = Regex::new(r"^101[01]{42}10101[01]{42}101$").unwrap();
}

pub struct UPC {}

fn digit_to_pattern_left(c: char) -> Result<&'static str, CodeError> {
    match c {
        '0' => Ok("0001101"),
        '1' => Ok("0011001"),
        '2' => Ok("0010011"),
        '3' => Ok("0111101"),
        '4' => Ok("0100011"),
        '5' => Ok("0110001"),
        '6' => Ok("0101111"),
        '7' => Ok("0111011"),
        '8' => Ok("0110111"),
        '9' => Ok("0001011"),
        _ => Err(CodeError::invalid_input_char(c)),
    }
}

fn digit_to_pattern_right(c: char) -> Result<&'static str, CodeError> {
    match c {
        '0' => Ok("1110010"),
        '1' => Ok("1100110"),
        '2' => Ok("1101100"),
        '3' => Ok("1000010"),
        '4' => Ok("1011100"),
        '5' => Ok("1001110"),
        '6' => Ok("1010000"),
        '7' => Ok("1000100"),
        '8' => Ok("1001000"),
        '9' => Ok("1110100"),
        _ => Err(CodeError::invalid_input_char(c)),
    }
}

impl UPC {
    fn check_digit(text: &str) -> String {
        todo!()
    }
}

impl Code for UPC {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::with_capacity(95);

        out.push_str(GUARD);

        for c in text.chars().take(6) {
            out.push_str(digit_to_pattern_left(c)?)
        }

        out.push_str(MIDDLE);

        for c in text.chars().skip(6).take(6) {
            out.push_str(digit_to_pattern_right(c)?)
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

        todo!()
    }
}

#[cfg(test)]
mod upc_tests {
    use super::*;

    #[test]
    fn encrypt() {}
}
