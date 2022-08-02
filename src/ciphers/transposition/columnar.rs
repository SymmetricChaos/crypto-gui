use crate::global_rng::get_global_rng;
use crate::grid::{str_to_char_grid, Grid, Symbol};
use crate::text_aux::{rank_str, PresetAlphabet::*, VecString};
use crate::{ciphers::Cipher, errors::Error};

pub struct Columnar {
    pub alphabet_string: String,
    alphabet: VecString,
    key: Vec<usize>,
    pub key_word: String,
}

impl Columnar {
    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet_string = String::from(alphabet);
        self.set_alphabet();
    }

    pub fn control_key(&mut self) -> &mut String {
        self.key = rank_str(&self.key_word, &self.alphabet_string);
        &mut self.key_word
    }

    pub fn set_key(&mut self) {
        self.key = rank_str(&self.key_word, &self.alphabet_string);
    }

    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.key = rank_str(&self.key_word, &self.alphabet_string);
    }
}

impl Default for Columnar {
    fn default() -> Self {
        Self {
            alphabet_string: String::from(BasicLatin),
            alphabet: VecString::from(BasicLatin),
            key: Vec::new(),
            key_word: String::new(),
        }
    }
}

impl Cipher for Columnar {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();
        if n_cols < 2 {
            return Err(Error::key(
                "The key for a columnar cipher must have at least two characters",
            ));
        }

        // TODO: Once this is in std or core use that instead
        let n_rows = num::Integer::div_ceil(&tlen, &self.key.len());

        let symbols = str_to_char_grid(text, '\0', '\0');
        let g = Grid::from_cols(symbols, n_rows, n_cols);

        let mut out = String::with_capacity(text.len());
        for k in self.key.iter() {
            let mut s: String = g.get_col(*k).map(|sym| sym.to_char()).collect();
            s = s.replace(crate::grid::EMPTY, "");
            s = s.replace(crate::grid::BLOCK, "");
            out.push_str(&s);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();
        if n_cols < 2 {
            return Err(Error::key(
                "The key for a columnar cipher must have at least two characters",
            ));
        }

        // TODO: Once this is in std or core use that instead
        let n_rows = num::Integer::div_ceil(&tlen, &n_cols);

        let mut g = Grid::new_empty(n_rows, n_cols);
        let mut symbols = text.chars();

        for n in tlen..(n_rows * n_cols) {
            let coord = g.coord_from_index(n).unwrap();
            g.block_cell(coord);
        }

        for n in self.key.iter() {
            let column = g.get_col_mut(*n);
            for cell in column {
                if !cell.is_blocked() {
                    *cell = Symbol::Character(symbols.next().unwrap())
                }
            }
        }

        Ok(g.read_cols_characters().collect())
    }

    fn randomize(&mut self) {
        let key: String = self
            .alphabet
            .get_rand_chars_replace(11, &mut get_global_rng())
            .iter()
            .collect();
        self.assign_key(&key);
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod columnar_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "ECOOMVHZGTUBNJSRLDHIRFUOTAOQKWXPEEY";

    #[test]
    fn encrypt_test() {
        let mut cipher = Columnar::default();
        cipher.assign_key("TEST");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Columnar::default();
        cipher.assign_key("TEST");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
