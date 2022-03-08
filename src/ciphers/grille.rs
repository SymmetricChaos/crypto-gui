use rand::{Rng, prelude::ThreadRng};
use super::Cipher;
use crate::{errors::CipherError, grid::{Grid, Symbol}, text_types::PresetAlphabet};

pub struct Grille {
    pub null_alphabet: String,
    pub grid: Grid
}
 
 
impl Default for Grille {
    fn default() -> Self {
        Grille { 
            null_alphabet: String::from(PresetAlphabet::BasicLatin),
            grid: Grid::new_empty(4, 4),
        }
    }
}
 
 
impl Cipher for Grille {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut grid = self.grid.clone();

        // clone self.grid
        // write text into Empty cells
        // write random from self.null_alphabet into Blocked cells
        // read off grid
        todo!()
    }
 
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        // zip text with self.grid and filter out Blocked
        todo!()
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
 
    fn randomize(&mut self, rng: &mut ThreadRng) {
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