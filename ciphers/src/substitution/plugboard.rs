use crate::{errors::CipherError, traits::Cipher};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Plugboard {
    wiring: HashMap<char, char>,
}

impl Default for Plugboard {
    fn default() -> Self {
        Self {
            wiring: HashMap::with_capacity(26),
        }
    }
}

impl Plugboard {
    pub fn set_plugboard(&mut self, pairs: &str) -> Result<(), CipherError> {
        let digraphs = pairs.split(" ");

        // Clear the wiring and rebuild it, returning an Error if anything goes wrong
        let mut wiring = HashMap::with_capacity(self.wiring.capacity());
        for d in digraphs {
            if d.is_empty() {
                continue;
            }
            if d.len() != 2 {
                return Err(CipherError::Key(
                    format!("plugboard settings must be given as pairs of letters seperated by spaces, found `{}` instead", d),
                ));
            }
            let mut cs = d.chars();
            let a = cs.next().unwrap();
            let b = cs.next().unwrap();
            if a == b || wiring.contains_key(&a) || wiring.contains_key(&b) {
                return Err(CipherError::key(
                    "plugboards cannot include cycles or chains",
                ));
            }
            wiring.insert(a, b);
            wiring.insert(b, a);
        }
        self.wiring = wiring;
        Ok(())
    }

    // Swap the character or return the original depending on if it is in the board
    pub fn swap(&self, c: char) -> char {
        *self.wiring.get(&c).unwrap_or_else(|| &c)
    }

    // Vector of pairs to show state
    pub fn show_settings(&self) -> Vec<String> {
        let mut out = Vec::with_capacity(self.wiring.len());
        for pair in self.wiring.iter() {
            out.push(format!("{} ⇒ {}", pair.0, pair.1))
        }
        out.sort();
        out
    }
}

impl Cipher for Plugboard {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let out = text.chars().map(|c| self.swap(c)).collect(); // This is infalliable
        Ok(out)
    }

    // Plugboards are naturally reciprocal
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.encrypt(text)
    }
}
