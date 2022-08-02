use crate::ciphers::Cipher;
use crate::errors::Error;
use crate::global_rng::get_global_rng;
use crate::text_aux::{prep_text, PresetAlphabet::*, VecString};
use rand::Rng;

pub struct Caesar {
    pub shift: i32,
    pub alphabet: VecString,
    pub alphabet_string: String,
}

impl Caesar {
    fn encrypt_char(&self, c: char) -> char {
        self.alphabet.get_shifted_char(c, self.shift).unwrap()
    }

    fn decrypt_char(&self, c: char) -> char {
        self.alphabet.get_shifted_char(c, -self.shift).unwrap()
    }

    pub fn check_input(&self, text: &str) -> Result<(), Error> {
        for c in text.chars() {
            if !self.alphabet.contains(c) {
                return Err(Error::invalid_input_char(c));
            }
        }
        Ok(())
    }

    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }

    pub fn control_alphabet(&mut self) -> &mut String {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
        &mut self.alphabet_string
    }

    fn _validate_settings(&self) -> Result<(), Error> {
        if (self.shift as usize) > self.alphabet.len() {
            return Err(Error::Key(String::from(
                "shift value must be less than the alphabet length",
            )));
        }
        Ok(())
    }
}

impl Default for Caesar {
    fn default() -> Self {
        Self {
            shift: 0,
            alphabet: VecString::from(BasicLatin),
            alphabet_string: String::from(BasicLatin),
        }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let text = prep_text(text, &self.alphabet.to_string())?;
        Ok(text.chars().map(|c| self.encrypt_char(c)).collect())
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let text = prep_text(text, &self.alphabet.to_string())?;
        Ok(text.chars().map(|c| self.decrypt_char(c)).collect())
    }

    fn randomize(&mut self) {
        self.shift = get_global_rng().gen_range(0..self.alphabet.len()) as i32;
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod caesar_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "WKHTXLFNEURZQIRAMXPSVRYHUWKHODCBGRJ";

    #[test]
    fn encrypt_test() {
        let mut cipher = Caesar::default();
        cipher.shift = 3;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Caesar::default();
        cipher.shift = 3;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
