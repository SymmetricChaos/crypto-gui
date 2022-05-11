use super::Cipher;
use crate::{
    errors::CipherError,
    text_aux::{
        keyed_alphabet, shuffled_str, validate_alphabet, Alphabet, PresetAlphabet,
        PresetAlphabet::*,
    },
};
use itertools::Itertools;
use num::integer::Roots;
use rand::prelude::StdRng;
use std::fmt;

pub struct PolybiusSquare {
    alphabet_string: &'static str, // custom alphabet strings aren't allowed
    grid: Alphabet,
    pub labels_string: String,
    labels: Alphabet,
    grid_side_len: usize,
    pub key_word: String,
}


impl Default for PolybiusSquare {
    fn default() -> Self {
        Self {
            alphabet_string: PresetAlphabet::BasicLatinNoQ.slice(),
            grid: Alphabet::from(PresetAlphabet::BasicLatinNoQ),
            grid_side_len: 5,
            labels: Alphabet::from(PresetAlphabet::Digits1),
            labels_string: PresetAlphabet::Digits1.string(),
            key_word: String::new(),
        }
    }
}


impl PolybiusSquare {
    pub fn alphabet(&self) -> &str {
        self.alphabet_string
    }

    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.grid = Alphabet::from(keyed_alphabet(&self.key_word, &self.alphabet_string));
    }

    pub fn set_key(&mut self) {
        self.grid = Alphabet::from(keyed_alphabet(&self.key_word, &self.alphabet_string));
    }

    pub fn assign_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoJ | BasicLatinNoQ | BasicLatinWithDigits | Base64 => {
                self.alphabet_string = mode.slice();
                self.grid = Alphabet::from(mode);
                self.grid_side_len = mode.len().sqrt();
            }
            _ => (),
        }
    }

    pub fn set_alphabet(&mut self) {
        todo!("should ensure a valid alphabet")
    }

    pub fn assign_labels(&mut self, labels: &str) {
        self.labels_string = labels.to_string();
        self.labels = Alphabet::from(&self.labels_string);
    }

    pub fn set_labels(&mut self) {
        self.labels = Alphabet::from(&self.labels_string);
    }

    fn pairs(&self, text: &str) -> Result<Vec<(char, char)>, CipherError> {
        if text.chars().count() % 2 != 0 {
            dbg!(text);
            dbg!(text.chars().count());
            return Err(CipherError::input(
                "Input text does not have an even number of characters.",
            ));
        }
        let out = text
            .chars()
            .chunks(2)
            .into_iter()
            .map(|x| x.collect_tuple().unwrap())
            .collect();
        Ok(out)
    }

    pub fn alphabet_len(&self) -> usize {
        self.grid.len()
    }

    fn char_to_position(&self, symbol: char) -> Result<(usize, usize), CipherError> {
        let num = match self.alphabet_string.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }

    fn position_to_char(&self, position: (char, char)) -> char {
        let y = self.labels.chars().position(|c| c == position.0).unwrap();
        let x = self.labels.chars().position(|c| c == position.1).unwrap();

        let num = y * self.grid_side_len + x;
        self.alphabet_string.chars().nth(num).unwrap()
    }

    fn _validate_settings(&self) -> Result<(), CipherError> {
        validate_alphabet(&self.alphabet_string)?;
        Ok(())
    }
}

impl Cipher for PolybiusSquare {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut out = String::with_capacity(text.chars().count() * 2);

        for c in text.chars() {
            let pos = self.char_to_position(c)?;
            out.push(self.labels.chars().nth(pos.0).unwrap());
            out.push(self.labels.chars().nth(pos.1).unwrap());
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let pairs = self.pairs(text)?;
        let mut out = String::with_capacity(text.chars().count() / 2);

        for p in pairs {
            out.push(self.position_to_char(p));
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut StdRng) {
        self.key_word = shuffled_str(&self.alphabet_string, rng);
        self.set_key();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl fmt::Display for PolybiusSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = String::from("  ");
        for xlab in self.labels.chars().take(self.grid_side_len) {
            square.push_str(&format!("{xlab} "))
        }
        for (n, c) in self.grid.chars().enumerate() {
            if n % self.grid_side_len == 0 {
                let ylab = self.labels.chars().nth(n / self.grid_side_len).unwrap();
                square.push_str(&format!("\n{ylab} "));
            }
            square.push_str(&format!("{c} "))
        }
        write!(f, "{square}")
    }
}

#[cfg(test)]
mod polybius_tests {
    use super::*;

    // Note Q replaced by K
    const PLAINTEXT: &'static str = "THEKUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str =
        "4423153145241331124235523421355325453341433551154244231532115554143522";

    #[test]
    fn encrypt_test() {
        let mut cipher = PolybiusSquare::default();
        cipher.assign_key("INVENTORY");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = PolybiusSquare::default();
        cipher.assign_key("INVENTORY");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
