use itertools::Itertools;

use utils::{functions::rank_str, grid::Grid, math_functions::Parity};

use crate::{Cipher, CipherError};

pub struct Amsco {
    pub key: Vec<usize>,
    pub parity: Parity,
    pub spacer: char,
}

impl Default for Amsco {
    fn default() -> Self {
        Self {
            key: Vec::new(),
            parity: Parity::Odd,
            spacer: 'X',
        }
    }
}

impl Amsco {
    pub fn assign_key(&mut self, key_word: &str, alphabet: &str) {
        self.key = rank_str(key_word, alphabet);
    }

    pub fn groups(&self, text: &str) -> Vec<(char, Option<char>)> {
        let mut pattern = self.parity.cycle();
        let mut c = text.chars();
        let mut out = Vec::new();
        loop {
            let dist = pattern.next().unwrap();
            if let Some(ch1) = c.next() {
                match dist {
                    Parity::Odd => out.push((ch1, None)),
                    Parity::Even => {
                        if let Some(ch2) = c.next() {
                            out.push((ch1, Some(ch2)))
                        } else {
                            out.push((ch1, Some(self.spacer)));
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }

        // AMSCO requires padding to a multiple of the key length
        while out.len() % self.key.len() != 0 {
            match pattern.next().unwrap() {
                Parity::Odd => out.push((self.spacer, None)),
                Parity::Even => out.push((self.spacer, Some(self.spacer))),
            };
        }
        out
    }
}

impl Cipher for Amsco {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let groups = self.groups(text);

        let n_cols = self.key.len();
        let n_rows = num::Integer::div_ceil(&groups.len(), &n_cols);
        let mut groups_iter = groups.into_iter();

        let mut g = Grid::<String>::new_default(n_rows, n_cols);

        for k in self.key.iter() {
            for row in g.get_col_mut(*k) {
                if let Some(a) = groups_iter.next() {
                    match a {
                        (c, None) => *row = c.to_string(),
                        (c1, Some(c2)) => *row = format!("{c1}{c2}"),
                    }
                }
            }
        }

        let out = g.get_rows().join("");
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // Build a grid of 1s and 2s, filled by columns in the order given by the key
        let mut pattern = self.parity.cycle();

        let groups_len = self.groups(text).len(); // We need the length but not the actual groups
        let n_cols = self.key.len();
        let n_rows = num::Integer::div_ceil(&groups_len, &n_cols);
        let mut g = Grid::<String>::new_default(n_rows, n_cols);

        for k in self.key.iter() {
            for cell in g.get_col_mut(*k) {
                match pattern.next().unwrap() {
                    Parity::Odd => *cell = "1".into(),
                    Parity::Even => *cell = "2".into(),
                }
            }
        }

        let mut cs = text.chars();

        // Read the ciphertext into the grid by rows taking the number of letters requested each time
        for cell in g.get_rows_mut() {
            if cell == "1" {
                match cs.next() {
                    Some(c) => *cell = c.to_string(),
                    None => *cell = "X".to_string(),
                }
            }
            if cell == "2" {
                if let Some(c1) = cs.next() {
                    match cs.next() {
                        Some(c2) => *cell = format!("{c1}{c2}"),
                        None => cell.clear(),
                    }
                } else {
                    *cell = "XX".to_string()
                }
            }
        }

        // Read off the grid by columns in the order given by the key
        let mut out = String::with_capacity(text.len());
        for k in self.key.iter() {
            for cell in g.get_col(*k) {
                out.push_str(cell)
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod amsco_tests {
    use utils::functions::rank_vec;

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "XZTVEKBJUYDHERRMOQTHOWPSGXUIENOXCLAFO";

    #[test]
    fn encrypt_test() {
        let mut cipher = Amsco::default();
        cipher.key = rank_vec(&vec![2, 4, 0, 3, 1]);
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Amsco::default();
        cipher.key = rank_vec(&vec![2, 4, 0, 3, 1]);
        assert_eq!(
            cipher.decrypt(CIPHERTEXT).unwrap(),
            "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGXX"
        );
    }
}