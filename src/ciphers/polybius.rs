use core::panic;
use std::fmt;
use itertools::Itertools;
use num::integer::Roots;
use rand::prelude::ThreadRng;
use super::Cipher;
use crate::errors::CipherErrors;
use crate::{errors::CipherError, text_functions::shuffled_str};
use crate::text_functions::{validate_alphabet, keyed_alphabet, PresetAlphabet};

pub struct Polybius {
    alphabet: String,
    inner_alphabet: String,
    labels: String,
    grid_side_len: usize,
    key_word: String,
}

impl Polybius {

    // Silently ignores invalid characters
    pub fn set_key(&mut self) -> &mut String {
        self.inner_alphabet = keyed_alphabet(&self.key_word, &self.alphabet);
        &mut self.key_word
    }

    pub fn set_mode(&mut self, mode: PresetAlphabet) {
        self.alphabet = mode.string();
        self.grid_side_len = mode.len().sqrt();
        if mode.len().sqrt().pow(2) != mode.len() {
            panic!("Cannot assign an alphabet with a non-square length to a square grid")
        }
    }

    fn pairs(&self, text: &str) -> Result<Vec<(char,char)>,CipherError> {
        if text.chars().count() % 2 == 1 {
            return Err(CipherError::input("Input text does not have an even number of characters."))
        }
        let out = text.chars().chunks(2).into_iter().map(|x| x.collect_tuple().unwrap()).collect();
        Ok(out)
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.chars().count()
    }

    pub fn grid_side_len(&self) -> usize {
        self.alphabet_len().sqrt()
    }

    fn char_to_position(&self,symbol: char) -> Result<(usize,usize),CipherError> {
        let num = match self.alphabet.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }
    
    // fn position_to_char(&self,position: (usize,usize)) -> char {
    //     let num = position.0*self.alphabet_len() + position.1;
    //     self.alphabet.chars().nth(num).unwrap()
    // }
}

impl Default for Polybius {
    fn default() -> Self {
        Self{ alphabet: String::from(PresetAlphabet::EnglishNoQ),
              inner_alphabet: String::from(PresetAlphabet::EnglishNoQ),
              grid_side_len: 5,
              labels: String::from(PresetAlphabet::Digits1),
              key_word: String::new(), }
    }
}

impl Cipher for Polybius {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        //self.validate_settings()?;
        let mut out = String::with_capacity(text.chars().count()*2);

        for c in text.chars() {
            let pos = self.char_to_position(c)?;
            out.push(self.labels.chars().nth(pos.0).unwrap());
            out.push(self.labels.chars().nth(pos.1).unwrap());

        }
        Ok(out)
    }
    
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        //self.validate_settings()?;
        let pairs = self.pairs(text)?;
        let mut out = String::with_capacity(text.chars().count()/2);
        for (l, r) in pairs {
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.key_word = shuffled_str(&self.inner_alphabet, rng)
    }

    fn get_input_alphabet(&mut self) -> &String {
        &mut self.alphabet
    }

    fn get_output_alphabet(&mut self) -> &String {
        &mut self.alphabet
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn validate_settings(&self) -> Result<(),CipherErrors> {
        let mut errors = Vec::new();
        match validate_alphabet(&self.alphabet) {
            Ok(_) => (),
            Err(e) => errors.push(e),
        }
        if errors.is_empty() {
            return Ok(())
        }
        Err(CipherErrors::new(errors))
    }
}


impl fmt::Display for Polybius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = String::from("  ");
        for xlab in self.labels.chars().take(self.grid_side_len) {
            square.push_str(&format!("{xlab} "))
        }
        for (n, c) in self.inner_alphabet.chars().enumerate() {
            if n % self.grid_side_len() == 0 {
                let ylab = self.labels.chars().nth(n/self.grid_side_len).unwrap();
                square.push_str(&format!("\n{ylab} "));
            }
            square.push_str(&format!("{c} "))
        };
        write!(f, "{square}")
    }
}