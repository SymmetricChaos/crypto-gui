pub mod ascii;

use crate::errors::CodeError;

pub trait Code {
    fn encode(&self, text: &str) -> Result<String,CodeError>;
    fn decode(&self, text: &str) -> Result<String,CodeError>;
}
 