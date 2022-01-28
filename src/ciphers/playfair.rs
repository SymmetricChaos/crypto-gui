use rand::prelude::ThreadRng;
use super::Cipher;
use crate::text_functions::{LATIN_UPPER, string_pairs};
use crate::errors::CipherError;


pub struct Playfair {
    pub key_word: String,
    alphabet: String,
}

impl Playfair {

    fn validate_key(&self) -> Result<(),CipherError> {
        if self.key_word.len() == 0 {
            return Err(CipherError::Key(String::from("No key word provided")))
        }
        for c in self.key_word.chars() {
            if !self.alphabet.contains(c) { return Err(CipherError::invalid_alphabet_char(c)) }
        }
        Ok(())
    }

    fn validate_input(&self, text: &str) -> Result<(),CipherError> {
        if self.key_word.len() == 0 {
            return Err(CipherError::Input(String::from("No input text provided")))
        }
        for c in text.chars() {
            if !self.alphabet.contains(c) { return Err(CipherError::invalid_input_char(c)) }
        }
        Ok(())
    }

    fn encrypt_char(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (t+k) % l ).unwrap()
    }

    fn decrypt_char(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (l+t-k) % l ).unwrap()
    }

}

impl Default for Playfair {
    fn default() -> Self {
        todo!()
    }
}

impl Cipher for Playfair {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        todo!()
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        todo!()
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
}