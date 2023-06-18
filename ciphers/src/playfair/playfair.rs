use crate::{errors::CipherError, traits::Cipher};
use num::integer::Roots;
use std::fmt;
use utils::{functions::keyed_alphabet, math_functions::is_square, preset_alphabet::Alphabet};

pub struct Playfair {
    pub square: String,
    spacer: char,
    grid_side_len: usize,
}

impl Playfair {
    pub fn assign_key(&mut self, key_word: &str, alphabet: &str) {
        self.grid_side_len = alphabet.chars().count().sqrt();
        self.square = keyed_alphabet(key_word, alphabet);
    }

    pub fn control_spacer(&mut self) -> &mut char {
        &mut self.spacer
    }

    pub fn grid_side_len(&self) -> usize {
        self.grid_side_len
    }

    fn pairs(&self, text: &str) -> Vec<(char, char)> {
        let mut symbols: Vec<char> = text.chars().rev().collect();
        let mut out = Vec::with_capacity(text.len() / 2);
        while symbols.len() >= 2 {
            //unwrap justified by condition above
            let l = symbols.pop().unwrap();
            let r = symbols.pop().unwrap();
            if l == r {
                symbols.push(r);
                out.push((l, self.spacer));
            } else {
                out.push((l, r));
            }
        }
        if symbols.len() != 0 {
            out.push((symbols.pop().unwrap(), self.spacer))
        }
        out
    }

    fn char_to_position(&self, symbol: char) -> Result<(usize, usize), CipherError> {
        let num = match self.square.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }

    // The inputs to this come only from internal functions that will never give invalid positions
    fn position_to_char(&self, position: (usize, usize)) -> char {
        let num = position.0 * self.grid_side_len + position.1;
        self.square.chars().nth(num).unwrap()
    }

    // Shift characters according to playfairs method
    fn playfair_shift(
        &self,
        lpos: (usize, usize),
        rpos: (usize, usize),
        encrypt: bool,
        // output: &mut String,
    ) -> Result<(char, char), CipherError> {
        let size = self.grid_side_len;
        let shift = match encrypt {
            true => size + 1,
            false => size - 1,
        };

        if lpos.0 == rpos.0 {
            let x = lpos.0;
            Ok((
                self.position_to_char((x, (lpos.1 + shift) % size)),
                self.position_to_char((x, (rpos.1 + shift) % size)),
            ))
        } else if lpos.1 == rpos.1 {
            let y = lpos.1;
            Ok((
                self.position_to_char(((lpos.0 + shift) % size, y)),
                self.position_to_char(((rpos.0 + shift) % size, y)),
            ))
        } else {
            Ok((
                self.position_to_char((lpos.0, rpos.1)),
                self.position_to_char((rpos.0, lpos.1)),
            ))
        }
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if !is_square(self.square.chars().count()) {
            return Err(CipherError::alphabet(
                "alphabet must have a square number of characters",
            ));
        }
        if !&self.square.contains(self.spacer) {
            return Err(CipherError::Key(format!(
                "spacer character {} is not in the alphabet",
                self.spacer
            )));
        }
        Ok(())
    }
}

impl Default for Playfair {
    fn default() -> Self {
        Self {
            square: String::from(Alphabet::BasicLatinNoQ),
            spacer: 'X',
            grid_side_len: 5,
        }
    }
}

impl Cipher for Playfair {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.chars().count());

        for (l, r) in pairs {
            if l == r {
                return Err(CipherError::Input(format!(
                    "found repeated character {}, a spacer should be inserted",
                    l
                )));
            }
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            let pair = self.playfair_shift(lpos, rpos, true)?;
            out.push(pair.0);
            out.push(pair.1);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.chars().count());

        for (l, r) in pairs {
            if l == r {
                return Err(CipherError::Input(format!(
                    "found repeated character {}, a spacer should be inserted",
                    l
                )));
            }
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            let pair = self.playfair_shift(lpos, rpos, false)?;
            out.push(pair.0);
            out.push(pair.1);
        }
        Ok(out)
    }
}

impl fmt::Display for Playfair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for (n, c) in self.square.chars().enumerate() {
            if n % self.grid_side_len == 0 {
                out.push_str("\n")
            }
            out.push_str(&format!("{} ", c))
        }
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod playfair_tests {
    use super::*;

    // Note Q replaced by K and the X used as padding
    const PLAINTEXT: &'static str = "THEKUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    const CIPHERTEXT: &'static str = "WGVOEGAOAWNXKHXEGLNKCMULTWIZVDLWCPIT";

    #[test]
    fn encrypt_test() {
        let mut cipher = Playfair::default();
        cipher.assign_key("VUVUZELAS", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Playfair::default();
        cipher.assign_key("VUVUZELAS", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
