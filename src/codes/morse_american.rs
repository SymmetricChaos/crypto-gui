use super::Code;
use crate::errors::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;

const LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ&1234567890,.?!";
const AMERICAN_MORSE_CODES: [&'static str; 41] = [
    "10111",
    "111010101",
    "101001",
    "1110101",
    "1",
    "1011101",
    "111011101",
    "1010101",
    "101",
    "11101011101",
    "111010111",
    "11111",
    "1110111",
    "11101",
    "1001",
    "101010101",
    "101011101",
    "10011",
    "10101",
    "111",
    "1010111",
    "101010111",
    "101110111",
    "101110101",
    "1010011",
    "10101001",
    "1001101",
    "10111011101",
    "10101110101",
    "10101011101",
    "10101010111",
    "11101110111",
    "10101010101",
    "11101110101",
    "11101010101",
    "11101010111",
    "1111111",
    "10111010111",
    "101011101110101",
    "1110101011101",
    "1110111011101",
];

lazy_static! {
    pub static ref AMERICAN_MORSE_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l, c) in LETTERS.chars().zip(AMERICAN_MORSE_CODES.iter()) {
            m.insert(l, *c);
        }
        m
    };
    pub static ref AMERICAN_MORSE_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l, c) in LETTERS.chars().zip(AMERICAN_MORSE_CODES.iter()) {
            m.insert(*c, l);
        }
        m
    };
}

pub struct MorseAmerican {
    _line_on: char,
    _line_off: char,
}

impl Default for MorseAmerican {
    fn default() -> Self {
        Self {
            _line_on: '1',
            _line_off: '0',
        }
    }
}

impl MorseAmerican {
    pub fn set_line_on(&mut self) {
        todo!("change the line_on symbol")
    }

    pub fn set_line_off(&mut self) {
        todo!("change the line_off symbol")
    }

    fn _print_mapping(&self) {
        for c in LETTERS.chars() {
            println!("{} {}", c, AMERICAN_MORSE_MAP.get(&c).unwrap())
        }
    }

    pub fn chars_codes(&self) -> impl Iterator<Item = (char, &'static str)> + '_ {
        LETTERS
            .chars()
            .zip(AMERICAN_MORSE_CODES.iter())
            .map(|(c, s)| (c, *s))
    }
}

impl Code for MorseAmerican {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match AMERICAN_MORSE_MAP.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_char(s)),
            }
        }
        Ok(out.join("000"))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        for s in text.split("000") {
            match AMERICAN_MORSE_MAP_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_group(s)),
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod morse_american_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CODETEXT:  &'static str = "1110001010101000100010101110100010101110001010001010010001110101110001110101010001001100010010001011101110001110100010111010001001000101110101000111010111010001010111000111011100010101010100010101000100100010101011100010001001100011100010101010001000111110001011100010101001000101001100011101010001001000111011101";

    #[test]
    fn encrypt_test() {
        let code = MorseAmerican::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CODETEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = MorseAmerican::default();
        assert_eq!(code.decode(CODETEXT).unwrap(), PLAINTEXT);
    }
}
