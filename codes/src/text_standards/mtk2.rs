use crate::traits::Code;
use bimap::BiMap;
use itertools::izip;
use utils::{
    errors::GeneralError,
    text_functions::{chunk_and_join, string_chunks},
};

pub const MTK_LETTERS: &'static str = "␑␍␊ QWERTYUIOPASDFGHJKLZXCVBNM␒␓";
pub const MTK_FIGURES: &'static str = "␑␍␊ 1234567890-'ЧЭШЩЮ()+/:=?,.␒␓";
pub const MTK_CYRILLIC: &'static str = "␑␍␊ ЯВЕPТЫУИОПАСДФГХЙКЛЗЬЦЖБНМ␒␓";

pub const CODES: [&'static str; 32] = [
    "00000", "00010", "01000", "00100", "11101", "11001", "10000", "01010", "00001", "10101",
    "11100", "01100", "00011", "01101", "11000", "10100", "10010", "10110", "01011", "00101",
    "11010", "11110", "01001", "10001", "10111", "01110", "01111", "10011", "00110", "00111",
    "11011", "11111",
];

crate::lazy_bimap!(
    LETTER_MAP: BiMap<char, &'static str> =
        MTK_LETTERS.chars().zip(CODES.into_iter());
    FIGURE_MAP: BiMap<char, &'static str> =
        MTK_FIGURES.chars().zip(CODES.into_iter());
    CYRILLIC_MAP: BiMap<char, &'static str> =
        MTK_CYRILLIC.chars().zip(CODES.into_iter());
);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mtk2Mode {
    Letters,
    Figures,
    Cyrillic,
}

pub struct Mtk2 {
    pub spaced: bool,
}

impl Mtk2 {
    const WIDTH: usize = 5;

    pub fn codes_chars(&self) -> Box<dyn Iterator<Item = (&str, String)> + '_> {
        Box::new(
            izip!(
                CODES,
                MTK_LETTERS.chars(),
                MTK_FIGURES.chars(),
                MTK_CYRILLIC.chars()
            )
            .map(|(a, b, c, d)| (a, format!("{} {} {}", b, c, d))),
        )
    }

    pub fn map(c: char, mode: &Mtk2Mode) -> Option<&&str> {
        match mode {
            Mtk2Mode::Letters => LETTER_MAP.get_by_left(&c),
            Mtk2Mode::Figures => FIGURE_MAP.get_by_left(&c),
            Mtk2Mode::Cyrillic => CYRILLIC_MAP.get_by_left(&c),
        }
    }

    pub fn map_inv(s: &str, mode: &Mtk2Mode) -> Option<&'static char> {
        match mode {
            Mtk2Mode::Letters => LETTER_MAP.get_by_right(s),
            Mtk2Mode::Figures => FIGURE_MAP.get_by_right(s),
            Mtk2Mode::Cyrillic => CYRILLIC_MAP.get_by_right(s),
        }
    }
}

impl Default for Mtk2 {
    fn default() -> Self {
        Mtk2 { spaced: false }
    }
}

impl Code for Mtk2 {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        // Always start in letter mode
        let mut mode = Mtk2Mode::Letters;

        let mut out = String::with_capacity(text.len() * Self::WIDTH);
        for s in text.chars() {
            match Self::map(s, &mode) {
                Some(code_group) => out.push_str(code_group),
                None => return Err(GeneralError::invalid_input_char(s)),
            }
            match s {
                '␑' => mode = Mtk2Mode::Cyrillic,
                '␒' => mode = Mtk2Mode::Figures,
                '␓' => mode = Mtk2Mode::Letters,
                _ => (),
            };
        }

        if self.spaced {
            Ok(chunk_and_join(&out, Self::WIDTH, ' '))
        } else {
            Ok(out)
        }
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        // Always start in letter mode
        let mut mode = Mtk2Mode::Letters;

        let mut out = String::with_capacity(text.len() / Self::WIDTH);
        for group in string_chunks(&text.replace(' ', ""), Self::WIDTH) {
            match Self::map_inv(&group, &mode) {
                Some(code_group) => out.push(*code_group),
                None => {
                    return Err(GeneralError::input(format!(
                        "The code group `{}` is not valid",
                        group
                    )))
                }
            }
            match group.as_str() {
                "00000" => mode = Mtk2Mode::Cyrillic,
                "11011" => mode = Mtk2Mode::Figures,
                "11111" => mode = Mtk2Mode::Letters,
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
    const CODETEXT: &'static str = "0000100101100001110111100011000111011110100110101000011110010011010110000111011101110000111010000001101001101100101000011110011001111111100100101011000100110000000110011010100110000100110000";

    #[test]
    #[ignore = "visual correctness check"]
    fn ita2_pairs() {
        for (letter, code) in MTK_CYRILLIC.chars().zip(CODES) {
            println!("{letter} {code}")
        }
    }

    #[test]
    fn encode_test() {
        let code = Mtk2::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CODETEXT);
    }

    #[test]
    fn decode_test() {
        let code = Mtk2::default();
        assert_eq!(code.decode(CODETEXT).unwrap(), PLAINTEXT);
    }
}
