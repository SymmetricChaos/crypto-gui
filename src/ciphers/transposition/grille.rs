use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    grid::{str_to_char_grid, Grid, Symbol},
    text_aux::{PresetAlphabet, VecString},
};
use itertools::Itertools;
use rand::{prelude::StdRng, Rng, SeedableRng};

pub struct Grille {
    pub null_alphabet_string: String,
    null_alphabet: VecString,
    pub grid: Grid<Symbol<char>>,
    pub seed: Option<u64>,
    pub use_nulls: bool,
}

impl Grille {
    fn _randomize_seeded(&mut self) {
        let mut rng = self.get_rng();
        for cell in self.grid.get_rows_mut() {
            if rng.gen_bool(0.5) {
                *cell = Symbol::Empty;
            } else {
                *cell = Symbol::Blocked;
            }
        }
    }

    fn random_nulls(&self, n: usize) -> Vec<Symbol<char>> {
        self.null_alphabet
            .get_rand_chars_replace(n, &mut get_global_rng())
            .iter()
            .map(|c| Symbol::Character(*c))
            .collect_vec()
    }

    fn get_rng(&self) -> StdRng {
        match self.seed {
            Some(n) => SeedableRng::seed_from_u64(n),
            None => SeedableRng::from_entropy(),
        }
    }
}

impl Default for Grille {
    fn default() -> Self {
        Grille {
            null_alphabet_string: String::from(PresetAlphabet::BasicLatin),
            null_alphabet: VecString::from(PresetAlphabet::BasicLatin),
            grid: Grid::new_empty(4, 4),
            seed: None,
            use_nulls: true,
        }
    }
}

impl Cipher for Grille {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        if self.grid.num_empty() < text.chars().count() {
            return Err(Error::Input(
                "The text is too long to fit into the open spaces of the Grille".to_string(),
            ));
        }

        if !self.use_nulls && self.grid.num_empty() != text.chars().count() {
            return Err(Error::Input(
                "The text must exactly fill the empty spaces in the Grille".to_string(),
            ));
        }

        let mut grid = self.grid.clone();
        let mut chars = text.chars();
        let mut nulls = self.random_nulls(self.grid.grid_size());

        for cell in grid.get_rows_mut() {
            match cell {
                Symbol::Character(_) => {
                    unreachable!("encryption should encounter no Symbol::Character cells")
                }
                Symbol::Empty => match chars.next() {
                    Some(c) => *cell = Symbol::Character(c),
                    None => *cell = nulls.pop().unwrap(),
                },
                Symbol::Blocked => *cell = nulls.pop().unwrap(),
            }
        }

        Ok(grid
            .get_cols()
            .filter(|x| x.is_character())
            .map(|x| x.contents().unwrap())
            .collect())
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        if self.use_nulls {
            if self.grid.grid_size() != text.chars().count() {
                return Err(Error::Input(
                    "Text is not the same size as the Grille".to_string(),
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
                return Err(Error::Input(
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
                        Some(c) => *cell = Symbol::Character(c),
                        None => (),
                    }
                }
            }

            // Read the Character cells by rows, convert them to char, and collect
            Ok(grid
                .get_rows()
                .filter(|x| x.is_character())
                .map(|x| x.to_char())
                .collect())
        }
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn randomize(&mut self) {
        let mut rng = self.get_rng();
        for cell in self.grid.get_rows_mut() {
            if rng.gen_bool(0.5) {
                *cell = Symbol::Empty;
            } else {
                *cell = Symbol::Blocked;
            }
        }
    }
}

#[cfg(test)]
mod grille_tests {

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYD";
    const CIPHERTEXT: &'static str =
        "TECLESRKCQPWTKTAQPRFUOEZTXKNOVUMZDBFMQIYHEROBBHONUUXGWEDHIOJPELC";
    const CIPHERTEXT_NO_NULLS: &'static str = "TECSRQWTAUOZKNVBFMYHROHUXEDIOJPEL";
    const SEED: Option<u64> = Some(1587782446298476294);

    #[test]
    fn encrypt_test_full_grid() {
        let mut cipher = Grille::default();
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test_full_grid() {
        let mut cipher = Grille::default();
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_full_grid_no_nulls() {
        let mut cipher = Grille::default();
        cipher.use_nulls = false;
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_NO_NULLS);
    }

    #[test]
    fn decrypt_test_full_grid_no_nulls() {
        let mut cipher = Grille::default();
        cipher.use_nulls = false;
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.decrypt(CIPHERTEXT_NO_NULLS).unwrap(), PLAINTEXT);
    }
}
