use std::num::ParseIntError;

use crate::{ciphers::Cipher, errors::CipherError, grid::{Grid, Symbol}, text_aux::{PresetAlphabet, VecString}};
use itertools::Itertools;
use rand::{prelude::StdRng, Rng, SeedableRng};



pub struct TurningGrille {
    pub null_alphabet_string: String,
    null_alphabet: VecString,
    pub grid: Grid<Symbol<char>>,
    pub seed: Option<u64>,
    pub key_string: String,
    key: Vec<usize>,
}

impl Default for TurningGrille {
    fn default() -> Self {
        TurningGrille {
            null_alphabet_string: String::from(PresetAlphabet::BasicLatin),
            null_alphabet: VecString::from(PresetAlphabet::BasicLatin),
            grid: Grid::new_blocked(8, 8),
            seed: None,
            key_string: String::new(),
            key: Vec::new(),
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

        self.grid.apply(|_| Symbol::Blocked);

        let w = self.subgrille_width();

        if self.key.len() != w*w {
            return Err(CipherError::key("not enough key values provided"))
        }

        for (pos, n) in self.key.iter().enumerate() {
            if pos % w == 0 {
                self.grid.rotate()
            }
            let col = n % w;
            let row = n / w;
            self.grid[(row, col)] = Symbol::Empty;
        }
        self.grid.rotate();
        Ok(())
    }

    pub fn build_key(&mut self) -> Result<(),ParseIntError> {
        let strings = self.key_string.split(',');
        let mut new_key = Vec::with_capacity(self.key.len());
        for s in strings.unique() {
            new_key.push( s.trim().parse::<usize>()? );
        }
        Ok(())
    }

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

    fn random_nulls(&self, n: usize) -> Vec<char> {
        let mut rng = self.get_rng();
        self.null_alphabet.get_rand_chars_replace(n, &mut rng).iter().map(|c| *c).collect_vec()
    }


    fn get_rng(&self) -> StdRng {
        match self.seed {
            Some(n) => SeedableRng::seed_from_u64(n),
            None => SeedableRng::from_entropy(),
        }
    }

    pub fn increase_size(&mut self) {
        if self.grid.num_cols() >= 20 {
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

    pub fn subgrille_width(&self) -> usize {
        self.grid.num_cols()/2
    }

}


impl Cipher for TurningGrille {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {

        let mut crypto_grid = self.grid.clone();
        let mut output_grid: Grid<char> = Grid::new_default(self.grid.num_rows(), self.grid.num_cols());

        let w = self.grid.num_cols();
        let section = crypto_grid.grid_size()/4;

        for i in 0..4 {
            let lo = i*section;
            let hi = lo+section;
            let mut snip = text[lo..hi].chars();
            for row in 0..w {
                for col in 0..w {
                    if crypto_grid[(row,col)].is_empty() {
                        output_grid[(row,col)] = snip.next().unwrap()
                    }
                }
            }
            crypto_grid.rotate();
        }
        
        Ok(output_grid.get_cols().collect::<String>())
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {

        let input_grid: Grid<char> = Grid::from_cols(text.chars().collect_vec(), self.grid.num_rows(), self.grid.num_cols());
        let mut crypto_grid = self.grid.clone();

        let w = self.grid.num_cols();

        let mut out = String::with_capacity(text.len());
        for _ in 0..4 {
            for row in 0..w {
                for col in 0..w {
                    if crypto_grid[(row,col)].is_empty() {
                        out.push(input_grid[(row,col)])
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

}
