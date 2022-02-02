use std::collections::VecDeque;

use rand::prelude::ThreadRng;
use super::{Cipher, PolyMode};
use crate::text_functions::{LATIN_UPPER, random_sample_replace};
use crate::errors::CipherError;


pub struct Autokey {
    pub key_word: String,
    alphabet: String,
    pub mode: PolyMode,
}

impl Autokey {

    fn key_vals(&self) -> impl Iterator<Item = usize> + '_ {
        self.key_word.chars().map(|x| self.alphabet.chars().position(|c| c == x).unwrap())
    }

    pub fn alpahbet_len(&self) -> usize {
        self.alphabet.chars().count()
    }

    fn validate_key(&self) -> Result<(),CipherError> {
        if self.key_word.len() == 0 {
            return Err(CipherError::Key(String::from("No key word provided")))
        }
        for c in self.key_word.chars() {
            if !self.alphabet.contains(c) { return Err(CipherError::invalid_alphabet_char(c)) }
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
    fn encrypt_char_beau(&self, text: usize, key: usize, l: usize) -> (char, usize) {
        let pos = (l+key-text) % l;
        (self.alphabet.chars().nth( pos ).unwrap(), pos)
    }

    fn encrypt_char_vig(&self, text: usize, key: usize, l: usize) -> char {
        self.alphabet.chars().nth( (text+key) % l ).unwrap()
    }

    fn decrypt_char_vig(&self, text: usize, key: usize, l: usize) -> (char,usize) {
        let pos = (l+text-key) % l;
        (self.alphabet.chars().nth( pos ).unwrap(), pos)
    }

    pub fn prep(&self, text: &str) -> Result<(usize, Vec<usize>, VecDeque<usize>,String),CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        let alpha_len = self.alpahbet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let akey: VecDeque<usize> = self.key_vals().collect();
        let out = String::with_capacity(text_nums.len());

        Ok((alpha_len, text_nums, akey, out))
    }

    fn encrypt_vigenere(&self, text: &str) -> Result<String,CipherError> {

        let (alpha_len, 
            text_nums, 
            mut akey, 
            mut out) = self.prep(text)?;
        
        for n in text_nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.encrypt_char_vig(n, k,alpha_len) )
        }

        Ok(out)
    }

    fn decrypt_vigenere(&self, text: &str) -> Result<String,CipherError> {
        let (alpha_len, 
            text_nums, 
            mut akey, 
            mut out) = self.prep(text)?;

        for n in text_nums {
            let k = akey.pop_front().unwrap();
            let ptxt_char = self.decrypt_char_vig(n, k,alpha_len);
            out.push( ptxt_char.0 );
            akey.push_back(ptxt_char.1);
        }
        Ok(out)
    }

    fn encrypt_beaufort(&self, text: &str) -> Result<String,CipherError> {
        let (alpha_len, 
            text_nums, 
            mut akey, 
            mut out) = self.prep(text)?;

        for n in text_nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.encrypt_char_beau(n, k, alpha_len).0 )
        }
        Ok(out)
    }

    fn decrypt_beaufort(&self, text: &str) -> Result<String,CipherError> {
        let (alpha_len, 
            text_nums, 
            mut akey, 
            mut out) = self.prep(text)?;

        for n in text_nums {
            let k = akey.pop_front().unwrap();
            let ptxt_char = self.encrypt_char_beau(n, k,alpha_len);
            out.push( ptxt_char.0 );
            akey.push_back(ptxt_char.1);
        }
        Ok(out)
    }

}

impl Default for Autokey {
    fn default() -> Self {
        Self { key_word: String::new(), alphabet: String::from(LATIN_UPPER), mode: PolyMode::Vigenere }
    }
}

impl Cipher for Autokey {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        match self.mode {
            PolyMode::Vigenere => self.encrypt_vigenere(text),
            PolyMode::Beaufort => self.encrypt_beaufort(text),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        match self.mode {
            PolyMode::Vigenere => self.decrypt_vigenere(text),
            PolyMode::Beaufort => self.decrypt_beaufort(text),
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

#[cfg(test)]
mod autokey_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT_VIG: &'static str = "TIGTYBJORLWYXGFLFHRDPXPQGLVZPRSFHZG";
    const CIPHERTEXT_BEA: &'static str = "HUYNKLFUPDUGXWDRNTTZFVZIYZHRTRUJBXU";

    #[test]
    fn encrypt_test_vigenere() {
        let mut cipher = Autokey::default();
        cipher.key_word = String::from("ABCDE");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_VIG);
    }

    #[test]
    fn decrypt_test_vigenere() {
        let mut cipher = Autokey::default();
        cipher.key_word = String::from("ABCDE");
        assert_eq!(cipher.decrypt(CIPHERTEXT_VIG).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_beaufort() {
        let mut cipher = Autokey::default();
        cipher.key_word = String::from("ABCDE");
        cipher.mode = PolyMode::Beaufort;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_BEA);
    }

    #[test]
    fn decrypt_test_beaufort() {
        let mut cipher = Autokey::default();
        cipher.key_word = String::from("ABCDE");
        cipher.mode = PolyMode::Beaufort;
        assert_eq!(cipher.decrypt(CIPHERTEXT_BEA).unwrap(), PLAINTEXT);
    }
}