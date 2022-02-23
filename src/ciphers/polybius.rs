use std::fmt;
use itertools::Itertools;
use num::integer::Roots;
use rand::prelude::ThreadRng;
use super::Cipher;
use crate::{errors::CipherError, text_functions::shuffled_str};
use crate::text_functions::{validate_alphabet, keyed_alphabet};
use crate::text_types::{PresetAlphabet::*, PresetAlphabet};

pub struct Polybius {
    alphabet: String,
    inner_alphabet: String,
    labels: String,
    grid_side_len: usize,
    key_word: String,
}

impl Polybius {

    // Silently ignores invalid characters
    pub fn control_key(&mut self) -> &mut String {
        self.inner_alphabet = keyed_alphabet(&self.key_word, &self.alphabet);
        &mut self.key_word
    }

    pub fn set_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.inner_alphabet = keyed_alphabet(key_word, &self.alphabet);
    }

    pub fn set_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoJ | BasicLatinNoQ | BasicLatinWithDigits | Base64 => {
                self.alphabet = mode.string();
                self.inner_alphabet = mode.string();
                self.grid_side_len = mode.len().sqrt();
            }
            _ => ()
        }
    }

    pub fn set_labels(&mut self, labels: String) {
        self.labels = labels
    }

    pub fn get_labels(&self) -> &String {
        &self.labels
    }

    fn pairs(&self, text: &str) -> Result<Vec<(char,char)>,CipherError> {
        if text.chars().count() % 2 != 0 {
            dbg!(text);
            dbg!(text.chars().count());
            return Err(CipherError::input("Input text does not have an even number of characters."))
        }
        let out = text.chars().chunks(2).into_iter().map(|x| x.collect_tuple().unwrap()).collect();
        Ok(out)
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.chars().count()
    }

    fn char_to_position(&self,symbol: char) -> Result<(usize,usize),CipherError> {
        let num = match self.alphabet.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }
    
    fn position_to_char(&self,position: (char,char)) -> char {
        let y = self.labels.chars().position(|c| c == position.0).unwrap();
        let x = self.labels.chars().position(|c| c == position.1).unwrap();

        let num = y*self.grid_side_len + x;
        self.alphabet.chars().nth(num).unwrap()
    }
}

impl Default for Polybius {
    fn default() -> Self {
        Self{ alphabet: String::from(PresetAlphabet::BasicLatinNoQ),
              inner_alphabet: String::from(PresetAlphabet::BasicLatinNoQ),
              grid_side_len: 5,
              labels: String::from(PresetAlphabet::Digits1),
              key_word: String::new(), }
    }
}

impl Cipher for Polybius {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut out = String::with_capacity(text.chars().count()*2);

        for c in text.chars() {
            let pos = self.char_to_position(c)?;
            out.push(self.labels.chars().nth(pos.0).unwrap());
            out.push(self.labels.chars().nth(pos.1).unwrap());
        }
        Ok(out)
    }
    
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let pairs = self.pairs(text)?;
        let mut out = String::with_capacity(text.chars().count()/2);

        for p in pairs {
            out.push(self.position_to_char(p));
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.key_word = shuffled_str(&self.inner_alphabet, rng)
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
        validate_alphabet(&self.alphabet)?;
        Ok(())
    }
}


impl fmt::Display for Polybius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = String::from("  ");
        for xlab in self.labels.chars().take(self.grid_side_len) {
            square.push_str(&format!("{xlab} "))
        }
        for (n, c) in self.inner_alphabet.chars().enumerate() {
            if n % self.grid_side_len == 0 {
                let ylab = self.labels.chars().nth(n/self.grid_side_len).unwrap();
                square.push_str(&format!("\n{ylab} "));
            }
            square.push_str(&format!("{c} "))
        };
        write!(f, "{square}")
    }
}



#[cfg(test)]
mod polybius_tests {
    use super::*;

    // Note Q replaced by K
    const PLAINTEXT: &'static str =  "THEKUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "4423153145241331124235523421355325453341433551154244231532115554143522";

    #[test]
    fn encrypt_test() {
        let mut cipher = Polybius::default();
        cipher.set_key("INVENTORY");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Polybius::default();
        cipher.set_key("INVENTORY");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}