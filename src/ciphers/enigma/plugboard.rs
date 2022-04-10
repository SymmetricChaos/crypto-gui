use std::collections::HashMap;
use crate::errors::CipherError;


 
#[derive(Clone,Debug)]
pub struct Plugboard {
    old_alphabet: String,
    wiring: HashMap<char,char>,
}
 
impl Plugboard {

    pub fn set_plugboard(&mut self, pairs: &str) -> Result<(),CipherError> {
        if &self.old_alphabet == pairs {
            return Ok(())
        }
        self.wiring.clear();
        let digraphs = pairs.split(" ");
        for d in digraphs {
            if d.len() != 2 {
                return Err(CipherError::input("Engima Plugboard settings must be given as pairs of letters"));
            }
            let mut cs = d.chars();
            let a = cs.next().unwrap();
            let b = cs.next().unwrap();
            if a == b || self.wiring.contains_key(&a) {
                return Err(CipherError::input("Enigma Plugboard settings cannot include cycles"));
            }
            self.wiring.insert(a,b);
            self.wiring.insert(b,a);
        }
        self.old_alphabet = pairs.to_string();
        Ok(())
    }
 
    pub fn swap(&self, character: char) -> char {
        if self.wiring.contains_key(&character) {
            self.wiring[&character]
        } else {
            character
        }
    }
}

impl Default for Plugboard {
    fn default() -> Self {
        Self { old_alphabet: String::new(), wiring: HashMap::with_capacity(13) }
    }
}