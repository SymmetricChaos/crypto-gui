use std::fmt::Display;

use crate::errors::CipherError;

use super::Cipher;

pub struct Alberti {
    fixed_alphabet: String,
    moving_alphabet: String,
    index: usize,
}
 
impl Alberti {
    pub fn validate_settings(&self) -> Result<(),CipherError> {
        for c in self.fixed_alphabet.chars() {
            if self.moving_alphabet.contains(c) {
                return Err(CipherError::key("The two alphabets must have no symbols in common."))
            }
        }
 
        if self.fixed_alphabet.chars().count() != self.moving_alphabet.chars().count() {
            return Err(CipherError::key("The two alphabets must have identical length."))
        }
 
        if self.index >= self.fixed_alphabet.chars().count() {
            return Err(CipherError::key("The index cannot exceed the length of the alphabet."))
        }

        Ok(())
    }

    pub fn alphabet_length(&self) -> usize {
        self.fixed_alphabet.chars().count()
    }
 
    fn fixed_symbol_position(&self, symbol: char) -> usize {
        self.fixed_alphabet.chars().position(|x| x == symbol).unwrap()
    }
 
    fn moving_symbol_position(&self, symbol: char) -> usize {
        self.moving_alphabet.chars().position(|x| x == symbol).unwrap()
    }
 
}

impl Cipher for Alberti {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut index = self.index.clone();
        let length = self.alphabet_length();
        let symbols = text.chars();
        let mut out = String::with_capacity(text.len());
        for s in symbols {
            let p = (self.fixed_symbol_position(s) + index) % length;
            let f = self.moving_alphabet.chars().nth(p).unwrap();
            out.push(f);
            if s.is_ascii_digit() {
                index = (index + (s.to_digit(10).unwrap() as usize)) % length;
            }
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        todo!()
    }

    fn randomize(&mut self, rng: &mut rand::prelude::ThreadRng) {
        todo!()
    }

    fn input_alphabet(&mut self) -> &mut String {
        todo!()
    }

    fn output_alphabet(&mut self) -> &mut String {
        todo!()
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }

}
 
impl Default for Alberti {
    fn default() -> Self {
        Self{ fixed_alphabet: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234"), 
              moving_alphabet: String::from("abcdefghijklmnopqrstuvwxyz!@#$"),
              index: 0} 
    }
}

impl Display for Alberti {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.fixed_alphabet.clone();
        out.push_str(&self.fixed_alphabet[self.index..]);
        out.push_str(&self.fixed_alphabet[0..self.index]);
        write!(f, "{}", out)
    }
}