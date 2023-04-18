use bimap::BiMap;
use itertools::Itertools;
use std::hash::Hash;

pub mod ascii;
pub use ascii::Ascii;

pub mod godel;
pub use godel::Godel;

pub mod fibonacci;
pub use fibonacci::FibonacciCode;
pub mod fibonacci_integers;
pub use fibonacci_integers::FibonacciCodeIntegers;

pub mod levenshtein;
pub use levenshtein::LevenshteinCode;
pub mod levenshtein_integers;
pub use levenshtein_integers::LevenshteinCodeIntegers;

// pub mod elias;
// pub use elias::EliasCode;
// pub mod elias_integers;
// pub use elias_integers::EliasCodeIntegers;

pub mod unary;
pub use unary::UnaryCode;

pub mod spelling_alphabet;
pub use spelling_alphabet::SpellingAlphabet;

pub mod base32;
pub use base32::Base32;

pub mod base64;
pub use base64::Base64;

pub mod unicode;
pub use unicode::Unicode;

pub mod baudot;
pub use baudot::Baudot;

pub mod bacon;
pub use bacon::Bacon;

pub mod punycode;
pub use punycode::Punycode;

pub mod pgp_words;
pub use pgp_words::PgpWords;

pub mod block;
pub use block::BlockCode;

pub mod morse;
pub use morse::Morse;
pub mod morse_encodings;

pub mod tap_code;
pub use tap_code::TapCode;

pub mod needle;
pub use needle::Needle;

pub mod romaji;
pub use romaji::romaji::Romaji;

use crate::errors::Error;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String, Error>;
    fn decode(&self, text: &str) -> Result<String, Error>;
    fn randomize(&mut self);
    fn reset(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOMode {
    Letter,
    Word,
    Integer,
}

#[derive(Debug)]
pub struct LetterAndWordCode<T> {
    pub letter_map: BiMap<char, T>,
    pub word_map: BiMap<String, T>,
    pub alphabet: String,
    pub words: Vec<String>,
    pub words_string: String,
}

impl<T: Default + Hash + Eq + PartialEq> Default for LetterAndWordCode<T> {
    fn default() -> Self {
        Self {
            letter_map: Default::default(),
            word_map: Default::default(),
            alphabet: Default::default(),
            words: Default::default(),
            words_string: Default::default(),
        }
    }
}

impl<T: Hash + Eq + PartialEq + ToString> LetterAndWordCode<T> {
    pub fn set_letter_map<F: Fn((usize, char)) -> T>(&mut self, ltr_map: F) {
        self.letter_map.clear();
        for (n, c) in self.alphabet.chars().enumerate() {
            self.letter_map.insert(c.clone(), ltr_map((n, c)));
        }
    }

    pub fn set_word_map<F: Fn((usize, &String)) -> T>(&mut self, ltr_map: F) {
        self.words = self
            .words_string
            .split(",")
            .map(|w| w.trim().to_string())
            .collect_vec();
        self.word_map.clear();
        for (n, c) in self.words.iter().enumerate() {
            self.word_map.insert(c.clone(), ltr_map((n, c)));
        }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &T)> + '_ {
        self.alphabet
            .chars()
            .map(|x| (x, self.letter_map.get_by_left(&x).unwrap()))
    }

    pub fn words_codes(&mut self) -> impl Iterator<Item = (&String, &T)> + '_ {
        self.words
            .iter()
            .map(|x| (x, self.word_map.get_by_left(x).unwrap()))
    }

    pub fn get_by_letter(&self, c: char) -> Result<&T, Error> {
        self.letter_map
            .get_by_left(&c)
            .ok_or_else(|| Error::invalid_input_char(c))
    }

    pub fn get_letter_by_code(&self, code: &T) -> Result<&char, Error> {
        self.letter_map
            .get_by_right(code)
            .ok_or_else(|| Error::invalid_input_group(&code.to_string()))
    }

    pub fn get_by_word(&self, s: &str) -> Result<&T, Error> {
        self.word_map
            .get_by_left(s)
            .ok_or_else(|| Error::invalid_input_group(s))
    }

    pub fn get_word_by_code(&self, code: &T) -> Result<&String, Error> {
        self.word_map
            .get_by_right(code)
            .ok_or_else(|| Error::invalid_input_group(&code.to_string()))
    }
}
