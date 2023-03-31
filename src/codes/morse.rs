use crate::errors::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Code;

const LETTERS: &'static str = "ABCDEÉFGHIJKLMNOPQRSTUVWXYZ1234567890.,:?'-/()\"=+@";
const ITU_ASCII: [&'static str; 50] = [
    ".-", "-...", "-.-.", "-..", ".", "..-..", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
    "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
    "--..", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.",
    "-----", ".-.-.-", "--..--", "---...", "..--..", ".---.", "-...-", "-..-.", "-.--.", "-.--.-",
    ".-..-.", "-...-", ".-.-.", ".--.-.",
];

const ITU_DOT_DASH: [&'static str; 50] = [
    "·–",
    "–···",
    "–·–·",
    "–··",
    "·",
    "··–··",
    "··–·",
    "––·",
    "····",
    "··",
    "·–––",
    "–·–",
    "·–··",
    "––",
    "–·",
    "–––",
    "·––·",
    "––·–",
    "·–·",
    "···",
    "–",
    "··–",
    "···–",
    "·––",
    "–··–",
    "–·––",
    "––··",
    "·––––",
    "··–––",
    "···––",
    "····–",
    "·····",
    "–····",
    "––···",
    "–––··",
    "––––·",
    "–––––",
    "·–·–·–",
    "––··––",
    "–––···",
    "··––··",
    "·–––·",
    "–···–",
    "–··–·",
    "–·––·",
    "–·––·–",
    "·–··–·",
    "–···–",
    "·–·–·",
    "·––·–·",
];
const ITU_BINARY: [&'static str; 50] = [
    "10111",
    "111010101",
    "11101011101",
    "1110101",
    "1",
    "10101110101",
    "101011101",
    "111011101",
    "1010101",
    "101",
    "1011101110111",
    "111010111",
    "101110101",
    "1110111",
    "11101",
    "11101110111",
    "10111011101",
    "1110111010111",
    "1011101",
    "10101",
    "111",
    "1010111",
    "101010111",
    "101110111",
    "11101010111",
    "1110101110111",
    "11101110101",
    "10111011101110111",
    "101011101110111",
    "1010101110111",
    "10101010111",
    "101010101",
    "11101010101",
    "1110111010101",
    "111011101110101",
    "11101110111011101",
    "1110111011101110111",
    "10111010111010111",
    "1110111010101110111",
    "11101110111010101",
    "101011101110101",
    "101110111011101",
    "1110101010111",
    "1110101011101",
    "111010111011101",
    "1110101110111010111",
    "101110101011101",
    "1110101010111",
    "1011101011101",
    "10111011101011101",
];

const ITU_HALFBLOCK: [&'static str; 50] = [
    "▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄",
    "▄",
    "▄ ▄ ▄▄▄ ▄ ▄",
    "▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄",
    "▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄▄▄",
    "▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄",
    "▄ ▄ ▄",
    "▄▄▄",
    "▄ ▄ ▄▄▄",
    "▄ ▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄▄▄",
    "▄▄▄ ▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄ ▄ ▄▄▄",
    "▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄▄▄ ▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄ ▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄ ▄ ▄",
    "▄ ▄ ▄▄▄ ▄▄▄ ▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄ ▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄▄▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄ ▄▄▄ ▄",
];

lazy_static! {
    pub static ref ITU_ASCII_MAP: HashMap<char, &'static str> =
        HashMap::from_iter(LETTERS.chars().zip(ITU_ASCII.iter().copied()));
    pub static ref ITU_BINARY_MAP: HashMap<char, &'static str> =
        HashMap::from_iter(LETTERS.chars().zip(ITU_BINARY.iter().copied()));
    pub static ref ITU_DOT_DASH_MAP: HashMap<char, &'static str> =
        HashMap::from_iter(LETTERS.chars().zip(ITU_DOT_DASH.iter().copied()));
    pub static ref ITU_HALFBLOCK_MAP: HashMap<char, &'static str> =
        HashMap::from_iter(LETTERS.chars().zip(ITU_HALFBLOCK.iter().copied()));
}

