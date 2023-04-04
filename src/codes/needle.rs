use crate::{errors::Error, text_aux::text_functions::bimap_from_iter};
use bimap::BiMap;
use lazy_static::lazy_static;

use super::Code;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Needle,
    Binary,
}

const NEEDLE_ALPHABET: &'static str = "ABDEFGHIKLMNOPRSTUWY";
const FIVE_NEEDLE_CODES: [&'static str; 20] = [
    r"/|||\", r"/||\|", r"|/||\", r"/|\||", r"|/|\|", r"||/|\", r"/\|||", r"|/\||", r"||/\|",
    r"|||/\", r"\/|||", r"|\/||", r"||\/|", r"|||\/", r"\|/||", r"|\|/|", r"||\|/", r"\||/|",
    r"|\||/", r"\|||/",
];

lazy_static! {
    pub static ref FIVE_NEEDLE_MAP: BiMap<char, &'static str> =
        bimap_from_iter(NEEDLE_ALPHABET.chars().zip(FIVE_NEEDLE_CODES.into_iter()));
}

pub struct Needle {}

impl Needle {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &str)> + '_> {
        Box::new(NEEDLE_ALPHABET.chars().zip(FIVE_NEEDLE_CODES.into_iter()))
    }
}

impl Default for Needle {
    fn default() -> Self {
        Needle {}
    }
}

impl Code for Needle {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut vec = Vec::with_capacity(text.len());
        for c in text.chars() {
            let code = FIVE_NEEDLE_MAP
                .get_by_left(&c)
                .ok_or_else(|| Error::invalid_input_char(c))?;
            vec.push(*code)
        }
        Ok(vec.join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let codes = text.split(" ");
        let mut output = String::with_capacity(codes.clone().count());
        for code in codes {
            let c = FIVE_NEEDLE_MAP
                .get_by_right(code)
                .ok_or_else(|| Error::invalid_input_group(code))?;
            output.push(*c)
        }
        Ok(output)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod needle_code_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ABDE";
    const CIPHERTEXT: &'static str = r"/|||\ /||\| |/||\ /|\||";

    #[test]
    fn encrypt_test() {
        let code = Needle::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Needle::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
