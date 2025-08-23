use super::morse_encodings::*;
use crate::traits::Code;
use bimap::BiMap;
use pest::{iterators::Pairs, Parser};
use utils::errors::GeneralError;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseStandard {
    Itu,
    American,
    Gerke,
    Greek,
    Russian,
}

#[derive(pest_derive::Parser)]
#[grammar = "text_standards/morse.pest"] // relative to src
pub struct MorseParser;

impl MorseStandard {
    pub fn parse<'a>(&self, text: &'a str) -> Pairs<'a, Rule> {
        match self {
            MorseStandard::Itu => MorseParser::parse(Rule::itu_passage, text).unwrap(),
            MorseStandard::American => MorseParser::parse(Rule::american_passage, text).unwrap(),
            MorseStandard::Gerke => MorseParser::parse(Rule::gerke_passage, text).unwrap(),
            MorseStandard::Greek => MorseParser::parse(Rule::greek_passage, text).unwrap(),
            MorseStandard::Russian => MorseParser::parse(Rule::russian_passage, text).unwrap(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseRep {
    HalfBlock,
    Ascii,
    Word,
}

impl MorseRep {
    pub fn letter_sep(&self) -> &str {
        match self {
            Self::Ascii => " ",
            Self::HalfBlock => "   ",
            Self::Word => "   ",
        }
    }

    pub fn word_sep(&self) -> &str {
        match self {
            Self::Ascii => "   ",
            Self::HalfBlock => "       ",
            Self::Word => "       ",
        }
    }

    pub fn map(&self, standard: MorseStandard) -> Result<&BiMap<&str, &str>, GeneralError> {
        Ok(match standard {
            MorseStandard::Itu => match self {
                Self::HalfBlock => &ITU_HALFBLOCK_MAP,
                Self::Ascii => &ITU_ASCII_MAP,
                Self::Word => &ITU_WORD_MAP,
            },
            MorseStandard::American => match self {
                Self::HalfBlock => &AMERICAN_HALFBLOCK_MAP,
                _ => {
                    return Err(GeneralError::state(
                        "Only line codes work for American Morse",
                    ))
                }
            },
            MorseStandard::Gerke => match self {
                Self::HalfBlock => &GERKE_HALFBLOCK_MAP,
                _ => return Err(GeneralError::state("Only line codes work for Gerke's code")),
            },
            MorseStandard::Greek => match self {
                Self::HalfBlock => &GREEK_HALFBLOCK_MAP,
                Self::Ascii => &GREEK_ASCII_MAP,
                Self::Word => &GREEK_WORD_MAP,
            },
            MorseStandard::Russian => match self {
                Self::HalfBlock => &RUSSIAN_HALFBLOCK_MAP,
                Self::Ascii => &RUSSIAN_ASCII_MAP,
                Self::Word => &RUSSIAN_WORD_MAP,
            },
        })
    }
}

pub struct Morse {
    pub representation: MorseRep,
    pub standard: MorseStandard,
}

impl Morse {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        match self.standard {
            MorseStandard::Itu => match self.representation {
                MorseRep::HalfBlock => Box::new(ITU_SIGNS.into_iter().zip(ITU_HALFBLOCK)),
                MorseRep::Ascii => Box::new(ITU_SIGNS.into_iter().zip(ITU_ASCII)),
                MorseRep::Word => Box::new(ITU_SIGNS.into_iter().zip(ITU_WORD)),
            },
            MorseStandard::Greek => match self.representation {
                MorseRep::HalfBlock => Box::new(GREEK_SIGNS.into_iter().zip(GREEK_HALFBLOCK)),
                MorseRep::Ascii => Box::new(GREEK_SIGNS.into_iter().zip(GREEK_ASCII)),
                MorseRep::Word => Box::new(GREEK_SIGNS.into_iter().zip(GREEK_WORD)),
            },
            MorseStandard::American => match self.representation {
                MorseRep::HalfBlock => {
                    Box::new(AMERICAN_LETTERS.into_iter().zip(AMERICAN_HALFBLOCK))
                }
                _ => Box::new(
                    std::iter::once("")
                        .zip(std::iter::once("Only line codes work for American Morse")),
                ),
            },
            MorseStandard::Gerke => match self.representation {
                MorseRep::HalfBlock => Box::new(GERKE_LETTERS.into_iter().zip(GERKE_HALFBLOCK)),
                _ => Box::new(
                    std::iter::once("")
                        .zip(std::iter::once("Only line codes work for Gerke's code")),
                ),
            },
            MorseStandard::Russian => match self.representation {
                MorseRep::HalfBlock => Box::new(RUSSIAN_LETTERS.into_iter().zip(RUSSIAN_HALFBLOCK)),
                MorseRep::Ascii => Box::new(RUSSIAN_LETTERS.into_iter().zip(RUSSIAN_ASCII)),
                MorseRep::Word => Box::new(RUSSIAN_LETTERS.into_iter().zip(RUSSIAN_WORD)),
            },
        }
    }
}

impl Default for Morse {
    fn default() -> Self {
        Self {
            representation: MorseRep::HalfBlock,
            standard: MorseStandard::Itu,
        }
    }
}

impl Code for Morse {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        // Specific rules for ITU transmissions
        let filtered = if self.standard == MorseStandard::Itu {
            let mut filtered = text.replace("×", "x");
            filtered = filtered.replace("′", "'");
            filtered = filtered.replace("%", "0/0");
            filtered = filtered.replace("‰", "0/00");
            filtered.to_uppercase()
        } else {
            text.to_uppercase()
        };

        let map = self.representation.map(self.standard)?;
        let mut out = Vec::new();
        for pair in self.standard.parse(&filtered).flatten() {
            match pair.as_rule() {
                Rule::unknown => return Err(GeneralError::invalid_input_group(pair.as_str())),
                Rule::itu_sign
                | Rule::gerke_sign
                | Rule::american_sign
                | Rule::greek_sign
                | Rule::russian_sign => match map.get_by_left(pair.as_str()) {
                    Some(s) => out.push(*s),
                    None => return Err(GeneralError::invalid_input_group(pair.as_str())),
                },
                Rule::space => out.push(" "),
                _ => (),
            }
        }

        Ok(out.join(self.representation.letter_sep()))
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();
        let mut word_buffer = String::new();
        let map = self.representation.map(self.standard)?;
        for word in text.split(self.representation.word_sep()) {
            for ch in word.split(self.representation.letter_sep()) {
                match map.get_by_right(&ch) {
                    Some(s) => word_buffer.push_str(s),
                    None => return Err(GeneralError::invalid_input_group(ch)),
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
    const MORSE_WORD: &'static str = "dah   di di di dit   dit       dah dah di dah   di di dah   di dit   dah di dah dit   dah di dah       dah di di dit   di dah dit   dah dah dah   di dah dah   dah dit       di di dah dit   dah dah dah   dah di di dah";

    // fn visualize_tree(pairs: Pairs<'_, Rule>, space: String) {
    //     for pair in pairs.into_iter() {
    //         println!("{space}{:?}({})", pair.as_rule(), pair.as_str());
    //         visualize_tree(pair.into_inner(), format!("{space} "))
    //     }
    // }

    #[test]
    fn encode_test_ascii() {
        let mut code = Morse::default();
        code.representation = MorseRep::Ascii;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), MORSE_ASCII);
    }

    #[test]
    fn decode_test_ascii() {
        let mut code = Morse::default();
        code.representation = MorseRep::Ascii;
        assert_eq!(code.decode(MORSE_ASCII).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encode_test_word() {
        let mut code = Morse::default();
        code.representation = MorseRep::Word;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), MORSE_WORD);
    }

    #[test]
    fn decode_test_word() {
        let mut code = Morse::default();
        code.representation = MorseRep::Word;
        assert_eq!(code.decode(MORSE_WORD).unwrap(), PLAINTEXT);
    }
}
