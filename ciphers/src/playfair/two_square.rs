use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::integer::Roots;
use utils::{math_functions::is_square, preset_alphabet::Alphabet, vecstring::VecString};

pub struct TwoSquare {
    pub alphabet: VecString,
    square1: VecString,
    square2: VecString,
    grid_side_len: usize,
    pub spacer: char,
}

impl Default for TwoSquare {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(Alphabet::BasicLatinNoQ),
            square1: VecString::from(Alphabet::BasicLatinNoQ),
            square2: VecString::from(Alphabet::BasicLatinNoQ),
            grid_side_len: 5,
            spacer: 'X',
        }
    }
}

impl TwoSquare {
    pub fn assign_keys(&mut self, keyword_1: &str, keyword_2: &str, alphabet: &str) {
        self.square1 = VecString::keyed_alphabet(keyword_1, alphabet);
        self.square2 = VecString::keyed_alphabet(keyword_2, alphabet);
        self.alphabet = VecString::unique_from(alphabet);
        self.grid_side_len = alphabet.chars().count().sqrt();
    }

    pub fn grid_side_len(&self) -> usize {
        self.grid_side_len
    }

    pub fn square1(&self) -> &VecString {
        &self.square1
    }

    pub fn square2(&self) -> &VecString {
        &self.square2
    }

    fn pairs(&self, text: &str) -> Vec<(char, char)> {
        let mut symbols: Vec<char> = text.chars().collect();
        if symbols.len() % 2 != 0 {
            symbols.push(self.spacer)
        };
        symbols
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|c| c.collect_tuple().unwrap())
            .collect_vec()
    }

    fn char_to_position(
        &self,
        symbol: char,
        alphabet: &VecString,
    ) -> Result<(usize, usize), CipherError> {
        let num = match alphabet.get_pos(symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }

    fn position_to_char(&self, position: (usize, usize), alphabet: &VecString) -> char {
        let num = position.0 * self.grid_side_len + position.1;
        *alphabet.get_char(num).unwrap()
    }

    // Shift characters according to playfairs method
    fn playfair_shift(
        &self,
        lpos: (usize, usize),
        rpos: (usize, usize),
        encrypt: bool,
    ) -> (char, char) {
        let size = self.grid_side_len;
        let shift = match encrypt {
            true => size + 1,
            false => size - 1,
        };

        if lpos.0 == rpos.0 {
            (
                self.position_to_char((lpos.0, (lpos.1 + shift) % size), &self.square1),
                self.position_to_char((lpos.0, (rpos.1 + shift) % size), &self.square2),
            )
        } else {
            (
                self.position_to_char((lpos.0, rpos.1), &self.square1),
                self.position_to_char((rpos.0, lpos.1), &self.square2),
            )
        }
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

    fn validate_settings(&self) -> Result<(), CipherError> {
        if !is_square(self.alphabet.chars().count()) {
            return Err(CipherError::alphabet(
                "alphabet must have a square number of characters",
            ));
        }
        Ok(())
    }
}

impl Cipher for TwoSquare {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let mut out = String::with_capacity(text.len());

        for (l, r) in self.pairs(text) {
            let lpos = self.char_to_position(l, &self.square1)?;
            let rpos = self.char_to_position(r, &self.square2)?;
            let pair = self.playfair_shift(lpos, rpos, true);
            out.push(pair.0);
            out.push(pair.1);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let mut out = String::with_capacity(text.len());

        for (l, r) in self.pairs(text) {
            let lpos = self.char_to_position(l, &self.square1)?;
            let rpos = self.char_to_position(r, &self.square2)?;
            let pair = self.playfair_shift(lpos, rpos, false);
            out.push(pair.0);
            out.push(pair.1);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod two_square_tests {
    use super::*;

    // Note the Q replaced by K and the X used as padding
    const PLAINTEXT: &'static str = "THEKUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    const CIPHERTEXT: &'static str = "RJXEYFLYCDSENFSUHXMPTWVENVHEBBWOFWJT";

    #[test]
    fn encrypt_test() {
        let mut cipher = TwoSquare::default();
        cipher.assign_keys("EXAMPLE", "KEYWORD", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = TwoSquare::default();
        cipher.assign_keys("EXAMPLE", "KEYWORD", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
