use crate::{errors::CipherError, traits::Cipher};
use utils::{
    functions::rank_str,
    grid::{str_to_char_grid, Grid, Symbol, BLOCK, EMPTY},
};

pub struct Columnar {
    pub key: Vec<usize>,
}

impl Columnar {
    pub fn assign_key(&mut self, key_word: &str, alphabet: &str) {
        self.key = rank_str(key_word, alphabet);
    }
}

impl Default for Columnar {
    fn default() -> Self {
        Self { key: Vec::new() }
    }
}

impl Cipher for Columnar {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();

        // TODO: Once this is in std or core use that instead
        let n_rows = num::Integer::div_ceil(&tlen, &self.key.len());

        let symbols = str_to_char_grid(text, '\0', '\0');
        let g = Grid::from_rows(symbols, n_rows, n_cols);

        let mut out = String::with_capacity(text.len());
        for k in self.key.iter() {
            let mut s: String = g.get_col(*k).map(|sym| sym.to_char()).collect();
            s = s.replace(EMPTY, "");
            s = s.replace(BLOCK, "");
            out.push_str(&s);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();

        // TODO: Once this is in std or core use that instead
        let n_rows = num::Integer::div_ceil(&tlen, &n_cols);

        let mut g = Grid::new_empty(n_rows, n_cols);

        for n in tlen..(n_rows * n_cols) {
            let coord = g.coord_from_index(n).unwrap();
            g.block_cell(coord);
        }

        let mut symbols = text.chars();
        for n in self.key.iter() {
            let column = g.get_col_mut(*n);
            for cell in column {
                if !cell.is_blocked() {
                    *cell = Symbol::Filled(symbols.next().unwrap())
                }
            }
        }

        Ok(g.read_rows_characters().collect())
    }
}

#[cfg(test)]
mod columnar_tests {
    use utils::preset_alphabet::Alphabet;

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "ECOOMVHZGTUBNJSRLDHIRFUOTAOQKWXPEEY";

    #[test]
    fn encrypt_test() {
        let mut cipher = Columnar::default();
        cipher.assign_key("TEST", Alphabet::BasicLatin.slice());
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Columnar::default();
        cipher.assign_key("TEST", Alphabet::BasicLatin.slice());
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
