use bimap::BiMap;

use crate::errors::Error;

use super::{morse_encodings::*, Code};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseRep {
    Binary,
    HalfBlock,
    Ascii,
    CdotNDash,
}

impl MorseRep {
    pub fn letter_sep(&self) -> &str {
        match self {
            MorseRep::Binary => "000",
            MorseRep::Ascii => " ",
            MorseRep::CdotNDash => " ",
            MorseRep::HalfBlock => "   ",
        }
    }

    pub fn word_sep(&self) -> &str {
        match self {
            MorseRep::Binary => "0000000",
            MorseRep::Ascii => "   ",
            MorseRep::CdotNDash => "   ",
            MorseRep::HalfBlock => "       ",
        }
    }

    pub fn map(&self, standard: MorseStandard) -> Result<&BiMap<char, &str>, Error> {
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
                        "Only line codes work for American Morse".into(),
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
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &str)> + '_> {
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
                        .zip(std::iter::once("Only line codes work for American Morse")),
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
            if s == ' ' {
                match self.mode {
                    MorseRep::Binary => out.push("0"),
                    MorseRep::HalfBlock => out.push(" "),
                    MorseRep::Ascii => out.push(" "),
                    MorseRep::CdotNDash => out.push(" "),
                }
                continue;
            }
            match map.get_by_left(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_char(s)),
            }
        }
        Ok(out.join(self.mode.letter_sep()))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = Vec::new();
        let mut word_buffer = String::new();
        let map = self.mode.map(self.standard)?;
        for word in text.split(self.mode.word_sep()) {
            for ch in word.split(self.mode.letter_sep()) {
                match map.get_by_right(&ch) {
                    Some(c) => word_buffer.push(*c),
                    None => return Err(Error::invalid_input_group(ch)),
                }
            }
            out.push(word_buffer.to_string());
            word_buffer.clear()
        }

        Ok(out.join(" "))
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod morseitu_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THE QUICK BROWN FOX";
    const CIPHERTEXT_ASCII: &'static str =
        "- .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   ..-. --- -..-";
    const CIPHERTEXT_BINARY: &'static str = "111000101010100010000000111011101011100010101110001010001110101110100011101011100000001110101010001011101000111011101110001011101110001110100000001010111010001110111011100011101010111";

    #[test]
    fn encode_test_binary() {
        let mut code = Morse::default();
        code.mode = MorseRep::Binary;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_BINARY);
        code.mode = MorseRep::Ascii;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_ASCII);
    }

    #[test]
    fn decode_test_binary() {
        let mut code = Morse::default();
        code.mode = MorseRep::Binary;
        assert_eq!(code.decode(CIPHERTEXT_BINARY).unwrap(), PLAINTEXT);
        code.mode = MorseRep::Ascii;
        assert_eq!(code.decode(CIPHERTEXT_ASCII).unwrap(), PLAINTEXT);
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
