use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cell::Cell;
use utils::text_functions::{bimap_from_iter, chunk_and_join, string_chunks};

use crate::{errors::CodeError, traits::Code};

pub const ITA1_LETTERS: &'static str = "␀␍␊ QWERTYUIOPASDFGHJKLZXCVBNM␎␏";
pub const ITA1_FIGURES: &'static str = "␀␍␊ 1234567890-'␅!&£␇()+/:=?,.␎␏";
pub const ITA2_LETTERS: &'static str = "␀␍␊ QWERTYUIOPASDFGHJKLZXCVBNM␎␏";
pub const ITA2_FIGURES: &'static str = "␀␍␊ 1234567890-'␅!&£␇()+/:=?,.␎␏";
pub const MTK_LETTERS: &'static str = "␀␍␊ QWERTYUIOPASDFGHJKLZXCVBNM␎␏";
pub const MTK_FIGURES: &'static str = "␑␍␊ 1234567890-'ЧЭШЩЮ()+/:=?,.␒␓";
pub const MTK_CYRILLIC: &'static str = "␑␍␊ ЯВЕPТЫУИОПАСДФГХЙКЛЗЬЦЖБНМ␒␓";
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
    pub static ref ITA1_LETTER_MAP: BiMap<char, &'static str> =
        bimap_from_iter(ITA1_LETTERS.chars().zip(GRAY_CODES.into_iter()));
    pub static ref ITA1_FIGURE_MAP: BiMap<char, &'static str> =
        bimap_from_iter(ITA1_FIGURES.chars().zip(GRAY_CODES.into_iter()));
    pub static ref ITA2_LETTER_MAP: BiMap<char, &'static str> =
        bimap_from_iter(ITA2_LETTERS.chars().zip(GRAY_CODES.into_iter()));
    pub static ref ITA2_FIGURE_MAP: BiMap<char, &'static str> =
        bimap_from_iter(ITA2_FIGURES.chars().zip(GRAY_CODES.into_iter()));
    pub static ref US_TTY_FIGURE_MAP: BiMap<char, &'static str> =
        bimap_from_iter(US_TTY_FIGURES.chars().zip(GRAY_CODES.into_iter()));
}

pub struct Baudot {
    pub version: BaudotVersion,
    pub spaced: bool,
}

impl Baudot {
    // Baudot codes are always five bits
    const WIDTH: usize = 5;

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

    pub fn codes_chars(&self) -> Box<dyn Iterator<Item = (&str, String)> + '_> {
        Box::new(GRAY_CODES.into_iter().map(|code| {
            (
                code,
                format!(
                    "{} {}",
                    self.letter_map().get_by_right(code).unwrap(),
                    self.figure_map().get_by_right(code).unwrap()
                ),
            )
        }))
    }

    pub fn figure_map(&self) -> &BiMap<char, &str> {
        match self.version {
            BaudotVersion::Ita1 => &ITA1_FIGURE_MAP,
            BaudotVersion::Ita2 => &ITA2_FIGURE_MAP,
            BaudotVersion::UsTty => &US_TTY_FIGURE_MAP,
        }
    }

    pub fn letter_map(&self) -> &BiMap<char, &str> {
        match self.version {
            BaudotVersion::Ita1 => &ITA1_LETTER_MAP,
            BaudotVersion::Ita2 => &ITA2_LETTER_MAP,
            BaudotVersion::UsTty => &ITA2_LETTER_MAP,
        }
    }

    pub fn map(&self, k: &char, mode: &BaudotMode) -> Option<&&str> {
        match self.version {
            BaudotVersion::Ita1 => match mode {
                BaudotMode::Letters => ITA1_LETTER_MAP.get_by_left(k),
                BaudotMode::Figures => ITA1_FIGURE_MAP.get_by_left(k),
            },
            BaudotVersion::Ita2 => match mode {
                BaudotMode::Letters => ITA2_LETTER_MAP.get_by_left(k),
                BaudotMode::Figures => ITA2_FIGURE_MAP.get_by_left(k),
            },
            BaudotVersion::UsTty => match mode {
                BaudotMode::Letters => ITA2_LETTER_MAP.get_by_left(k),
                BaudotMode::Figures => US_TTY_FIGURE_MAP.get_by_left(k),
            },
        }
    }

    pub fn map_inv(&self, k: &str, mode: &BaudotMode) -> Option<&char> {
        match self.version {
            BaudotVersion::Ita1 => match mode {
                BaudotMode::Letters => ITA1_LETTER_MAP.get_by_right(k),
                BaudotMode::Figures => ITA1_FIGURE_MAP.get_by_right(k),
            },
            BaudotVersion::Ita2 => match mode {
                BaudotMode::Letters => ITA2_LETTER_MAP.get_by_right(k),
                BaudotMode::Figures => ITA2_FIGURE_MAP.get_by_right(k),
            },
            BaudotVersion::UsTty => match mode {
                BaudotMode::Letters => ITA2_LETTER_MAP.get_by_right(k),
                BaudotMode::Figures => US_TTY_FIGURE_MAP.get_by_right(k),
            },
        }
    }
}

impl Default for Baudot {
    fn default() -> Self {
        Baudot {
            version: BaudotVersion::Ita2,
            spaced: false,
        }
    }
}

impl Code for Baudot {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        // Always start in letter mode
        let mut mode = BaudotMode::Letters;

        let mut out = String::with_capacity(text.len() * Self::WIDTH);
        for s in text.chars() {
            match self.map(&s, &mode) {
                Some(code_group) => out.push_str(code_group),
                None => return Err(CodeError::invalid_input_char(s)),
            }
            match s {
                '␎' => mode = BaudotMode::Figures,
                '␏' => mode = BaudotMode::Letters,
                _ => (),
            };
        }

        if self.spaced {
            Ok(chunk_and_join(&out, Self::WIDTH, ' '))
        } else {
            Ok(out)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        // Always start in letter mode
        let mut mode = BaudotMode::Letters;

        let mut out = String::with_capacity(text.len() / Self::WIDTH);
        for group in string_chunks(&text.replace(' ', ""), Self::WIDTH) {
            match self.map_inv(&group, &mode) {
                Some(code_group) => out.push(*code_group),
                None => {
                    return Err(CodeError::Input(format!(
                        "The code group `{}` is not valid",
                        group
                    )))
                }
            }
            match group.as_str() {
                "11011" => mode = BaudotMode::Figures,
                "11111" => mode = BaudotMode::Letters,
                _ => (),
            };
        }

        Ok(out)
    }
}

#[cfg(test)]
mod baudot_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXCOSTS␎£572␏WHILEONSALE";
    const CIPHERTEXT: &'static str = "0000100101100001110111100011000111011110100110101000011110010011010110000111011101110000111010000001101001101100101000011110011001111111100100101011000100110000000110011010100110000100110000";

    #[test]
    #[ignore = "visual correctness check"]
    fn ita2_pairs() {
        for (letter, code) in ITA2_LETTERS.chars().zip(GRAY_CODES) {
            println!("{letter} {code}")
        }
    }

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
