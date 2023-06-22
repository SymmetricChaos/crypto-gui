use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::integer::Roots;
use utils::{math_functions::is_square, preset_alphabet::Alphabet, vecstring::VecString};

pub struct FourSquare {
    pub alphabet: VecString,
    square1: VecString,
    square2: VecString,
    grid_side_len: usize,
    pub spacer: char,
}

impl Default for FourSquare {
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

impl FourSquare {
    pub fn assign_keys(&mut self, keyword_1: &str, keyword_2: &str, alphabet: &str) {
        self.square1 = VecString::keyed_alphabet(keyword_1, alphabet);
        self.square2 = VecString::keyed_alphabet(keyword_2, alphabet);
        self.alphabet = VecString::unique_from(alphabet);
        self.grid_side_len = alphabet.chars().count().sqrt();
    }

    pub fn grid_side_len(&self) -> usize {
        self.grid_side_len
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

    pub fn grid_lines(&self) -> String {
        let mut lines = String::new();

        let mut left_side = String::new();
        let mut right_side = String::new();

        for (n, (l, r)) in self.square1.chars().zip(self.alphabet.chars()).enumerate() {
            left_side.push(l);
            left_side.push(' ');
            right_side.push(r);
            right_side.push(' ');
            if (n + 1) % self.grid_side_len == 0 {
                lines.push_str(&format!("{left_side}   {right_side}\n"));
                left_side.clear();
                right_side.clear();
            }
        }

        lines.push('\n');

        for (n, (l, r)) in self.alphabet.chars().zip(self.square2.chars()).enumerate() {
            left_side.push(l);
            left_side.push(' ');
            right_side.push(r);
            right_side.push(' ');
            if (n + 1) % self.grid_side_len == 0 {
                lines.push_str(&format!("{left_side}   {right_side}\n"));
                left_side.clear();
                right_side.clear();
            }
        }
        lines
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

impl Cipher for FourSquare {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.len());
        for (l, r) in pairs {
            let lpos = self.char_to_position(l, &self.alphabet)?;
            let rpos = self.char_to_position(r, &self.alphabet)?;
            // Unlike Playfair and Two Square the Four Square cipher has no special cases to handle
            out.push(self.position_to_char((lpos.0, rpos.1), &self.square1));
            out.push(self.position_to_char((rpos.0, lpos.1), &self.square2));
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.len());
        for (l, r) in pairs {
            let lpos = self.char_to_position(l, &self.square1)?;
            let rpos = self.char_to_position(r, &self.square2)?;
            // Unlike Playfair and Two Square the Four Square cipher has no special cases to handle
            out.push(self.position_to_char((lpos.0, rpos.1), &self.alphabet));
            out.push(self.position_to_char((rpos.0, lpos.1), &self.alphabet));
        }
        Ok(out)
    }
}

#[cfg(test)]
mod four_square_tests {
    use super::*;

    const PLAINTEXT: &'static str = "HELPMEOBIWANKENOBI";
    const CIPHERTEXT: &'static str = "FYGMKYHOBXMFKKKIMD";

    #[test]
    fn encrypt_test() {
        let mut cipher = FourSquare::default();
        cipher.assign_keys("EXAMPLE", "KEYWORD", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = FourSquare::default();
        cipher.assign_keys("EXAMPLE", "KEYWORD", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
