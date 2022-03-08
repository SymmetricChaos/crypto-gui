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
        if self.grid.num_empty() < text.chars().count() {
            return Err(CipherError::Input("The text is too long to fit into the open spaces of the Grille".to_string()))
        }

        let mut rng = ThreadRng::default();
        let range = 0..self.null_alphabet.chars().count();

        let mut grid = self.grid.clone();
        let mut chars = text.chars();
        
        // Must be a better way to select random characters
        for cell in grid.get_rows_mut() {
            match cell {
                Symbol::Character(_) => unreachable!("there should be no characters in the Grille"),
                Symbol::Empty => { 
                    match chars.next() {
                        Some(_) => *cell = Symbol::Character(chars.next().unwrap()),
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

        let mut out = String::with_capacity(self.grid.num_empty());
        for (c,s) in text.chars().zip(self.grid.get_rows()) {
            if s.is_empty() {
                out.push(c)
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