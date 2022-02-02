use rand::prelude::ThreadRng;
use super::{Cipher, PolyMode};
use crate::text_functions::{LATIN_UPPER, random_sample_replace};
use crate::errors::CipherError;


pub struct ProgressiveKey {
    pub key_word: String,
    alphabet: String,
    pub shift: usize,
    pub mode: PolyMode,
}

impl ProgressiveKey {

    fn key_vals(&self) -> impl Iterator<Item = usize> + '_ {
        self.key_word.chars().map(|x| self.alphabet.chars().position(|c| c == x).unwrap()).cycle()
    }

    fn key_len(&self) -> usize {
        self.key_word.chars().count()
    }

    pub fn alphabet_len(&self) -> usize {
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
        if text.len() == 0 {
            return Err(CipherError::Input(String::from("No input text provided")))
        }
        for c in text.chars() {
            if !self.alphabet.contains(c) { return Err(CipherError::invalid_input_char(c)) }
        }
        Ok(())
    }


    // Unwraps for the character methods are justified by validating the input

    // The Beaufort cipher is reciprocal so no decrypt methods are needed
    fn encrypt_char_beau(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (l+k-t) % l ).unwrap()
    }

    fn encrypt_char_vig(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (t+k) % l ).unwrap()
    }

    fn decrypt_char_vig(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (l+t-k) % l ).unwrap()
    }



    fn encrypt_vigenere(&self, text: &str) -> Result<String,CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;

        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(text_nums.len());
        
        let mut cur_shift = 0 as usize;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (n, k) in text_nums.iter().zip(self.key_vals()) {
            out.push(self.encrypt_char_vig(*n, k+cur_shift, alpha_len) );
            ctr = (ctr+1) % key_len;
            if ctr == 0 {
                cur_shift += self.shift;
            }
        }
        Ok(out)
    }

    fn decrypt_vigenere(&self, text: &str) -> Result<String,CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        
        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(text_nums.len());
        
        let mut cur_shift = 0;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (n, k) in text_nums.iter().zip(self.key_vals()) {
            out.push(self.decrypt_char_vig(*n, k+cur_shift, alpha_len) );
            ctr = (ctr+1) % key_len;
            if ctr == 0 {
                cur_shift += self.shift;
            }
        }
        Ok(out)
    }

    // There is no decrypt for Beaufort because it is reciprocal
    fn encrypt_beaufort(&self, text: &str) -> Result<String,CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        
        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(text_nums.len());
        
        let mut cur_shift = 0;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (n, k) in text_nums.iter().zip(self.key_vals()) {
            out.push(self.encrypt_char_beau(*n, k+cur_shift, alpha_len) );
            ctr = (ctr+1) % key_len;
            if ctr == 0 {
                cur_shift += self.shift;
            }
        }
        Ok(out)
    }
}

impl Default for ProgressiveKey {
    fn default() -> Self {
        Self { key_word: String::new(), alphabet: String::from(LATIN_UPPER), shift: 0, mode: PolyMode::Vigenere }
    }
}

impl Cipher for ProgressiveKey {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        match self.mode {
            PolyMode::Vigenere => self.encrypt_vigenere(text),
            PolyMode::Beaufort => self.encrypt_beaufort(text),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        match self.mode {
            PolyMode::Vigenere => self.decrypt_vigenere(text),
            PolyMode::Beaufort => self.encrypt_beaufort(text),
        }
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.key_word =  random_sample_replace(&self.alphabet, 11, rng);
    
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

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }
}