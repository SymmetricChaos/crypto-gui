use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

lazy_static! {
    pub static ref BIQUINARY_MAP: BiMap<char, &'static str> = bimap_from_iter(
        "0123456789".chars().zip(
            [
                "01-00001", "01-00010", "01-00100", "01-01000", "01-10000", "10-00001", "10-00010",
                "10-00100", "10-01000", "10-10000"
            ]
            .into_iter()
        )
    );
    pub static ref BIQUINARY_MAP_INV_LOWER: BiMap<char, &'static str> = bimap_from_iter(
        "0123456789".chars().zip(
            [
                "01-11110", "01-11101", "01-11011", "01-10111", "01-01111", "10-11110", "10-11101",
                "10-11011", "10-10111", "10-01111"
            ]
            .into_iter()
        )
    );
}

#[derive(Debug, PartialEq, Eq)]
pub enum BiQuinaryMode {
    TwoOfSeven,
    Abacus,
}

impl BiQuinaryMode {
    fn map(&self) -> &BiMap<char, &'static str> {
        match self {
            BiQuinaryMode::TwoOfSeven => &BIQUINARY_MAP,
            BiQuinaryMode::Abacus => &BIQUINARY_MAP_INV_LOWER,
        }
    }

    fn encode(&self, c: char) -> Option<&'static str> {
        self.map().get_by_left(&c).copied()
    }

    fn decode(&self, s: &str) -> Option<char> {
        self.map().get_by_right(s).copied()
    }
}

pub struct BiquinaryDecimal {
    pub mode: BiQuinaryMode,
}

impl Default for BiquinaryDecimal {
    fn default() -> Self {
        Self {
            mode: BiQuinaryMode::Abacus,
        }
    }
}

impl BiquinaryDecimal {
    pub fn chars_codes(&self) -> impl Iterator<Item = (char, &str)> + '_ {
        "0123456789"
            .chars()
            .map(|c| (c, self.mode.encode(c).unwrap()))
    }
}

impl Code for BiquinaryDecimal {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        for c in text.split(",").map(|s| s.trim()) {
            match self.mode.encode(c.chars().next().unwrap()) {
                Some(s) => {
                    out.push_str(s);
                }
                None => out.push(c.chars().next().unwrap()),
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        for s in text.split(",").map(|s| s.trim()) {
            match self.mode.decode(s) {
                Some(c) => {
                    out.push(c);
                }
                None => out.push_str(s),
            }
            out.push_str(", ");
        }
        out.pop();
        out.pop();
        Ok(out)
    }
}

#[cfg(test)]
mod balanced_ternary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0, 9, 1, 8, 2";
    const ENCODEDTEXT: &'static str = "01-11110, 10-01111, 01-11101, 10-10111, 01-11011";

    #[test]
    fn encode_test() {
        let code = BiquinaryDecimal::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = BiquinaryDecimal::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
