use std::fmt;

use super::Cipher;
use crate::{
    errors::CipherError,
    text_aux::{keyed_alphabet, shuffled_str, Alphabet, PresetAlphabet::*},
};
use rand::prelude::StdRng;

pub struct Slidefair {
    alphabet: Alphabet,
    pub alphabet_string: String,
    pub key_word: String,
    spacer: String,
}

impl Slidefair {
    pub fn cyclic_key(&self) -> impl Iterator<Item = usize> + '_ {
        let v = self.key().collect::<Vec<usize>>();
        v.into_iter().cycle()
    }

    pub fn key(&self) -> impl Iterator<Item = usize> + '_ {
        let key: Vec<usize> = self
            .key_word
            .chars()
            .map(|x| self.alphabet.get_pos_of(x).unwrap())
            .collect();
        key.into_iter()
    }

    // Set or assign alphabet
    pub fn set_alphabet(&mut self) {
        self.alphabet = Alphabet::from(&self.alphabet_string)
    }

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = Alphabet::from(alphabet);
        self.alphabet_string = alphabet.to_string();
    }

    // Set or assign key
    pub fn set_key(&mut self) {
        self.alphabet = Alphabet::from(keyed_alphabet(&self.key_word, &self.alphabet.to_string()));
    }

    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.alphabet = Alphabet::from(keyed_alphabet(key_word, &self.alphabet.to_string()));
    }

    pub fn control_spacer(&mut self) -> &mut String {
        self.spacer = self.spacer.chars().next().unwrap_or('X').to_string();
        &mut self.spacer
    }

    fn pairs(&self, text: &str) -> Vec<(char, char)> {
        let mut symbols: Vec<char> = text.chars().rev().collect();
        let mut out = Vec::with_capacity(text.len() / 2);
        while symbols.len() >= 2 {
            //unwrap justified by condition above
            let l = symbols.pop().unwrap();
            let r = symbols.pop().unwrap();
            out.push((l, r))
        }
        if symbols.len() != 0 {
            out.push((symbols.pop().unwrap(), self.spacer.chars().next().unwrap()))
        }
        out
    }

    fn encrypt_pair(&self, left: char, right: char, slide: usize, output: &mut String) {
        let left_index = self.alphabet.get_pos_of(left).unwrap();
        let right_index = self.alphabet.get_pos_offset(right, slide as i32).unwrap();

        output.push(self.alphabet.get_char_at(right_index).unwrap());
        output.push(
            self.alphabet
                .get_char_offset(left_index, slide as i32)
                .unwrap(),
        );
    }

    fn decrypt_pair(&self, left: char, right: char, slide: usize, output: &mut String) {
        let left_index = self.alphabet.get_pos_of(left).unwrap();
        let right_index = self.alphabet.get_pos_offset(right, slide as i32).unwrap();

        output.push(self.alphabet.get_char_at(right_index).unwrap());
        output.push(
            self.alphabet
                .get_char_offset(left_index, slide as i32)
                .unwrap(),
        );
    }

    pub fn rows(&self) -> Vec<String> {
        let mut rows = Vec::with_capacity(self.alphabet.len());
        for n in 0..self.alphabet.len() {
            let alpha = String::from(&self.alphabet_string);
            let mut row = String::from(&alpha[n..]);
            row.push_str(&alpha[0..n]);
            rows.push(row);
        }
        rows
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if !&self.alphabet.contains(self.spacer.chars().next().unwrap()) {
            return Err(CipherError::Key(format!(
                "spacer character {} is not in the alphabet",
                self.spacer
            )));
        }
        Ok(())
    }
}

impl Default for Slidefair {
    fn default() -> Self {
        Self {
            alphabet: Alphabet::from(BasicLatin),
            alphabet_string: String::from(BasicLatin),
            spacer: String::from("X"),
            key_word: String::new(),
        }
    }
}

impl Cipher for Slidefair {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.len());
        for ((left, right), slide) in pairs.iter().zip(self.cyclic_key()) {
            self.encrypt_pair(*left, *right, slide, &mut out)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.len());
        for ((left, right), slide) in pairs.iter().zip(self.cyclic_key()) {
            self.decrypt_pair(*left, *right, slide, &mut out)
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut StdRng) {
        self.alphabet = Alphabet::from(shuffled_str(&self.alphabet.to_string(), rng))
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl fmt::Display for Slidefair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        let alpha = String::from(&self.alphabet_string);
        for (n, _) in self.alphabet.chars().enumerate() {
            out.push_str(&alpha[n..]);
            out.push_str(&alpha[0..n]);
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod slidefair_tests {
    use super::*;

    // Note X used as padding
    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    const CIPHERTEXT: &'static str = "HTPFGWHFRBVPDPURUJONMUBYTRDIYNVCODWH";

    #[test]
    fn encrypt_test() {
        let mut cipher = Slidefair::default();
        cipher.assign_key("ABCD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Slidefair::default();
        cipher.assign_key("ABCD");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
