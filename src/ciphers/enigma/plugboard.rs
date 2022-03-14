use std::collections::HashMap;
use crate::errors::CipherError;


fn _parse_plugboard(pairs: &str) -> Result<HashMap<char,char>,CipherError> {
    let mut wiring = HashMap::new();
    let digraphs = pairs.split(" ");
    for d in digraphs {
        if d.len() != 2 {
            panic!("plugboard settings must be pairs of letters")
        }
        let mut cs = d.chars();
        let a = cs.next().unwrap();
        let b = cs.next().unwrap();
        wiring.insert(a,b);
        wiring.insert(b,a);
    }
    Ok(wiring)
}
 
#[derive(Clone,Debug)]
pub struct Plugboard {
    wiring: HashMap<char,char>,
}
 
impl Plugboard {
 
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
        Self { wiring: HashMap::new() }
    }
}