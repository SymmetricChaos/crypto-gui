use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::{text_types::PresetAlphabet::Ascii128, errors::CodeError};

use super::Code;

lazy_static! {

    pub static ref SEVEN_BIT_ASCII_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(128);
        for n in 0..128 {
            v.push(format!("{:07b}",n))
        }
        v
    };

    pub static ref EIGHT_BIT_ASCII_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(128);
        for n in 0..128 {
            v.push(format!("{:08b}",n))
        }
        v
    };

    pub static ref ASCII_MAP8: HashMap<char, &'static String,> = {
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

    pub static ref ASCII_MAP7: HashMap<char, &'static String,> = {
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
 
 
pub struct ASCII {
    map: HashMap<char, &'static String,>,
    map_inv: HashMap<&'static String, char>,
    width: usize,
    alphabet: &'static str,
}
 
impl ASCII {
 
    pub fn default8() -> ASCII {
        ASCII{ map: ASCII_MAP8.clone(), map_inv: ASCII_MAP_INV8.clone(), width: 8, alphabet: Ascii128.slice() }
    }
 
    pub fn default7() -> ASCII {
        ASCII{ map: ASCII_MAP7.clone(), map_inv: ASCII_MAP_INV7.clone(), width: 7, alphabet: Ascii128.slice() }
    }

    pub fn input_set(&self) -> &'static str {
        self.alphabet
    }
 
}
 
 
impl Code for ASCII {
 
    fn encode(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::with_capacity(text.chars().count()*self.width);
        for s in text.chars() {
            match self.map.get(&s) {
                Some(code_group) => out.push_str(code_group),
                None => return Err(CodeError::Input(format!("The symbol `{}` is not in the ASCII alphabet",s)))
            }
        }
        Ok(out)
    }
 
    fn decode(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::with_capacity(text.chars().count()/self.width);
        let w = self.width;
        for p in 0..(text.len()/w) {
            let group = &text[(p*w)..(p*w)+w];
            match self.map_inv.get(&group.to_string()) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::Input(format!("The code group `{}` is not valid",group)))
            }
        }
        Ok(out)
    }
}





#[cfg(test)]
mod polybius_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "10101001001000100010110100011010101100100110000111001011100001010100101001111101011110011101000110100111110110001001010101010110011011010000101001110011111010110100010110100101010100100100010001011001100100000110110101011001100010010011111000111";

    #[test]
    fn encrypt_test() {
        let code = ASCII::default7();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = ASCII::default7();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
