use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::{collections::HashSet, num::ParseIntError};

use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    grid::{Grid, Symbol},
    text_aux::{PresetAlphabet, VecString},
};

pub struct TurningGrille {
    pub null_alphabet_string: String,
    null_alphabet: VecString,
    pub grid: Grid<Symbol<char>>,
    pub key_strings: [String; 4],
    keys: [Vec<usize>; 4],
}

impl Default for TurningGrille {
    fn default() -> Self {
        TurningGrille {
            null_alphabet_string: String::from(PresetAlphabet::BasicLatin),
            null_alphabet: VecString::from(PresetAlphabet::BasicLatin),
            grid: Grid::new_blocked(8, 8),
            key_strings: [String::new(), String::new(), String::new(), String::new()],
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
    pub fn build_grid(&mut self) -> Result<(), Error> {
        // These next two blocks find likely errors
        if self.key_length() != self.subgrille_size() {
            return Err(Error::Key(format!(
                "there should be {} key values provided but {} were found",
                self.subgrille_size(),
                self.key_length()
            )));
        }

        let mut set = HashSet::with_capacity(self.subgrille_size());
        for key in &self.keys {
            for n in key {
                if n >= &self.subgrille_size() {
                    return Err(Error::Key(format!("invalid key value found: {}", n)));
                }
                if !set.insert(n) {
                    return Err(Error::Key(format!("duplicate key value found: {}", n)));
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

    pub fn build_key(&mut self) -> Result<(), ParseIntError> {
        for (n, string) in self.key_strings.iter().enumerate() {
            self.keys[n].clear();
            let nums = string.split(',');
            for s in nums {
                self.keys[n].push(s.trim().parse::<usize>()?);
            }
        }
        Ok(())
    }

    // I love .fold()
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
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let mut crypto_grid = self.grid.clone();
        let mut output_grid: Grid<char> =
            Grid::new_default(self.grid.num_rows(), self.grid.num_cols());

        let w = self.grid.num_cols();
        let section = crypto_grid.grid_size() / 4;
        let mut rng = get_global_rng();

        for i in 0..4 {
            let lo = i * section;
            let hi = lo + section;
            let mut snip = text[lo..hi].chars();
            for row in 0..w {
                for col in 0..w {
                    if crypto_grid[(row, col)].is_empty() {
                        output_grid[(row, col)] = snip
                            .next()
                            .unwrap_or(self.null_alphabet.get_rand_char(&mut rng))
                    }
                }
            }
            crypto_grid.rotate();
        }

        Ok(output_grid.get_cols().collect::<String>())
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
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

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn randomize(&mut self) {
        let mut rng = get_global_rng();
        let mut nums = (0..self.subgrille_size()).collect_vec();
        nums.shuffle(&mut *rng);
        let mut ctr = 0;

        for n in 0..4 {
            self.key_strings[n].clear();
            self.keys[n].clear();
        }

        for n in nums {
            self.keys[ctr].push(n);
            if !self.key_strings[ctr].is_empty() {
                self.key_strings[ctr].push_str(", ")
            }
            self.key_strings[ctr].push_str(&n.to_string());
            ctr = (ctr + 1) % 4
        }

        self.build_grid().unwrap();
    }
}

#[cfg(test)]
mod turning_grille_tests {

    use crate::global_rng::seed_global_rng;

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYD";
    const CIPHERTEXT: &'static str =
        "TECLESRKCQPWTKTAQPRFUOEZTXKNOVUMZDBFMQIYHEROBBHONUUXGWEDHIOJPELC";

    #[test]
    fn encrypt_test_full_grid() {
        seed_global_rng(1587782446298476294);
        let mut cipher = TurningGrille::default();
        cipher.randomize();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test_full_grid() {
        seed_global_rng(1587782446298476294);
        let mut cipher = TurningGrille::default();
        cipher.randomize();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
