use itertools::Itertools;
use strum::{Display, EnumIter};

use crate::errors::CodeError;

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum IOMode {
    Letter,
    Word,
    Integer,
}

// For relating characters and words to their positions in the list
pub struct LetterWordIntCode {
    pub alphabet: String,
    pub words: Vec<String>,
}

impl LetterWordIntCode {
    pub fn new() -> Self {
        Self {
            alphabet: String::new(),
            words: Vec::new(),
        }
    }
}

impl LetterWordIntCode {
    pub fn char_to_int(&self, c: char) -> Result<usize, CodeError> {
        self.alphabet
            .find(|x| x == c)
            .ok_or_else(|| CodeError::invalid_input_char(c))
    }

    pub fn int_to_char(&self, n: usize) -> Result<char, CodeError> {
        self.alphabet
            .chars()
            .nth(n)
            .ok_or_else(|| CodeError::input("no character at position given"))
    }

    pub fn word_to_int(&self, s: &str) -> Result<usize, CodeError> {
        self.words
            .iter()
            .position(|x| x == s)
            .ok_or_else(|| CodeError::invalid_input_group(s))
    }

    pub fn int_to_word(&self, n: usize) -> Result<&String, CodeError> {
        self.words
            .iter()
            .nth(n)
            .ok_or_else(|| CodeError::input("no character at position given"))
    }

    pub fn set_words(&mut self, s: &str) {
        self.words = s
            .split(",")
            .map(|w| w.trim().to_string())
            .filter(|word| !word.is_empty())
            .unique()
            .collect_vec();
    }

    pub fn ints_chars(&self) -> impl Iterator<Item = (usize, char)> + '_ {
        self.alphabet.chars().enumerate()
    }

    pub fn ints_words(&self) -> impl Iterator<Item = (usize, &String)> + '_ {
        self.words.iter().enumerate()
    }
}
