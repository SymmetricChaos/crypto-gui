use crate::{ciphers::Cipher, errors::CipherError, grid::{Grid, Symbol, str_to_char_grid}, text_aux::{PresetAlphabet, VecString}};
use itertools::Itertools;
use rand::{prelude::StdRng, Rng, SeedableRng};

pub struct TurningGrille {
    pub null_alphabet_string: String,
    null_alphabet: VecString,
    pub grid: Grid<char>,
    pub seed: Option<u64>,
    pub use_nulls: bool,
}

impl TurningGrille {
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

    fn random_null(&self) -> Symbol<char> {
        let mut rng = self.get_rng();
        Symbol::Character(self.null_alphabet.get_rand_char(&mut rng))
    }

    fn get_rng(&self) -> StdRng {
        match self.seed {
            Some(n) => SeedableRng::seed_from_u64(n),
            None => SeedableRng::from_entropy(),
        }
    }

    pub fn increase_size(&mut self) {
        if self.grid.num_cols() >= 16 {
            return ()
        }
        self.grid.add_col();
        self.grid.add_col();
        self.grid.add_row();
        self.grid.add_row();
    }

    pub fn decrease_size(&mut self) {
        if self.grid.num_cols() <= 4 {
            return ()
        }
        self.grid.del_col();
        self.grid.del_col();
        self.grid.del_row();
        self.grid.del_row();
    }

}

impl Default for TurningGrille {
    fn default() -> Self {
        TurningGrille {
            null_alphabet_string: String::from(PresetAlphabet::BasicLatin),
            null_alphabet: VecString::from(PresetAlphabet::BasicLatin),
            grid: Grid::new_blocked(8, 8),
            seed: None,
            use_nulls: true,
        }
    }
}

impl Cipher for TurningGrille {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn randomize(&mut self, _rng: &mut StdRng) {
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
mod turning_grille_tests {

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYD";
    const CIPHERTEXT: &'static str =
        "TECLESRKCQPWTKTAQPRFUOEZTXKNOVUMZDBFMQIYHEROBBHONUUXGWEDHIOJPELC";
    const CIPHERTEXT_NO_NULLS: &'static str = "TECSRQWTAUOZKNVBFMYHROHUXEDIOJPEL";
    const SEED: Option<u64> = Some(1587782446298476294);

    #[test]
    fn encrypt_test_full_grid() {
        let mut cipher = TurningGrille::default();
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test_full_grid() {
        let mut cipher = TurningGrille::default();
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_full_grid_no_nulls() {
        let mut cipher = TurningGrille::default();
        cipher.use_nulls = false;
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_NO_NULLS);
    }

    #[test]
    fn decrypt_test_full_grid_no_nulls() {
        let mut cipher = TurningGrille::default();
        cipher.use_nulls = false;
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.decrypt(CIPHERTEXT_NO_NULLS).unwrap(), PLAINTEXT);
    }
}
