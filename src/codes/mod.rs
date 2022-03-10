pub mod ascii;
pub use ascii::ASCII;

pub mod morse;
pub use morse::MorseITU;

use crate::errors::CodeError;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String,CodeError>;
    fn decode(&self, text: &str) -> Result<String,CodeError>;
}