pub mod damm;
pub mod hamming;
pub mod luhn;
pub mod m_of_n;
pub mod parity_check;
pub mod repetition;
pub mod verhoeff;

use lazy_static::lazy_static;
use regex::Regex;
use utils::bits::Bit;

use crate::errors::CodeError;

lazy_static! {
    pub static ref IS_BITS: Regex = Regex::new(r"^[01\s]*$").unwrap();
}

pub fn check_bitstring(text: &str) -> Result<(), CodeError> {
    if !IS_BITS.is_match(text) {
        return Err(CodeError::Input(format!(
            "bitstrings can only contain 0, 1, and whitespace",
        )));
    } else {
        Ok(())
    }
}

pub fn bits_from_bitstring(text: &str) -> Result<impl Iterator<Item = Bit> + '_, CodeError> {
    check_bitstring(text)?;
    Ok(text
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Bit::try_from(c).unwrap()))
}
