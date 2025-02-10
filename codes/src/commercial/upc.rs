use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use lazy_static::lazy_static;
use regex::Regex;
use utils::text_functions::bimap_from_iter;

const GUARD: &'static str = "101"; // Start and End guard pattern
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

pub fn is_valid_upc_a(digits: &str) -> bool {
    if !UPCA_DIGITS.is_match(digits) {
        return false;
    }
    let coefs = [3, 1].into_iter().cycle();
    let mut check = 0;
    for (d, co) in digits.chars().zip(coefs) {
        match d.to_digit(10) {
            Some(d) => check += d * co,
            None => return false,
        }
    }
    check % 10 == 0
}

pub fn upc_a_check_digit(digits: &str) -> Result<char, CodeError> {
    if digits.is_ascii() && digits.len() == 11 {
        let coefs = [3, 1].into_iter().cycle();
        let mut check = 0;
        for (d, co) in digits.chars().zip(coefs) {
            match d.to_digit(10) {
                Some(d) => check += d * co,
                None => return Err(CodeError::invalid_input_char(d)),
            }
        }
        match check % 10 {
            0 => Ok('0'),
            1 => Ok('9'),
            2 => Ok('8'),
            3 => Ok('7'),
            4 => Ok('6'),
            5 => Ok('5'),
            6 => Ok('4'),
            7 => Ok('3'),
            8 => Ok('2'),
            9 => Ok('1'),
            _ => unreachable!("an integer modulo 10 is between 0 and 9"),
        }
    } else {
        return Err(CodeError::input(
            "a UPC-A check digit can only be calculated for 11 digits",
        ));
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

        if is_valid_upc_a(&out) {
            Ok(out)
        } else {
            Err(CodeError::input("check digit is incorrect"))
        }
    }
}

#[cfg(test)]
mod upc_tests {
    use super::*;

    #[test]
    fn check_digit() {
        assert_eq!(upc_a_check_digit("03600029145").unwrap(), '2')
    }

    #[test]
    fn encode() {
        let code = Upc::default();
        assert_eq!(code.encode("036000291452").unwrap(), "10100011010111101010111100011010001101000110101010110110011101001100110101110010011101101100101");
    }

    #[test]
    fn decode() {
        let code = Upc::default();
        assert_eq!(code.decode("10100011010111101010111100011010001101000110101010110110011101001100110101110010011101101100101").unwrap(), "036000291452");
    }
}
