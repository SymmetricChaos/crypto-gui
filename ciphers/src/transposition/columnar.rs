use crate::traits::Cipher;
use utils::{
    errors::GeneralError,
    grid::{str_to_char_grid, Grid, Symbol, BLOCK, EMPTY},
    text_functions::{rank_str, rank_vec, StringRankError},
};

pub struct Columnar {
    pub key: Vec<usize>,
    pub key_ranks: Vec<usize>,
}

impl Columnar {
    pub fn assign_key(&mut self, keyword: &str, alphabet: &str) -> Result<(), StringRankError> {
        self.key = rank_str(keyword, alphabet)?;
        self.key_ranks = rank_vec(&self.key);
        Ok(())
    }
}

impl Default for Columnar {
    fn default() -> Self {
        Self {
            key: Vec::new(),
            key_ranks: Vec::new(),
        }
    }
}

impl Cipher for Columnar {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();

        // TODO: Once this is in std or core use that instead
        let n_rows = num::Integer::div_ceil(&tlen, &self.key.len());

        let symbols = str_to_char_grid(text, '\0', '\0');
        let g = Grid::from_rows(symbols, n_rows, n_cols);

        let mut out = String::with_capacity(text.len());
        for k in self.key_ranks.iter() {
            let mut s: String = g.get_col(*k).map(|sym| sym.to_char()).collect();
            s = s.replace(EMPTY, "");
            s = s.replace(BLOCK, "");
            out.push_str(&s);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
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
        for n in self.key_ranks.iter() {
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

    const PTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CTEXT: &'static str = "EKNUVEDQBFMELOHCWJOHYUROPRAGTIOXSTZ";

    #[test]
    fn encrypt_test() {
        let mut cipher = Columnar::default();
        cipher
            .assign_key("ECABD", Alphabet::BasicLatin.slice())
            .unwrap();
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Columnar::default();
        cipher
            .assign_key("ECABD", Alphabet::BasicLatin.slice())
            .unwrap();
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
