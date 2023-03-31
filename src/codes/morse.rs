use crate::errors::Error;
use std::collections::HashMap;

use super::{morse_encodings::*, Code};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseRep {
    Binary,
    HalfBlock,
    Ascii,
    CdotNDash,
}

impl MorseRep {
    pub fn dit(&self) -> &str {
        match self {
            MorseRep::Binary => "1",
            MorseRep::Ascii => "-",
            MorseRep::CdotNDash => "–",
            MorseRep::HalfBlock => "▄",
        }
    }

    pub fn dah(&self) -> &str {
        match self {
            MorseRep::Binary => "111",
            MorseRep::Ascii => ".",
            MorseRep::CdotNDash => "·",
            MorseRep::HalfBlock => "▄▄▄",
        }
    }

    pub fn intra_char_sep(&self) -> &str {
        match self {
            MorseRep::Binary => "0",
            MorseRep::Ascii => "",
            MorseRep::CdotNDash => "",
            MorseRep::HalfBlock => " ",
        }
    }

    pub fn letter_sep(&self) -> &str {
        match self {
            MorseRep::Binary => "000",
            MorseRep::Ascii => " ",
            MorseRep::CdotNDash => " ",
            MorseRep::HalfBlock => "   ",
        }
    }

    pub fn map(&self, standard: MorseStandard) -> Result<&HashMap<char, &str>, Error> {
        Ok(match standard {
            MorseStandard::Itu => match self {
                MorseRep::Binary => &ITU_BINARY_MAP,
                MorseRep::HalfBlock => &ITU_HALFBLOCK_MAP,
                MorseRep::Ascii => &ITU_ASCII_MAP,
                MorseRep::CdotNDash => &ITU_DOT_DASH_MAP,
            },
            MorseStandard::American => match self {
                MorseRep::Binary => &AMERICAN_BINARY_MAP,
                MorseRep::HalfBlock => &AMERICAN_HALFBLOCK_MAP,
                MorseRep::Ascii | MorseRep::CdotNDash => {
                    return Err(Error::State(
                        "American Morse only suppots line code representation".into(),
                    ))
                }
            },
        })
    }

    pub fn map_inv(&self, standard: MorseStandard) -> Result<&HashMap<&str, char>, Error> {
        Ok(match standard {
            MorseStandard::Itu => match self {
                MorseRep::Binary => &ITU_BINARY_MAP_INV,
                MorseRep::HalfBlock => &ITU_HALFBLOCK_MAP_INV,
                MorseRep::Ascii => &ITU_ASCII_MAP_INV,
                MorseRep::CdotNDash => &ITU_DOT_DASH_MAP_INV,
            },
            MorseStandard::American => match self {
                MorseRep::Binary => &AMERICAN_BINARY_MAP_INV,
                MorseRep::HalfBlock => &AMERICAN_HALFBLOCK_MAP_INV,
                MorseRep::Ascii | MorseRep::CdotNDash => {
                    return Err(Error::State(
                        "American Morse only suppots line code representation".into(),
                    ))
                }
            },
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseStandard {
    Itu,
    American,
}

pub struct Morse {
    pub mode: MorseRep,
    pub standard: MorseStandard,
}

impl Morse {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &'static str)> + '_> {
        match self.standard {
            MorseStandard::Itu => match self.mode {
                MorseRep::Binary => Box::new(ITU_LETTERS.chars().zip(ITU_BINARY)),
                MorseRep::HalfBlock => Box::new(ITU_LETTERS.chars().zip(ITU_HALFBLOCK)),
                MorseRep::Ascii => Box::new(ITU_LETTERS.chars().zip(ITU_ASCII)),
                MorseRep::CdotNDash => Box::new(ITU_LETTERS.chars().zip(ITU_DOT_DASH)),
            },
            MorseStandard::American => match self.mode {
                MorseRep::Binary => Box::new(AMERICAN_LETTERS.chars().zip(AMERICAN_BINARY)),
                MorseRep::HalfBlock => Box::new(AMERICAN_LETTERS.chars().zip(AMERICAN_HALFBLOCK)),
                MorseRep::Ascii | MorseRep::CdotNDash => Box::new(
                    std::iter::once(' ')
                        .zip(std::iter::once("Only Line codes work for American Morse")),
                ),
            },
        }
    }
}

impl Default for Morse {
    fn default() -> Self {
        Self {
            mode: MorseRep::HalfBlock,
            standard: MorseStandard::Itu,
        }
    }
}

impl Code for Morse {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let map = self.mode.map(self.standard)?;
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match map.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_char(s)),
            }
        }
        Ok(out.join(self.mode.letter_sep()))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        let map = self.mode.map_inv(self.standard)?;
        for s in text.split(self.mode.letter_sep()) {
            match map.get(&s) {
                Some(c) => out.push(*c),
                None => return Err(Error::invalid_input_group(s)),
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod morseitu_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    //const CIPHERTEXT_ASCII: &'static str = "- .... . --.- ..- .. -.-. -.- -... .-. --- .-- -. ..-. --- -..- .--- ..- -- .--. ... --- ...- . .-. - .... . .-.. .- --.. -.-- -.. --- --.";
    const CIPHERTEXT_BINARY: &'static str = "111000101010100010001110111010111000101011100010100011101011101000111010111000111010101000101110100011101110111000101110111000111010001010111010001110111011100011101010111000101110111011100010101110001110111000101110111010001010100011101110111000101010111000100010111010001110001010101000100010111010100010111000111011101010001110101110111000111010100011101110111000111011101";

    #[test]
    fn encode_test_binary() {
        let mut code = Morse::default();
        code.mode = MorseRep::Binary;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_BINARY);
    }

    #[test]
    fn decode_test_binary() {
        let mut code = Morse::default();
        code.mode = MorseRep::Binary;
        assert_eq!(code.decode(CIPHERTEXT_BINARY).unwrap(), PLAINTEXT);
    }

    // #[test]
    // fn encode_test_ascii() {
    //     let mut code = Morse::default();
    //     code.mode = MorseRep::Ascii;
    //     assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_ASCII);
    // }

    // #[test]
    // fn decode_test_ascii() {
    //     let mut code = Morse::default();
    //     code.mode = MorseRep::Ascii;
    //     assert_eq!(code.decode(CIPHERTEXT_ASCII).unwrap(), PLAINTEXT);
    // }
}
