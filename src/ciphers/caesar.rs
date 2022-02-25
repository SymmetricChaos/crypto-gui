use rand::{Rng, prelude::ThreadRng};
use crate::{errors::{CipherError}, text_types::Alphabet};
use super::Cipher;
use crate::text_types::{PresetAlphabet::*};

pub struct Caesar {
    pub shift: i32,
    pub alphabet: Alphabet,
}

impl Caesar {
    fn encrypt_char(&self, c: char) -> char {
        self.alphabet.offset_char(c, self.shift).unwrap()
    }
 
    fn decrypt_char(&self, c: char) -> char {
        self.alphabet.offset_char(c, -self.shift).unwrap()
    }

    pub fn check_input(&self, text: &str) -> Result<(), CipherError> {
        for c in text.chars() {
            if !self.alphabet.contains(c) {
                return Err(CipherError::invalid_input_char(c))
            }
        }
        Ok(())
    }
}

impl Default for Caesar {
    fn default() -> Self {
        Self { shift: 0, alphabet: Alphabet::from(BasicLatin) }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        self.check_input(text)?;
        Ok( text.chars().map(|c| self.encrypt_char(c)).collect() )
    }
 
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.check_input(text)?;
        Ok( text.chars().map(|c| self.decrypt_char(c)).collect() )

    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.shift = rng.gen_range(0..self.alphabet.len()) as i32;
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet.inner
    }

    fn get_output_alphabet(&self) -> &String {
        &self.alphabet.inner
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet.inner
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet.inner
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if (self.shift as usize) > self.alphabet.len() {
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