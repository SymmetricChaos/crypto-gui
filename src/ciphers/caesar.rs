use rand::{Rng, prelude::StdRng};
use crate::errors::CipherError;
use crate::text_aux::{prep_text, Alphabet, PresetAlphabet::*};
use super::Cipher;

pub struct Caesar {
    pub shift: i32,
    pub alphabet: Alphabet,
    pub alphabet_string: String,
}

impl Caesar {

    fn encrypt_char(&self, c: char) -> char {
        self.alphabet.get_shifted_char(c, self.shift).unwrap()
    }
 
    fn decrypt_char(&self, c: char) -> char {
        self.alphabet.get_shifted_char(c, -self.shift).unwrap()
    }

    pub fn check_input(&self, text: &str) -> Result<(), CipherError> {
        for c in text.chars() {
            if !self.alphabet.contains(c) {
                return Err(CipherError::invalid_input_char(c))
            }
        }
        Ok(())
    }

    pub fn set_alphabet(&mut self) {
        self.alphabet = Alphabet::from(&self.alphabet_string);
    }

    pub fn control_alphabet(&mut self) -> &mut String {
        self.alphabet = Alphabet::from(&self.alphabet_string);
        &mut self.alphabet_string
    }

    fn _validate_settings(&self) -> Result<(), CipherError> {
        if (self.shift as usize) > self.alphabet.len() {
            return Err(CipherError::Key(String::from("shift value must be less than the alphabet length")))
        }
        Ok(())
    }

}

impl Default for Caesar {
    fn default() -> Self {
        Self { shift: 0, alphabet: Alphabet::from(BasicLatin), alphabet_string: String::from(BasicLatin) }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let text = prep_text(text, &self.alphabet.to_string())?;
        Ok( text.chars().map(|c| self.encrypt_char(c)).collect() )
    }
 
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let text = prep_text(text, &self.alphabet.to_string())?;
        Ok( text.chars().map(|c| self.decrypt_char(c)).collect() )

    }

    fn randomize(&mut self, rng: &mut StdRng) {
        self.shift = rng.gen_range(0..self.alphabet.len()) as i32;
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