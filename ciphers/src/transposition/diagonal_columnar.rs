use std::fmt::Display;

use crate::{errors::CipherError, traits::Cipher};
use utils::{
    grid::{Grid, Symbol},
    text_functions::{rank_str, rank_vec, StringRankError},
};

pub struct DiagonalColumnar {
    pub key: Vec<usize>,
    pub key_ranks: Vec<usize>,
}

impl DiagonalColumnar {
    pub fn assign_key(&mut self, keyword: &str, alphabet: &str) -> Result<(), StringRankError> {
        self.key = rank_str(keyword, alphabet)?;
        self.key_ranks = rank_vec(&self.key);
        Ok(())
    }
}

impl Default for DiagonalColumnar {
    fn default() -> Self {
        Self {
            key: Vec::new(),
            key_ranks: Vec::new(),
        }
    }
}

impl Cipher for DiagonalColumnar {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();

        // TODO: Once this is in std or core use that instead
        let n_rows = num::Integer::div_ceil(&tlen, &self.key.len());

        // Build the grid with first all Empty symbols and then Block the disrupted areas

        let mut g: Grid<Symbol<char>> = Grid::new_empty(n_rows, n_cols);

        let mut disruption_ctr = 0;
        let mut disruption_start = self.key.iter().position(|n| *n == disruption_ctr).unwrap();
        for row in 0..n_rows {
            g.get_row_mut(row)
                .skip(disruption_start)
                .for_each(|l| *l = Symbol::Blocked);

            if disruption_start >= n_cols {
                disruption_ctr += 1;
                disruption_start = match self.key.iter().position(|n| *n == disruption_ctr) {
                    Some(n) => n,
                    None => break,
                };
                continue;
            }
            disruption_start += 1;
        }

        println!("{g}");
        // Now fill the Empty cells left to right and top to bottom, then fill the Blocked cells the same way
        let symbols = text.chars();

        let out = String::with_capacity(text.len());
        // for k in self.key_ranks.iter() {}

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
mod diagonal_columnar_tests {
    use utils::preset_alphabet::Alphabet;

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    // const CIPHERTEXT: &'static str = "EKNUVEDQBFMELOHCWJOHYUROPRAGTIOXSTZ";

    #[test]
    fn display_test() {
        let mut cipher = DiagonalColumnar::default();
        cipher
            .assign_key("ECABD", Alphabet::BasicLatin.slice())
            .unwrap();
        cipher.encrypt(PLAINTEXT).unwrap();
    }

    // #[test]
    // fn encrypt_test() {
    //     let mut cipher = Columnar::default();
    //     cipher
    //         .assign_key("ECABD", Alphabet::BasicLatin.slice())
    //         .unwrap();
    //     assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    // }

    // #[test]
    // fn decrypt_test() {
    //     let mut cipher = Columnar::default();
    //     cipher
    //         .assign_key("ECABD", Alphabet::BasicLatin.slice())
    //         .unwrap();
    //     assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    // }
}
