use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::errors::CodeError;

use super::Code;

lazy_static! {
    pub static ref LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    pub static ref ITU_CODES: [&'static str; 36] = ["·-", "-···", "-·-·", "-··", "·", "··-·", "--·", "····", "··", "·---", 
                                                "-·-", "·-··", "--", "-·", "---", "·--·", "--·-", "·-·", "···", "-", 
                                                "··-", "···-", "·--", "-··-", "-·--", "--··", "·----", "··---", 
                                                "···--", "····-", "·····", "-····", "--···", "---··", "----·", 
                                                "-----"];
    pub static ref ITU_CODES_BINARY: [&'static str; 36] = ["10111", "111010101", "11101011101", "1110101", "1", "101011101", "111011101", "1010101", "101", "1011101110111", 
                                            "111010111", "101110101", "1110111", "11101", "11101110111", "10111011101", "1110111010111", "1011101", "10101", "111", 
                                            "1010111", "101010111", "101110111", "11101010111", "1110101110111", "11101110101", "10111011101110111", "101011101110111", 
                                            "1010101110111", "10101010111", "101010101", "11101010101", "1110111010101", "111011101110101", "11101110111011101", 
                                            "1110111011101110111"];
 

    pub static ref ITU_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(ITU_CODES.iter()) {
            m.insert(l, *c);
        }
        m
    };

    pub static ref ITU_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(ITU_CODES.iter()) {
            m.insert(*c, l);
        }
        m
    };

    pub static ref ITU_MAP_BINARY: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(ITU_CODES_BINARY.iter()) {
            m.insert(l, *c);
        }
        m
    };

    pub static ref ITU_MAP_BINARY_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(ITU_CODES_BINARY.iter()) {
            m.insert(*c, l);
        }
        m
    };
}
 
pub enum MorseMode {
    DitDah,
    Binary
}

pub struct MorseITU {
    mode: MorseMode
}

impl MorseITU {
    fn _print_mapping(&self) {
        for c in LETTERS.chars() {
            println!("{} {}",c,ITU_MAP.get(&c).unwrap())
        }
    }

    pub fn chars_codes(&self) -> impl Iterator<Item=(char, &str)> {
        match self.mode {
            MorseMode::DitDah => LETTERS.chars().zip(ITU_CODES.iter()).map(|(c,s)| (c,*s)),
            //MorseMode::Binary => LETTERS.chars().zip(ITU_CODES_BINARY.iter()).map(|(c,s)| (c,*s)),
            _ => unreachable!("")
        }
    }
}

impl Default for MorseITU {
    fn default() -> Self {
        Self { mode: MorseMode::DitDah }
    }
}

impl MorseITU {
    fn encode_ditdah(&self, text: &str) -> Result<String,CodeError> {
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match ITU_MAP.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::Input("Unknown character".into()))
            }
        }
        Ok(out.join(" "))
    }

    fn encode_binary(&self, text: &str) -> Result<String,CodeError> {
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match ITU_MAP_BINARY.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::Input("Unknown character".into()))
            }
        }
        Ok(out.join("00"))
    }

    fn decode_ditdah(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::new();
        for s in text.split(" ") {
            match ITU_MAP_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::Input("Unknown code group".into()))
            }
        }
        Ok(out)
    }

    fn decode_binary(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::new();
        for s in text.split("00") {
            match ITU_MAP_BINARY_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::Input("Unknown code group".into()))
            }
        }
        Ok(out)
    }
}

impl Code for MorseITU {
    fn encode(&self, text: &str) -> Result<String,CodeError> {
        match self.mode {
            MorseMode::DitDah => self.encode_ditdah(text),
            MorseMode::Binary => self.encode_binary(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String,CodeError> {
        match self.mode {
            MorseMode::DitDah => self.decode_ditdah(text),
            MorseMode::Binary => self.decode_binary(text),
        }
    }
}


#[cfg(test)]
mod morse_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
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