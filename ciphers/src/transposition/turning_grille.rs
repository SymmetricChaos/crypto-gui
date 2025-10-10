use crate::traits::Cipher;
use itertools::Itertools;
use rand::thread_rng;
use std::{collections::HashSet, num::ParseIntError};
use utils::{
    errors::GeneralError,
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
        let mut new = TurningGrille {
            null_alphabet: VecString::from(Alphabet::BasicLatin),
            grid: Grid::new_blocked(8, 8),
            keys: [
                vec![0, 1, 2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11],
                vec![12, 13, 14, 15],
            ],
        };
        new.build_grid().unwrap();
        new
    }
}

impl TurningGrille {
    // Concept is simple:
    //     user provides a list of numbers from 0 to the size of the subgrille
    //     a new blocked grid is created
    //     the first quarter of the numbers are used to punch out spaces
    //     then the grid is rotated and the next quarters, and so on
    pub fn build_grid(&mut self) -> Result<(), GeneralError> {
        // These next two blocks find likely errors
        if self.key_length() != self.subgrille_size() {
            return Err(GeneralError::key(format!(
                "there should be {} key values provided but {} were found",
                self.subgrille_size(),
                self.key_length()
            )));
        }

        let mut set = HashSet::with_capacity(self.subgrille_size());
        for key in &self.keys {
            for n in key {
                if n >= &self.subgrille_size() {
                    return Err(GeneralError::key(format!("invalid key value found: {}", n)));
                }
                if !set.insert(n) {
                    return Err(GeneralError::key(format!(
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

    pub fn grille_width(&self) -> usize {
        self.grid.num_cols()
    }

    pub fn grille_size(&self) -> usize {
        self.grid.grid_size()
    }

    pub fn subgrille_width(&self) -> usize {
        self.grille_width() / 2
    }

    pub fn subgrille_size(&self) -> usize {
        self.grille_size() / 4
    }

    pub fn assign_null_alphabet(&mut self, alphabet: &str) {
        self.null_alphabet = VecString::unique_from(alphabet)
    }
}

impl Cipher for TurningGrille {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        if text.chars().count() > self.grille_size() {
            return Err(GeneralError::input(format!(
                "a {}x{} turning grille cipher can encrypt a maximum of {} characters at a time",
                self.grille_width(),
                self.grille_width(),
                self.grille_size()
            )));
        }

        let mut crypto_grid = self.grid.clone();
        let mut output_grid: Grid<char> =
            Grid::new_default(self.grid.num_rows(), self.grid.num_cols());

        let nulls = self
            .null_alphabet
            .get_rand_chars_replace(self.grille_size() - text.chars().count(), &mut thread_rng());

        let padded_text = {
            let mut s = String::with_capacity(self.grille_size());
            s.push_str(text);
            for c in nulls {
                s.push(c)
            }
            s
        };

        let w = self.grid.num_cols();
        let section = self.subgrille_size();

        for mut chunk in &padded_text.chars().chunks(section) {
            for row in 0..w {
                for col in 0..w {
                    if crypto_grid[(row, col)].is_empty() {
                        output_grid[(row, col)] = chunk.next().unwrap()
                    }
                }
            }
            crypto_grid.rotate();
        }

        Ok(output_grid.get_cols().collect::<String>())
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        if text.chars().count() != self.grille_size() {
            return Err(GeneralError::input(format!(
                "to decrypt a {}x{} turning grille cipher exactly {} characters are needed",
                self.grille_width(),
                self.grille_width(),
                self.grille_size()
            )));
        }

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

    const PTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGPOHTZAXHOSKWEZSOXIBTNEXJKQVNV";
    const CTEXT: &'static str = "TUGXEXJVHMPIBROXEPOBRAZYQSHTAXHOUICKTOKSZSONHWQKDOTZENVWJOVELFNE";

    #[test]
    fn encrypt_test_full_grid() {
        let cipher = TurningGrille::default();
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test_full_grid() {
        let cipher = TurningGrille::default();
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn encrypt_decrypt_with_nulls() {
        let cipher = TurningGrille::default();
        let ptext = "THISISASMALLAMOUNTOFSAMPLETEXT";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(&cipher.decrypt(&ctext).unwrap()[0..30], ptext);
    }
}
