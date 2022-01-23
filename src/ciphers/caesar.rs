use rand::{Rng, prelude::ThreadRng};
use crate::text_functions::LATIN_UPPER;

use super::Cipher;

pub struct Caesar {
    pub shift: usize,
    pub alphabet: String,
}

impl Caesar {
    pub fn new(shift: usize, alphabet: &str) -> Caesar {
        Caesar{ shift, alphabet: alphabet.to_string() }
    }

    fn char_to_val(&self, c: char) -> Option<usize> {
        self.alphabet.chars().position(|x| x == c)
    }

    fn val_to_char(&self, v: usize) -> Option<char> {
        self.alphabet.chars().nth(v)
    }

    pub fn length(&self) -> usize {
        self.alphabet.chars().count()
    }
}

impl Default for Caesar {
    fn default() -> Self {
        Self { shift: 0, alphabet: String::from(LATIN_UPPER) }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => (v + self.shift) % self.length(),
                None => return Err("Unknown character encountered")
            };
            let char = match self.val_to_char(n) {
                Some(c) => c,
                None => return Err("Unknown character encountered")
            };
            out.push(char)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,&'static str> {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => (v + self.length() - self.shift) % self.length(),
                None => return Err("Unknown character encountered")
            };
            let char = match self.val_to_char(n) {
                Some(c) => c,
                None => return Err("Unknown character encountered")
            };
            out.push(char)
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        let length = self.alphabet.len();
        self.shift = rng.gen_range(0..length);
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
}