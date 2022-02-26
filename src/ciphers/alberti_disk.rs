use std::fmt::Display;
use rand::{prelude::ThreadRng, Rng};
use crate::errors::CipherError;
use crate::text_types::{PresetAlphabet::*};
use super::Cipher;

pub struct Alberti {
    pub fixed_alphabet: String,
    pub moving_alphabet: String,
    pub start_index: usize,
}
 
impl Alberti {
 
    pub fn control_fixed_alphabet(&mut self) -> &mut String {
        &mut self.fixed_alphabet
    }

    pub fn control_moving_alphabet(&mut self) -> &mut String {
        &mut self.fixed_alphabet
    }

    fn encrypt_char(&self, symbol: char, index: usize) -> char {
        let position = self.fixed_alphabet.chars().position(|x| x == symbol).unwrap();
        self.moving_alphabet.chars().nth((position + index) % self.alphabet_len()).unwrap()
    }
 
    fn decrypt_char(&self, symbol: char, index: usize) -> char {
        let position = self.moving_alphabet.chars().position(|x| x == symbol).unwrap();
        self.fixed_alphabet.chars().nth((self.alphabet_len() + position - index) % self.alphabet_len()).unwrap()
    }
 
 
    pub fn alphabet_len(&self) -> usize {
        self.fixed_alphabet.chars().count()
    }
 

}

impl Cipher for Alberti {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut index = self.start_index.clone();
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.fixed_alphabet.contains(s) {
                out.push(self.encrypt_char(s,index));
            } else if self.moving_alphabet.contains(s) {
                index = self.moving_alphabet.chars().position(|x| x == s).unwrap();
                out.push(self.fixed_alphabet.chars().nth(index).unwrap());
            } else {
                return Err(CipherError::invalid_input_char(s))
            }
        }
        Ok(out)
    }


    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut index = self.start_index.clone();
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.moving_alphabet.contains(s) {
                out.push(self.decrypt_char(s,index));
            } else if self.fixed_alphabet.contains(s) {
                index = self.fixed_alphabet.chars().position(|x| x == s).unwrap();
                out.push(self.moving_alphabet.chars().nth(index).unwrap());
            } else {
                return Err(CipherError::invalid_input_char(s))
            }
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        let length = self.moving_alphabet.len();
        self.start_index = rng.gen_range(0..length);
    }

    fn get_input_alphabet(&self) -> &String {
        &self.fixed_alphabet
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.fixed_alphabet
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

}
 
impl Default for Alberti {
    fn default() -> Self {
        Self{ fixed_alphabet:  String::from(BasicLatin), 
              moving_alphabet: String::from(BasicLatin.string().to_ascii_lowercase()),
              start_index: 0} 
    }
}

impl Display for Alberti {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.fixed_alphabet.clone();
        out.push('\n');
        out.push_str(&self.moving_alphabet[self.start_index..]);
        out.push_str(&self.moving_alphabet[0..self.start_index]);
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod alberti_tests {
    use super::*;

    const PLAINTEXT:  &'static str = "THEQUItCKBReOWNFOsXJUMPStOVERTiHELAZYDnOG";
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
