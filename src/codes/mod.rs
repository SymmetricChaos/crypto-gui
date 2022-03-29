pub mod ascii;
pub use ascii::Ascii;

pub mod morse_itu;
pub use morse_itu::MorseITU;

pub mod morse_american;
pub use morse_american::MorseAmerican;

pub mod godel;
pub use godel::Godel;

pub mod fibonnaci;
pub use fibonnaci::FibonacciCode;

pub mod romaji;


pub mod elias;

use crate::errors::CodeError;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String,CodeError>;
    fn decode(&self, text: &str) -> Result<String,CodeError>;
}