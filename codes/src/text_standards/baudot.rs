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
pub const GCHQ: &'static str = "/E3A9SIU4DRJNFCKTZLWHYPQOBG5MXV8"; // I only know of this mapping from the GCHQ code chef

/// LSB right
pub const CODES_R: [&'static str; 32] = [
    "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
    "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010", "10011",
    "10100", "10101", "10110", "10111", "11000", "11001", "11010", "11011", "11100", "11101",
    "11110", "11111",
];

/// LSB left
const CODES_L: [&'static str; 32] = [
    "00000", "10000", "01000", "11000", "00100", "10100", "01100", "11100", "00010", "10010",
    "01010", "11010", "00110", "10110", "01110", "11110", "00001", "10001", "01001", "11001",
    "00101", "10101", "01101", "11101", "00011", "10011", "01011", "11011", "00111", "10111",
    "01111", "11111",
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitOrder {
    LsbR,
    LsbL,
}

crate::lazy_bimap!(
    ITA2_LETTER_MAP_R: BiMap<char, &'static str> = ITA2_LETTERS.chars().zip(CODES_R.into_iter());
    ITA2_FIGURE_MAP_R: BiMap<char, &'static str> = ITA2_FIGURES.chars().zip(CODES_R.into_iter());
    GCHQ_MAP_R: BiMap<char, &'static str> = GCHQ.chars().zip(CODES_R.into_iter());
    ITA2_LETTER_MAP_L: BiMap<char, &'static str> = ITA2_LETTERS.chars().zip(CODES_L.into_iter());
    ITA2_FIGURE_MAP_L: BiMap<char, &'static str> = ITA2_FIGURES.chars().zip(CODES_L.into_iter());
    GCHQ_MAP_L: BiMap<char, &'static str> = GCHQ.chars().zip(CODES_L.into_iter());
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
    pub fn toggle(&mut self) {
        *self = !*self;
    }

    pub fn to_figures(&mut self) {
        *self = BaudotMode::Figures;
    }

    pub fn to_letters(&mut self) {
        *self = BaudotMode::Letters;
    }

    pub fn is_figures(&self) -> bool {
        *self == BaudotMode::Figures
    }

    pub fn is_letters(&self) -> bool {
        *self == BaudotMode::Letters
    }

    /// For whatever mode is chosen return to code that indicates a switch to the other
    pub fn shift_from_code(&self) -> &str {
        match self {
            BaudotMode::Letters => "11011",
            BaudotMode::Figures => "11111",
        }
    }
}

fn map_r(k: &char, mode: BaudotMode) -> Option<&str> {
    let map = match mode {
        BaudotMode::Letters => &ITA2_LETTER_MAP_R,
        BaudotMode::Figures => &ITA2_FIGURE_MAP_R,
    };
    map.get_by_left(k).cloned()
}

fn map_r_inv(k: &str, mode: BaudotMode) -> Option<char> {
    let map = match mode {
        BaudotMode::Letters => &ITA2_LETTER_MAP_R,
        BaudotMode::Figures => &ITA2_FIGURE_MAP_R,
    };
    map.get_by_right(k).cloned()
}

fn map_r_inv_gchq(k: &str) -> Option<&char> {
    GCHQ_MAP_R.get_by_right(k)
}

fn map_l(k: &char, mode: BaudotMode) -> Option<&str> {
    let map = match mode {
        BaudotMode::Letters => &ITA2_LETTER_MAP_L,
        BaudotMode::Figures => &ITA2_FIGURE_MAP_L,
    };
    map.get_by_left(k).cloned()
}

fn map_l_inv(k: &str, mode: BaudotMode) -> Option<char> {
    let map = match mode {
        BaudotMode::Letters => &ITA2_LETTER_MAP_L,
        BaudotMode::Figures => &ITA2_FIGURE_MAP_L,
    };
    map.get_by_right(k).cloned()
}

fn map_l_inv_gchq(k: &str) -> Option<&char> {
    GCHQ_MAP_L.get_by_right(k)
}

pub fn encode_ita2(text: &str, bit_order: BitOrder) -> Result<String, CodeError> {
    let mut text = text.to_string();
    text = text.replace("\\0", "␀");
    text = text.replace("\\r", "␍");
    text = text.replace("\\n", "␊");
    text = text.replace("\\e", "␅");
    text = text.replace("\\a", "␇");
    text = text.replace("\0", "␀");
    text = text.replace("\r", "␍");
    text = text.replace("\n", "␊");
    text = text.to_ascii_uppercase();
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
        if bit_order == BitOrder::LsbR {
            match map_r(&s, mode) {
                Some(code_group) => out.push_str(code_group),
                None => match map_r(&s, !mode) {
                    Some(code_group) => {
                        out.push_str(mode.shift_from_code());
                        out.push_str(code_group);
                        mode = !mode;
                    }
                    None => return Err(CodeError::invalid_input_char(s)),
                },
            }
        } else {
            match map_l(&s, mode) {
                Some(code_group) => out.push_str(code_group),
                None => match map_l(&s, !mode) {
                    Some(code_group) => {
                        out.push_str(mode.shift_from_code());
                        out.push_str(code_group);
                        mode = !mode;
                    }
                    None => return Err(CodeError::invalid_input_char(s)),
                },
            }
        }
    }
    Ok(out)
}

