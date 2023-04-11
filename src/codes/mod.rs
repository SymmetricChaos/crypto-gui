use itertools::Itertools;

pub mod ascii;
pub use ascii::Ascii;

pub mod godel;
pub use godel::Godel;

pub mod fibonacci;
pub use fibonacci::FibonacciCode;
pub mod fibonacci_integers;
pub use fibonacci_integers::FibonacciCodeIntegers;

pub mod unary;
pub use unary::UnaryCode;

pub mod spelling_alphabet;
pub use spelling_alphabet::SpellingAlphabet;

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

pub mod elias;
pub use elias::EliasCode;

pub mod needle;
pub use needle::Needle;

//pub mod levenshtein;

pub mod romaji;

use crate::errors::Error;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String, Error>;
    fn decode(&self, text: &str) -> Result<String, Error>;
    fn randomize(&mut self);
    fn reset(&mut self);
}

pub struct CodeWords {
    pub words: Vec<String>,
    pub string: String,
    pub sep: String,
}

impl CodeWords {
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            string: String::new(),
            sep: String::from(","),
        }
    }

    pub fn update_code_words(&mut self) {
        self.words = self
            .string
            .split(&self.sep)
            .map(|w| w.trim().to_string())
            .collect_vec();
    }

    pub fn code_words(&self) -> std::slice::Iter<'_, String> {
        self.words.iter()
    }
}
