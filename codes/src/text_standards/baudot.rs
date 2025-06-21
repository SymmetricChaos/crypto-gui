use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use std::ops::Not;
use utils::text_functions::{chunk_and_join, string_chunks};

const WIDTH: usize = 5;

// pub const ITA1_EU_LETTERS: &'static str = "␀AEÉYUIO␎JGHBCFD -XZSTWV␡KMLRQNP";
// pub const ITA1_EU_FIGURES: &'static str = "␀12⅟34";
// pub const US_TTY_FIGURES: &'static str = "␀3␊- ␇87␍$4',!:(5\")2#6019?&␎./;␏";
pub const ITA2_LETTERS: &'static str = "␀E␊A SIU␍DRJNFCKTZLWHYPQOBG␎MXV␏";
pub const ITA2_FIGURES: &'static str = "␀3␊- '87␍␅4␇,!:(5+)2£6019?&␎./=␏";

pub const CODES: [&'static str; 32] = [
    "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
    "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010", "10011",
    "10100", "10101", "10110", "10111", "11000", "11001", "11010", "11011", "11100", "11101",
    "11110", "11111",
];

crate::lazy_bimap!(
    ITA2_LETTER_MAP: BiMap<char, &'static str> = ITA2_LETTERS.chars().zip(CODES.into_iter());
    ITA2_FIGURE_MAP: BiMap<char, &'static str> = ITA2_FIGURES.chars().zip(CODES.into_iter());
);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BaudotMode {
    Letters,
    Figures,
}

impl Not for BaudotMode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BaudotMode::Letters => BaudotMode::Figures,
            BaudotMode::Figures => BaudotMode::Letters,
        }
    }
}

impl BaudotMode {
    /// Change from Letters to Figures or from Figure to Letters
    fn toggle(&mut self) {
        *self = !*self;
    }

    fn to_figures(&mut self) {
        *self = BaudotMode::Figures;
    }

    fn to_letters(&mut self) {
        *self = BaudotMode::Letters;
    }

    fn is_figures(&self) -> bool {
        *self == BaudotMode::Figures
    }

    fn is_letters(&self) -> bool {
        *self == BaudotMode::Letters
    }

    /// For whatever mode is chosen return to code that indicates a switch to the other
    fn shift_from_code(&self) -> &str {
        match self {
            BaudotMode::Letters => "11011",
            BaudotMode::Figures => "11111",
        }
    }
}

fn map(k: &char, mode: BaudotMode) -> Option<&str> {
    let map = match mode {
        BaudotMode::Letters => &ITA2_LETTER_MAP,
        BaudotMode::Figures => &ITA2_FIGURE_MAP,
    };
    map.get_by_left(k).cloned()
}

fn map_inv(k: &str, mode: BaudotMode) -> Option<char> {
    let map = match mode {
        BaudotMode::Letters => &ITA2_LETTER_MAP,
        BaudotMode::Figures => &ITA2_FIGURE_MAP,
    };
    map.get_by_right(k).cloned()
}

pub fn encode_ita2(text: &str) -> Result<String, CodeError> {
    let mut mode = BaudotMode::Letters;
    let mut out = String::with_capacity(text.len() * WIDTH);
    for s in text.chars() {
        // Handle explicit use of the Shift Out Unicode symbol
        if s == '␎' {
            out.push_str("11011");
            mode.to_figures();
            continue;
        }
        // Handle explicit use of the Shift In Unicode symbol
        if s == '␏' {
            out.push_str("11111");
            mode.to_letters();
            continue;
        }
        // Based on implementation from GCHQ the space is always coded as a letter
        if mode == BaudotMode::Figures && s == ' ' {
            out.push_str("11111");
            out.push_str("00100");
            mode.to_letters();
            continue;
        }
        match map(&s, mode) {
            Some(code_group) => out.push_str(code_group),
            None => match map(&s, !mode) {
                Some(code_group) => {
                    out.push_str(mode.shift_from_code());
                    out.push_str(code_group);
                    mode = !mode;
                }
                None => return Err(CodeError::invalid_input_char(s)),
            },
        }
    }
    Ok(out)
}

pub fn decode_ita2(text: &str) -> Result<String, CodeError> {
    let mut mode = BaudotMode::Letters;
    let mut out = String::with_capacity(text.len() / WIDTH);
    for group in string_chunks(
        &text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        WIDTH,
    ) {
        // Note that repeated shifts of the same kind are the same as a single shift, so an input that repeats a shift code for error correct is handled correctly by thi
        if group == "11011" {
            mode.to_figures();
            continue;
        }
        if group == "11111" {
            mode.to_letters();
            continue;
        }
        match map_inv(&group, mode) {
            Some(code_group) => out.push(code_group),
            None => {
                return Err(CodeError::Input(format!(
                    "The code group `{}` is not valid in ITA2",
                    group
                )))
            }
        }
    }

    Ok(out)
}

pub struct Baudot {
    pub spaced: bool,
}

impl Baudot {
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
        Box::new(CODES.into_iter().map(|code| {
            (
                code,
                format!(
                    "{} {}",
                    ITA2_LETTER_MAP.get_by_right(code).unwrap(),
                    ITA2_FIGURE_MAP.get_by_right(code).unwrap()
                ),
            )
        }))
    }
}

impl Default for Baudot {
    fn default() -> Self {
        Baudot { spaced: false }
    }
}

impl Code for Baudot {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let out = encode_ita2(text)?;

        if self.spaced {
            Ok(chunk_and_join(&out, WIDTH, ' '))
        } else {
            Ok(out)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        Ok(decode_ita2(text)?)
    }
}

#[cfg(test)]
mod baudot_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXCOSTS£572WHILEONSALE";
    const CODETEXT: &'static str = "1000010100000011011100111001100111001111110010101011000100110110001101110001110101110110000010110000001011101110100100000011110011111111001110100001101001000001110000110000101000111001000001";

    #[test]
    #[ignore = "visual correctness check"]
    fn ita2_pairs() {
        for (letter, code) in ITA2_LETTERS.chars().zip(CODES) {
            println!("{letter} {code}")
        }
    }

    #[test]
    fn encode_test() {
        let code = Baudot::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CODETEXT);
    }

    #[test]
    fn decode_test() {
        let code = Baudot::default();
        assert_eq!(code.decode(CODETEXT).unwrap(), PLAINTEXT);
    }
}
