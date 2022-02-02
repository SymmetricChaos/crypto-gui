use rand::prelude::ThreadRng;
use crate::text_functions::{shuffled_str, LATIN_UPPER};
use super::Cipher;
use std::collections::HashMap;
use crate::errors::CipherError;

pub struct GeneralSubstitution {
    alphabet1: String,
    alphabet2: String,
    map: HashMap<char,char>,
    map_inv: HashMap<char,char>,
}

impl GeneralSubstitution {
    // The alphabets must be the same length but we need to handle that in the panel itself
    pub fn new(alphabet1: &str, alphabet2: &str) -> Self {
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in alphabet1.chars().zip(alphabet2.chars()) {
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        GeneralSubstitution{ alphabet1: alphabet1.to_string(), alphabet2: alphabet2.to_string(), map, map_inv }
    }
}

impl Default for GeneralSubstitution {
    fn default() -> Self {
        let alphabet1 = String::from(LATIN_UPPER);
        let alphabet2 = String::from(LATIN_UPPER);
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in alphabet1.chars().zip(alphabet2.chars()) {
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        Self { alphabet1, alphabet2, map, map_inv }
    }
}

impl Cipher for GeneralSubstitution {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut out = "".to_string();
        for c in text.chars() {
            match self.map.get(&c) {
                Some(o) => out.push(*o),
                None => return Err(CipherError::invalid_input_char(c))
            }
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut out = "".to_string();
        for c in text.chars() {
            match self.map_inv.get(&c) {
                Some(o) => out.push(*o),
                None => return Err(CipherError::invalid_input_char(c))
            }
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.alphabet2 = shuffled_str(&self.alphabet1, rng);
        self.map.clear();
        self.map_inv.clear();
        for (a, b) in self.alphabet1.chars().zip(self.alphabet2.chars()) {
            self.map.insert(a, b);
            self.map_inv.insert(b, a);
        }
    }

    fn get_input_alphabet(&mut self) -> &String {
        &mut self.alphabet1
    }

    fn get_output_alphabet(&mut self) -> &String {
        &mut self.alphabet2
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet1
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet2
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }
}