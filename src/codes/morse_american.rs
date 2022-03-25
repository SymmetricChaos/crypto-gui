use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::errors::CodeError;
 
use super::Code;
 

lazy_static! {
    pub static ref LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ&1234567890,.?!";
    pub static ref AMERICAN_CODES: [&'static str; 41] = ["·-", "-···", "·· ·", "-··", "·", 
                                                         "·-·", "--·", "····", "··", 
                                                         "-·-·", "-·-", "⸺", "--", "-·", 
                                                         "· ·", "·····", "··-·", "··-·", "· ··", 
                                                         "-", "··-", "···-", "·--", "·-··", 
                                                         "·· ··", "··· ·", "· ···", "·--·", "··-··", 
                                                         "···-·", "····-","---", "······", "--··", 
                                                         "-····", "-··-", "⸻", "·-·-", "··--··", 
                                                         "-··-·", "---·"];
    pub static ref AMERICAN_CODES_BINARY: [&'static str; 41] = ["101110", "1110101010", "1010010", "11101010", "10", "10111010", "1110111010", "10101010", "1010", "111010111010", "111010111", "11111", "11101110", "111010", "10010", "1010101010", "1010111010", "1010111010", "1001010", "1110", "10101110", "1010101110", "1011101110", "1011101010", "101001010", "101010010", "100101010", "101110111010", "101011101010", "101010111010", "101010101110","111011101110", "101010101010", "111011101010", "111010101010", "111010101110", "1111111", "101110101110", "1010111011101010", "11101010111010", "11101110111010"];

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

pub enum MorseMode {
    DitDah,
    Binary
}

pub struct MorseAmerican {
    mode: MorseMode
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
                None => return Err(CodeError::Input("Unknown character".into()))
            }
        }
        Ok(out.join("  "))
    }
 
    fn encode_binary(&self, text: &str) -> Result<String,CodeError> {
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match AMERICAN_MAP_BINARY.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::Input("Unknown character".into()))
            }
        }
        Ok(out.join("000"))
    }
 
    fn decode_ditdah(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::new();
        for s in text.split("  ") {
            match AMERICAN_MAP_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::Input("Unknown code group".into()))
            }
        }
        Ok(out)
    }
 
    fn decode_binary(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::new();
        for s in text.split("000") {
            match AMERICAN_MAP_BINARY_INV.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::Input("Unknown code group".into()))
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