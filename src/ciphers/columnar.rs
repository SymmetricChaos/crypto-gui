use num::Integer;

use crate::errors::CipherError;
use crate::grid::{Grid, Symbol};
use crate::text_functions::rank_str;
use super::Cipher;

pub struct Columnar {
    alphabet: String,
    key: Vec<usize>,
    key_name: String,
}

impl Columnar {
    pub fn set_key(&mut self) -> &mut String {
        self.key = rank_str(&self.key_name, &self.alphabet);
        &mut self.key_name
    }
}

impl Cipher for Columnar {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();
        let n_rows = tlen.div_ceil(&self.key.len());
        let g = Grid::from_rows(text, n_rows, n_cols, '\0', '\0');

        let mut out = String::with_capacity(text.len());
        for k in self.key.iter() {
            let mut s: String = g.get_col(*k).map(|sym| sym.to_char()).collect();
            s = s.replace(crate::grid::EMPTY, "");
            out.push_str(&s);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();
        let n_rows = tlen.div_ceil(&n_cols);
     
        let mut g = Grid::new_empty(n_rows, n_cols);
        let mut symbols = text.chars();
     
        for n in tlen..(n_rows*n_cols) {
            let coord = g.coord_from_index(n).unwrap();
            g.block_cell(coord);
        }
     
        for n in self.key.iter() {
            let column = g.get_col_mut(*n);
            for cell in column {
                if !cell.is_blocked() {
                    *cell = Symbol::Character(symbols.next().unwrap())
                }
            }
        }
     
        let out: String = g.read_rows().map(|s| s.to_char()).collect();
     
        Ok(out)   
    }

    fn randomize(&mut self, rng: &mut rand::prelude::ThreadRng) {
        todo!()
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        todo!()
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        todo!()
    }

    fn get_input_alphabet(&mut self) -> &String {
        todo!()
    }

    fn get_output_alphabet(&mut self) -> &String {
        todo!()
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }


}