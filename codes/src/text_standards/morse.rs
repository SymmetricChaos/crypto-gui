use super::morse_encodings::*;
use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseStandard {
    Itu,
    American,
    Gerke,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseRep {
    Binary,
    HalfBlock,
    Ascii,
    CdotNDash,
    Word,
}

impl MorseRep {
    pub fn letter_sep(&self) -> &str {
        match self {
            Self::Binary => "000",
            Self::Ascii | Self::CdotNDash => " ",
            Self::HalfBlock => "   ",
            Self::Word => "   ",
        }
    }

    pub fn word_sep(&self) -> &str {
        match self {
            Self::Binary => "0000000",
            Self::Ascii | Self::CdotNDash => "   ",
            Self::HalfBlock => "       ",
            Self::Word => "       ",
        }
    }

    pub fn map(&self, standard: MorseStandard) -> Result<&BiMap<char, &str>, CodeError> {
        Ok(match standard {
            MorseStandard::Itu => match self {
                Self::Binary => &ITU_BINARY_MAP,
                Self::HalfBlock => &ITU_HALFBLOCK_MAP,
                Self::Ascii => &ITU_ASCII_MAP,
                Self::CdotNDash => &ITU_DOT_DASH_MAP,
                Self::Word => &ITU_WORD_MAP,
            },
            MorseStandard::American => match self {
                Self::Binary => &AMERICAN_BINARY_MAP,
                Self::HalfBlock => &AMERICAN_HALFBLOCK_MAP,
                _ => {
                    return Err(CodeError::State(
                        "Only line codes work for American Morse".into(),
                    ))
                }
            },
            MorseStandard::Gerke => match self {
                Self::Binary => &GERKE_BINARY_MAP,
                Self::HalfBlock => &GERKE_HALFBLOCK_MAP,
                _ => {
                    return Err(CodeError::State(
                        "Only line codes work for Gerke's code".into(),
                    ))
                }
            },
        })
    }
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
                MorseRep::Word => Box::new(ITU_LETTERS.chars().zip(ITU_WORD)),
            },
            MorseStandard::American => match self.mode {
                MorseRep::Binary => Box::new(AMERICAN_LETTERS.chars().zip(AMERICAN_BINARY)),
                MorseRep::HalfBlock => Box::new(AMERICAN_LETTERS.chars().zip(AMERICAN_HALFBLOCK)),
                _ => Box::new(
                    std::iter::once(' ')
                        .zip(std::iter::once("Only line codes work for American Morse")),
                ),
            },
            MorseStandard::Gerke => match self.mode {
                MorseRep::Binary => Box::new(GERKE_LETTERS.chars().zip(GERKE_BINARY)),
                MorseRep::HalfBlock => Box::new(GERKE_LETTERS.chars().zip(GERKE_HALFBLOCK)),
                _ => Box::new(
                    std::iter::once(' ')
                        .zip(std::iter::once("Only line codes work for Gerke's code")),
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
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let map = self.mode.map(self.standard)?;
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            if s == ' ' {
                match self.mode {
                    MorseRep::Binary => out.push("0"),
                    _ => out.push(" "),
                }
                continue;
            }
            match map.get_by_left(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(CodeError::invalid_input_char(s)),
            }
        }
        Ok(out.join(self.mode.letter_sep()))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let mut word_buffer = String::new();
        let map = self.mode.map(self.standard)?;
        for word in text.split(self.mode.word_sep()) {
            for ch in word.split(self.mode.letter_sep()) {
                match map.get_by_right(&ch) {
                    Some(c) => word_buffer.push(*c),
                    None => return Err(CodeError::invalid_input_group(ch)),
                }
            }
            out.push(word_buffer.to_string());
            word_buffer.clear()
        }

        Ok(out.join(" "))
    }
}

#[cfg(test)]
mod morseitu_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THE QUICK BROWN FOX";
    const MORSE_ASCII: &'static str =
        "- .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   ..-. --- -..-";
    const MORSE_BINARY: &'static str = "111000101010100010000000111011101011100010101110001010001110101110100011101011100000001110101010001011101000111011101110001011101110001110100000001010111010001110111011100011101010111";
    const MORSE_WORD: &'static str = "dah   di di di dit   dit       dah dah di dah   di di dah   di dit   dah di dah dit   dah di dah       dah di di dit   di dah dit   dah dah dah   di dah dah   dah dit       di di dah dit   dah dah dah   dah di di dah";

    #[test]
    fn encode_test_binary() {
        let mut code = Morse::default();
        code.mode = MorseRep::Binary;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), MORSE_BINARY);
        code.mode = MorseRep::Ascii;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), MORSE_ASCII);
    }

    #[test]
    fn decode_test_binary() {
        let mut code = Morse::default();
        code.mode = MorseRep::Binary;
        assert_eq!(code.decode(MORSE_BINARY).unwrap(), PLAINTEXT);
        code.mode = MorseRep::Ascii;
        assert_eq!(code.decode(MORSE_ASCII).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encode_test_ascii() {
        let mut code = Morse::default();
        code.mode = MorseRep::Ascii;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), MORSE_ASCII);
    }

    #[test]
    fn decode_test_ascii() {
        let mut code = Morse::default();
        code.mode = MorseRep::Ascii;
        assert_eq!(code.decode(MORSE_ASCII).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encode_test_word() {
        let mut code = Morse::default();
        code.mode = MorseRep::Word;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), MORSE_WORD);
    }

    #[test]
    fn decode_test_word() {
        let mut code = Morse::default();
        code.mode = MorseRep::Word;
        assert_eq!(code.decode(MORSE_WORD).unwrap(), PLAINTEXT);
    }
}
