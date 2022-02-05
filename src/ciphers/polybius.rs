use std::fmt;
use itertools::Itertools;
use num::integer::Roots;
use rand::prelude::ThreadRng;
use super::Cipher;
use crate::errors::CipherErrors;
use crate::{errors::CipherError, text_functions::shuffled_str};
use crate::text_functions::{LATIN_UPPER_NO_J, LATIN_UPPER_NO_Q, LATIN_UPPER_DIGITS, validate_alphabet, keyed_alphabet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolybiusMode {
    NoQ,
    NoJ,
    AlphaNum,
}

pub struct Polybius {
    alphabet: String,
    key_word: String,
}

impl Polybius {

    pub fn set_mode(&mut self, mode: PolybiusMode) {
        match mode {
            PolybiusMode::NoQ => self.alphabet = String::from(LATIN_UPPER_NO_Q),
            PolybiusMode::NoJ => self.alphabet = String::from(LATIN_UPPER_NO_J),
            PolybiusMode::AlphaNum => self.alphabet = String::from(LATIN_UPPER_DIGITS),
        };
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

    pub fn grid_size(&self) -> usize {
        self.alphabet_len().sqrt()
    }

    fn char_to_position(&self,symbol: char) -> Result<(usize,usize),CipherError> {
        let num = match self.alphabet.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.alphabet_len(), num % self.alphabet_len()))
    }
    
    // The inputs to this come only from internal functions that will never give invalid positions
    fn position_to_char(&self,position: (usize,usize)) -> char {
        let num = position.0*self.alphabet_len() + position.1;
        self.alphabet.chars().nth(num).unwrap()
    }


}

impl Default for Polybius {
    fn default() -> Self {
        Self{ alphabet: String::from("ABCDEFGHIJKLMNOPRSTUVWXYZ"), key_word: String::new() }
    }
}

impl Cipher for Polybius {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text)?;
        let mut out = String::with_capacity(text.chars().count());
        let size = self.size();
        let s = size+1;
        for (l,r) in pairs {
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            
            // The Polybius_pairs() function ensures l and r never match
            self.Polybius_shift(lpos, rpos, size, s, &mut out);
        }
        Ok(out)
    }
    
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text)?;
        let mut out = String::with_capacity(text.chars().count());
        let size = self.size();
        let s = size-1;
        for (l,r) in pairs {
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            
            // The Polybius_pairs() function ensures l and r never match
            self.Polybius_shift(lpos, rpos, size, s, &mut out);
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.alphabet = shuffled_str(&self.alphabet, rng)
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
        let kalpha = keyed_alphabet(&self.key_word, &self.alphabet).unwrap();
        let mut square = String::new();
        for (n, c) in kalpha.chars().enumerate() {
            if n % self.grid_size() == 0 {
                square.push_str("\n")
            }
            square.push_str(&format!("{} ",c))
        };
        write!(f, "{}", square)
    }
}