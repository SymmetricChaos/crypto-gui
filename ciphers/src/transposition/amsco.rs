use itertools::Itertools;
use num::Integer;
use utils::{
    functions::rank_str,
    grid::{str_to_char_grid, Grid},
    preset_alphabet::Alphabet,
    vecstring::VecString,
};

use crate::{Cipher, CipherError};

pub enum Parity {
    Odd,
    Even,
}

pub struct Amsco {
    pub alphabet: VecString,
    pub key: Vec<usize>,
    pub parity: Parity,
}

impl Default for Amsco {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(Alphabet::BasicLatin),
            key: Vec::new(),
            parity: Parity::Odd,
        }
    }
}

impl Amsco {
    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(alphabet);
    }

    pub fn assign_key(&mut self, key_word: &str, alphabet: &str) {
        self.key = rank_str(key_word, alphabet);
    }

    pub fn groups(&self, text: &str) -> Vec<(char, Option<char>)> {
        let pattern = match self.parity {
            Parity::Odd => [1, 2].iter().cycle(),
            Parity::Even => [2, 1].iter().cycle(),
        };
        let mut c = text.chars();
        let mut out = Vec::new();
        for dist in pattern {
            if let Some(ch1) = c.next() {
                if dist == &1 {
                    out.push((ch1, None));
                }
                if dist == &2 {
                    if let Some(ch2) = c.next() {
                        out.push((ch1, Some(ch2)))
                    } else {
                        out.push((ch1, None));
                        break;
                    }
                }
            } else {
                break;
            }
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
        for c in 0..4 {
            println!("{:?}", g.get_col(c).collect_vec());
        }
        let out = g.get_rows().join("");
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}

#[cfg(test)]
mod amsco_tests {
    use utils::functions::rank_vec;

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "MTERPSHELAOWOQZNVEUIYDFORCOXTHKBGJU";

    #[test]
    fn grouping_test() {
        let cipher = Amsco::default();
        println!("{:?}", cipher.groups(PLAINTEXT));
    }

    #[test]
    fn encrypt_test() {
        let mut cipher = Amsco::default();
        cipher.key = rank_vec(&vec![2, 0, 3, 1]);
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }
}
