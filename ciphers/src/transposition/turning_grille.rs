use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use rand::thread_rng;
use std::{collections::HashSet, num::ParseIntError};
use utils::{
    grid::{Grid, Symbol},
    preset_alphabet::Alphabet,
    vecstring::VecString,
};

pub struct TurningGrille {
    pub null_alphabet: VecString,
    pub grid: Grid<Symbol<char>>,
    pub keys: [Vec<usize>; 4],
}

impl Default for TurningGrille {
    fn default() -> Self {
        TurningGrille {
            null_alphabet: VecString::from(Alphabet::BasicLatin),
            grid: Grid::new_blocked(8, 8),
            keys: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        }
    }
}

impl TurningGrille {
    // Concept is simple:
    //     user provides a list of numbers from 0 to the size of the subgrille
    //     a new blocked grid is created
    //     the first quarter of the numbers are used to punch out spaces
    //     then the grid is rotated and the next quarters, and so on
    pub fn build_grid(&mut self) -> Result<(), CipherError> {
        // These next two blocks find likely errors
        if self.key_length() != self.subgrille_size() {
            return Err(CipherError::Key(format!(
                "there should be {} key values provided but {} were found",
                self.subgrille_size(),
                self.key_length()
            )));
        }

        let mut set = HashSet::with_capacity(self.subgrille_size());
        for key in &self.keys {
            for n in key {
                if n >= &self.subgrille_size() {
                    return Err(CipherError::Key(format!("invalid key value found: {}", n)));
                }
                if !set.insert(n) {
                    return Err(CipherError::Key(format!(
                        "duplicate key value found: {}",
                        n
                    )));
                }
            }
        }

        // Block off the whole grid
        self.grid.apply(|_| Symbol::Blocked);

        // "Stamp" each chunk onto the grid, rotating after each stamp
        for key in &self.keys {
            for n in key {
                let row = n / self.subgrille_width();
                let col = n % self.subgrille_width();
                self.grid[(row, col)] = Symbol::Empty;
            }
            self.grid.rotate();
        }
        Ok(())
    }

    pub fn build_key(&mut self, key_strings: &[String; 4]) -> Result<(), ParseIntError> {
        for (n, string) in key_strings.iter().enumerate() {
            self.keys[n].clear();
            let nums = string.split(',');
            for s in nums {
                if s.is_empty() {
                    continue;
                }
                self.keys[n].push(s.trim().parse::<usize>()?);
            }
        }
        Ok(())
    }

    fn key_length(&self) -> usize {
        self.keys.iter().fold(0, |acc, vec| acc + vec.len())
    }

    pub fn increase_size(&mut self) {
        if self.grid.num_cols() >= 20 {
            return ();
        }
        self.grid.add_col();
        self.grid.add_col();
        self.grid.add_row();
        self.grid.add_row();
        self.grid.apply(|_| Symbol::Blocked);
    }

    pub fn decrease_size(&mut self) {
        if self.grid.num_cols() <= 4 {
            return ();
        }
        self.grid.del_col();
        self.grid.del_col();
        self.grid.del_row();
        self.grid.del_row();
        self.grid.apply(|_| Symbol::Blocked);
    }

    pub fn subgrille_width(&self) -> usize {
        self.grid.num_cols() / 2
    }

    pub fn subgrille_size(&self) -> usize {
        self.subgrille_width().pow(2)
    }
}

impl Cipher for TurningGrille {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut crypto_grid = self.grid.clone();
        let mut output_grid: Grid<char> =
            Grid::new_default(self.grid.num_rows(), self.grid.num_cols());

        let w = self.grid.num_cols();
        let section = crypto_grid.grid_size() / 4;

        for i in 0..4 {
            let lo = i * section;
            let hi = lo + section;
            let mut snip = text[lo..hi].chars();
            for row in 0..w {
                for col in 0..w {
                    if crypto_grid[(row, col)].is_empty() {
                        output_grid[(row, col)] = snip
                            .next()
                            .unwrap_or(self.null_alphabet.get_rand_char(&mut thread_rng()))
                    }
                }
            }
            crypto_grid.rotate();
        }

        Ok(output_grid.get_cols().collect::<String>())
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let input_grid: Grid<char> = Grid::from_cols(
            text.chars().collect_vec(),
            self.grid.num_rows(),
            self.grid.num_cols(),
        );
        let mut crypto_grid = self.grid.clone();

        let w = self.grid.num_cols();

        let mut out = String::with_capacity(text.len());
        for _ in 0..4 {
            for row in 0..w {
                for col in 0..w {
                    if crypto_grid[(row, col)].is_empty() {
                        out.push(input_grid[(row, col)])
                    }
                }
            }
            crypto_grid.rotate();
        }
        Ok(out)
    }
}

#[cfg(test)]
mod turning_grille_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYD";
    const CIPHERTEXT: &'static str =
        "TECLESRKCQPWTKTAQPRFUOEZTXKNOVUMZDBFMQIYHEROBBHONUUXGWEDHIOJPELC";

    #[test]
    fn encrypt_test_full_grid() {
        let cipher = TurningGrille::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test_full_grid() {
        let cipher = TurningGrille::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
