use crate::{errors::CipherError, traits::Cipher};
use utils::{
    grid::{Grid, Symbol, BLOCK, EMPTY},
    text_functions::{rank_str, rank_vec, StringRankError},
};

pub struct DiagonalColumnar {
    pub key: Vec<usize>,
    pub key_ranks: Vec<usize>,
    pub filler: String,
}

impl DiagonalColumnar {
    pub fn assign_key(&mut self, keyword: &str, alphabet: &str) -> Result<(), StringRankError> {
        self.key = rank_str(keyword, alphabet)?;
        self.key_ranks = rank_vec(&self.key);
        Ok(())
    }

    pub fn display_page(&self, text_length: usize) -> String {
        let n_cols = self.key.len();
        let n_rows = num::Integer::div_ceil(&text_length, &self.key.len());

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

        g.to_string()
    }
}

impl Default for DiagonalColumnar {
    fn default() -> Self {
        Self {
            key: Vec::new(),
            key_ranks: Vec::new(),
            filler: String::from("X"),
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

        // Now fill the Empty cells left to right and top to bottomy
        let mut symbols = text.chars().chain(self.filler.chars().cycle());
        for cell in g.get_rows_mut() {
            if cell.is_empty() {
                match symbols.next() {
                    Some(c) => *cell = Symbol::Filled(c),
                    None => break,
                }
            }
        }

        // Now fill the Blocked cells left to right and top to bottomy
        for cell in g.get_rows_mut() {
            if cell.is_blocked() {
                match symbols.next() {
                    Some(c) => *cell = Symbol::Filled(c),
                    None => break,
                }
            }
        }

        let mut out = String::with_capacity(text.len());
        for k in self.key_ranks.iter() {
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

        if tlen != n_cols * n_rows {
            return Err(CipherError::input(
                "inocrrect number of characters, diagonal transposition must use a full grid",
            ));
        }
        let mut g = Grid::new_empty(n_rows, n_cols);

        // Fill the grid by columns
        let mut symbols = text.chars();
        for n in self.key_ranks.iter() {
            let column = g.get_col_mut(*n);
            for cell in column {
                *cell = Symbol::Filled(symbols.next().unwrap())
            }
        }

        let mut out = String::new();

        // Read the outer part of the disrupted sections
        let mut disruption_ctr = 0;
        let mut disruption_start = self.key.iter().position(|n| *n == disruption_ctr).unwrap();
        for row in 0..n_rows {
            g.get_row_mut(row).take(disruption_start).for_each(|l| {
                out.push(l.to_char());
                *l = Symbol::Blocked;
            });

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

        // Read the inner parts of the disrupted sections
        for row in 0..n_rows {
            g.get_row(row)
                .filter(|cell| !cell.is_blocked())
                .for_each(|cell| out.push(cell.to_char()))
        }

        Ok(out)
    }
}

#[cfg(test)]
mod diagonal_columnar_tests {
    use utils::preset_alphabet::Alphabet;

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "HUKWJPEEABNDSRHQCOXMVLZYFOGTTEIROUO";

    // #[test]
    // fn display_test() {
    //     let mut cipher = DiagonalColumnar::default();
    //     cipher
    //         .assign_key("ECABD", Alphabet::BasicLatin.slice())
    //         .unwrap();
    //     println!("{}", cipher.display_page(PLAINTEXT.chars().count()));
    // }

    #[test]
    fn encrypt_test() {
        let mut cipher = DiagonalColumnar::default();
        cipher
            .assign_key("ECABD", Alphabet::BasicLatin.slice())
            .unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = DiagonalColumnar::default();
        cipher
            .assign_key("ECABD", Alphabet::BasicLatin.slice())
            .unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
