use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::errors::CodeError;
 
use super::Code;
 

lazy_static! {
    pub static ref LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ&1234567890,.?!";
    pub static ref AMERICAN_CODES: [&'static str; 41] = ["·-", "-···", "·· ·", "-··", "·", 
                                                         "·-·", "--·", "····", "··", 
                                                         "-·-·", "-·-", "⸺", "--", "-·", 
                                                         "· ·", "·····", "··-·", "· ··", "···", 
                                                         "-", "··-", "···-", "·--", "·-··", 
                                                         "·· ··", "··· ·", "· ···", "·--·", "··-··", 
                                                         "···-·", "····-","---", "······", "--··", 
                                                         "-····", "-··-", "⸻", "·-·-", "··--··", 
                                                         "-··-·", "---·"];
    pub static ref AMERICAN_CODES_BINARY: [&'static str; 41] = ["10111", "111010101", "101001", "1110101", "1", 
                                                                "1011101", "111011101", "1010101", "101", 
                                                                "11101011101", "111010111", "11111", "1110111", "11101", 
                                                                "1001", "101010101", "101011101", "10011", "10101", 
                                                                "111", "1010111", "101010111", "101110111", "101110101", 
                                                                "1010011", "10101001", "1001101", "10111011101", "10101110101", 
                                                                "10101011101", "10101010111","11101110111", "10101010101", "11101110101", 
                                                                "11101010101", "11101010111", "1111111", "10111010111", "101011101110101", 
                                                                "1110101011101", "1110111011101"];
    pub static ref AMERICAN_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(AMERICAN_CODES.iter()) {
            m.insert(l, *c);
        }
        m
    };

    pub static ref AMERICAN_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(AMERICAN_CODES.iter()) {
            m.insert(*c, l);
        }
        m
    };

    pub static ref AMERICAN_MAP_BINARY: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(AMERICAN_CODES_BINARY.iter()) {
            m.insert(l, *c);
        }
        m
    };

    pub static ref AMERICAN_MAP_BINARY_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(AMERICAN_CODES_BINARY.iter()) {
            m.insert(*c, l);
        }
        m
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MorseMode {
    DitDah,
    Binary
}

pub struct MorseAmerican {
    pub mode: MorseMode
}

impl MorseAmerican {
    fn _print_mapping(&self) {
        for c in LETTERS.chars() {
            println!("{} {}",c,AMERICAN_MAP.get(&c).unwrap())
        }
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item=(char, &'static str)> + '_> {
        match self.mode {
            MorseMode::DitDah => Box::new( LETTERS.chars().zip(AMERICAN_CODES.iter()).map(|(c,s)| (c,*s)) ),
            MorseMode::Binary => Box::new( LETTERS.chars().zip(AMERICAN_CODES_BINARY.iter()).map(|(c,s)| (c,*s)) ),
        }
    }
}

impl Default for MorseAmerican {
    fn default() -> Self {
        Self { mode: MorseMode::DitDah }
    }
}

impl MorseAmerican {
    fn encode_ditdah(&self, text: &str) -> Result<String,CodeError> {
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match AMERICAN_MAP.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::invalid_char(s))
            }
        }
        Ok(out.join("  "))
    }
 
    fn encode_binary(&self, text: &str) -> Result<String,CodeError> {
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match AMERICAN_MAP_BINARY.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::invalid_char(s))
            }
        }
        Ok(out.join("000"))
    }
 
    fn decode_ditdah(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::new();
        for s in text.split("  ") {
            match AMERICAN_MAP_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::invalid_code_group(s))
            }
        }
        Ok(out)
    }
 
    fn decode_binary(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::new();
        for s in text.split("000") {
            match AMERICAN_MAP_BINARY_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::invalid_code_group(s))
            }
        }
        Ok(out)
    }
}

impl Code for MorseAmerican {
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
mod morseamerican_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT_DITDAH: &'static str = "-  ····  ·  ··-·  ··-  ··  ·· ·  -·-  -···  · ··  · ·  ·--  -·  ·-·  · ·  ·-··  -·-·  ··-  --  ·····  ···  · ·  ···-  ·  · ··  -  ····  ·  ⸺  ·-  ··· ·  ·· ··  -··  · ·  --·";
    const CIPHERTEXT_BINARY: &'static str = "1110001010101000100010101110100010101110001010001010010001110101110001110101010001001100010010001011101110001110100010111010001001000101110101000111010111010001010111000111011100010101010100010101000100100010101011100010001001100011100010101010001000111110001011100010101001000101001100011101010001001000111011101";

    #[test]
    fn encrypt_test() {
        let code = MorseAmerican::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_DITDAH);
    }

    #[test]
    fn decrypt_test() {
        let code = MorseAmerican::default();
        assert_eq!(code.decode(CIPHERTEXT_DITDAH).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_binary() {
        let mut code = MorseAmerican::default();
        code.mode = MorseMode::Binary;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_BINARY);
    }

    #[test]
    fn decrypt_test_binary() {
        let mut code = MorseAmerican::default();
        code.mode = MorseMode::Binary;
        assert_eq!(code.decode(CIPHERTEXT_BINARY).unwrap(), PLAINTEXT);
    }
}