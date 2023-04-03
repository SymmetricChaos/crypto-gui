use crate::{
    errors::Error,
    text_aux::text_functions::{bimap_from_iter, chunk_and_join},
};
use bimap::BiMap;
use lazy_static::lazy_static;
use std::cell::Cell;

use super::Code;

// ITA2
pub const ITA2_LETTERS: &'static str = "␀␍␊ QWERTYUIOPASDFGHJKLZXCVBNM␎␏";
pub const ITA2_FIGURES: &'static str = "␀␍␊ 1234567890-'␅!&£␇()+/:=?,.␎␏";
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
    pub static ref BAUDOT_LETTER_BIMAP: BiMap<char, &'static str> =
        bimap_from_iter(ITA2_LETTERS.chars().zip(BAUDOT_CODES.iter().copied()));
    pub static ref BAUDOT_FIGURE_BIMAP: BiMap<char, &'static str> =
        bimap_from_iter(ITA2_FIGURES.chars().zip(BAUDOT_CODES.iter().copied()));
}

pub struct Baudot {
    mode: Cell<BaudotMode>, // interior mutability to make encoding and decoding easier
}

impl Baudot {
    // Baudot codes are always five bits
    const WIDTH: usize = 5;

    pub fn letter_shift(&self) {
        self.mode.set(BaudotMode::Letters)
    }

    pub fn figure_shift(&self) {
        self.mode.set(BaudotMode::Figures)
    }

    pub fn letters_codes(&self) -> Box<dyn Iterator<Item = (char, &&str)> + '_> {
        Box::new(
            ITA2_LETTERS
                .chars()
                .map(|x| (x, BAUDOT_LETTER_BIMAP.get_by_left(&x).unwrap())),
        )
    }

    pub fn figures_codes(&self) -> Box<dyn Iterator<Item = (char, &&str)> + '_> {
        Box::new(
            ITA2_FIGURES
                .chars()
                .map(|x| (x, BAUDOT_FIGURE_BIMAP.get_by_left(&x).unwrap())),
        )
    }

    pub fn codes_chars(&self) -> Box<dyn Iterator<Item = (&&str, String)> + '_> {
        Box::new(BAUDOT_CODES.iter().map(|code| {
            (
                code,
                format!(
                    "{} {}",
                    BAUDOT_LETTER_BIMAP.get_by_right(code).unwrap(),
                    BAUDOT_FIGURE_BIMAP.get_by_right(code).unwrap()
                ),
            )
        }))
    }

    pub fn map(&self, k: &char) -> Option<&&str> {
        match self.mode.get() {
            BaudotMode::Letters => BAUDOT_LETTER_BIMAP.get_by_left(k),
            BaudotMode::Figures => BAUDOT_FIGURE_BIMAP.get_by_left(k),
        }
    }

    pub fn map_inv(&self, k: &str) -> Option<&char> {
        match self.mode.get() {
            BaudotMode::Letters => BAUDOT_LETTER_BIMAP.get_by_right(k),
            BaudotMode::Figures => BAUDOT_FIGURE_BIMAP.get_by_right(k),
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
        // Always start in letter mode
        self.letter_shift();

        let mut out = String::with_capacity(text.len() * Self::WIDTH);
        for s in text.chars() {
            match self.map(&s) {
                Some(code_group) => out.push_str(code_group),
                None => return Err(Error::invalid_input_char(s)),
            }
            match s {
                '␎' => {
                    self.figure_shift();
                }
                '␏' => {
                    self.letter_shift();
                }
                _ => (),
            };
        }
        self.mode.replace(BaudotMode::Letters);
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        // Always start in letter mode
        self.letter_shift();

        let mut out = String::with_capacity(text.len() / Self::WIDTH);
        for group in chunk_and_join(text, Self::WIDTH, ' ').split(' ') {
            match self.map_inv(&group) {
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
                    self.figure_shift();
                }
                "11111" => {
                    self.letter_shift();
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
