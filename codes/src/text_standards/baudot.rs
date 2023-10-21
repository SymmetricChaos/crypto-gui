use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cell::Cell;
use utils::text_functions::{bimap_from_iter, chunk_and_join, string_chunks};

use crate::{errors::CodeError, traits::Code};

pub const ITA1_LETTERS: &'static str = "␀␍␊ QWERTYUIOPASDFGHJKLZXCVBNM␎␏";
pub const ITA1_FIGURES: &'static str = "␀␍␊ 1234567890-'␅!&£␇()+/:=?,.␎␏";
pub const ITA2_LETTERS: &'static str = "␀E␊A SIU␍DRJNFCKTZLWHYPQOBG␎MXV␏";
pub const ITA2_FIGURES: &'static str = "␀3␊- '87␍␅4␇,!:(5+)2£6019?&␎./=␏";
pub const US_TTY_FIGURES: &'static str = "␀3␊- ␇87␍$4',!:(5\")2#6019?&␎./;␏";

pub const GRAY_CODES: [&'static str; 32] = [
    "00000", "00010", "01000", "00100", "11101", "11001", "10000", "01010", "00001", "10101",
    "11100", "01100", "00011", "01101", "11000", "10100", "10010", "10110", "01011", "00101",
    "11010", "11110", "01001", "10001", "10111", "01110", "01111", "10011", "00110", "00111",
    "11011", "11111",
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BaudotMode {
    Letters,
    Figures,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BaudotVersion {
    Ita1,
    Ita2,
    UsTty,
}

lazy_static! {
    pub static ref FIVE_BIT_CODES: Vec<String> =
        (0..32).map(|n| format!("{:05b}", n)).collect_vec();
    pub static ref ITA1_LETTER_MAP: BiMap<char, String> =
        bimap_from_iter(ITA1_LETTERS.chars().zip(FIVE_BIT_CODES.iter().cloned()));
    pub static ref ITA1_FIGURE_MAP: BiMap<char, String> =
        bimap_from_iter(ITA1_FIGURES.chars().zip(FIVE_BIT_CODES.iter().cloned()));
    pub static ref ITA2_LETTER_MAP: BiMap<char, String> =
        bimap_from_iter(ITA2_LETTERS.chars().zip(FIVE_BIT_CODES.iter().cloned()));
    pub static ref ITA2_FIGURE_MAP: BiMap<char, String> =
        bimap_from_iter(ITA2_FIGURES.chars().zip(FIVE_BIT_CODES.iter().cloned()));
    pub static ref US_TTY_FIGURE_MAP: BiMap<char, String> =
        bimap_from_iter(US_TTY_FIGURES.chars().zip(FIVE_BIT_CODES.iter().cloned()));
}

pub struct Baudot {
    mode: Cell<BaudotMode>, // interior mutability to make encoding and decoding easier
    pub version: BaudotVersion,
    pub spaced: bool,
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

    // pub fn letters_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
    //     Box::new(
    //         ITA2_LETTERS
    //             .chars()
    //             .map(|x| (x, ITA2_LETTER_MAP.get_by_left(&x).unwrap())),
    //     )
    // }

    // pub fn figures_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
    //     Box::new(
    //         ITA2_FIGURES
    //             .chars()
    //             .map(|x| (x, ITA2_FIGURE_MAP.get_by_left(&x).unwrap())),
    //     )
    // }

    pub fn codes_chars(&self) -> Box<dyn Iterator<Item = (&&str, String)> + '_> {
        Box::new(GRAY_CODES.iter().map(|code| {
            (
                code,
                format!(
                    "{} {}",
                    self.letter_map().get_by_right(&code.to_string()).unwrap(),
                    self.figure_map().get_by_right(&code.to_string()).unwrap()
                ),
            )
        }))
    }

    pub fn figure_map(&self) -> &BiMap<char, String> {
        match self.version {
            BaudotVersion::Ita1 => &ITA1_FIGURE_MAP,
            BaudotVersion::Ita2 => &ITA2_FIGURE_MAP,
            BaudotVersion::UsTty => &US_TTY_FIGURE_MAP,
        }
    }

    pub fn letter_map(&self) -> &BiMap<char, String> {
        match self.version {
            BaudotVersion::Ita1 => &ITA1_LETTER_MAP,
            BaudotVersion::Ita2 => &ITA2_LETTER_MAP,
            BaudotVersion::UsTty => &ITA2_LETTER_MAP,
        }
    }

    pub fn map(&self, k: &char) -> Option<&String> {
        match self.version {
            BaudotVersion::Ita1 => match self.mode.get() {
                BaudotMode::Letters => ITA1_LETTER_MAP.get_by_left(k),
                BaudotMode::Figures => ITA1_FIGURE_MAP.get_by_left(k),
            },
            BaudotVersion::Ita2 => match self.mode.get() {
                BaudotMode::Letters => ITA2_LETTER_MAP.get_by_left(k),
                BaudotMode::Figures => ITA2_FIGURE_MAP.get_by_left(k),
            },
            BaudotVersion::UsTty => match self.mode.get() {
                BaudotMode::Letters => ITA2_LETTER_MAP.get_by_left(k),
                BaudotMode::Figures => US_TTY_FIGURE_MAP.get_by_left(k),
            },
        }
    }

    pub fn map_inv(&self, k: &str) -> Option<&char> {
        match self.version {
            BaudotVersion::Ita1 => match self.mode.get() {
                BaudotMode::Letters => ITA1_LETTER_MAP.get_by_right(k),
                BaudotMode::Figures => ITA1_FIGURE_MAP.get_by_right(k),
            },
            BaudotVersion::Ita2 => match self.mode.get() {
                BaudotMode::Letters => ITA2_LETTER_MAP.get_by_right(k),
                BaudotMode::Figures => ITA2_FIGURE_MAP.get_by_right(k),
            },
            BaudotVersion::UsTty => match self.mode.get() {
                BaudotMode::Letters => ITA2_LETTER_MAP.get_by_right(k),
                BaudotMode::Figures => US_TTY_FIGURE_MAP.get_by_right(k),
            },
        }
    }
}

impl Default for Baudot {
    fn default() -> Self {
        Baudot {
            mode: Cell::new(BaudotMode::Letters),
            version: BaudotVersion::Ita2,
            spaced: false,
        }
    }
}

impl Code for Baudot {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        // Always start in letter mode
        self.letter_shift();

        let mut out = String::with_capacity(text.len() * Self::WIDTH);
        for s in text.chars() {
            match self.map(&s) {
                Some(code_group) => out.push_str(code_group),
                None => return Err(CodeError::invalid_input_char(s)),
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
        // Always return to letter mode
        self.letter_shift();

        if self.spaced {
            Ok(chunk_and_join(&out, Self::WIDTH, ' '))
        } else {
            Ok(out)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        // Always start in letter mode
        self.letter_shift();

        let mut out = String::with_capacity(text.len() / Self::WIDTH);
        for group in string_chunks(&text.replace(' ', ""), Self::WIDTH) {
            match self.map_inv(&group) {
                Some(code_group) => out.push(*code_group),
                None => {
                    return Err(CodeError::Input(format!(
                        "The code group `{}` is not valid",
                        group
                    )))
                }
            }
            match group.as_str() {
                "11011" => {
                    self.figure_shift();
                }
                "11111" => {
                    self.letter_shift();
                }
                _ => (),
            };
        }

        // Always return to letter mode
        self.letter_shift();

        Ok(out)
    }
}

#[cfg(test)]
mod baudot_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXCOSTS␎£572␏WHILEONSALE";
    const CIPHERTEXT: &'static str = "1000010100000011011100111001100111001111110010101011000100110110001101110001110101110110000010110000001011101110100100000011110011111111001110100001101001000001110000110000101000111001000001";

    #[test]
    fn encode_test() {
        let code = Baudot::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decode_test() {
        let code = Baudot::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
