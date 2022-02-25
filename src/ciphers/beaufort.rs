use std::collections::VecDeque;

use rand::prelude::ThreadRng;
use super::{Cipher, PolyMode};
use crate::text_functions::random_sample_replace;
use crate::errors::CipherError;
use crate::text_types::{PresetAlphabet::*};

pub struct Beaufort {
    pub key_words: [String; 5],
    alphabet: String,
    pub prog_shift: usize,
    pub mode: PolyMode,
    pub multikey: bool,
}

impl Beaufort {

    // Some weirdness needed to make types match
    pub fn key(&self) -> impl Iterator<Item = usize> + '_ {
        if self.multikey {
            let mut effective_key = vec![0usize; self.key_len()];
            for key in self.key_words.iter().filter(|s| !s.is_empty()) {
                for (pos, sym) in key.chars().cycle().take(self.key_len()).enumerate() {
                    let p = self.alphabet.chars().position(|c| c == sym).unwrap();
                    effective_key[pos] += p
                }
            }
            effective_key = effective_key.into_iter().map(|v| v % self.alphabet_len()).collect();
            effective_key.into_iter()
        } else {
            let key: Vec<usize> = self.key_words[0].chars().map(|x| self.alphabet.chars().position(|c| c == x).unwrap()).collect();
            key.into_iter()
        }

    }

    pub fn cyclic_key(&self) -> impl Iterator<Item = usize> + '_ {
        let v = self.key().collect::<Vec<usize>>();
        v.into_iter().cycle()
    }
 
    //Should multiply together ignoring common factors. [9,6] should give 18
    pub fn key_len(&self) -> usize {
        if self.multikey {
            self.key_words.iter().filter(|s| !s.is_empty()).map(|s| s.chars().count() ).fold(1, num::integer::lcm)
        } else {
            self.key_words[0].chars().count()
        }
    }
 
    // Unwrap justified by bounds on key
    pub fn key_word(&self) -> String {
        if self.multikey {
            self.key().map(|v| self.alphabet.chars().nth(v).unwrap()).collect()
        } else {
            self.key_words[0].clone()
        }
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
        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let mut out = String::with_capacity(text_nums.len());
        
        let mut cur_shift = 0;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (n, k) in text_nums.iter().zip(self.cyclic_key()) {
            out.push(self.encrypt_char(*n, (k+cur_shift) % alpha_len, alpha_len) );
            ctr = (ctr+1) % key_len;
            if ctr == 0 {
                cur_shift = (cur_shift + self.prog_shift) % alpha_len;
            }
        }
        Ok(out)
    }

}

impl Default for Beaufort {
    fn default() -> Self {
        Self { key_words: [String::new(), String::new(), String::new(), String::new(), String::new()], 
               alphabet: String::from(BasicLatin), 
               mode: PolyMode::CylicKey, 
               prog_shift: 0,
               multikey: false,        
        }
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
mod beaufort_tests {
    use super::*;

    const PLAINTEXT: &'static str =         "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGANDTHENSOMEMORETOMAKEALONGERPLAINTEXTFORTHISCIPHERTOUSE";
    const CIPHERTEXT_CYCLIC: &'static str = "LGYIVLGEMNZGLKFLFTSYKKUPRVGACCZRQUINRKJRLGQABABOHLAUCNUJCNBGCKWPCCQCAERUZZLFILQMFPGYHWFOWJ";
    const CIPHERTEXT_AUTO: &'static str =   "LGYIVLGEMCTIDPUFBHFZEZKKGQNIESPXBDNHRIHSUQWSOJRFQAUECMUIOGQGIIJVTADBUCHYKXJPGLMVLQHNCOUYKE";
    const CIPHERTEXT_PROG: &'static str =   "LGYIVLGEMQCJONIOIWYEQQAVXBMJLLIAZDRWDWVDXSCMNPQDWAPJRCMBUFTYUCOKXXLXVZMPXXJDGJOKDQHZIXGPXK";

    #[test]
    fn encrypt_test_cyclic() {
        let mut cipher = Beaufort::default();
        cipher.key_words[1] = String::from("GOOD");
        cipher.key_words[0] = String::from("ENCYPTION");

        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_CYCLIC);
    }

    #[test]
    fn decrypt_test_cyclic() {
        let mut cipher = Beaufort::default();
        cipher.key_words[1] = String::from("GOOD");
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_CYCLIC).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_auto() {
        let mut cipher = Beaufort::default();
        cipher.key_words[1] = String::from("GOOD");
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_AUTO);
    }

    #[test]
    fn decrypt_test_auto() {
        let mut cipher = Beaufort::default();
        cipher.key_words[1] = String::from("GOOD");
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_AUTO).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_prog() {
        let mut cipher = Beaufort::default();
        cipher.key_words[1] = String::from("GOOD");
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.prog_shift = 3;
        cipher.mode = PolyMode::ProgKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_PROG);
    }

    #[test]
    fn decrypt_test_prog() {
        let mut cipher = Beaufort::default();
        cipher.key_words[1] = String::from("GOOD");
        cipher.key_words[0] = String::from("ENCYPTION");
        cipher.prog_shift = 3;
        cipher.mode = PolyMode::ProgKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_PROG).unwrap(), PLAINTEXT);
    }
}