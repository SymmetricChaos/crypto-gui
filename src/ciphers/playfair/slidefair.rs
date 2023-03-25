use std::fmt::{self, Formatter};

use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{shuffled_str, PresetAlphabet::*, VecString},
};
use itertools::Itertools;

pub struct Slidefair {
    alphabet: VecString,
    pub alphabet_string: String,
    pub key_word: String,
    key: Vec<usize>,
    spacer_string: String,
    spacer: char,
}

impl Default for Slidefair {
    fn default() -> Self {
        Self {
            alphabet_string: String::from(BasicLatin),
            alphabet: VecString::from(BasicLatin),
            spacer_string: String::from("X"),
            spacer: 'X',
            key_word: String::new(),
            key: Vec::new(),
        }
    }
}

impl Slidefair {
    // Set or assign alphabet
    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string)
    }

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet_string = String::from(alphabet);
        self.set_alphabet();
    }

    // Set or assign key
    pub fn set_key(&mut self) {
        self.key = self
            .key_word
            .chars()
            .map(|x| self.alphabet.get_pos_of(x).unwrap())
            .collect();
    }

    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.set_key();
    }

    // Create cyclic key
    pub fn cyclic_key(&self) -> impl Iterator<Item = &usize> + '_ {
        self.key.iter().cycle()
    }

    // Set the spacer
    pub fn control_spacer(&mut self) -> &mut String {
        self.spacer = self.spacer_string.chars().next().unwrap_or('X');
        &mut self.spacer_string
    }

    fn encrypt_pair(&self, pair: &[char], slide: usize, output: &mut String) {
        let left_index = self.alphabet.get_pos_of(pair[0]).unwrap();
        let right_index = self.alphabet.get_pos_offset(pair[1], slide as i32).unwrap();

        if left_index != right_index {
            output.push(self.alphabet.get_char_at(right_index).unwrap());
            output.push(
                self.alphabet
                    .get_char_offset(left_index, slide as i32)
                    .unwrap(),
            );
        } else {
            output.push(self.alphabet.get_char_offset(left_index, -1).unwrap());
            output.push(self.alphabet.get_char_offset(right_index, -1).unwrap());
        }
    }

    fn decrypt_pair(&self, pair: &[char], slide: usize, output: &mut String) {
        let left_index = self.alphabet.get_pos_of(pair[0]).unwrap();
        let right_index = self.alphabet.get_pos_offset(pair[1], slide as i32).unwrap();

        if left_index != right_index {
            output.push(self.alphabet.get_char_at(right_index).unwrap());
            output.push(
                self.alphabet
                    .get_char_offset(left_index, slide as i32)
                    .unwrap(),
            );
        } else {
            output.push(self.alphabet.get_char_offset(left_index, 1).unwrap());
            output.push(self.alphabet.get_char_offset(right_index, 1).unwrap());
        }
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

    fn validate_settings(&self) -> Result<(), Error> {
        if !&self.alphabet.contains(self.spacer) {
            return Err(Error::Key(format!(
                "spacer character `{}` is not in the alphabet",
                self.spacer
            )));
        }
        Ok(())
    }
}

impl Cipher for Slidefair {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        self.validate_settings()?;
        let mut symbols = text.chars().collect_vec();
        if symbols.len() % 2 != 0 {
            symbols.push(self.spacer)
        }
        let pairs = symbols.chunks(2);
        let mut out = String::with_capacity(text.len());
        for (pair, slide) in pairs.zip(self.cyclic_key()) {
            self.encrypt_pair(pair, *slide, &mut out)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        self.validate_settings()?;
        let mut symbols = text.chars().collect_vec();
        if symbols.len() % 2 != 0 {
            symbols.push(self.spacer)
        }
        let pairs = symbols.chunks(2);
        let mut out = String::with_capacity(text.len());
        for (pair, slide) in pairs.zip(self.cyclic_key()) {
            self.decrypt_pair(pair, *slide, &mut out)
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        self.alphabet = VecString::from(shuffled_str(
            &self.alphabet.to_string(),
            &mut get_global_rng(),
        ))
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl fmt::Display for Slidefair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
