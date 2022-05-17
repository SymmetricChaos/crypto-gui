use crate::errors::CodeError;
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Code;

const BAUDOT_LETTERS: &'static str = "␀␍␊ QWERTYUIOPASDFGHJKLZXCVBNM␎\0";
const BAUDOT_FIGURES: &'static str = "␀␍␊ 1234567890-'␅!&£␇()+/:=?,.\0␏";

pub enum BaudotMode {
    Letters,
    Figures
}

lazy_static! {

    pub static ref BAUDOT_CODES: Vec<String> = {
        let mut v = Vec::with_capacity(32);
        for n in 0..32 {
            v.push(format!("{:05b}", n))
        }
        v
    };


    pub static ref BAUDOT_LETTER_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        for (letter, code) in BAUDOT_LETTERS.chars().zip(BAUDOT_CODES.iter()) {
            m.insert(letter, code.clone());
        }
        m
    };
    pub static ref BAUDOT_FIGURE_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        for (letter, code) in BAUDOT_FIGURES.chars().zip(BAUDOT_CODES.iter()) {
            m.insert(letter, code.clone());
        }
        m
    };


    pub static ref BAUDOT_LETTER_MAP_INV: HashMap<String, char> = {
        let mut m = HashMap::new();
        for (letter, code) in BAUDOT_LETTERS.chars().zip(BAUDOT_CODES.iter()) {
            m.insert(code.clone(), letter);
        }
        m

    };
    pub static ref BAUDOT_FIGURE_MAP_INV: HashMap<String, char> = {
        let mut m = HashMap::new();
        for (letter, code) in BAUDOT_FIGURES.chars().zip(BAUDOT_CODES.iter()) {
            m.insert(code.clone(), letter);
        }
        m
    };
}

pub struct Baudot {
    pub mode: BaudotMode,
}

impl Baudot {
    // Baudot codes are always five bits
    const WIDTH: usize = 5;

    pub fn letters_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        Box::new(
                BAUDOT_LETTERS
                    .chars()
                    .map(|x| (x, BAUDOT_LETTER_MAP.get(&x).unwrap())),
        )
    }

    pub fn figures_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        Box::new(
                BAUDOT_FIGURES
                    .chars()
                    .map(|x| (x, BAUDOT_FIGURE_MAP.get(&x).unwrap())),
        )
    }

    pub fn map(&self, k: &char) -> Option<&String> {
        match self.mode {
            BaudotMode::Letters => BAUDOT_LETTER_MAP.get(k),
            BaudotMode::Figures => BAUDOT_FIGURE_MAP.get(k),
        }
    }

    pub fn map_inv(&self, k: &str) -> Option<&char> {
        match self.mode {
            BaudotMode::Letters => BAUDOT_LETTER_MAP_INV.get(k),
            BaudotMode::Figures => BAUDOT_FIGURE_MAP_INV.get(k),
        }
    }
}

impl Default for Baudot {
    fn default() -> Self {
        Baudot {
            mode: BaudotMode::Letters,
        }
    }
}

impl Code for Baudot {

    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::with_capacity(text.len() * Self::WIDTH);
        for s in text.chars() {
            match self.map(&s) {
                Some(code_group) => out.push_str(code_group),
                None => {
                    return Err(CodeError::Input(format!(
                        "The symbol `{}` is not in the ASCII alphabet",
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
            match self.map_inv(&group.to_string()) {
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
mod baudot_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let code = Baudot::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Baudot::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
