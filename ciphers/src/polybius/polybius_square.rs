use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::{integer::Roots, Integer};
use std::fmt::{self, Formatter};
use utils::{math_functions::is_square, preset_alphabet::Alphabet, vecstring::VecString};

pub struct PolybiusSquare {
    pub square: VecString,
    pub labels: VecString,
    pub side_len: usize,
    pub spaced: bool,
}

impl Default for PolybiusSquare {
    fn default() -> Self {
        Self {
            square: VecString::from(Alphabet::BasicLatinNoQ),
            side_len: 5,
            labels: VecString::from(Alphabet::Digits1),
            spaced: false,
        }
    }
}

impl PolybiusSquare {
    pub fn assign_key(&mut self, key_word: &str, alphabet: &str) {
        self.square = VecString::keyed_alphabet(key_word, alphabet);
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
        let num = self.square.get_pos(symbol).unwrap();
        (num / self.side_len, num % self.side_len)
    }

    // Cannot fail due to checks in encrypt/decrypt
    fn position_to_char(&self, position: (char, char)) -> Result<char, CipherError> {
        let y = self
            .labels
            .get_pos(position.0)
            .ok_or(CipherError::invalid_input_char(position.0))?;
        let x = self
            .labels
            .get_pos(position.1)
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
                let ylab = self.labels.get_char(n / self.side_len).unwrap_or(&' ');
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
                *self
                    .labels
                    .get_char(pos.0)
                    .ok_or(CipherError::invalid_input_char(c))?,
            );
            out.push(
                *self
                    .labels
                    .get_char(pos.1)
                    .ok_or(CipherError::invalid_input_char(c))?,
            );
            if self.spaced {
                out.push(' ')
            }
        }

        if self.spaced {
            out.pop();
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_settings()?;

        if self.spaced {
            let mut out = String::with_capacity(text.chars().count() / 3);
            for pair in text.split(' ') {
                if pair.is_empty() {
                    continue;
                }
                if pair.chars().count() != 2 {
                    return Err(CipherError::input(
                        "input groups must consists of two symbols",
                    ));
                }
                let p: (char, char) = pair.chars().collect_tuple().unwrap();
                out.push(self.position_to_char(p)?);
            }
            Ok(out)
        } else {
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
        "1535144252113142252221531233215441524445512113142215351443245523322134";
    const CIPHERTEXT_SPACED: &'static str =
        "15 35 14 42 52 11 31 42 25 22 21 53 12 33 21 54 41 52 44 45 51 21 13 14 22 15 35 14 43 24 55 23 32 21 34";

    #[test]
    fn encrypt_test() {
        let mut cipher = PolybiusSquare::default();
        cipher.assign_key("INVENTORY", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = PolybiusSquare::default();
        cipher.assign_key("INVENTORY", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_spaced() {
        let mut cipher = PolybiusSquare::default();
        cipher.assign_key("INVENTORY", Alphabet::BasicLatinNoQ.into());
        cipher.spaced = true;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_SPACED);
    }

    #[test]
    fn decrypt_test_spaced() {
        let mut cipher = PolybiusSquare::default();
        cipher.assign_key("INVENTORY", Alphabet::BasicLatinNoQ.into());
        cipher.spaced = true;
        assert_eq!(cipher.decrypt(CIPHERTEXT_SPACED).unwrap(), PLAINTEXT);
    }
}
