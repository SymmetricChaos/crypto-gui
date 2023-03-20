use crate::errors::Error;
use lazy_static::lazy_static;
use std::{cell::Cell, collections::HashMap};

use super::Code;

pub const BAUDOT_LETTERS: &'static str = "␀␍␊ QWERTYUIOPASDFGHJKLZXCVBNM␎\0";
pub const BAUDOT_FIGURES: &'static str = "␀␍␊ 1234567890-'␅!&£␇()+/:=?,.\0␏";
pub const BAUDOT_CODES: [&'static str; 32] = [
    "00000", "00010", "01000", "00100", "11101", "11001", "10000", "01010", "00001", "10101",
    "11100", "01100", "00011", "01101", "11000", "10100", "10010", "10110", "01011", "00101",
    "11010", "11110", "01001", "10001", "10111", "01110", "01111", "10011", "00110", "00111",
    "11011", "11111",
];

#[derive(Debug, Copy, Clone)]
pub enum BaudotMode {
    Letters,
    Figures,
}

lazy_static! {
    pub static ref BAUDOT_LETTER_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (letter, code) in BAUDOT_LETTERS.chars().zip(BAUDOT_CODES.iter()) {
            m.insert(letter, *code);
        }
        m
    };
    pub static ref BAUDOT_FIGURE_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (letter, code) in BAUDOT_FIGURES.chars().zip(BAUDOT_CODES.iter()) {
            m.insert(letter, *code);
        }
        m
    };
    pub static ref BAUDOT_LETTER_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (letter, code) in BAUDOT_LETTERS.chars().zip(BAUDOT_CODES.iter()) {
            m.insert(*code, letter);
        }
        m
    };
    pub static ref BAUDOT_FIGURE_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (letter, code) in BAUDOT_FIGURES.chars().zip(BAUDOT_CODES.iter()) {
            m.insert(*code, letter);
        }
        m
    };
}

pub struct Baudot {
    mode: Cell<BaudotMode>, // interior mutability to make encoding and decoding easier
}

impl Baudot {
    // Baudot codes are always five bits
    const WIDTH: usize = 5;

    pub fn switch_mode(&self) {
        match self.mode.get() {
            BaudotMode::Letters => todo!(),
            BaudotMode::Figures => todo!(),
        }
    }

    pub fn letters_codes(&self) -> Box<dyn Iterator<Item = (char, &&str)> + '_> {
        Box::new(
            BAUDOT_LETTERS
                .chars()
                .map(|x| (x, BAUDOT_LETTER_MAP.get(&x).unwrap())),
        )
    }

    pub fn figures_codes(&self) -> Box<dyn Iterator<Item = (char, &&str)> + '_> {
        Box::new(
            BAUDOT_FIGURES
                .chars()
                .map(|x| (x, BAUDOT_FIGURE_MAP.get(&x).unwrap())),
        )
    }

    pub fn map(&self, k: &char) -> Option<&&str> {
        match self.mode.get() {
            BaudotMode::Letters => BAUDOT_LETTER_MAP.get(k),
            BaudotMode::Figures => BAUDOT_FIGURE_MAP.get(k),
        }
    }

    pub fn map_inv(&self, k: &str) -> Option<&char> {
        match self.mode.get() {
            BaudotMode::Letters => BAUDOT_LETTER_MAP_INV.get(k),
            BaudotMode::Figures => BAUDOT_FIGURE_MAP_INV.get(k),
        }
    }
}

impl Default for Baudot {
    fn default() -> Self {
        Baudot {
            mode: Cell::new(BaudotMode::Letters),
        }
    }
}

impl Code for Baudot {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len() * Self::WIDTH);
        for s in text.chars() {
            match self.map(&s) {
                Some(code_group) => out.push_str(code_group),
                None => {
                    return Err(Error::Input(format!(
                        "The symbol `{}` is not in the Baudot alphabet",
                        s
                    )))
                }
            }
            match s {
                '␎' => {
                    self.mode.replace(BaudotMode::Figures);
                }
                '␏' => {
                    self.mode.replace(BaudotMode::Letters);
                }
                _ => (),
            };
        }
        self.mode.replace(BaudotMode::Letters);
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len() / Self::WIDTH);
        for p in 0..(text.len() / Self::WIDTH) {
            let group = &text[(p * Self::WIDTH)..(p * Self::WIDTH) + Self::WIDTH];
            match self.map_inv(&group.to_string()) {
                Some(code_group) => out.push(*code_group),
                None => {
                    return Err(Error::Input(format!(
                        "The code group `{}` is not valid",
                        group
                    )))
                }
            }
            match group {
                "11011" => {
                    self.mode.replace(BaudotMode::Figures);
                }
                "11111" => {
                    self.mode.replace(BaudotMode::Letters);
                }
                _ => (),
            };
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod baudot_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXCOSTS␎£572␏WHILEONSALE";
    const CIPHERTEXT: &'static str = "0000100101100001110111100011000111011110100110101000011110010011010110000111011101110000111010000001101001101100101000011110011001111111100100101011000100110000000110011010100110000100110000";

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
