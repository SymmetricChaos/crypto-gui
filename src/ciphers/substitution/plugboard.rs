use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{PresetAlphabet, VecString},
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Plugboard {
    alphabet: VecString,
    pub pairs: String,
    wiring: HashMap<char, char>,
}

impl Default for Plugboard {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            pairs: String::new(),
            wiring: HashMap::with_capacity(15),
        }
    }
}

impl Plugboard {
    pub fn set_plugboard(&mut self) -> Result<(), Error> {
        let digraphs = self.pairs.split(" ");

        // Clear the wiring and rebuild it, returning an Error if anything goes wrong
        let mut wiring = HashMap::with_capacity(self.wiring.capacity());
        for d in digraphs {
            if d.len() != 2 {
                return Err(Error::key(
                    "Plugboard settings must be given as pairs of letters",
                ));
            }
            let mut cs = d.chars();
            let a = cs.next().unwrap();
            let b = cs.next().unwrap();
            if a == b || wiring.contains_key(&a) || wiring.contains_key(&b) {
                return Err(Error::key(
                    "Plugboard settings cannot include cycles or chains",
                ));
            }
            wiring.insert(a, b);
            wiring.insert(b, a);
        }
        self.wiring = wiring;
        Ok(())
    }

    // Infallible setter that just skips any incorrect inputs
    pub fn set_plugboard_silent(&mut self) {
        let digraphs = self.pairs.split(" ");

        let mut wiring = HashMap::with_capacity(self.wiring.capacity());
        for d in digraphs {
            if d.len() != 2 {
                continue;
            }
            let mut cs = d.chars();
            let a = cs.next().unwrap();
            let b = cs.next().unwrap();
            if a == b || wiring.contains_key(&a) || wiring.contains_key(&b) {
                continue;
            }
            wiring.insert(a, b);
            wiring.insert(b, a);
        }
        self.wiring = wiring;
    }

    // Swap the character or return the original depending on if it is in the board
    pub fn swap(&self, character: char) -> char {
        *self.wiring.get(&character).unwrap_or_else(|| &character)
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
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let out = text.chars().map(|c| self.swap(c)).collect();
        Ok(out)
    }

    // Plugboards are naturally reciprocal
    fn decrypt(&self, text: &str) -> Result<String, Error> {
        self.encrypt(text)
    }

    fn randomize(&mut self) {
        let half = self.alphabet.len() / 2 + 1;

        let alpha = self.alphabet.shuffled(&mut get_global_rng());
        let mut chars = alpha.chars();
        let mut pairs = String::with_capacity(half * 3);
        for _ in 0..half {
            pairs.push(chars.next().unwrap());
            pairs.push(chars.next().unwrap());
            pairs.push(' ');
        }

        self.pairs = pairs;
        self.set_plugboard().unwrap();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}
