use std::collections::{VecDeque};

use rand::prelude::ThreadRng;
use super::{Cipher, PolyMode};
use crate::text_functions::{random_sample_replace, PresetAlphabet};
use crate::errors::CipherError;


pub struct VigenereMultiKey {
    pub key_words: [String; 5],
    alphabet: String,
    pub prog_shift: usize,
    pub mode: PolyMode,
}

impl VigenereMultiKey {

    pub fn cyclic_key(&self) -> impl Iterator<Item = usize> + '_ {
        let mut effective_key: Vec<usize> = Vec::with_capacity(self.key_len());
        for key in self.key_words.iter() {
            for (sym, num) in key.chars().cycle().zip(effective_key.iter_mut()) {
                let p = self.alphabet.chars().position(|c| c == sym).unwrap();
                *num += p;
            }
        }
        effective_key.into_iter().map(|v| v % self.alphabet_len()).cycle()
    }

    //Should multiply together ignoring common factors. [9,6] should give 18
    pub fn key_len(&self) -> usize {
        self.key_words.iter().map(|s| s.chars().count()).fold(1, num::integer::lcm)
    }

    pub fn key(&self) -> impl Iterator<Item = usize> + '_ {
        let mut effective_key: Vec<usize> = Vec::with_capacity(self.key_len());
        for key in self.key_words.iter() {
            for (sym, num) in key.chars().cycle().zip(effective_key.iter_mut()) {
                let p = self.alphabet.chars().position(|c| c == sym).unwrap();
                *num += p;
            }
        }
        effective_key.into_iter().map(|v| v % self.alphabet_len())
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.chars().count()
    }

    fn validate_key(&self) -> Result<(),CipherError> {
        for key in self.key_words.iter() {
            for c in key.chars() {
                if !self.alphabet.contains(c) { return Err(CipherError::invalid_alphabet_char(c)) }
            }
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


    fn autokey_prep(&self, text: &str) -> Result<(usize, Vec<usize>, VecDeque<usize>,String),CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let akey: VecDeque<usize> = self.key().collect();
        let out = String::with_capacity(text_nums.len());

        Ok((alpha_len, text_nums, akey, out))
    }


    // Unwraps for the character methods are justified by validating the input
    fn encrypt_char(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (t+k) % l ).unwrap()
    }

    fn decrypt_char(&self, t: usize, k: usize, l: usize) -> char {
        self.alphabet.chars().nth( (l+t-k) % l ).unwrap()
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
        let alpha_len = self.alphabet_len();
        let length = self.alphabet_len();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() + length ).collect();
        let mut out = String::with_capacity(nums.len());
        for (n,k) in nums.iter().zip(self.cyclic_key()) {
            out.push(self.decrypt_char(*n,k,alpha_len) )
        }
        Ok(out)
    }

    fn encrypt_auto(&self, text: &str) -> Result<String,CipherError> {
        let (alpha_len, 
             text_nums, 
             mut akey, 
             mut out) = self.autokey_prep(text)?;
        
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
             mut out) = self.autokey_prep(text)?;

        for n in text_nums {
            let k = akey.pop_front().unwrap();
            let ptxt_char = self.decrypt_char(n, k,alpha_len);
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
        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(text_nums.len());
        
        let mut cur_shift = 0;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (n, k) in text_nums.iter().zip(self.cyclic_key()) {
            out.push(self.decrypt_char(*n, (k+cur_shift) % alpha_len, alpha_len) );
            ctr = (ctr+1) % key_len;
            if ctr == 0 {
                cur_shift = (cur_shift + self.prog_shift) % alpha_len;
            }
        }
        Ok(out)
    }

}

impl Default for VigenereMultiKey {
    fn default() -> Self {
        Self { key_words: [String::new(), String::new(), String::new(), String::new(), String::new()], 
               alphabet: String::from(PresetAlphabet::English), 
               mode: PolyMode::CylicKey, 
               prog_shift: 0 }
    }
}

impl Cipher for VigenereMultiKey {
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
        self.key_words[0] = random_sample_replace(&self.alphabet, 3, rng);
        self.key_words[1] = random_sample_replace(&self.alphabet, 5, rng);
        self.key_words[2] = random_sample_replace(&self.alphabet, 7, rng);
        self.key_words[3] = String::new();
        self.key_words[4] = String::new();
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
mod vigenere_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT_CYCLIC: &'static str = "XUGOJBKYOVBYLUHFXHQCUMKXZHUIYCXNWWU";
    const CIPHERTEXT_AUTO: &'static str = "XUGOJBKYOKVADZWZTVDDOBASOCBQASNTHFZ";
    const CIPHERTEXT_PROG: &'static str = "XUGOJBKYOYEBOXKIAKWIASQDFNARHLGWFFD";

    #[test]
    fn encrypt_test_cyclic() {
        let mut cipher = VigenereMultiKey::default();
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.key_words[1] = String::from("IS");
        cipher.key_words[2] = String::from("COMPLEX");

        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_CYCLIC);
    }

    #[test]
    fn decrypt_test_cyclic() {
        let mut cipher = VigenereMultiKey::default();
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.key_words[1] = String::from("IS");
        cipher.key_words[2] = String::from("COMPLEX");
        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_CYCLIC).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_auto() {
        let mut cipher = VigenereMultiKey::default();
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.key_words[1] = String::from("IS");
        cipher.key_words[2] = String::from("COMPLEX");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_AUTO);
    }

    #[test]
    fn decrypt_test_auto() {
        let mut cipher = VigenereMultiKey::default();
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.key_words[1] = String::from("IS");
        cipher.key_words[2] = String::from("COMPLEX");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_AUTO).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_prog() {
        let mut cipher = VigenereMultiKey::default();
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.key_words[1] = String::from("IS");
        cipher.key_words[2] = String::from("COMPLEX");
        cipher.prog_shift = 3;
        cipher.mode = PolyMode::ProgKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_PROG);
    }

    #[test]
    fn decrypt_test_prog() {
        let mut cipher = VigenereMultiKey::default();
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.key_words[1] = String::from("IS");
        cipher.key_words[2] = String::from("COMPLEX");
        cipher.prog_shift = 3;
        cipher.mode = PolyMode::ProgKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_PROG).unwrap(), PLAINTEXT);
    }
}