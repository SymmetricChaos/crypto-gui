use crate::errors::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Code;

lazy_static! {
    pub static ref LETTERS: &'static str = "ABCDEÉFGHIJKLMNOPQRSTUVWXYZ1234567890.,:?'-/()\"=+@";
    pub static ref ITU_CODES: [&'static str; 50] = [
        "·-",
        "-···",
        "-·-·",
        "-··",
        "·",
        "··-··",
        "··-·",
        "--·",
        "····",
        "··",
        "·---",
        "-·-",
        "·-··",
        "--",
        "-·",
        "---",
        "·--·",
        "--·-",
        "·-·",
        "···",
        "-",
        "··-",
        "···-",
        "·--",
        "-··-",
        "-·--",
        "--··",
        "·----",
        "··---",
        "···--",
        "····-",
        "·····",
        "-····",
        "--···",
        "---··",
        "----·",
        "-----",
        "·-·-·-",
        "--··--",
        "---···",
        "··--··",
        "·---·",
        "-···-",
        "-··-·",
        "-·--·",
        "-·--·-",
        "·-··-·",
        "-···-",
        "·-·-·",
        "·--·-·"
    ];
    pub static ref ITU_CODES_BINARY: [&'static str; 50] = [
        "10111",
        "111010101",
        "11101011101",
        "1110101",
        "1",
        "10101110101",
        "101011101",
        "111011101",
        "1010101",
        "101",
        "1011101110111",
        "111010111",
        "101110101",
        "1110111",
        "11101",
        "11101110111",
        "10111011101",
        "1110111010111",
        "1011101",
        "10101",
        "111",
        "1010111",
        "101010111",
        "101110111",
        "11101010111",
        "1110101110111",
        "11101110101",
        "10111011101110111",
        "101011101110111",
        "1010101110111",
        "10101010111",
        "101010101",
        "11101010101",
        "1110111010101",
        "111011101110101",
        "11101110111011101",
        "1110111011101110111",
        "10111010111010111",
        "1110111010101110111",
        "11101110111010101",
        "101011101110101",
        "101110111011101",
        "1110101010111",
        "1110101011101",
        "111010111011101",
        "1110101110111010111",
        "101110101011101",
        "1110101010111",
        "1011101011101",
        "10111011101011101"
    ];
    pub static ref ITU_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l, c) in LETTERS.chars().zip(ITU_CODES.iter()) {
            m.insert(l, *c);
        }
        m
    };
    pub static ref ITU_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l, c) in LETTERS.chars().zip(ITU_CODES.iter()) {
            m.insert(*c, l);
        }
        m
    };
    pub static ref ITU_MAP_BINARY: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l, c) in LETTERS.chars().zip(ITU_CODES_BINARY.iter()) {
            m.insert(l, *c);
        }
        m
    };
    pub static ref ITU_MAP_BINARY_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l, c) in LETTERS.chars().zip(ITU_CODES_BINARY.iter()) {
            m.insert(*c, l);
        }
        m
    };
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseMode {
    DitDah,
    Binary,
}

pub struct MorseITU {
    pub mode: MorseMode,
}

impl MorseITU {
    fn _print_mapping(&self) {
        for c in LETTERS.chars() {
            println!("{} {}", c, ITU_MAP.get(&c).unwrap())
        }
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &'static str)> + '_> {
        match self.mode {
            MorseMode::DitDah => Box::new(LETTERS.chars().map(|x| (x, *ITU_MAP.get(&x).unwrap()))),
            MorseMode::Binary => Box::new(
                LETTERS
                    .chars()
                    .map(|x| (x, *ITU_MAP_BINARY.get(&x).unwrap())),
            ),
        }
    }
}

impl Default for MorseITU {
    fn default() -> Self {
        Self {
            mode: MorseMode::Binary,
        }
    }
}

impl MorseITU {
    fn encode_ditdah(&self, text: &str) -> Result<String, Error> {
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match ITU_MAP.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_char(s)),
            }
        }
        Ok(out.join(" "))
    }

    fn encode_binary(&self, text: &str) -> Result<String, Error> {
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match ITU_MAP_BINARY.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_char(s)),
            }
        }
        Ok(out.join("00"))
    }

    fn decode_ditdah(&self, text: &str) -> Result<String, Error> {
        let text = text.replace(".", "·");
        let mut out = String::new();
        for s in text.split(" ") {
            match ITU_MAP_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_group(s)),
            }
        }
        Ok(out)
    }

    fn decode_binary(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        for s in text.split("00") {
            match ITU_MAP_BINARY_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_group(s)),
            }
        }
        Ok(out)
    }
}

impl Code for MorseITU {
    fn encode(&self, text: &str) -> Result<String, Error> {
        match self.mode {
            MorseMode::DitDah => self.encode_ditdah(text),
            MorseMode::Binary => self.encode_binary(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        match self.mode {
            MorseMode::DitDah => self.decode_ditdah(text),
            MorseMode::Binary => self.decode_binary(text),
        }
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod morseitu_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT_DITDAH: &'static str = "- ···· · --·- ··- ·· -·-· -·- -··· ·-· --- ·-- -· ··-· --- -··- ·--- ··- -- ·--· ··· --- ···- · ·-· - ···· · ·-·· ·- --·· -·-- -·· --- --·";
    const CIPHERTEXT_BINARY: &'static str = "11100101010100100111011101011100101011100101001110101110100111010111001110101010010111010011101110111001011101110011101001010111010011101110111001110101011100101110111011100101011100111011100101110111010010101001110111011100101010111001001011101001110010101010010010111010100101110011101110101001110101110111001110101001110111011100111011101";

    #[test]
    fn encrypt_test() {
        let code = MorseITU::default();
        //code._print_mapping();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_DITDAH);
    }

    #[test]
    fn decrypt_test() {
        let code = MorseITU::default();
        assert_eq!(code.decode(CIPHERTEXT_DITDAH).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_binary() {
        let mut code = MorseITU::default();
        code.mode = MorseMode::Binary;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_BINARY);
    }

    #[test]
    fn decrypt_test_binary() {
        let mut code = MorseITU::default();
        code.mode = MorseMode::Binary;
        assert_eq!(code.decode(CIPHERTEXT_BINARY).unwrap(), PLAINTEXT);
    }
}
