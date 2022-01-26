use rand::prelude::ThreadRng;
use super::Cipher;
use crate::text_functions::{LATIN_UPPER, random_sample_replace};

pub enum VigenereMode {
    Standard,
    Autokey,
}

pub struct Vigenere {
    pub key_word: String,
    alphabet: String,
    mode: VigenereMode,
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
}

impl Default for Vigenere {
    fn default() -> Self {
        Self { key_word: String::new(), alphabet: String::from(LATIN_UPPER), mode: VigenereMode::Standard }
    }
}

impl Cipher for Vigenere {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        self.validate_key()?;
        let length = self.alpahbet_len();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(nums.len());
        for (n,k) in nums.iter().zip(self.key_vals()) {
            out.push(self.alphabet.chars().nth( (n+k)%length ).unwrap() )
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,&'static str> {
        self.validate_key()?;
        let length = self.alpahbet_len();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() + length ).collect();
        let mut out = String::with_capacity(nums.len());
        for (n,k) in nums.iter().zip(self.key_vals()) {
            out.push(self.alphabet.chars().nth( (n-k)%length ).unwrap() )
        }
        Ok(out)
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