use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::{integer::Roots, Integer};
use std::fmt::{self, Formatter};
use utils::{math_functions::is_square, preset_alphabet::Alphabet, vecstring::VecString};

pub struct PolybiusSquare {
    pub square: VecString,
    pub labels: VecString,
    side_len: usize,
}

impl Default for PolybiusSquare {
    fn default() -> Self {
        Self {
            square: VecString::from(Alphabet::BasicLatinNoQ),
            side_len: 5,
            labels: VecString::from(Alphabet::Digits1),
        }
    }
}

impl PolybiusSquare {
    pub fn assign_key(&mut self, key_word: &str, alphabet: &str) {
        self.square = VecString::keyed_alphabet(key_word, alphabet);
        self.square = VecString::unique_from(alphabet);
        self.side_len = alphabet.chars().count().sqrt();
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
        self.square.len()
    }

    // Cannot fail due to checks in encrypt/decrypt
    fn char_to_position(&self, symbol: char) -> (usize, usize) {
        let num = self.square.get_pos_of(symbol).unwrap();
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
        Ok(self.square.chars().nth(num).unwrap())
    }

    fn check_settings(&self) -> Result<(), CipherError> {
        if self.labels.len() < self.side_len {
            return Err(CipherError::key("not enough labels for grid size"));
        }
        if !is_square(self.square.chars().count()) {
            return Err(CipherError::alphabet(
                "alphabet must have a square number of characters",
            ));
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

        for (n, c) in self.square.chars().enumerate() {
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
        self.check_settings()?;

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
        self.check_settings()?;
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
        for (n, c) in self.square.chars().enumerate() {
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
        cipher.assign_key("INVENTORY", Alphabet::BasicLatinNoJ);
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = PolybiusSquare::default();
        cipher.assign_key("INVENTORY", Alphabet::BasicLatinNoJ);
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
