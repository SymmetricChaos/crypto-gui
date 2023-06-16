use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use utils::{
    grid::{str_to_char_grid, Grid, Symbol},
    preset_alphabet::Alphabet,
    vecstring::VecString,
};

pub struct Grille {
    null_alphabet: VecString,
    pub grid: Grid<Symbol<char>>,
    pub use_nulls: bool,
}

impl Grille {
    fn random_nulls<R: Rng>(&self, n: usize, rng: &mut R) -> Vec<Symbol<char>> {
        self.null_alphabet
            .get_rand_chars_replace(n, rng)
            .iter()
            .map(|c| Symbol::Filled(*c))
            .collect_vec()
    }

    pub fn assign_null_alphabet(&mut self, alphabet: &str) {
        self.null_alphabet = VecString::unique_from(alphabet)
    }
}

impl Default for Grille {
    fn default() -> Self {
        let mut grid = Grid::new_blocked(4, 4);
        grid.empty_cell((1, 0));
        grid.empty_cell((2, 2));
        grid.empty_cell((2, 0));
        grid.empty_cell((3, 1));
        grid.empty_cell((1, 3));
        grid.empty_cell((3, 3));
        grid.empty_cell((0, 3));
        grid.empty_cell((3, 2));

        Grille {
            null_alphabet: VecString::from(Alphabet::BasicLatin),
            grid,
            use_nulls: true,
        }
    }
}

impl Cipher for Grille {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        if self.grid.num_empty() < text.chars().count() {
            return Err(CipherError::Input(
                "the text is too long to fit into the open spaces of the grille".to_string(),
            ));
        }

        if !self.use_nulls && self.grid.num_empty() != text.chars().count() {
            return Err(CipherError::Input(
                "the text must exactly fill the empty spaces in the grille".to_string(),
            ));
        }

        let mut grid = self.grid.clone();
        let mut chars = text.chars();
        let mut nulls = self.random_nulls(self.grid.grid_size(), &mut thread_rng());

        if self.use_nulls {
            for cell in grid.get_rows_mut() {
                match cell {
                    Symbol::Filled(_) => {
                        unreachable!(
                            "the grille is predefined and should contain no Symbol::Filled cells"
                        )
                    }
                    Symbol::Empty => match chars.next() {
                        Some(c) => *cell = Symbol::Filled(c),
                        None => *cell = nulls.pop().unwrap(),
                    },
                    Symbol::Blocked => *cell = nulls.pop().unwrap(),
                }
            }
        } else {
            for cell in grid.get_rows_mut() {
                match cell {
                    Symbol::Filled(_) => {
                        unreachable!(
                            "the grille is predefined and should contain no Symbol::Filled cells"
                        )
                    }
                    Symbol::Empty => match chars.next() {
                        Some(c) => *cell = Symbol::Filled(c),
                        None => (),
                    },
                    Symbol::Blocked => (),
                }
            }
        }

        Ok(grid
            .get_cols()
            .filter(|x| x.is_filled())
            .map(|x| x.contents().unwrap())
            .collect())
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        if self.use_nulls {
            if self.grid.grid_size() != text.chars().count() {
                return Err(CipherError::Input(
                    "text is not the same size as the grille".to_string(),
                ));
            }

            let symbols = str_to_char_grid(text, '\n', '\n');
            let filled_grid = Grid::from_cols(symbols, self.grid.num_rows(), self.grid.num_cols());

            let mut out = String::with_capacity(self.grid.num_empty());
            for (c, s) in filled_grid.get_rows().zip(self.grid.get_rows()) {
                if s.is_empty() {
                    out.push(c.to_char())
                }
            }

            Ok(out)
        } else {
            if self.grid.num_empty() != text.chars().count() {
                return Err(CipherError::Input(
                    "The text must exactly fill the empty spaces in the Grille".to_string(),
                ));
            }

            let mut grid = self.grid.clone();
            let mut chars = text.chars();

            // Iterates slowly over columns indexes and quickly over rows indexes
            let coords = (0..grid.num_cols()).cartesian_product(0..grid.num_rows());

            // Write characters in by columns, skipping blocked cells
            for (c, r) in coords {
                let cell = grid.get_mut((r, c)).unwrap();
                if cell.is_empty() {
                    match chars.next() {
                        Some(c) => *cell = Symbol::Filled(c),
                        None => (),
                    }
                }
            }

            // Read the Character cells by rows, convert them to char, and collect
            Ok(grid
                .get_rows()
                .filter(|x| x.is_filled())
                .map(|x| x.to_char())
                .collect())
        }
    }
}

#[cfg(test)]
mod grille_tests {

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICK";

    #[test]
    fn encrypt_decrypt_test() {
        let cipher = Grille::default();
        let ptext = cipher.encrypt(PLAINTEXT).unwrap();
        assert_eq!(&cipher.decrypt(&ptext).unwrap()[0..8], PLAINTEXT);
    }

    #[test]
    fn encrypt_test_full_no_nulls() {
        let mut cipher = Grille::default();
        cipher.use_nulls = false;
        let ptext = cipher.encrypt(PLAINTEXT).unwrap();
        assert_eq!(&cipher.decrypt(&ptext).unwrap(), PLAINTEXT);
    }
}
