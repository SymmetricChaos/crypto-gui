use rand::{Rng, prelude::StdRng, SeedableRng};
use super::Cipher;
use crate::{errors::CipherError, grid::{Grid, Symbol}, text_types::PresetAlphabet};

pub struct Grille {
    pub null_alphabet: String,
    pub grid: Grid,
    pub seed: Option<u64>,
}

impl Grille {
    fn _randomize_seeded(&mut self) {
        let mut rng: StdRng = match self.seed {
            Some(n) => SeedableRng::seed_from_u64(n),
            None => SeedableRng::from_entropy(),
        };
        for cell in self.grid.get_rows_mut() {
            if rng.gen_bool(0.5) {
                *cell = Symbol::Empty;
            } else {
                *cell = Symbol::Blocked;
            }
        }
    }
}
 
 
impl Default for Grille {
    fn default() -> Self {
        Grille { 
            null_alphabet: String::from(PresetAlphabet::BasicLatin),
            grid: Grid::new_empty(8, 8),
            seed: None,
        }
    }
}

impl Cipher for Grille {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        if self.grid.num_empty() < text.chars().count() {
            return Err(CipherError::Input("The text is too long to fit into the open spaces of the Grille".to_string()))
        }

        let mut rng: StdRng = match self.seed {
            Some(n) => SeedableRng::seed_from_u64(n),
            None => SeedableRng::from_entropy(),
        };

        let range = 0..self.null_alphabet.chars().count();

        let mut grid = self.grid.clone();
        let mut chars = text.chars();
        
        // Must be a better way to select random characters
        for cell in grid.get_rows_mut() {
            match cell {
                Symbol::Character(_) => unreachable!("there should be no characters in the Grille"),
                Symbol::Empty => { 
                    match chars.next() {
                        Some(c) => *cell = Symbol::Character(c),
                        None => *cell = Symbol::Character( self.null_alphabet.chars().nth(rng.gen_range(range.clone())).unwrap() ),
                    }
                },
                Symbol::Blocked => { *cell = Symbol::Character( self.null_alphabet.chars().nth(rng.gen_range(range.clone())).unwrap() ) },
            }
        }

        Ok(grid.get_cols().map(|x| x.to_char()).collect())
    }
 
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        if self.grid.grid_size() != text.chars().count() {
            return Err(CipherError::Input("Text is not the same size as the Grille".to_string()))
        }

        let filled_grid = Grid::from_cols(text, self.grid.num_rows(), self.grid.num_cols(), '\n', '\n');

        let mut out = String::with_capacity(self.grid.num_empty());
        for (c,s) in filled_grid.get_rows().zip(self.grid.get_rows()) {
            if s.is_empty() {
                out.push(c.to_char())
            }
        }

        Ok(out)
    }
 
    fn get_input_alphabet(&self) -> &String {
        unimplemented!("all characters are accepted, only the null alphabet can be changed")
    }
 
    fn get_mut_input_alphabet(&mut self) -> &mut String {
        unimplemented!("all characters are accepted, only the null alphabet can be changed")
    }
 
    fn reset(&mut self) {
        *self = Self::default();
    }
 
    fn randomize(&mut self, rng: &mut StdRng) {
        for cell in self.grid.get_rows_mut() {
            if rng.gen_bool(0.5) {
                *cell = Symbol::Empty;
            } else {
                *cell = Symbol::Blocked;
            }
        }
    }
 
    fn validate_settings(&self) -> Result<(), CipherError> {
        todo!()
    }
}



#[cfg(test)]
mod grille_tests {

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYD";
    const CIPHERTEXT: &'static str = "TECLESRKCQPWTKTAQPRFUOEZTXKNOVUMZDBFMQIYHEROBBHONUUXGWEDHIOJPELC";
    const SEED: Option<u64> = Some(1587782446298476294);

    #[test]
    fn encrypt_test() {
        let mut cipher = Grille::default();
        cipher.seed = SEED;
        cipher._randomize_seeded();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Grille::default();
        cipher.seed = SEED;
        cipher._randomize_seeded();
        println!("encrypting\n{}",cipher.grid);
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}