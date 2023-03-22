pub mod ascii;
pub use ascii::Ascii;

pub mod morse_itu;
pub use morse_itu::MorseITU;

pub mod morse_american;
pub use morse_american::MorseAmerican;

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

use crate::errors::Error;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String, Error>;
    fn decode(&self, text: &str) -> Result<String, Error>;
    fn randomize(&mut self);
    fn reset(&mut self);
}
