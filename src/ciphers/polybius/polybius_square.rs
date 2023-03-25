use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{
        shuffled_str, text_functions::validate_text, PresetAlphabet, PresetAlphabet::*, VecString,
    },
};
use itertools::Itertools;
use num::Integer;
use std::fmt::{self, Formatter};

pub struct PolybiusSquare {
    pub alphabet_string: String,
    grid: VecString,
    pub labels_string: String,
    labels: VecString,
    side_len: usize,
    pub key_word: String,
}

impl Default for PolybiusSquare {
    fn default() -> Self {
        Self {
            alphabet_string: String::from(PresetAlphabet::BasicLatinNoQ),
            grid: VecString::from(PresetAlphabet::BasicLatinNoQ),
            side_len: 5,
            labels: VecString::from(PresetAlphabet::Digits1),
            labels_string: PresetAlphabet::Digits1.string(),
            key_word: String::new(),
        }
    }
}

impl PolybiusSquare {
    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.grid = VecString::keyed_alphabet(&self.key_word, &self.alphabet_string);
    }

    pub fn set_key(&mut self) {
        self.grid = VecString::keyed_alphabet(&self.key_word, &self.alphabet_string);
    }

    pub fn assign_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoJ | BasicLatinNoQ | BasicLatinWithDigits | Base64 => {
                self.alphabet_string = String::from(mode);
                self.grid = VecString::from(mode);
                self.side_len = (mode.len() as f64).sqrt().ceil() as usize;
            }
            _ => (),
        }
    }

    pub fn set_alphabet(&mut self) -> Result<(), Error> {
        let new_alpha_len = self.alphabet_string.chars().count();

        if new_alpha_len > 100 {
            return Err(Error::alphabet(
                "alphabet length currently limited to 100 characters",
            ));
        }

        self.grid = VecString::unique_from(&self.alphabet_string);
        self.side_len = (new_alpha_len as f64).sqrt().ceil() as usize;

        Ok(())
    }

    pub fn assign_labels(&mut self, labels: &str) {
        self.labels_string = labels.to_string();
        self.labels = VecString::unique_from(&self.labels_string);
    }

    pub fn set_labels(&mut self) {
        self.labels = VecString::unique_from(&self.labels_string);
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
        self.grid.len()
    }

    // Cannot fail due to checks in encrypt/decrypt
    fn char_to_position(&self, symbol: char) -> (usize, usize) {
        let num = self.grid.get_pos_of(symbol).unwrap();
        (num / self.side_len, num % self.side_len)
    }

    // Cannot fail due to checks in encrypt/decrypt
    fn position_to_char(&self, position: (char, char)) -> char {
        let y = self.labels.get_pos_of(position.0).unwrap();
        let x = self.labels.get_pos_of(position.1).unwrap();

        let num = y * self.side_len + x;
        self.alphabet_string.chars().nth(num).unwrap()
    }

    fn check_labels(&self) -> Result<(), Error> {
        if self.labels.len() < self.side_len {
            return Err(Error::key("not enough labels for grid size"));
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

        for (n, c) in self.grid.chars().enumerate() {
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
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        self.check_labels()?;
        validate_text(text, &self.grid)?;

        let mut out = String::with_capacity(text.chars().count() * 2);

        for c in text.chars() {
            let pos = self.char_to_position(c);
            out.push(self.labels.get_char_at(pos.0).unwrap());
            out.push(self.labels.get_char_at(pos.1).unwrap());
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        self.check_labels()?;
        validate_text(text, &self.labels)?;
        if !text.chars().count().is_multiple_of(&2) {
            return Err(Error::input(
                "Input text must have a length that is a multiple of three.",
            ));
        }

        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.chars().count() / 2);

        for p in pairs {
            out.push(self.position_to_char(p));
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        self.key_word = shuffled_str(&self.alphabet_string, &mut get_global_rng());
        self.set_key();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl fmt::Display for PolybiusSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut square = String::from("  ");
        for xlab in self.labels.chars().take(self.side_len) {
            square.push_str(&format!("{xlab} "))
        }
        for (n, c) in self.grid.chars().enumerate() {
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
