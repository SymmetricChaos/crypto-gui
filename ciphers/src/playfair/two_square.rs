use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::integer::Roots;
use utils::{math_functions::is_square, preset_alphabet::Alphabet, vecstring::VecString};

pub struct TwoSquare {
    pub alphabet: VecString,
    square1: VecString,
    square2: VecString,
    grid_side_len: usize,
}

impl Default for TwoSquare {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(Alphabet::BasicLatinNoQ),
            square1: VecString::from(Alphabet::BasicLatinNoQ),
            square2: VecString::from(Alphabet::BasicLatinNoQ),
            grid_side_len: 5,
        }
    }
}

impl TwoSquare {
    pub fn assign_keys(&mut self, key_word_1: &str, key_word_2: &str, alphabet: &str) {
        self.square1 = VecString::keyed_alphabet(key_word_1, alphabet);
        self.square2 = VecString::keyed_alphabet(key_word_2, alphabet);
        self.alphabet = VecString::unique_from(alphabet);
        self.grid_side_len = alphabet.chars().count().sqrt();
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
    fn encrypt_pair(&self, lpos: (usize, usize), rpos: (usize, usize), output: &mut String) {
        let size = self.grid_side_len;
        let shift = self.grid_side_len + 1;
        // The pairs() function ensures l and r never match so that case is not handled
        if lpos.0 == rpos.0 {
            let x = lpos.0;
            output.push(self.position_to_char((x, (lpos.1 + shift) % size), &self.square1));
            output.push(self.position_to_char((x, (rpos.1 + shift) % size), &self.square2));
        } else {
            output.push(self.position_to_char((lpos.0, rpos.1), &self.square1));
            output.push(self.position_to_char((rpos.0, lpos.1), &self.square2));
        }
    }

    fn decrypt_pair(&self, lpos: (usize, usize), rpos: (usize, usize), output: &mut String) {
        let size = self.grid_side_len;
        let shift = self.grid_side_len - 1;
        // The pairs() function ensures l and r never match so that case is not handled
        if lpos.0 == rpos.0 {
            let x = lpos.0;
            output.push(self.position_to_char((x, (lpos.1 + shift) % size), &self.square1));
            output.push(self.position_to_char((x, (rpos.1 + shift) % size), &self.square2));
        } else {
            output.push(self.position_to_char((lpos.0, rpos.1), &self.square1));
            output.push(self.position_to_char((rpos.0, lpos.1), &self.square2));
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
            self.encrypt_pair(lpos, rpos, &mut out);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let mut out = String::with_capacity(text.len());

        for (l, r) in self.pairs(text) {
            let lpos = self.char_to_position(l, &self.square1)?;
            let rpos = self.char_to_position(r, &self.square2)?;
            self.decrypt_pair(lpos, rpos, &mut out);
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
