use std::fmt;

use rand::prelude::ThreadRng;
use super::Cipher;
use crate::text_types::{PresetAlphabet::*, Alphabet};
use crate::{errors::CipherError, text_functions::shuffled_str};
use crate::text_functions::keyed_alphabet;

pub struct Slidefair {
    alphabet: Alphabet,
    key_word: String,
    spacer: char,
}

impl Slidefair {

    pub fn cyclic_key(&self) -> impl Iterator<Item = usize> + '_ {
        let v = self.key().collect::<Vec<usize>>();
        v.into_iter().cycle()
    }

    pub fn key(&self) -> impl Iterator<Item = usize> + '_ {
        let key: Vec<usize> = self.key_word.chars().map(|x| self.alphabet.pos(x,0).unwrap()).collect();
        key.into_iter()
    }

    // Silently ignores invalid characters
    pub fn control_key(&mut self) -> &mut String {
        self.alphabet = Alphabet::from(keyed_alphabet(&self.key_word, self.alphabet.slice()));
        &mut self.key_word
    }

    pub fn set_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.alphabet = Alphabet::from(keyed_alphabet(key_word, self.alphabet.slice()));
    }

    pub fn control_spacer(&mut self) -> &mut char {
        &mut self.spacer
    }

    fn pairs(&self, text: &str) -> Vec<(char,char)> {
        let mut symbols: Vec<char> = text.chars().rev().collect();
        let mut out = Vec::with_capacity(text.len()/2);
        while symbols.len() >= 2 {
            //unwrap justified by condition above
            let l = symbols.pop().unwrap();
            let r = symbols.pop().unwrap();
            out.push((l,r))
        }
        if symbols.len() != 0 {
            out.push( (symbols.pop().unwrap(), self.spacer) )
        }
        out
    }

    fn encrypt_pair(&self, left: char, right: char, slide: usize, output: &mut String) {
        let left_index = self.alphabet.pos(left, 0).unwrap();
        let right_index = self.alphabet.pos(right, slide as i32).unwrap();

        output.push(self.alphabet.nth(right_index, 0).unwrap());
        output.push(self.alphabet.nth(left_index, slide as i32).unwrap());

    }

    fn decrypt_pair(&self, left: char, right: char, slide: usize, output: &mut String) {
        let left_index = self.alphabet.pos(left, 0).unwrap();
        let right_index = self.alphabet.pos(right, slide as i32).unwrap();

        output.push(self.alphabet.nth(right_index, 0).unwrap());
        output.push(self.alphabet.nth(left_index, slide as i32).unwrap());

    }

}

impl Default for Slidefair {
    fn default() -> Self {
        Self{ alphabet: Alphabet::from(BasicLatin), 
              spacer: 'X',
              key_word: String::new() }
    }
}

impl Cipher for Slidefair {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.chars().count());
        for ((left, right), slide) in pairs.iter().zip(self.cyclic_key()) {
            self.encrypt_pair(*left, *right, slide, &mut out)
        }
        Ok(out)
    }
    
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.chars().count());
        for ((left, right), slide) in pairs.iter().zip(self.cyclic_key()) {
            self.decrypt_pair(*left, *right, slide, &mut out)
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.alphabet = Alphabet::from(shuffled_str(&self.alphabet.slice(), rng))
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet.inner
    }

    fn get_output_alphabet(&self) -> &String {
        todo!()
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet.inner
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        todo!()
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if !&self.alphabet.contains(self.spacer) {
            return Err(CipherError::Key(format!("spacer character {} is not in the alphabet",self.spacer)))
        }
        Ok(())
    }
}


impl fmt::Display for Slidefair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for (n, _) in self.alphabet.inner.chars().enumerate() {
            out.push_str(&self.alphabet.inner[n..]);
            out.push_str(&self.alphabet.inner[0..n]);
            out.push('\n');
        };
        write!(f, "{}", out)
    }
}


#[cfg(test)]
mod slidefair_tests {
    use super::*;

    // Note X used as padding
    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    const CIPHERTEXT: &'static str = "HTPFGWHFRBVPDPURUJONMUBYTRDIYNVCODWH";

    #[test]
    fn encrypt_test() {
        let mut cipher = Slidefair::default();
        cipher.set_key("ABCD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Slidefair::default();
        cipher.set_key("ABCD");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}

