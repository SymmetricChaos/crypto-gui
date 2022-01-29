use std::collections::VecDeque;
use crate::errors::CipherError;
use rand::prelude::ThreadRng;
use super::{Cipher,PolyalphabeticMode};
use crate::text_functions::{LATIN_UPPER, random_sample_replace};



pub struct Beaufort {
    pub key_word: String,
    alphabet: String,
    pub mode: PolyalphabeticMode,
}

impl Beaufort {

    pub fn set_mode(&mut self, mode: PolyalphabeticMode) {
        self.mode = mode
    }

    fn cyclic_key_vals(&self) -> impl Iterator<Item = usize> + '_ {
        self.key_word.chars().map(|x| self.alphabet.chars().position(|c| c == x).unwrap()).cycle()
    }

    fn key_vals(&self) -> impl Iterator<Item = usize> + '_ {
        self.key_word.chars().map(|x| self.alphabet.chars().position(|c| c == x).unwrap())
    }

    fn alpahbet_len(&self) -> usize {
        self.alphabet.chars().count()
    }

    fn validate_key(&self) -> Result<(),CipherError> {
        if self.key_word.len() == 0 {
            return Err(CipherError::Key(String::from("No key word provided")))
        }
        for c in self.key_word.chars() {
            if !self.alphabet.contains(c) { return Err(CipherError::invalid_key_char(c)) }
        }
        Ok(())
    }

    fn validate_input(&self, text: &str) -> Result<(),CipherError> {
        for c in text.chars() {
            if !self.alphabet.contains(c) { return Err(CipherError::invalid_input_char(c)) }
        }
        Ok(())
    }

    // The Beaufort cipher is reciprocal so no decrypt methods are needed
    fn encrypt_char(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (l+k-t) % l ).unwrap()
    }

    fn encrypt_standard(&self, text: &str) -> Result<String,CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        let alpha_len = self.alpahbet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(text_nums.len());
        for (n,k) in text_nums.iter().zip(self.cyclic_key_vals()) {
            out.push(self.encrypt_char(*n, k, alpha_len) )
        }
        Ok(out)
    }

    fn encrypt_autokey(&self, text: &str) -> Result<String,CipherError> {
        self.validate_key()?;
        let alpha_len = self.alpahbet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut akey: VecDeque<usize> = self.key_vals().collect();
        let mut out = String::with_capacity(text_nums.len());
        for n in text_nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.encrypt_char(n, k, alpha_len) )
        }
        Ok(out)
    }


    fn encrypt_progressive_key(&self, text: &str, shift: u8) -> Result<String,CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        let alpha_len = self.alpahbet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(text_nums.len());
        let mut cur_shift = 0 as usize;
        let mut ctr = 0;
        let key_len = self.key_vals().count();
        for (n,k) in text_nums.iter().zip(self.cyclic_key_vals()) {
            out.push(self.encrypt_char(*n, k+cur_shift, alpha_len) );
            ctr = (ctr+1) % key_len;
            if ctr == 0 {
                cur_shift += shift as usize;
            }
        }
        Ok(out)
    }
}

impl Default for Beaufort {
    fn default() -> Self {
        Self { key_word: String::new(), alphabet: String::from(LATIN_UPPER), mode: PolyalphabeticMode::Cyclic }
    }
}

impl Cipher for Beaufort {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        match self.mode {
            PolyalphabeticMode::Cyclic => self.encrypt_standard(text),
            PolyalphabeticMode::Autokey => self.encrypt_autokey(text),
            PolyalphabeticMode::Progressive(shift) => self.encrypt_progressive_key(text, shift),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        match self.mode {
            PolyalphabeticMode::Cyclic => self.encrypt_standard(text),
            PolyalphabeticMode::Autokey => self.encrypt_autokey(text),
            PolyalphabeticMode::Progressive(shift) => self.encrypt_progressive_key(text, shift),
        }
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.key_word = random_sample_replace(&self.alphabet, 11, rng);
    
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
}