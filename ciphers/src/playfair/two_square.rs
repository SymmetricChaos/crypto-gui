use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::integer::Roots;
use utils::{preset_alphabet::PresetAlphabet, vecstring::VecString};

pub struct TwoSquare {
    pub alphabet: VecString,
    square1: VecString,
    square2: VecString,

    grid_side_len: usize,
}

impl Default for TwoSquare {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatinNoQ),
            square1: VecString::from(PresetAlphabet::BasicLatinNoQ),
            square2: VecString::from(PresetAlphabet::BasicLatinNoQ),
            grid_side_len: 5,
        }
    }
}

impl TwoSquare {
    pub fn assign_key1(&mut self, key_word: &str) {
        self.square1 = VecString::keyed_alphabet(key_word, &self.alphabet.to_string());
    }

    pub fn assign_key2(&mut self, key_word: &str) {
        self.square2 = VecString::keyed_alphabet(key_word, &self.alphabet.to_string());
    }

    pub fn pick_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            PresetAlphabet::BasicLatinNoJ
            | PresetAlphabet::BasicLatinNoQ
            | PresetAlphabet::BasicLatinWithDigits
            | PresetAlphabet::Base64 => {
                self.alphabet = VecString::from(mode);
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

    fn char_to_position(
        &self,
        symbol: char,
        alphabet: &VecString,
    ) -> Result<(usize, usize), CipherError> {
        let num = match alphabet.get_pos_of(symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }

    fn position_to_char(&self, position: (usize, usize), alphabet: &VecString) -> char {
        let num = position.0 * self.grid_side_len + position.1;
        alphabet.get_char_at(num).unwrap()
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
}

impl Cipher for TwoSquare {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut out = String::with_capacity(text.len());

        for (l, r) in self.pairs(text) {
            let lpos = self.char_to_position(l, &self.square1)?;
            let rpos = self.char_to_position(r, &self.square2)?;
            self.encrypt_pair(lpos, rpos, &mut out);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut out = String::with_capacity(text.len());

        for (l, r) in self.pairs(text) {
            let lpos = self.char_to_position(l, &self.square1)?;
            let rpos = self.char_to_position(r, &self.square2)?;
            self.decrypt_pair(lpos, rpos, &mut out);
        }

        Ok(out)
    }

    // fn randomize(&mut self) {
    //     self.key_word1 = shuffled_str(&self.alphabet.to_string(), &mut get_global_rng());
    //     self.key_word2 = shuffled_str(&self.alphabet.to_string(), &mut get_global_rng());
    //     self.set_key1();
    //     self.set_key2();
    // }
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
