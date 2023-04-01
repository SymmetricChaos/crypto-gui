pub mod ascii;
pub use ascii::Ascii;

pub mod godel;
pub use godel::Godel;

pub mod fibonacci;
pub use fibonacci::FibonacciCode;

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

pub mod romaji;

pub mod elias;

pub mod punycode;
pub use punycode::Punycode;

pub mod pgp_words;
pub use pgp_words::PgpWords;

pub mod block;
pub use block::BlockCode;

pub mod morse;
pub use morse::Morse;
pub mod morse_encodings;

use crate::errors::Error;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String, Error>;
    fn decode(&self, text: &str) -> Result<String, Error>;
    fn randomize(&mut self);
    fn reset(&mut self);
}
