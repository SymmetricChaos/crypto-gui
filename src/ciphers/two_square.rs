use super::Cipher;
use crate::{
    errors::CipherError,
    text_aux::{
        keyed_alphabet, shuffled_str,
        PresetAlphabet::{self, *}, Alphabet,
    },
};
use itertools::Itertools;
use num::integer::Roots;
use rand::prelude::StdRng;
use std::fmt;

pub struct TwoSquare {
    pub alphabet: String,
    square1: Alphabet,
    square2: Alphabet,
    pub key_word1: String,
    pub key_word2: String,
    grid_side_len: usize,
}

impl Default for TwoSquare {
    fn default() -> Self {
        Self {
            alphabet: String::from(PresetAlphabet::BasicLatinNoQ),
            square1: Alphabet::from(PresetAlphabet::BasicLatinNoQ),
            square2: Alphabet::from(PresetAlphabet::BasicLatinNoQ),
            key_word1: String::new(),
            key_word2: String::new(),
            grid_side_len: 5,
        }
    }
}

impl TwoSquare {
    pub fn assign_key1(&mut self, key_word: &str) {
        self.key_word1 = key_word.to_string();
        self.square1 = Alphabet::from(keyed_alphabet(&self.key_word1, &self.alphabet));
    }

    pub fn set_key1(&mut self) {
        self.square1 = Alphabet::from(keyed_alphabet(&self.key_word1, &self.alphabet));
    }

    pub fn assign_key2(&mut self, key_word: &str) {
        self.key_word2 = key_word.to_string();
        self.square2 = Alphabet::from(keyed_alphabet(&self.key_word2, &self.alphabet));
    }

    pub fn set_key2(&mut self) {
        self.square2 = Alphabet::from(keyed_alphabet(&self.key_word2, &self.alphabet));
    }


    pub fn set_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoJ | BasicLatinNoQ | BasicLatinWithDigits | Base64 => {
                self.alphabet = mode.string();
                self.square1 = Alphabet::from(mode);
                self.square2 = Alphabet::from(mode);
                self.grid_side_len = mode.len().sqrt();
            }
            _ => (),
        }
    }

    pub fn grid_side_len(&self) -> usize {
        self.grid_side_len
    }

    fn pairs(&self, text: &str) -> Vec<(char, char)> {
        text.chars()
            .collect_vec()
            .chunks(2)
            .map(|x| (x[0], x[1]))
            .collect_vec()
    }

    fn char_to_position1(&self, symbol: char) -> Result<(usize, usize), CipherError> {
        let num = match self.square1.get_pos_of(symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }

    fn char_to_position2(&self, symbol: char) -> Result<(usize, usize), CipherError> {
        let num = match self.square2.get_pos_of(symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }

    fn position_to_char1(&self, position: (usize, usize)) -> char {
        let num = position.0 * self.grid_side_len + position.1;
        self.square1.get_char_at(num).unwrap()
    }

    fn position_to_char2(&self, position: (usize, usize)) -> char {
        let num = position.0 * self.grid_side_len + position.1;
        self.square2.get_char_at(num).unwrap()
    }


    // Shift characters according to playfairs method
    fn encrypt_pair(
        &self,
        lpos: (usize, usize),
        rpos: (usize, usize),
        shift: usize,
        output: &mut String,
    ) {
        output.push(self.position_to_char1((lpos.0, rpos.1)));
        output.push(self.position_to_char2((rpos.0, lpos.1)));
    }
    

    pub fn show_square1(&self) -> String {
        let mut out = String::new();
        for (n, c) in self.square1.chars().enumerate() {
            if n % self.grid_side_len == 0 {
                out.push_str("\n")
            }
            out.push_str(&format!("{} ", c))
        }
        out
    }

    pub fn show_square2(&self) -> String {
        let mut out = String::new();
        for (n, c) in self.square2.chars().enumerate() {
            if n % self.grid_side_len == 0 {
                out.push_str("\n")
            }
            out.push_str(&format!("{} ", c))
        }
        out
    }
}

impl Cipher for TwoSquare {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.len());
        let shift = self.grid_side_len + 1;
        for (l, r) in pairs {
            let lpos = self.char_to_position1(l)?;
            let rpos = self.char_to_position2(r)?;
            self.encrypt_pair(lpos, rpos, shift, &mut out);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.len());
        let shift = self.grid_side_len - 1;
        for (l, r) in pairs {
            let lpos = self.char_to_position1(l)?;
            let rpos = self.char_to_position2(r)?;
            self.encrypt_pair(lpos, rpos, shift, &mut out);
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut StdRng) {
        self.key_word1 = shuffled_str(&self.alphabet, rng);
        self.key_word2 = shuffled_str(&self.alphabet, rng);
        self.set_key1();
        self.set_key2();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod two_square_tests {
    use super::*;

    // Note the X used as padding
    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    const CIPHERTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let mut cipher = TwoSquare::default();
        cipher.assign_key1("EXAMPLE");
        cipher.assign_key2("KEYWORD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = TwoSquare::default();
        cipher.assign_key1("EXAMPLE");
        cipher.assign_key2("KEYWORD");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
