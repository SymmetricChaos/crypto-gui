use std::collections::VecDeque;

use rand::prelude::ThreadRng;
use super::Cipher;
use crate::text_functions::{LATIN_UPPER, random_sample_replace};

#[derive(Debug,Copy,Clone,PartialEq, Eq)]
pub enum VigenereMode {
    Standard,
    Autokey,
}

pub struct Vigenere {
    pub key_word: String,
    alphabet: String,
    pub mode: VigenereMode,
}

impl Vigenere {
    fn key_vals(&self) -> impl Iterator<Item = usize> + '_ {
        self.key_word.chars().map(|x| self.alphabet.chars().position(|c| c == x).unwrap()).cycle()
    }

    fn alpahbet_len(&self) -> usize {
        self.alphabet.chars().count()
    }

    fn validate_key(&self) -> Result<(),&'static str> {
        for c in self.key_word.chars() {
            if !self.alphabet.contains(c) { return Err("unknown character in key") }
        }
        Ok(())
    }

    fn validate_input(&self, text: &str) -> Result<(),&'static str> {
        for c in text.chars() {
            if !self.alphabet.contains(c) { return Err("unknown character in key") }
        }
        Ok(())
    }

    fn encrypt_char(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (t+k) % l ).unwrap()
    }

    fn decrypt_char(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (l+t-k) % l ).unwrap()
    }

    fn encrypt_standard(&self, text: &str) -> Result<String,&'static str> {
        self.validate_key()?;
        self.validate_input(text)?;
        let alpha_len = self.alpahbet_len();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(nums.len());
        for (n,k) in nums.iter().zip(self.key_vals()) {
            out.push(self.encrypt_char(*n,k,alpha_len) )
        }
        Ok(out)
    }

    fn decrypt_standard(&self, text: &str) -> Result<String,&'static str> {
        self.validate_key()?;
        self.validate_input(text)?;
        let alpha_len = self.alpahbet_len();
        let length = self.alpahbet_len();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() + length ).collect();
        let mut out = String::with_capacity(nums.len());
        for (n,k) in nums.iter().zip(self.key_vals()) {
            out.push(self.decrypt_char(*n,k,alpha_len) )
        }
        Ok(out)
    }



    fn encrypt_autokey(&self, text: &str) -> Result<String,&'static str> {
        self.validate_key()?;
        let alpha_len = self.alpahbet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut akey: VecDeque<usize> = self.key_vals().collect();
        let mut out = String::with_capacity(text_nums.len());

        for n in text_nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.encrypt_char(n,k,alpha_len) )
        }
        Ok(out)
    }

    fn decrypt_autokey(&self, text: &str) -> Result<String,&'static str> {
        self.validate_key()?;
        let alpha_len = self.alpahbet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut akey: VecDeque<usize> = self.key_vals().collect();
        let mut out = String::with_capacity(text_nums.len());

        for n in text_nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.decrypt_char(n,k,alpha_len) )
        }
        Ok(out)
    }
}

impl Default for Vigenere {
    fn default() -> Self {
        Self { key_word: String::new(), alphabet: String::from(LATIN_UPPER), mode: VigenereMode::Standard }
    }
}

impl Cipher for Vigenere {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        match self.mode {
            VigenereMode::Standard => self.encrypt_standard(text),
            VigenereMode::Autokey => self.encrypt_autokey(text),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String,&'static str> {
        match self.mode {
            VigenereMode::Standard => self.decrypt_standard(text),
            VigenereMode::Autokey => self.decrypt_autokey(text),
        }
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.key_word =  random_sample_replace(&self.alphabet, 11, rng);
    
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
}