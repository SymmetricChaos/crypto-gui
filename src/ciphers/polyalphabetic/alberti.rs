use rand::Rng;

use crate::ciphers::Cipher;
use crate::errors::Error;
use crate::global_rng::GLOBAL_RNG;
use crate::text_aux::text_functions::validate_text;
use crate::text_aux::{PresetAlphabet::*, VecString};
use std::fmt::Display;

pub struct Alberti {
    pub fixed_alphabet_string: String,
    fixed_alphabet: VecString,
    pub moving_alphabet_string: String,
    moving_alphabet: VecString,
    pub start_index: usize,
}

impl Alberti {
    pub fn set_fixed_alphabet(&mut self) {
        self.fixed_alphabet = VecString::unique_from(&self.fixed_alphabet_string);
    }

    pub fn assign_fixed_alphabet(&mut self, alphabet: &str) {
        self.fixed_alphabet_string = alphabet.to_string();
        self.set_fixed_alphabet()
    }

    pub fn set_moving_alphabet(&mut self) {
        self.moving_alphabet = VecString::unique_from(&self.moving_alphabet_string);
    }

    pub fn assign_moving_alphabet(&mut self, alphabet: &str) {
        self.moving_alphabet_string = alphabet.to_string();
        self.set_moving_alphabet()
    }

    // Unwrap justified by checks made in encrypt()
    fn encrypt_char(&self, symbol: char, index: usize) -> char {
        let position = self.fixed_alphabet.get_pos_of(symbol).unwrap();
        self.moving_alphabet
            .get_char_offset(position, index as i32)
            .unwrap()
    }

    // Unwrap justified by checks made in decrypt()
    fn decrypt_char(&self, symbol: char, index: usize) -> char {
        let position = self.moving_alphabet.get_pos_of(symbol).unwrap();
        self.fixed_alphabet
            .get_char_offset(position, -(index as i32))
            .unwrap()
    }

    pub fn alphabet_len(&self) -> usize {
        self.fixed_alphabet.chars().count()
    }
}

impl Cipher for Alberti {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        validate_text(text, &self.fixed_alphabet)?;
        let mut index = self.start_index.clone();
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.fixed_alphabet.contains(s) {
                out.push(self.encrypt_char(s, index));
            } else if self.moving_alphabet.contains(s) {
                index = self.moving_alphabet.get_pos_of(s).unwrap();
                out.push(self.fixed_alphabet.get_char_at(index).unwrap());
            } else {
                return Err(Error::invalid_input_char(s));
            }
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        validate_text(text, &self.moving_alphabet)?;
        let mut index = self.start_index.clone();
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.moving_alphabet.contains(s) {
                out.push(self.decrypt_char(s, index));
            } else if self.fixed_alphabet.contains(s) {
                index = self.fixed_alphabet.get_pos_of(s).unwrap();
                out.push(self.moving_alphabet.get_char_at(index).unwrap());
            } else {
                return Err(Error::invalid_input_char(s));
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        let length = self.moving_alphabet.len();
        self.start_index = GLOBAL_RNG.lock().unwrap().gen_range(0..length);
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for Alberti {
    fn default() -> Self {
        Self {
            fixed_alphabet_string: String::from(BasicLatin),
            fixed_alphabet: VecString::from(BasicLatin),
            moving_alphabet_string: String::from(BasicLatin.string().to_ascii_lowercase()),
            moving_alphabet: VecString::from(BasicLatin.string().to_ascii_lowercase()),
            start_index: 0,
        }
    }
}

impl Display for Alberti {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.fixed_alphabet_string.clone();
        out.push('\n');
        out.push_str(&self.moving_alphabet_string[self.start_index..]);
        out.push_str(&self.moving_alphabet_string[0..self.start_index]);
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod alberti_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUItCKBReOWNFOsXJUMPStOVERTiHELAZYDnOG";
    const CIPHERTEXT: &'static str = "thequiTvdukEsarjsSpbmehkThoxkmIpmtihglNbt";

    #[test]
    fn encrypt_test() {
        let cipher = Alberti::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = Alberti::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
