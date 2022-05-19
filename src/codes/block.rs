use crate::{errors::CodeError, text_aux::PresetAlphabet::BasicLatin};
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Code;

lazy_static! {
    pub static ref FOUR_BIT_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(16);
        for n in 0..16 {
            v.push(format!("{:04b}", n))
        }
        v
    };

    pub static ref FIVE_BIT_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(32);
        for n in 0..32 {
            v.push(format!("{:05b}", n))
        }
        v
    };

    pub static ref SIX_BIT_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(64);
        for n in 0..64 {
            v.push(format!("{:06b}", n))
        }
        v
    };

    pub static ref SEVEN_BIT_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(128);
        for n in 0..64 {
            v.push(format!("{:07b}", n))
        }
        v
    };

    pub static ref EIGHT_BIT_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(256);
        for n in 0..64 {
            v.push(format!("{:08b}", n))
        }
        v
    };
    
}

pub struct BlockCode {
    width: u8, // 
    symbols: String, // enforce comma seperated values
}

impl Default for BlockCode {
    fn default() -> Self {
        BlockCode {
            width: 5,
            symbols: String::new(),
        }
    }
}

impl BlockCode {
    pub fn assign_width(&mut self, width: u8) {
        if width > 3 && width <= 8 {
            self.width = width
        }
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        Box::new(BasicLatin
            .chars()
            .map(|x| (x, *BLOCK_CODE_MAP.get(&x).unwrap())))
    }
}

impl Code for BlockCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {

        let mut out = String::with_capacity(text.len() * Self::WIDTH);
        for s in text.chars() {
            match BlockCode_MAP.get(&s) {
                Some(code_group) => out.push_str(code_group),
                None => {
                    return Err(CodeError::Input(format!(
                        "The symbol `{}` is not valid",
                        s
                    )))
                }
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {

        let mut out = String::with_capacity(text.len() / Self::WIDTH);
        for p in 0..(text.len() / Self::WIDTH) {
            let group = &text[(p * Self::WIDTH)..(p * Self::WIDTH) + Self::WIDTH];
            match BlockCode_MAP_INV.get(&group.to_string()) {
                Some(code_group) => out.push(*code_group),
                None => {
                    return Err(CodeError::Input(format!(
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
mod block_code_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let code = BlockCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = BlockCode::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
