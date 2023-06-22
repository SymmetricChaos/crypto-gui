use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::integer::Roots;
use std::fmt;
use utils::{functions::keyed_alphabet, math_functions::is_square, preset_alphabet::Alphabet};

pub struct Playfair {
    pub square: String,
    pub spacer: char,
    grid_side_len: usize,
}

impl Playfair {
    pub fn assign_key(&mut self, keyword: &str, alphabet: &str) {
        self.grid_side_len = alphabet.chars().count().sqrt();
        self.square = keyed_alphabet(keyword, alphabet);
    }

    pub fn grid_side_len(&self) -> usize {
        self.grid_side_len
    }

    fn pairs(&self, text: &str) -> Vec<(char, char)> {
        // The only automatic spacer symbol is the make the total length even
        // pairs with matching letters will be caught later
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

    pub fn char_to_position(&self, symbol: char) -> Result<(usize, usize), CipherError> {
        let num = match self.square.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }

    // The inputs to this come only from internal functions that will never give invalid positions
    pub fn position_to_char(&self, position: (usize, usize)) -> char {
        let num = position.0 * self.grid_side_len + position.1;
        self.square.chars().nth(num).unwrap()
    }

    // Shift characters according to playfairs method
    pub fn playfair_shift(
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

        // identical characters are caught before this function is called so it is guaranteed that lpos != rpos

        // Same row
        if lpos.0 == rpos.0 {
            let x = lpos.0;
            (
                self.position_to_char((x, (lpos.1 + shift) % size)),
                self.position_to_char((x, (rpos.1 + shift) % size)),
            )
        // Same column
        } else if lpos.1 == rpos.1 {
            let y = lpos.1;
            (
                self.position_to_char(((lpos.0 + shift) % size, y)),
                self.position_to_char(((rpos.0 + shift) % size, y)),
            )
        } else {
            (
                self.position_to_char((lpos.0, rpos.1)),
                self.position_to_char((rpos.0, lpos.1)),
            )
        }
    }

    pub fn validate_settings(&self) -> Result<(), CipherError> {
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

        for (n, (l, r)) in pairs.into_iter().enumerate() {
            if l == r {
                return Err(CipherError::Input(format!(
                    "found repeated character {l} at pair {n}, a spacer should be inserted",
                )));
            }
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            let pair = self.playfair_shift(lpos, rpos, true);
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
            let pair = self.playfair_shift(lpos, rpos, false);
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
    fn test_pair_types() {
        let cipher = Playfair::default();
        assert_eq!(cipher.encrypt("AB").unwrap(), "BC");
        assert_eq!(cipher.encrypt("DE").unwrap(), "EA");
        assert_eq!(cipher.encrypt("BG").unwrap(), "GL");
        assert_eq!(cipher.encrypt("RW").unwrap(), "WB");
    }

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
