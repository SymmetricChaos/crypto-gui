use crate::errors::Error;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct EnigmaPlugboard {
    old_alphabet: String,
    wiring: HashMap<char, char>,
}

impl EnigmaPlugboard {
    pub fn set_plugboard(&mut self, pairs: &str) -> Result<(), Error> {
        // Don't rebuild unless we need to
        if &self.old_alphabet == pairs {
            return Ok(());
        }

        // Check that no more than 13 pairs are included
        let digraphs = pairs.split(" ");
        if digraphs.clone().count() > 13 {
            return Err(Error::key(
                "Engima Plugboard cannot include more than 13 pairs of letters",
            ));
        }

        // Clear the wiring and rebuild it, returning an Error if anything goes wrong
        self.wiring.clear();
        for d in digraphs {
            if d.len() != 2 {
                return Err(Error::key(
                    "Engima Plugboard settings must be given as pairs of letters",
                ));
            }
            let mut cs = d.chars();
            let a = cs.next().unwrap();
            let b = cs.next().unwrap();
            if a == b || self.wiring.contains_key(&a) || self.wiring.contains_key(&b) {
                return Err(Error::key(
                    "Enigma Plugboard settings cannot include cycles or chains",
                ));
            }
            self.wiring.insert(a, b);
            self.wiring.insert(b, a);
        }
        self.old_alphabet = pairs.to_string();
        Ok(())
    }

    // Swap the character or return the original depending on if it is in the board
    pub fn swap(&self, character: char) -> char {
        *self.wiring.get(&character).unwrap_or_else(|| &character)
    }
}

impl Default for EnigmaPlugboard {
    fn default() -> Self {
        Self {
            old_alphabet: String::new(),
            wiring: HashMap::with_capacity(13),
        }
    }
}
