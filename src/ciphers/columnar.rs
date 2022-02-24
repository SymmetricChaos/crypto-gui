use num::Integer;
use rand::prelude::ThreadRng;

use crate::errors::CipherError;
use crate::grid::{Grid, Symbol};
use crate::text_functions::{rank_str, random_sample_replace};
use super::Cipher;
use crate::text_types::{PresetAlphabet::*};

pub struct Columnar {
    alphabet: String,
    key: Vec<usize>,
    key_word: String,
}

impl Columnar {
    pub fn control_key(&mut self) -> &mut String {
        self.key = rank_str(&self.key_word, &self.alphabet);
        &mut self.key_word
    }

    pub fn set_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.key = rank_str(&self.key_word, &self.alphabet);
    }
}

impl Default for Columnar {
    fn default() -> Self {
        Self { alphabet: String::from(BasicLatin), key: Vec::new(), key_word: String::new() }
    }
}

impl Cipher for Columnar {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();
        if n_cols < 2 {
            return Err(CipherError::key("The key for a columnar cipher must have at least two characters"))
        }
        let n_rows = tlen.div_ceil(&self.key.len());
        let g = Grid::from_rows(text, n_rows, n_cols, '\0', '\0');

        let mut out = String::with_capacity(text.len());
        for k in self.key.iter() {
            let mut s: String = g.get_col(*k).map(|sym| sym.to_char()).collect();
            s = s.replace(crate::grid::EMPTY, "");
            s = s.replace(crate::grid::BLOCK, "");
            out.push_str(&s);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();
        if n_cols < 2 {
            return Err(CipherError::key("The key for a columnar cipher must have at least two characters"))
        }
        let n_rows = tlen.div_ceil(&n_cols);
     
        let mut g = Grid::new_empty(n_rows, n_cols);
        let mut symbols = text.chars();
     
        for n in tlen..(n_rows*n_cols) {
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
    
        Ok(g.read_rows_characters().collect())   
    }


    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.key_word = random_sample_replace(&self.alphabet, 11, rng);
        self.key = rank_str(&self.key_word, &self.alphabet);
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        todo!("transposition may output any alphabet")
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet
    }

    fn get_output_alphabet(&self) -> &String {
        todo!("transposition may output any alphabet")
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        todo!()
    }
}


#[cfg(test)]
mod columnar_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "ECOOMVHZGTUBNJSRLDHIRFUOTAOQKWXPEEY";

    #[test]
    fn encrypt_test() {
        let mut cipher = Columnar::default();
        cipher.set_key("TEST");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Columnar::default();
        cipher.set_key("TEST");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