lazy_static! {
    pub static ref ITU_ASCII_MAP_INV: HashMap<&'static str, char> =
        HashMap::from_iter(ITU_ASCII.iter().copied().zip(LETTERS.chars()));
    pub static ref ITU_BINARY_MAP_INV: HashMap<&'static str, char> =
        HashMap::from_iter(ITU_BINARY.iter().copied().zip(LETTERS.chars()));
    pub static ref ITU_DOT_DASH_MAP_INV: HashMap<&'static str, char> =
        HashMap::from_iter(ITU_DOT_DASH.iter().copied().zip(LETTERS.chars()));
    pub static ref ITU_HALFBLOCK_MAP_INV: HashMap<&'static str, char> =
        HashMap::from_iter(ITU_HALFBLOCK.iter().copied().zip(LETTERS.chars()));
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorseRep {
    Binary,
    Ascii,
    CdotNDash,
    HalfBlock,
}

impl MorseRep {
    pub fn dit(&self) -> &str {
        match self {
            MorseRep::Binary => "1",
            MorseRep::Ascii => "-",
            MorseRep::CdotNDash => "–",
            MorseRep::HalfBlock => "▄",
        }
    }

    pub fn dah(&self) -> &str {
        match self {
            MorseRep::Binary => "111",
            MorseRep::Ascii => ".",
            MorseRep::CdotNDash => "·",
            MorseRep::HalfBlock => "▄▄▄",
        }
    }

    pub fn intra_char_sep(&self) -> &str {
        match self {
            MorseRep::Binary => "0",
            MorseRep::Ascii => "",
            MorseRep::CdotNDash => "",
            MorseRep::HalfBlock => " ",
        }
    }

    pub fn letter_sep(&self) -> &str {
        match self {
            MorseRep::Binary => "000",
            MorseRep::Ascii => " ",
            MorseRep::CdotNDash => " ",
            MorseRep::HalfBlock => "   ",
        }
    }

    pub fn map(&self) -> &HashMap<char, &str> {
        match self {
            MorseRep::Binary => &ITU_BINARY_MAP,
            MorseRep::Ascii => &ITU_ASCII_MAP,
            MorseRep::CdotNDash => &ITU_DOT_DASH_MAP,
            MorseRep::HalfBlock => &ITU_HALFBLOCK_MAP,
        }
    }

    pub fn map_inv(&self) -> &HashMap<&str, char> {
        match self {
            MorseRep::Binary => &ITU_BINARY_MAP_INV,
            MorseRep::Ascii => &ITU_ASCII_MAP_INV,
            MorseRep::CdotNDash => &ITU_DOT_DASH_MAP_INV,
            MorseRep::HalfBlock => &ITU_HALFBLOCK_MAP_INV,
        }
    }
}

pub struct Morse {
    pub mode: MorseRep,
}

impl Morse {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &'static str)> + '_> {
        Box::new(match self.mode {
            MorseRep::Binary => LETTERS.chars().zip(ITU_BINARY),
            MorseRep::Ascii => LETTERS.chars().zip(ITU_ASCII),
            MorseRep::CdotNDash => LETTERS.chars().zip(ITU_DOT_DASH),
            MorseRep::HalfBlock => LETTERS.chars().zip(ITU_HALFBLOCK),
        })
    }
}

impl Default for Morse {
    fn default() -> Self {
        Self {
            mode: MorseRep::HalfBlock,
        }
    }
}

impl Code for Morse {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let map = self.mode.map();
        let mut out = Vec::with_capacity(text.chars().count());
        for s in text.chars() {
            match map.get(&s) {
                Some(code_group) => out.push(*code_group),
                None => return Err(Error::invalid_input_char(s)),
            }
        }
        Ok(out.join(self.mode.letter_sep()))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        let map = self.mode.map_inv();
        for s in text.split(self.mode.letter_sep()) {
            match map.get(&s) {
                Some(c) => out.push(*c),
                None => return Err(Error::invalid_input_group(s)),
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod morseitu_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT_DITDAH: &'static str = "- ···· · --·- ··- ·· -·-· -·- -··· ·-· --- ·-- -· ··-· --- -··- ·--- ··- -- ·--· ··· --- ···- · ·-· - ···· · ·-·· ·- --·· -·-- -·· --- --·";
    const CIPHERTEXT_BINARY: &'static str = "111000101010100010001110111010111000101011100010100011101011101000111010111000111010101000101110100011101110111000101110111000111010001010111010001110111011100011101010111000101110111011100010101110001110111000101110111010001010100011101110111000101010111000100010111010001110001010101000100010111010100010111000111011101010001110101110111000111010100011101110111000111011101";

    #[test]
    fn encrypt_test_binary() {
        let mut code = Morse::default();
        code.mode = MorseRep::Binary;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_BINARY);
    }

    #[test]
    fn decrypt_test_binary() {
        let mut code = Morse::default();
        code.mode = MorseRep::Binary;
        assert_eq!(code.decode(CIPHERTEXT_BINARY).unwrap(), PLAINTEXT);
    }
}
