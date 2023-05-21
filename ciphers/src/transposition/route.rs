use itertools::Itertools;


use crate::{grid::Grid, text_aux::PresetAlphabet, errors::CipherError, text_functions::{random_sample_replace, rank_str}};

use super::Cipher;

pub enum RouteType {
    Snake,
    Stripe,
}

pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    
}

pub struct Route {
    pub alphabet: PresetAlphabet,
    dimensions: (usize,usize), // sliders to control this should have a minimum of 4 and a maximum of something
    pub route_type: RouteType,
    pub start: Corner,
    key: Vec<usize>,
    key_word: String,
}

impl Default for Route {
    fn default() -> Route {
        Route{ alphabet: PresetAlphabet::BasicLatin,
               dimensions: (4,5), 
               route_type: RouteType::Snake,
               start: Corner::TopLeft,
               key: Vec::new(),
               key_word: String::new(),
        }
    }
}

impl Route {

    // these two traversals should be much easier with genawaiter
    fn traverse_stripe(&self) -> impl Iterator<Item = (usize,usize)> {
        let rows = 0..self.dimensions.0;
        let cols = 0..self.dimensions.1;
        match self.start {
            TopLeft => rows.cartesian_product(cols),
            TopRight => rows.cartesian_product(cols.rev()),
            BottomLeft => rows.rev().cartesian_product(cols),
            BottomRight => rows.rev().cartesian_product(cols.rev()),
        }
    }

    fn traverse_snake(&self) -> impl Iterator<Item = (usize,usize)> {
        let rows = 0..self.dimensions.0;
        let cols = 0..self.dimensions.1;
        // match self.start {
            
        // }
        todo!()
    }
    
    fn grid(&self) -> Grid {
        Grid::new_empty(self.dimensions.0, self.dimensions.1)
    }

    pub fn control_key(&mut self) -> &mut String {
        self.key = rank_str(&self.key_word, self.alphabet.slice());
        &mut self.key_word
    }

    pub fn set_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.key = rank_str(&self.key_word, self.alphabet.slice());
    }
    
    fn encrypt_stripe(&self, text: &str) -> String {
        let mut g = self.grid();
        let cell_idxs = self.traverse_stripe();
        
        for (idx,c) in cell_idxs.zip(text.chars()) {
            g.replace(idx,c);
        }
        
        // Then read it out by columns according to the key
        let mut out = String::with_capacity(text.len());
        for k in self.key {
            g.get_col(k).map(|c| out.push(c));
        }
        out
    }
    
    fn decrypt_stripe(&self, text: &str) -> String {
        let mut g = self.grid();
        let columns = text.chars().collect_vec().chunks_exact(self.dimensions.0);

        // Write the text in by columns according to the key
        for (k, col) in self.key.iter().zip( columns ) {
            let c: String = col.iter().collect();
            g.write_col_n(*k, &c );
        }
        
    }

    fn encrypt_snake(&self, text: &str) -> String {
    
        let mut g = self.grid();
        
        // Create an iterator of (usize,usize) covering the grid from start to the opposite corner
        let cell_idxs = self.traverse();
        
        // Write the text into the rows reversing the order each time
        for (idx,c) in cell_idxs.zip(text.chars()) {
            g.replace(idx,c);
        }
        
        // Then read it out by columns according to the key
        let mut out = String::with_capacity(text.len());
        for k in self.key {
            g.get_col(k).map(|c| out.push(c));
        }
        out
    }
    
    fn decrypt_snake(&self, text: &str) -> String {
        
        let mut g = self.grid();
        let columns = text.chars().collect_vec().chunks_exact(self.dimensions.0);

        // Write the text in by columns according to the key
        for (k, col) in self.key.iter().zip( columns ) {
            let c: String = col.iter().collect();
            g.write_col_n(*k, &c );
        }

        // Read it off row by row reversing the order each time
        let mut out = String::with_capacity(text.len());
        for n in 0..self.dimensions.0 {
            if n % 2 == 0{
                g.get_row(n).map(|c| out.push(c));
            } else {
                g.get_row(n).rev().map(|c| out.push(c));
            }

        }

        out
    }
}

impl Cipher for Route {

    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        if self.dimensions.0 * self.dimensions.1 != text.chars().count() {
            return Err(CipherError::Input())
        }
        match self.route_type {
            Snake => Ok(self.encrypt_snake(text)),
            Stripe => Ok(self.encrypt_stripe(text))
        }
    }
    
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        if self.dimensions.0 * self.dimensions.1 != text.chars().count() {
            return Err(CipherError::Input())
        }
        match self.route_type {
            Snake => Ok(self.decrypt_snake(text)),
            Stripe => Ok(self.decrypt_stripe(text))
        }
    }

    fn randomize(&mut self, rng: &mut StdRng) {
        self.key_word = random_sample_replace(self.alphabet.slice(), self.dimensions.1, rng);
        self.key = rank_str(&self.key_word, self.alphabet.slice());
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}