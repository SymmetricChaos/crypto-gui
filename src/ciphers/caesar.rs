use rand::{Rng, prelude::ThreadRng};
use crate::{errors::{CipherError}};
use super::Cipher;
use crate::text_types::{PresetAlphabet::*};

pub struct Caesar {
    pub shift: usize,
    pub alphabet: String,
}

impl Caesar {
    pub fn new(shift: usize, alphabet: &str) -> Caesar {
        Caesar{ shift, alphabet: alphabet.to_string() }
    }
 
    fn encrypt_char(&self, c: char) -> char {
        let alen = self.alphabet_len();
        let pos = (self.alphabet.chars().position(|x| x == c).unwrap() + self.shift) % alen;
        self.alphabet.chars().nth(pos).unwrap()
    }
 
    fn decrypt_char(&self, c: char) -> char {
        let alen = self.alphabet_len();
        let pos = (self.alphabet.chars().position(|x| x == c).unwrap() + alen - self.shift) % alen;
        self.alphabet.chars().nth(pos).unwrap()
    }
 
    pub fn alphabet_len(&self) -> usize {
        self.alphabet.chars().count()
    }
}

impl Default for Caesar {
    fn default() -> Self {
        Self { shift: 0, alphabet: String::from(BasicLatin) }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        for c in text.chars() {
            if !self.alphabet.contains(c) {
                return Err(CipherError::invalid_input_char(c))
            }
        }
        let out: String = text.chars().map(|c| self.encrypt_char(c)).collect();
        Ok(out)
    }
 
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        for c in text.chars() {
            if !self.alphabet.contains(c) {
                return Err(CipherError::invalid_input_char(c))
            }
        }
        let out: String = text.chars().map(|c| self.decrypt_char(c)).collect();
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        let length = self.alphabet.len();
        self.shift = rng.gen_range(0..length);
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet
    }

    fn get_output_alphabet(&self) -> &String {
        &self.alphabet
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if self.shift > self.alphabet_len() {
            return Err(CipherError::Key(String::from("shift value must be less than the alphabet length")))
        }
        Ok(())
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