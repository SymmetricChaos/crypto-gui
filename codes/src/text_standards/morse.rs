use super::morse_encodings::*;
use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use pest::{iterators::Pairs, Parser};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseStandard {
    Itu,
    American,
    Gerke,
    Greek,
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
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseRep {
    Binary,
    HalfBlock,
    Ascii,
    Word,
}

impl MorseRep {
    pub fn letter_sep(&self) -> &str {
        match self {
            Self::Binary => "000",
            Self::Ascii => " ",
            Self::HalfBlock => "   ",
            Self::Word => "   ",
        }
    }

    pub fn word_sep(&self) -> &str {
        match self {
            Self::Binary => "0000000",
            Self::Ascii => "   ",
            Self::HalfBlock => "       ",
            Self::Word => "       ",
        }
    }

    pub fn map(&self, standard: MorseStandard) -> Result<&BiMap<&str, &str>, CodeError> {
        Ok(match standard {
            MorseStandard::Itu => match self {
                Self::Binary => &ITU_BINARY_MAP,
                Self::HalfBlock => &ITU_HALFBLOCK_MAP,
                Self::Ascii => &ITU_ASCII_MAP,
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
            MorseStandard::Greek => match self {
                Self::Binary => &GREEK_BINARY_MAP,
                Self::HalfBlock => &GREEK_HALFBLOCK_MAP,
                Self::Ascii => &GREEK_ASCII_MAP,
                Self::Word => &GREEK_WORD_MAP,
            },
        })
    }

    // pub fn map(&self, standard: MorseStandard) -> Result<&BiMap<&str, &str>, CodeError> {
    //     Ok(match standard {
    //         MorseStandard::Itu => match self {
    //             Self::Binary => &ITU_BINARY_MAP,
    //             Self::HalfBlock => &ITU_HALFBLOCK_MAP,
    //             Self::Ascii => &ITU_ASCII_MAP,
    //             Self::Word => &ITU_WORD_MAP,
    //         },
    //         MorseStandard::American => match self {
    //             Self::Binary => &AMERICAN_BINARY_MAP,
    //             Self::HalfBlock => &AMERICAN_HALFBLOCK_MAP,
    //             _ => {
    //                 return Err(CodeError::State(
    //                     "Only line codes work for American Morse".into(),
    //                 ))
    //             }
    //         },
    //         MorseStandard::Gerke => match self {
    //             Self::Binary => &GERKE_BINARY_MAP,
    //             Self::HalfBlock => &GERKE_HALFBLOCK_MAP,
    //             _ => {
    //                 return Err(CodeError::State(
    //                     "Only line codes work for Gerke's code".into(),
    //                 ))
    //             }
    //         },
    //         MorseStandard::Greek => match self {
    //             Self::Binary => &GREEK_BINARY_MAP,
    //             Self::HalfBlock => &GREEK_HALFBLOCK_MAP,
    //             Self::Ascii => &GREEK_ASCII_MAP,
    //             Self::Word => &GREEK_WORD_MAP,
    //         },
    //     })
    // }
}

pub struct Morse {
    pub representation: MorseRep,
    pub standard: MorseStandard,
}

impl Morse {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        match self.standard {
            MorseStandard::Itu => match self.representation {
                MorseRep::Binary => Box::new(ITU_SIGNS.into_iter().zip(ITU_BINARY)),
                MorseRep::HalfBlock => Box::new(ITU_SIGNS.into_iter().zip(ITU_HALFBLOCK)),
                MorseRep::Ascii => Box::new(ITU_SIGNS.into_iter().zip(ITU_ASCII)),
                MorseRep::Word => Box::new(ITU_SIGNS.into_iter().zip(ITU_WORD)),
            },
            MorseStandard::Greek => match self.representation {
                MorseRep::Binary => Box::new(GREEK_SIGNS.into_iter().zip(GREEK_BINARY)),
                MorseRep::HalfBlock => Box::new(GREEK_SIGNS.into_iter().zip(GREEK_HALFBLOCK)),
                MorseRep::Ascii => Box::new(GREEK_SIGNS.into_iter().zip(GREEK_ASCII)),
                MorseRep::Word => Box::new(GREEK_SIGNS.into_iter().zip(GREEK_WORD)),
            },
            MorseStandard::American => match self.representation {
                MorseRep::Binary => Box::new(AMERICAN_LETTERS.into_iter().zip(AMERICAN_BINARY)),
                MorseRep::HalfBlock => {
                    Box::new(AMERICAN_LETTERS.into_iter().zip(AMERICAN_HALFBLOCK))
                }
                _ => Box::new(
                    std::iter::once("")
                        .zip(std::iter::once("Only line codes work for American Morse")),
                ),
            },
            MorseStandard::Gerke => match self.representation {
                MorseRep::Binary => Box::new(GERKE_LETTERS.into_iter().zip(GERKE_BINARY)),
                MorseRep::HalfBlock => Box::new(GERKE_LETTERS.into_iter().zip(GERKE_HALFBLOCK)),
                _ => Box::new(
                    std::iter::once("")
                        .zip(std::iter::once("Only line codes work for Gerke's code")),
                ),
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
    fn encode(&self, text: &str) -> Result<String, CodeError> {
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
                Rule::unknown => return Err(CodeError::invalid_input_group(pair.as_str())),
                Rule::itu_sign | Rule::gerke_sign | Rule::american_sign | Rule::greek_sign => {
                    match map.get_by_left(pair.as_str()) {
                        Some(s) => out.push(*s),
                        None => return Err(CodeError::invalid_input_group(pair.as_str())),
                    }
                }
                Rule::space => match self.representation {
                    MorseRep::Binary => out.push("0"),
                    _ => out.push(" "),
                },
                Rule::itu_passage
                | Rule::gerke_passage
                | Rule::american_passage
                | Rule::greek_passage => (),
            }
        }

        Ok(out.join(self.representation.letter_sep()))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let mut word_buffer = String::new();
        let map = self.representation.map(self.standard)?;
        for word in text.split(self.representation.word_sep()) {
            for ch in word.split(self.representation.letter_sep()) {
                match map.get_by_right(&ch) {
                    Some(s) => word_buffer.push_str(s),
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

    // fn visualize_tree(pairs: Pairs<'_, Rule>, space: String) {
    //     for pair in pairs.into_iter() {
    //         println!("{space}{:?}({})", pair.as_rule(), pair.as_str());
    //         visualize_tree(pair.into_inner(), format!("{space} "))
    //     }
    // }

    #[test]
    fn encode_test_binary() {
        let mut code = Morse::default();
        code.representation = MorseRep::Binary;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), MORSE_BINARY);
    }

    #[test]
    fn decode_test_binary() {
        let mut code = Morse::default();
        code.representation = MorseRep::Binary;
        assert_eq!(code.decode(MORSE_BINARY).unwrap(), PLAINTEXT);
    }

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
