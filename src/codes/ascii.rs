use crate::{errors::Error, text_aux::PresetAlphabet::Ascii128};
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Code;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AsciiMode {
    SevenBit,
    EightBit,
}

impl AsciiMode {
    pub fn width(&self) -> usize {
        match self {
            AsciiMode::SevenBit => 7,
            AsciiMode::EightBit => 8,
        }
    }

    pub fn map(&self) -> HashMap<char, &'static String> {
        match self {
            AsciiMode::SevenBit => ASCII_MAP7.clone(),
            AsciiMode::EightBit => ASCII_MAP8.clone(),
        }
    }

    pub fn map_inv(&self) -> HashMap<&'static String, char> {
        match self {
            AsciiMode::SevenBit => ASCII_MAP_INV7.clone(),
            AsciiMode::EightBit => ASCII_MAP_INV8.clone(),
        }
    }
}

lazy_static! {
    pub static ref SEVEN_BIT_ASCII_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(128);
        for n in 0..128 {
            v.push(format!("{:07b}", n))
        }
        v
    };
    pub static ref EIGHT_BIT_ASCII_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(128);
        for n in 0..128 {
            v.push(format!("{:08b}", n))
        }
        v
    };
    pub static ref ASCII_MAP8: HashMap<char, &'static String> = {
        let mut m = HashMap::new();
        for (letter, code) in Ascii128.chars().zip(EIGHT_BIT_ASCII_CODES.iter()) {
            m.insert(letter, code);
        }
        m
    };
    pub static ref ASCII_MAP_INV8: HashMap<&'static String, char> = {
        let mut m = HashMap::new();
        for (letter, code) in Ascii128.chars().zip(EIGHT_BIT_ASCII_CODES.iter()) {
            m.insert(code, letter);
        }
        m
    };
    pub static ref ASCII_MAP7: HashMap<char, &'static String> = {
        let mut m = HashMap::new();
        for (letter, code) in Ascii128.chars().zip(SEVEN_BIT_ASCII_CODES.iter()) {
            m.insert(letter, code);
        }
        m
    };
    pub static ref ASCII_MAP_INV7: HashMap<&'static String, char> = {
        let mut m = HashMap::new();
        for (letter, code) in Ascii128.chars().zip(SEVEN_BIT_ASCII_CODES.iter()) {
            m.insert(code, letter);
        }
        m
    };
}

pub struct Ascii {
    pub mode: AsciiMode,
    alphabet: &'static str,
}

impl Ascii {
    pub fn input_set(&self) -> &'static str {
        self.alphabet
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        match self.mode {
            AsciiMode::SevenBit => Box::new(
                self.alphabet
                    .chars()
                    .map(|x| (x, *ASCII_MAP7.get(&x).unwrap())),
            ),
            AsciiMode::EightBit => Box::new(
                self.alphabet
                    .chars()
                    .map(|x| (x, *ASCII_MAP8.get(&x).unwrap())),
            ),
        }
    }
}

impl Default for Ascii {
    fn default() -> Self {
        Ascii {
            mode: AsciiMode::EightBit,
            alphabet: Ascii128.slice(),
        }
    }
}

impl Code for Ascii {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let w = self.mode.width();
        let map = self.mode.map();
        let mut out = String::with_capacity(text.chars().count() * w);
        for s in text.chars() {
            match map.get(&s) {
                Some(code_group) => out.push_str(code_group),
                None => {
                    return Err(Error::Input(format!(
                        "The symbol `{}` is not in the ASCII alphabet",
                        s
                    )))
                }
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let w = self.mode.width();
        let map_inv = self.mode.map_inv();
        let mut out = String::with_capacity(text.chars().count() / w);
        for p in 0..(text.len() / w) {
            let group = &text[(p * w)..(p * w) + w];
            match map_inv.get(&group.to_string()) {
                Some(code_group) => out.push(*code_group),
                None => {
                    return Err(Error::Input(format!(
                        "The code group `{}` is not valid",
                        group
                    )))
                }
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod ascii_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "0101010001001000010001010101000101010101010010010100001101001011010000100101001001001111010101110100111001000110010011110101100001001010010101010100110101010000010100110100111101010110010001010101001001010100010010000100010101001100010000010101101001011001010001000100111101000111";

    #[test]
    fn encrypt_test() {
        let code = Ascii::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Ascii::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
