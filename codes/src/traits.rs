use crate::errors::CodeError;
use bimap::BiMap;
use itertools::Itertools;
use std::hash::Hash;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String, CodeError>;
    fn decode(&self, text: &str) -> Result<String, CodeError>;
    fn randomize(&mut self);
    fn reset(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            .filter(|word| !word.is_empty())
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

    pub fn codes_chars(&mut self) -> impl Iterator<Item = (&T, char)> + '_ {
        self.alphabet
            .chars()
            .map(|x| (self.letter_map.get_by_left(&x).unwrap(), x))
    }

    pub fn codes_words(&mut self) -> impl Iterator<Item = (&T, &String)> + '_ {
        self.words
            .iter()
            .map(|x| (self.word_map.get_by_left(x).unwrap(), x))
    }

    pub fn get_by_letter(&self, c: char) -> Result<&T, CodeError> {
        self.letter_map
            .get_by_left(&c)
            .ok_or_else(|| CodeError::invalid_input_char(c))
    }

    pub fn get_letter_by_code(&self, code: &T) -> Result<&char, CodeError> {
        self.letter_map
            .get_by_right(code)
            .ok_or_else(|| CodeError::invalid_input_group(&code.to_string()))
    }

    pub fn get_by_word(&self, s: &str) -> Result<&T, CodeError> {
        self.word_map
            .get_by_left(s)
            .ok_or_else(|| CodeError::invalid_input_group(s))
    }

    pub fn get_word_by_code(&self, code: &T) -> Result<&String, CodeError> {
        self.word_map
            .get_by_right(code)
            .ok_or_else(|| CodeError::invalid_input_group(&code.to_string()))
    }
}
