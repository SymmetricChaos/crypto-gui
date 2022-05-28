use crate::{text_aux::Alphabet, errors::CipherError};

use super::Cipher;

pub enum QuagmireVersion {
    V1,
    V2,
    V3,
    V4,
}

pub struct Quagmire {
    pub version: QuagmireVersion,
    pub alphabet_string: String,
    pub pt_key_string: String,
    pt_key: Alphabet,
    pub ct_key_string: String,
    ct_key: Alphabet,
    pub ind_key_string: String,
    ind_key: Vec<usize>,
    pub indicator: char,
}

impl Default for Quagmire {
    fn default() -> Quagmire {
        Self{
            version: QuagmireVersion::V1,
            alphabet_string: String::new(),
            pt_key_string: String::new(),
            pt_key: Alphabet::new(),
            ct_key_string: String::new(),
            ct_key: Alphabet::new(),
            ind_key_string: String::new(),
            ind_key: Vec::new(),
            indicator: 'A',
        }
    }
}

impl Quagmire {
    pub fn assign_pt_key(&mut self, key: &str) {
        self.pt_key_string = key.to_string();
        self.set_pt_key();
    }
    
    pub fn assign_ct_key(&mut self, key: &str) {
        self.ct_key_string = key.to_string();
        self.set_ct_key();
    }
    
    pub fn assign_ind_key(&mut self, key: &str) {
        self.ind_key_string = key.to_string();
        self.set_ind_key();
    }
    
    pub fn set_pt_key(&mut self) {
        self.pt_key = Alphabet::from(&self.pt_key_string);
    }
    
    pub fn set_ct_key(&mut self) {
        self.ct_key = Alphabet::from(&self.ct_key_string);
    }
    
    // Converts the ind_key_string into a vector of usize that represent how
    // many spaces the ct_alphabet is rotated relative to its starting position
    pub fn set_ind_key(&mut self) {
        self.ind_key.clear();
        let len = self.pt_key.len();
        for c in self.ind_key_string.chars() {
            let sh = len + self.indicator_position() - self.ct_key.get_pos_of(c).unwrap();
            self.ind_key.push(sh % len)
        }
    }
    
    pub fn indicator_position(&self) -> usize {
        self.pt_key.get_pos_of(self.indicator).unwrap()
    }
    
    pub fn indicator_cyclic_key(&self) -> std::iter::Cycle<std::slice::Iter<usize>> {
        self.ind_key.iter().cycle()
    }
    

}

impl Cipher for Quagmire {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::errors::CipherError> {
        todo!()
    }

    fn randomize(&mut self, rng: &mut rand::prelude::StdRng) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}