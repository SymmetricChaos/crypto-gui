use lazy_static::lazy_static;
use std::collections::HashMap;

pub const ITU_LETTERS: &'static str = "ABCDEÉFGHIJKLMNOPQRSTUVWXYZ1234567890.,:?'-/()\"=+@";
pub const ITU_ASCII: [&'static str; 50] = [
    ".-", "-...", "-.-.", "-..", ".", "..-..", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
    "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
    "--..", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.",
    "-----", ".-.-.-", "--..--", "---...", "..--..", ".---.", "-...-", "-..-.", "-.--.", "-.--.-",
    ".-..-.", "-...-", ".-.-.", ".--.-.",
];

pub const ITU_DOT_DASH: [&'static str; 50] = [
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
pub const ITU_BINARY: [&'static str; 50] = [
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

pub const ITU_HALFBLOCK: [&'static str; 50] = [
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

pub const AMERICAN_LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ&1234567890,.?!";
pub const AMERICAN_BINARY: [&'static str; 41] = [
    "10111",
    "111010101",
    "101001",
    "1110101",
    "1",
    "1011101",
    "111011101",
    "1010101",
    "101",
    "11101011101",
    "111010111",
    "11111",
    "1110111",
    "11101",
    "1001",
    "101010101",
    "101011101",
    "10011",
    "10101",
    "111",
    "1010111",
    "101010111",
    "101110111",
    "101110101",
    "1010011",
    "10101001",
    "1001101",
    "10111011101",
    "10101110101",
    "10101011101",
    "10101010111",
    "11101110111",
    "10101010101",
    "11101110101",
    "11101010101",
    "11101010111",
    "1111111",
    "10111010111",
    "101011101110101",
    "1110101011101",
    "1110111011101",
];
pub const AMERICAN_HALFBLOCK: [&'static str; 41] = [
    "▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄",
    "▄ ▄  ▄",
    "▄▄▄ ▄ ▄",
    "▄",
    "▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄",
    "▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄▄▄",
    "▄▄▄▄▄",
    "▄▄▄ ▄▄▄",
    "▄▄▄ ▄",
    "▄  ▄",
    "▄ ▄ ▄ ▄ ▄",
    "▄ ▄ ▄▄▄ ▄",
    "▄  ▄▄",
    "▄ ▄ ▄",
    "▄▄▄",
    "▄ ▄ ▄▄▄",
    "▄ ▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄",
    "▄ ▄  ▄▄",
    "▄ ▄ ▄  ▄",
    "▄  ▄▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄",
    "▄ ▄ ▄▄▄ ▄ ▄",
    "▄ ▄ ▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄ ▄ ▄▄▄",
    "▄▄▄▄▄▄▄",
    "▄ ▄▄▄ ▄ ▄▄▄",
    "▄ ▄ ▄▄▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄",
];

lazy_static! {
    pub static ref ITU_ASCII_MAP: HashMap<char, &'static str> =
        HashMap::from_iter(ITU_LETTERS.chars().zip(ITU_ASCII.iter().copied()));
    pub static ref ITU_BINARY_MAP: HashMap<char, &'static str> =
        HashMap::from_iter(ITU_LETTERS.chars().zip(ITU_BINARY.iter().copied()));
    pub static ref ITU_DOT_DASH_MAP: HashMap<char, &'static str> =
        HashMap::from_iter(ITU_LETTERS.chars().zip(ITU_DOT_DASH.iter().copied()));
    pub static ref ITU_HALFBLOCK_MAP: HashMap<char, &'static str> =
        HashMap::from_iter(ITU_LETTERS.chars().zip(ITU_HALFBLOCK.iter().copied()));
    pub static ref AMERICAN_BINARY_MAP: HashMap<char, &'static str> = HashMap::from_iter(
        AMERICAN_LETTERS
            .chars()
            .zip(AMERICAN_BINARY.iter().copied())
    );
    pub static ref AMERICAN_HALFBLOCK_MAP: HashMap<char, &'static str> = HashMap::from_iter(
        AMERICAN_LETTERS
            .chars()
            .zip(AMERICAN_HALFBLOCK.iter().copied())
    );
}

lazy_static! {
    pub static ref ITU_ASCII_MAP_INV: HashMap<&'static str, char> =
        HashMap::from_iter(ITU_ASCII.iter().copied().zip(ITU_LETTERS.chars()));
    pub static ref ITU_BINARY_MAP_INV: HashMap<&'static str, char> =
        HashMap::from_iter(ITU_BINARY.iter().copied().zip(ITU_LETTERS.chars()));
    pub static ref ITU_DOT_DASH_MAP_INV: HashMap<&'static str, char> =
        HashMap::from_iter(ITU_DOT_DASH.iter().copied().zip(ITU_LETTERS.chars()));
    pub static ref ITU_HALFBLOCK_MAP_INV: HashMap<&'static str, char> =
        HashMap::from_iter(ITU_HALFBLOCK.iter().copied().zip(ITU_LETTERS.chars()));
    pub static ref AMERICAN_BINARY_MAP_INV: HashMap<&'static str, char> = HashMap::from_iter(
        AMERICAN_BINARY
            .iter()
            .copied()
            .zip(AMERICAN_LETTERS.chars())
    );
    pub static ref AMERICAN_HALFBLOCK_MAP_INV: HashMap<&'static str, char> = HashMap::from_iter(
        AMERICAN_HALFBLOCK
            .iter()
            .copied()
            .zip(AMERICAN_LETTERS.chars())
    );
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

pub enum MorseStandard {
    Itu,
    American,
}

#[cfg(test)]
mod morseitu_tests {
    use super::*;

    #[test]
    fn itu_pairs() {
        for (letter, code) in ITU_LETTERS.chars().zip(ITU_HALFBLOCK) {
            println!("{letter} {code}")
        }
    }
}
