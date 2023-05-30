use utils::{
    functions::rank_str, grid::str_to_char_grid, preset_alphabet::Alphabet, vecstring::VecString,
};

use crate::{Cipher, CipherError};

pub enum Parity {
    Odd,
    Even,
}

pub struct Amsco {
    alphabet: VecString,
    key: Vec<usize>,
    parity: Parity,
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

    pub fn groups(&self, text: &str) -> Vec<(char, char)> {
        let pattern = match self.parity {
            Parity::Odd => [1, 2].iter().cycle(),
            Parity::Even => [2, 1].iter().cycle(),
        };
        let mut c = text.chars();
        let mut out = Vec::new();
        for dist in pattern {
            if let Some(ch1) = c.next() {
                if dist == &1 {
                    out.push((ch1, '\n'));
                }
                if dist == &2 {
                    if let Some(ch2) = c.next() {
                        out.push((ch1, ch2))
                    } else {
                        out.push((ch1, '\n'));
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

// impl Cipher for Amsco {
//     fn encrypt(&self, text: &str) -> Result<String, CipherError> {
//         let tlen = text.chars().count();
//         let n_cols = self.key.len();
//         if n_cols < 2 {
//             return Err(CipherError::key(
//                 "The key for an AMSCO cipher must have at least two characters",
//             ));
//         }

//         let symbols = str_to_char_grid(text, '\0', '\0');
//         let g = Grid::<&str>::from_cols(symbols, n_rows, n_cols);

//         let mut out = String::with_capacity(text.len());
//         for k in self.key.iter() {
//             let mut s: String = g.get_col(*k).map(|sym| sym.to_char()).collect();
//             s = s.replace(crate::grid::EMPTY, "");
//             s = s.replace(crate::grid::BLOCK, "");
//             out.push_str(&s);
//         }

//         Ok(out)
//     }

//     fn decrypt(&self, text: &str) -> Result<String, CipherError> {
//         let tlen = text.chars().count();
//         let n_cols = self.key.len();
//         if n_cols < 2 {
//             return Err(CipherError::key(
//                 "The key for an AMSCO cipher must have at least two characters",
//             ));
//         }
//         todo!()
//     }
// }

#[cfg(test)]
mod amsco_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "WKHTXLFNEURZQIRAMXPSVRYHUWKHODCBGRJ";

    #[test]
    fn grouping_test() {
        let mut cipher = Amsco::default();
        println!("{}", cipher.groups(PLAINTEXT));
    }
}
