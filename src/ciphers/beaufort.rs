use std::collections::VecDeque;

use rand::prelude::ThreadRng;
use super::{Cipher, PolyMode};
use crate::text_functions::{random_sample_replace, PresetAlphabet};
use crate::errors::CipherError;


pub struct Beaufort {
    pub key_word: String,
    alphabet: String,
    pub prog_shift: usize,
    pub mode: PolyMode,
}

impl Beaufort {

    pub fn cyclic_key(&self) -> impl Iterator<Item = usize> + '_ {
        self.key_word.chars().map(|x| self.alphabet.chars().position(|c| c == x).unwrap()).cycle()
    }

    pub fn key(&self) -> impl Iterator<Item = usize> + '_ {
        self.key_word.chars().map(|x| self.alphabet.chars().position(|c| c == x).unwrap())
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.chars().count()
    }

    pub fn key_len(&self) -> usize {
        self.key().count()
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


    pub fn prep(&self, text: &str) -> Result<(usize, Vec<usize>, VecDeque<usize>,String),CipherError> {
        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let akey: VecDeque<usize> = self.key().collect();
        let out = String::with_capacity(text_nums.len());

        Ok((alpha_len, text_nums, akey, out))
    }


    // The Beaufort cipher is reciprocal so no decrypt methods are needed
    fn encrypt_char(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (l+k-t) % l ).unwrap()
    }

    fn encrypt_cyclic(&self, text: &str) -> Result<String,CipherError> {
        let alpha_len = self.alphabet_len();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(nums.len());
        for (n,k) in nums.iter().zip(self.cyclic_key()) {
            out.push(self.encrypt_char(*n,k,alpha_len) )
        }
        Ok(out)
    }

    fn decrypt_cyclic(&self, text: &str) -> Result<String,CipherError> {
        self.encrypt_cyclic(text)
    }

    fn encrypt_auto(&self, text: &str) -> Result<String,CipherError> {
        let (alpha_len, 
             text_nums, 
             mut akey, 
             mut out) = self.prep(text)?;
        
        for n in text_nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.encrypt_char(n, k,alpha_len) )
        }

        Ok(out)
    }

    fn decrypt_auto(&self, text: &str) -> Result<String,CipherError> {
        let (alpha_len, 
             text_nums, 
             mut akey, 
             mut out) = self.prep(text)?;

        for n in text_nums {
            let k = akey.pop_front().unwrap();
            let ptxt_char = self.encrypt_char(n, k,alpha_len);
            out.push( ptxt_char );
            let new_key_val = self.alphabet.chars().position(|x| x == ptxt_char).unwrap();
            akey.push_back( new_key_val );
        }
        Ok(out)
    }

    fn encrypt_prog(&self, text: &str) -> Result<String,CipherError> {
        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(text_nums.len());
        
        let mut cur_shift = 0 as usize;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (n, k) in text_nums.iter().zip(self.cyclic_key()) {
            out.push(self.encrypt_char(*n, k+cur_shift, alpha_len) );
            ctr = (ctr+1) % key_len;
            if ctr == 0 {
                cur_shift += self.prog_shift;
            }
        }
        Ok(out)
    }

    fn decrypt_prog(&self, text: &str) -> Result<String,CipherError> {
        self.encrypt_prog(text)
    }

}

impl Default for Beaufort {
    fn default() -> Self {
        Self { key_word: String::new(), alphabet: String::from(PresetAlphabet::BasicLatin), mode: PolyMode::CylicKey, prog_shift: 0 }
    }
}

impl Cipher for Beaufort {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        match self.mode {
            PolyMode::CylicKey => self.encrypt_cyclic(text),
            PolyMode::Autokey => self.encrypt_auto(text),
            PolyMode::ProgKey => self.encrypt_prog(text),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        match self.mode {
            PolyMode::CylicKey => self.decrypt_cyclic(text),
            PolyMode::Autokey => self.decrypt_auto(text),
            PolyMode::ProgKey => self.decrypt_prog(text),
        }
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.key_word =  random_sample_replace(&self.alphabet, 11, rng);
    
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        todo!()
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet
    }

    fn get_output_alphabet(&self) -> &String {
        &self.alphabet
    }
}

#[cfg(test)]
mod beaufort_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT_CYCLIC: &'static str = "LGYIVLGEMNZGLKFLFTSYKKUPRVGACCZRQUI";
    const CIPHERTEXT_AUTO: &'static str = "LGYIVLGEMCTIDPUFBHFZEZKKGQNIESPXBDN";
    const CIPHERTEXT_PROG: &'static str = "LGYIVLGEMQCJONIOIWYEQQAVXBMJLLIAZDR";

    #[test]
    fn encrypt_test_cyclic() {
        let mut cipher = Beaufort::default();
        cipher.key_word = String::from("ENCYPTION");
        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_CYCLIC);
    }

    #[test]
    fn decrypt_test_cyclic() {
        let mut cipher = Beaufort::default();
        cipher.key_word = String::from("ENCYPTION");
        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_CYCLIC).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_auto() {
        let mut cipher = Beaufort::default();
        cipher.key_word = String::from("ENCYPTION");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_AUTO);
    }

    #[test]
    fn decrypt_test_auto() {
        let mut cipher = Beaufort::default();
        cipher.key_word = String::from("ENCYPTION");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_AUTO).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_prog() {
        let mut cipher = Beaufort::default();
        cipher.key_word = String::from("ENCYPTION");
        cipher.prog_shift = 3;
        cipher.mode = PolyMode::ProgKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_PROG);
    }

    #[test]
    fn decrypt_test_prog() {
        let mut cipher = Beaufort::default();
        cipher.key_word = String::from("ENCYPTION");
        cipher.prog_shift = 3;
        cipher.mode = PolyMode::ProgKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_PROG).unwrap(), PLAINTEXT);
    }
}