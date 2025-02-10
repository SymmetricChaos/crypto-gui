use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;

const AMERICAN_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz!'-,;:?\"";
const AMERICAN_BRAILLE: &'static str = "⠁⠣⠚⠙⠂⠋⠛⠓⠊⠽⠗⠇⠍⠬⠑⠩⠟⠉⠅⠃⠥⠧⠺⠷⠜⠻⠾⠈⠒⠄⠆⠴⠲⠦";
const ENGLISH_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz!'-,;:.?";
const ENGLISH_BRAILLE: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠖⠄⠤⠂⠆⠒⠲⠦";
const FRENCH_LETTERS: &'static str = "abcdefghijklmnopqrstuvxyzçéàèùâêîôûëïüœw!'-,;:.?";
const FRENCH_BRAILLE: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠭⠽⠵⠯⠿⠷⠮⠾⠡⠣⠩⠹⠱⠫⠻⠳⠪⠺⠖⠄⠤⠂⠆⠒⠲⠢";

crate::lazy_bimap!(
    ENGLISH_MAP: BiMap<char, char> = ENGLISH_LETTERS.chars().zip(ENGLISH_BRAILLE.chars());
    FRENCH_MAP: BiMap<char, char> = FRENCH_LETTERS.chars().zip(FRENCH_BRAILLE.chars()); // These are all normalized single character symbols so .chars() can be used
    AMERICAN_MAP: BiMap<char, char> = AMERICAN_LETTERS.chars().zip(AMERICAN_BRAILLE.chars());
);

#[derive(Debug, PartialEq, Eq)]
pub enum BrailleLanguage {
    English,
    French,
    American,
}

impl BrailleLanguage {
    pub fn chars_codes(&self) -> impl Iterator<Item = (char, char)> {
        match self {
            Self::English => ENGLISH_LETTERS.chars().zip(ENGLISH_BRAILLE.chars()),
            Self::French => {
                FRENCH_LETTERS // These are all normalized single character symbols so .chars() can be used
                    .chars()
                    .zip(FRENCH_BRAILLE.chars())
            }
            Self::American => AMERICAN_LETTERS.chars().zip(AMERICAN_BRAILLE.chars()),
        }
    }

    pub fn encode(&self, c: char) -> Option<&char> {
        match self {
            Self::English => ENGLISH_MAP.get_by_left(&c),
            Self::French => FRENCH_MAP.get_by_left(&c),
            Self::American => AMERICAN_MAP.get_by_left(&c),
        }
    }

    pub fn decode(&self, c: char) -> Option<&char> {
        match self {
            Self::English => ENGLISH_MAP.get_by_right(&c),
            Self::French => FRENCH_MAP.get_by_right(&c),
            Self::American => AMERICAN_MAP.get_by_right(&c),
        }
    }

    pub fn capital_sign(&self) -> Option<char> {
        match self {
            Self::English => Some('⠠'),
            Self::French => Some('⠨'),
            Self::American => Some('⠤'),
        }
    }

    pub fn number_sign(&self) -> Option<char> {
        match self {
            Self::English => Some('⠼'),
            Self::French => Some('⠼'),
            Self::American => None,
        }
    }

    pub fn encode_number(&self, n: char) -> Option<char> {
        match n {
            '1' => Some('⠁'),
            '2' => Some('⠃'),
            '3' => Some('⠉'),
            '4' => Some('⠙'),
            '5' => Some('⠑'),
            '6' => Some('⠋'),
            '7' => Some('⠛'),
            '8' => Some('⠓'),
            '9' => Some('⠊'),
            '0' => Some('⠚'),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BrailleMode {
    Letter,
    Numeric,
    Capital,
}

pub struct SimpleBraille {
    pub language: BrailleLanguage,
}

impl Default for SimpleBraille {
    fn default() -> Self {
        Self {
            language: BrailleLanguage::English,
        }
    }
}

impl Code for SimpleBraille {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();

        for c in text.chars() {
            // Ignore whitespace
            if c.is_whitespace() {
                out.push(c);
                continue;
            }

            // Handle uppercase by prepending capital
            if c.is_uppercase() {
                out.push(
                    self.language
                        .capital_sign()
                        .expect("all versions currently have a capital sign"),
                );
                for code_point in c.to_lowercase().into_iter() {
                    out.push(
                        *self
                            .language
                            .encode(code_point)
                            .ok_or_else(|| CodeError::invalid_input_char(code_point))?,
                    )
                }
                continue;
            }

            // Handle numbers by prepending the numeric sign and converting to a character
            if c.is_ascii_digit() {
                out.push(self.language.number_sign().ok_or_else(|| {
                    CodeError::state("numeric characters are not handled in this encoding")
                })?);
                out.push(
                    self.language
                        .encode_number(c)
                        .expect("all chars other than ascii digits already caught"),
                );
                continue;
            }

            out.push(
                *self
                    .language
                    .encode(c)
                    .ok_or_else(|| CodeError::invalid_input_char(c))?,
            )
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();

        let mut mode = BrailleMode::Letter;
        for c in text.chars() {
            // Skip whitespace
            if c.is_whitespace() {
                out.push(c);
                continue;
            }
            // Detect the capital or number sign
            if Some(c) == self.language.capital_sign() {
                mode = BrailleMode::Capital;
                continue;
            }
            if Some(c) == self.language.number_sign() {
                mode = BrailleMode::Numeric;
                continue;
            }
            let x = self
                .language
                .decode(c)
                .ok_or_else(|| CodeError::invalid_input_char(c))?;

            match mode {
                BrailleMode::Letter => out.push(*x),
                BrailleMode::Numeric => {
                    if c.is_ascii_digit() {
                        out.push(((c as u32) - 49) as u8 as char)
                    } else {
                        return Err(CodeError::Input(format!(
                            "character `{}` encountered in numeric mode, where it has no meaning",
                            c
                        )));
                    }
                }
                BrailleMode::Capital => out.push_str(&x.to_uppercase().collect::<String>()),
            }

            mode = BrailleMode::Letter;
        }

        Ok(out)
    }
}

#[cfg(test)]
mod braille_tests {
    use super::*;

    const PLAINTEXT: &'static str = "The Quick 023";
    const CIPHERTEXT: &'static str = "⠠⠞⠓⠑ ⠠⠟⠥⠊⠉⠅ ⠼⠚⠼⠃⠼⠉";

    #[test]
    #[ignore = "letter pairing test"]
    fn letter_pairing() {
        // println!("American");
        // for c in AMERICAN_LETTERS.chars() {
        //     println!("{} {}", c, AMERICAN_MAP.get_by_left(&c).unwrap())
        // }
        // println!("Unified English");
        // for c in ENGLISH_LETTERS.chars() {
        //     println!("{} {}", c, ENGLISH_MAP.get_by_left(&c).unwrap())
        // }
        // println!("French");
        // for c in FRENCH_LETTERS.chars() {
        //     println!("{} {}", c, FRENCH_MAP.get_by_left(&c).unwrap())
        // }
    }

    #[test]
    fn encode_test() {
        let code = SimpleBraille::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decode_test() {
        let code = SimpleBraille::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
