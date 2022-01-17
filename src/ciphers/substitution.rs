use rand::prelude::ThreadRng;
use super::cipher_trait::Cipher;
use std::collections::HashMap;
use crate::math::shuffle_str;

pub struct Substitution {
    alphabet1: String,
    alphabet2: String,
    map: HashMap<char,char>,
    map_inv: HashMap<char,char>,
}

impl Substitution {
    // The alphabets must be the same length but we need to handle that in the panel itself
    pub fn new(alphabet1: &str, alphabet2: &str) -> Self {
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in alphabet1.chars().zip(alphabet2.chars()) {
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        Substitution{ alphabet1: alphabet1.to_string(), alphabet2: alphabet2.to_string(), map, map_inv }
    }

}

impl Cipher for Substitution {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        let mut out = "".to_string();
        for c in text.chars() {
            match self.map.get(&c) {
                Some(o) => out.push(*o),
                None => return Err("Unknown character encountered"),
            }
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,&'static str> {
        let mut out = "".to_string();
        for c in text.chars() {
            match self.map_inv.get(&c) {
                Some(o) => out.push(*o),
                None => return Err("Unknown character encountered"),
            }
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.alphabet2 = shuffle_str(&self.alphabet1, rng);
        self.map.clear();
        self.map_inv.clear();
        for (a, b) in self.alphabet1.chars().zip(self.alphabet2.chars()) {
            self.map.insert(a, b);
            self.map_inv.insert(b, a);
        }
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet1
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet2
    }
}