use bimap::BiMap;
use lazy_static::lazy_static;
use utils::{preset_alphabet::Alphabet, text_functions::bimap_from_iter};

use crate::{errors::CodeError, traits::Code};

const BRAILLE_ENGLISH: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵";
const BRAILLE_FRENCH: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠭⠽⠵⠯⠿⠷⠮⠾⠡⠣⠩⠹⠱⠫⠻⠳⠪⠺";

lazy_static! {
    pub static ref ENGLISH_MAP: BiMap<char, char> =
        bimap_from_iter(Alphabet::BasicLatin.chars().zip(BRAILLE_ENGLISH.chars()));
    pub static ref FRENCH_MAP: BiMap<char, char> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVXYZÇÉÀÈÙÂÊÎÔÛËÏÜŒW" // These are all normalized single character symbols to .chars() can be used
            .chars()
            .zip(BRAILLE_FRENCH.chars())
    );
}

pub enum BrailleLanguage {
    English,
    French,
}

impl BrailleLanguage {
    pub fn encode(&self, c: char) -> Option<&char> {
        ENGLISH_MAP.get_by_left(&c)
    }

    pub fn decode(&self, c: char) -> Option<&char> {
        ENGLISH_MAP.get_by_right(&c)
    }
}

pub struct Braille {
    language: BrailleLanguage,
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
        for c in text.chars() {
            let x = self
                .language
                .decode(c)
                .ok_or_else(|| CodeError::invalid_input_char(c))?;
            out.push(*x)
        }
        Ok(out)
    }
}
