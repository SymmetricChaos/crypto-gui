use crate::ciphers::Cipher;
use crate::errors::Error;
use crate::global_rng::get_global_rng;
use crate::text_aux::PresetAlphabet;
use crate::text_aux::VecString;
use itertools::Itertools;
use lazy_static::lazy_static;

// Porta Cipher uses a sequence of 13 alphabets to encrypt characters. The visible pattern ensures the cipher is reciprocal.
lazy_static! {
    pub static ref PORTA_TABLEAUX: [&'static str; 13] = [
        "NOPQRSTUVWXYZABCDEFGHIJKLM",
        "OPQRSTUVWXYZNMABCDEFGHIJKL",
        "PQRSTUVWXYZNOLMABCDEFGHIJK",
        "QRSTUVWXYZNOPKLMABCDEFGHIJ",
        "RSTUVWXYZNOPQJKLMABCDEFGHI",
        "STUVWXYZNOPQRIJKLMABCDEFGH",
        "TUVWXYZNOPQRSHIJKLMABCDEFG",
        "UVWXYZNOPQRSTGHIJKLMABCDEF",
        "VWXYZNOPQRSTUFGHIJKLMABCDE",
        "WXYZNOPQRSTUVEFGHIJKLMABCD",
        "XYZNOPQRSTUVWDEFGHIJKLMABC",
        "YZNOPQRSTUVWXCDEFGHIJKLMAB",
        "ZNOPQRSTUVWXYBCDEFGHIJKLMA"
    ];
}

pub struct Porta {
    pub key: String,
    key_vals: Vec<usize>,
    alphabet: VecString,
}

impl Default for Porta {
    fn default() -> Self {
        Self {
            key: String::new(),
            key_vals: Vec::new(),
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
        }
    }
}

impl Porta {
    pub fn set_key(&mut self) {
        self.key_vals = self
            .key
            .chars()
            .map(|c| self.alphabet.get_pos_of(c).unwrap())
            .collect_vec();
    }

    pub fn assign_key(&mut self, key: &str) {
        self.key = key.to_string();
        self.key_vals = self
            .key
            .chars()
            .map(|c| self.alphabet.get_pos_of(c).unwrap())
            .collect_vec();
    }

    pub fn tableaux(&self) -> std::slice::Iter<'_, &str> {
        PORTA_TABLEAUX.iter()
    }
}

impl Cipher for Porta {
    // Need to incorporate graceful failure or guarantee of correctness
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len());
        let ckey = self.key_vals.iter().cycle();
        for (c, k) in text.chars().zip(ckey) {
            let row = PORTA_TABLEAUX.get(*k).unwrap();
            let pos = row.chars().position(|x| x == c).unwrap();
            out.push(self.alphabet.get_char_at(pos).unwrap())
        }
        Ok(out)
    }

    // The Porta cipher is reciprocal
    fn decrypt(&self, text: &str) -> Result<String, Error> {
        self.encrypt(text)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn randomize(&mut self) {
        let rng = get_global_rng();
        todo!("{:?}", rng)
    }
}

// #[cfg(test)]
// mod porta_tests {
//     use super::*;
// }