pub fn decode_ita2(text: &str, bit_order: BitOrder) -> Result<String, CodeError> {
    let mut mode = BaudotMode::Letters;
    let mut out = String::with_capacity(text.len() / WIDTH);
    for group in string_chunks(
        &text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        WIDTH,
    ) {
        // Note that repeated shifts of the same kind are the same as a single shift, so an input that repeats a shift code for error correction is handled correctly
        if group == "11011" {
            mode.to_figures();
            continue;
        }
        if group == "11111" {
            mode.to_letters();
            continue;
        }
        match bit_order {
            BitOrder::LsbR => match map_r_inv(&group, mode) {
                Some(code_group) => out.push(code_group),
                None => {
                    return Err(CodeError::Input(format!(
                        "The code group `{}` is not valid in ITA2",
                        group
                    )))
                }
            },
            BitOrder::LsbL => match map_l_inv(&group, mode) {
                Some(code_group) => out.push(code_group),
                None => {
                    return Err(CodeError::Input(format!(
                        "The code group `{}` is not valid in ITA2",
                        group
                    )))
                }
            },
        }
    }

    Ok(out)
}

fn decode_ita2_gchq(text: &str, bit_order: BitOrder) -> Result<String, CodeError> {
    let mut mode = BaudotMode::Letters;
    let mut out = String::with_capacity(text.len() / WIDTH);
    for group in string_chunks(
        &text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        WIDTH,
    ) {
        // Note that repeated shifts of the same kind are the same as a single shift, so an input that repeats a shift code for error correction is handled correctly
        if group == "11011" {
            mode.to_figures();
            continue;
        }
        if group == "11111" {
            mode.to_letters();
            continue;
        }
        match bit_order {
            BitOrder::LsbR => match map_r_inv_gchq(&group) {
                Some(code_group) => out.push(*code_group),
                None => {
                    return Err(CodeError::Input(format!(
                        "The code group `{}` is not valid in ITA2",
                        group
                    )))
                }
            },
            BitOrder::LsbL => match map_l_inv_gchq(&group) {
                Some(code_group) => out.push(*code_group),
                None => {
                    return Err(CodeError::Input(format!(
                        "The code group `{}` is not valid in ITA2",
                        group
                    )))
                }
            },
        }
    }

    Ok(out)
}

pub struct Baudot {
    pub spaced: bool,
    pub alt_decode: bool,
    pub bit_order: BitOrder,
}

impl Default for Baudot {
    fn default() -> Self {
        Baudot {
            spaced: false,
            alt_decode: false,
            bit_order: BitOrder::LsbR,
        }
    }
}

impl Baudot {
    pub fn codes_chars(&self) -> Box<dyn Iterator<Item = (&str, String)> + '_> {
        match self.bit_order {
            BitOrder::LsbR => Box::new(CODES_R.into_iter().map(|code| {
                (
                    code,
                    format!(
                        "{} {}",
                        ITA2_LETTER_MAP_R.get_by_right(code).unwrap(),
                        ITA2_FIGURE_MAP_R.get_by_right(code).unwrap()
                    ),
                )
            })),
            BitOrder::LsbL => Box::new(CODES_L.into_iter().map(|code| {
                (
                    code,
                    format!(
                        "{} {}",
                        ITA2_LETTER_MAP_L.get_by_right(code).unwrap(),
                        ITA2_FIGURE_MAP_L.get_by_right(code).unwrap()
                    ),
                )
            })),
        }
    }
}

impl Code for Baudot {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let out = encode_ita2(text, self.bit_order)?;

        if self.spaced {
            Ok(chunk_and_join(&out, WIDTH, ' '))
        } else {
            Ok(out)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        if self.alt_decode {
            Ok(decode_ita2_gchq(text, self.bit_order)?)
        } else {
            Ok(decode_ita2(text, self.bit_order)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXCOSTS£572WHILEONSALE";
    const CODETEXT: &'static str = "1000010100000011011100111001100111001111110010101011000100110110001101110001110101110110000010110000001011101110100100000011110011111111001110100001101001000001110000110000101000111001000001";

    #[test]
    #[ignore = "visual correctness check"]
    fn ita2_pairs() {
        for (letter, code) in ITA2_LETTERS.chars().zip(CODES_R) {
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
