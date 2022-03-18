pub mod ascii;
pub mod romaji;
pub use ascii::ASCII;

pub mod morse;
pub use morse::MorseITU;

pub mod godel;
pub use godel::Godel;

pub mod fibonnaci;
pub use fibonnaci::FibonacciCode;

use crate::errors::CodeError;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String,CodeError>;
    fn decode(&self, text: &str) -> Result<String,CodeError>;
}