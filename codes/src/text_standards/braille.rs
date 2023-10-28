use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

use crate::{errors::CodeError, traits::Code};

const ENGLISH_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz!'-,;:.?";
const BRAILLE_ENGLISH: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠖⠄⠤⠂⠆⠒⠲⠦";
const FRENCH_CHARS: &'static str = "abcdefghijklmnopqrstuvxyzçéàèùâêîôûëïüœw!'-,;:.?";
const BRAILLE_FRENCH: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠭⠽⠵⠯⠿⠷⠮⠾⠡⠣⠩⠹⠱⠫⠻⠳⠪⠺⠖⠄⠤⠂⠆⠒⠲⠢";

lazy_static! {
    pub static ref ENGLISH_MAP: BiMap<char, char> =
        bimap_from_iter(ENGLISH_CHARS.chars().zip(BRAILLE_ENGLISH.chars()));
    pub static ref FRENCH_MAP: BiMap<char, char> = bimap_from_iter(
        FRENCH_CHARS // These are all normalized single character symbols so .chars() can be used
            .chars()
            .zip(BRAILLE_FRENCH.chars())
    );
}

#[derive(Debug, PartialEq, Eq)]
pub enum BrailleLanguage {
    English,
    French,
}

impl BrailleLanguage {
    pub fn chars_codes(&self) -> std::iter::Zip<std::str::Chars<'_>, std::str::Chars<'_>> {
        match self {
            BrailleLanguage::English => BRAILLE_ENGLISH.chars().zip(ENGLISH_CHARS.chars()),
            BrailleLanguage::French => {
                BRAILLE_FRENCH // These are all normalized single character symbols so .chars() can be used
                    .chars()
                    .zip(FRENCH_CHARS.chars())
            }
        }
    }

    pub fn encode(&self, c: char) -> Option<&char> {
        match self {
            Self::English => ENGLISH_MAP.get_by_left(&c),
            Self::French => FRENCH_MAP.get_by_left(&c),
        }
    }

    pub fn decode(&self, c: char) -> Option<&char> {
        match self {
            Self::English => ENGLISH_MAP.get_by_right(&c),
            Self::French => FRENCH_MAP.get_by_right(&c),
        }
    }

    pub fn capital_sign(&self) -> char {
        match self {
            Self::English => '⠠',
            Self::French => '⠨',
        }
    }

    pub fn number_sign(&self) -> char {
        match self {
            Self::English => '⠼',
            Self::French => '⠼',
        }
    }
}

pub struct Braille {
    pub language: BrailleLanguage,
}

impl Default for Braille {
    fn default() -> Self {
        Self {
            language: BrailleLanguage::English,
        }
    }
}

impl Code for Braille {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        for c in text.chars() {
            if c.is_whitespace() {
                out.push(c);
                continue;
            }
            if c.is_uppercase() {
                out.push(self.language.capital_sign())
            }
            let x = self
                .language
                .encode(c)
                .ok_or_else(|| CodeError::invalid_input_char(c))?;
            out.push(*x)
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        let mut capital = false;
        for c in text.chars() {
            if c.is_whitespace() {
                out.push(c);
                continue;
            }
            if c == self.language.capital_sign() {
                capital = true;
                continue;
            }
            let x = self
                .language
                .decode(c)
                .ok_or_else(|| CodeError::invalid_input_char(c))?;
            if capital {
                out.push_str(&x.to_uppercase().collect::<String>())
            } else {
                out.push(*x)
            }
        }
        Ok(out)
    }
}
