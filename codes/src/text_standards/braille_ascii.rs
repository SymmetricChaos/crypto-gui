use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

use crate::{errors::CodeError, traits::Code};

const ASCII_LETTERS: &'static str =
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_";
const ASCII_BRAILLE: &'static str =
    "⠮⠐⠼⠫⠩⠯⠄⠷⠾⠡⠬⠠⠤⠨⠌⠴⠂⠆⠒⠲⠢⠖⠶⠦⠔⠱⠰⠣⠿⠜⠹⠈⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠪⠳⠻⠘⠸";

lazy_static! {
    pub static ref ASCII_MAP: BiMap<char, char> =
        bimap_from_iter(ASCII_LETTERS.chars().zip(ASCII_BRAILLE.chars()));
}

pub struct BrailleAscii {}

impl Default for BrailleAscii {
    fn default() -> Self {
        Self {}
    }
}

impl BrailleAscii {
    pub fn chars_codes() -> impl Iterator<Item = (char, char)> {
        ASCII_LETTERS.chars().zip(ASCII_BRAILLE.chars())
    }
}

impl Code for BrailleAscii {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        // Braille ASCII is a strict 1:1 coding of a range of ASCII values

        for c in text.chars() {
            if c.is_whitespace() {
                out.push(c);
                continue;
            }

            out.push(
                *ASCII_MAP
                    .get_by_left(&c.to_ascii_uppercase())
                    .ok_or_else(|| CodeError::invalid_input_char(c))?,
            );
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();

        for c in text.chars() {
            if c.is_whitespace() {
                out.push(c);
                continue;
            }
            out.push(
                *ASCII_MAP
                    .get_by_right(&c)
                    .ok_or_else(|| CodeError::invalid_input_char(c))?,
            );
        }

        Ok(out)
    }
}

#[cfg(test)]
mod braille_ascii_tests {
    use super::*;

    #[test]
    #[ignore = "letter pairing test"]
    fn letter_pairing() {
        println!("ASCII");
        for c in ASCII_LETTERS.chars() {
            println!("{} {}", c, ASCII_MAP.get_by_left(&c).unwrap())
        }
    }
}
