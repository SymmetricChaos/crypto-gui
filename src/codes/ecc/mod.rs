use crate::errors::Error;

pub mod hamming;
pub mod m_of_n;
pub mod repetition;

pub fn char_to_bit(c: char) -> Result<usize, Error> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        _ => Err(Error::invalid_input_char(c)),
    }
}

pub fn bits_from_bitstring(text: &str) -> impl Iterator<Item = Result<usize, Error>> + '_ {
    text.chars()
        .filter(|b| !b.is_whitespace())
        .map(|b| char_to_bit(b))
}
