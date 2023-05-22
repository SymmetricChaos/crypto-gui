use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::Integer;
use std::fmt::{self, Formatter};
use utils::{preset_alphabet::PresetAlphabet, vecstring::VecString};

pub struct PolybiusSquare {
    pub alphabet: VecString,
    pub labels: VecString,
    pub key_word: String,
    side_len: usize,
}

impl Default for PolybiusSquare {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatinNoQ),
            side_len: 5,
            labels: VecString::from(PresetAlphabet::Digits1),
            key_word: String::new(),
        }
    }
}

impl PolybiusSquare {
    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.alphabet = VecString::keyed_alphabet(&self.key_word, &self.alphabet.to_string());
    }

    pub fn pick_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            PresetAlphabet::BasicLatinNoJ
            | PresetAlphabet::BasicLatinNoQ
            | PresetAlphabet::BasicLatinWithDigits
            | PresetAlphabet::Base64 => {
                self.alphabet = VecString::from(mode);
                self.side_len = (mode.len() as f64).sqrt().ceil() as usize;
            }
            _ => (),
        }
    }

    pub fn assign_labels(&mut self, labels: &str) {
        self.labels = VecString::unique_from(labels);
    }

    // Cannot fail due to checks in encrypt/decrypt
    fn pairs(&self, text: &str) -> Vec<(char, char)> {
        let out = text
            .chars()
            .chunks(2)
            .into_iter()
            .map(|x| x.collect_tuple().unwrap())
            .collect();
        out
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }

    // Cannot fail due to checks in encrypt/decrypt
    fn char_to_position(&self, symbol: char) -> (usize, usize) {
        let num = self.alphabet.get_pos_of(symbol).unwrap();
        (num / self.side_len, num % self.side_len)
    }

    // Cannot fail due to checks in encrypt/decrypt
    fn position_to_char(&self, position: (char, char)) -> Result<char, CipherError> {
        let y = self
            .labels
            .get_pos_of(position.0)
            .ok_or(CipherError::invalid_input_char(position.0))?;
        let x = self
            .labels
            .get_pos_of(position.1)
            .ok_or(CipherError::invalid_input_char(position.1))?;

        let num = y * self.side_len + x;
        Ok(self.alphabet.chars().nth(num).unwrap())
    }

    fn check_labels(&self) -> Result<(), CipherError> {
        if self.labels.len() < self.side_len {
            return Err(CipherError::key("not enough labels for grid size"));
        }
        Ok(())
    }

    pub fn show_grid(&self) -> String {
        let size = (self.side_len + 2) * (self.side_len + 1);
        let mut square = String::with_capacity(size);
        square.push_str("  ");

        for xlab in self.labels.chars().take(self.side_len) {
            square.push(xlab);
            square.push(' ');
        }

        for (n, c) in self.alphabet.chars().enumerate() {
            if n % self.side_len == 0 {
                let ylab = self.labels.get_char_at(n / self.side_len).unwrap_or(' ');
                square.push_str(&format!("\n{ylab} "));
            }
            square.push(c);
            square.push(' ');
        }
        square
    }
}

impl Cipher for PolybiusSquare {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_labels()?;

        let mut out = String::with_capacity(text.chars().count() * 2);

        for c in text.chars() {
            let pos = self.char_to_position(c);
            out.push(
                self.labels
                    .get_char_at(pos.0)
                    .ok_or(CipherError::invalid_input_char(c))?,
            );
            out.push(
                self.labels
                    .get_char_at(pos.1)
                    .ok_or(CipherError::invalid_input_char(c))?,
            );
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_labels()?;
        if !text.chars().count().is_multiple_of(&2) {
            return Err(CipherError::input(
                "Input text must have a length that is a multiple of two.",
            ));
        }

        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.chars().count() / 2);

        for p in pairs {
            out.push(self.position_to_char(p)?);
        }
        Ok(out)
    }
}

impl fmt::Display for PolybiusSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut square = String::from("  ");
        for xlab in self.labels.chars().take(self.side_len) {
            square.push_str(&format!("{xlab} "))
        }
        for (n, c) in self.alphabet.chars().enumerate() {
            if n % self.side_len == 0 {
                let ylab = self.labels.chars().nth(n / self.side_len).unwrap();
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
