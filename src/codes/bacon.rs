use crate::{errors::Error, text_aux::PresetAlphabet::BasicLatin};
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Code;

lazy_static! {
    pub static ref FIVE_BIT_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(32);
        for n in 0..32 {
            v.push(format!("{:05b}", n))
        }
        v
    };
    pub static ref BACON_MAP: HashMap<char, &'static String> = {
        let mut m = HashMap::new();
        for (letter, code) in BasicLatin.chars().zip(FIVE_BIT_CODES.iter()) {
            m.insert(letter, code);
        }
        m
    };
    pub static ref BACON_MAP_INV: HashMap<&'static String, char> = {
        let mut m = HashMap::new();
        for (letter, code) in BasicLatin.chars().zip(FIVE_BIT_CODES.iter()) {
            m.insert(code, letter);
        }
        m
    };
}

pub struct Bacon {
    pub false_text: String,
}

impl Default for Bacon {
    fn default() -> Self {
        Bacon {
            false_text: String::new(),
        }
    }
}

impl Bacon {
    const WIDTH: usize = 5;

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        Box::new(BasicLatin.chars().map(|x| (x, *BACON_MAP.get(&x).unwrap())))
    }
}

impl Code for Bacon {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len() * Self::WIDTH);
        for s in text.chars() {
            match BACON_MAP.get(&s) {
                Some(code_group) => out.push_str(code_group),
                None => return Err(Error::Input(format!("The symbol `{}` is not valid", s))),
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len() / Self::WIDTH);
        for p in 0..(text.len() / Self::WIDTH) {
            let group = &text[(p * Self::WIDTH)..(p * Self::WIDTH) + Self::WIDTH];
            match BACON_MAP_INV.get(&group.to_string()) {
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

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod bacon_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "1001100111001001000010100010000001001010000011000101110101100110100101011101011101001101000110001111100100111010101001001000110011001110010001011000001100111000000110111000110";

    #[test]
    fn encrypt_test() {
        let code = Bacon::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Bacon::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
