use std::collections::HashMap;

use crate::errors::CodeError;

use super::Code;

// https://en.wikipedia.org/wiki/Levenshtein_coding



pub struct LevenshteinCode {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    alphabet: String,
    old_alphabet: String,
    max_code_len: usize,
}

impl LevenshteinCode {

    pub fn control_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    // This needs to be called before encoding or decoding to be
    // sure that the maps are up to date. In the egui interface
    // this is taken care of by embedding it in the chars_codes()
    // method.
    // It would make more sense to put it in the control_alphabet()
    // method but that causes a panic due to interaction with
    // the chars_codes() method.
    pub fn set_maps(&mut self) {
        if self.alphabet != self.old_alphabet {
            let codes = FibStr::new();
            self.map.clear();
            self.map_inv.clear();
            for (l,c) in self.alphabet.chars().zip(codes) {
                self.map.insert(l,c.clone() );
                self.map_inv.insert(c.clone(), l);
            }
            self.max_code_len = self.map[&self.alphabet.chars().last().unwrap()].chars().count();
            self.old_alphabet = self.alphabet.clone();
        }
    }

    pub fn new(alphabet: &str) -> Self {
        let codes = FibStr::new();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,c) in alphabet.chars().zip(codes) {
            map.insert(l,c.clone() );
            map_inv.insert(c, l);
        }
        LevenshteinCode{ map, map_inv, alphabet: alphabet.to_string(), old_alphabet: alphabet.to_string(), max_code_len: 8 }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item=(char, &String)> + '_ {
        self.set_maps();
        self.alphabet.chars()
            .map(|x| (x, self.map.get(&x).unwrap()) )
    }
}

impl Default for LevenshteinCode {
    fn default() -> Self {
        Self::new("ETAOINSHRDLCUMWFGYPBVKJXQZ")
    }
}

impl Code for LevenshteinCode {

    fn encode(&self, text: &str) -> Result<String,CodeError> {
        let mut output = String::new();
        for s in text.chars() {
            output.push_str(&self.map[&s])
        }
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String,CodeError> {
        let mut output = String::new();
        let mut buffer = String::with_capacity(self.max_code_len);
        let mut ctr = 0;
        for b in text.chars() {
            buffer.push(b);
            ctr += 1;
            
            println!("{}",&buffer);
            if let Some(s) = self.map_inv.get(&buffer) {
                output.push(*s);
                buffer.clear();
                ctr = 0;
            }
            // If we have an impossible code ignore it and start again, it will eventually
            // resychronize
            if ctr == self.max_code_len {
                buffer.clear();
                ctr = 0;
            }
        }
        Ok(output)
    }
}
 