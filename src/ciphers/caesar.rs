use rand::{Rng, prelude::ThreadRng};
use crate::text_functions::LATIN_UPPER;
use crate::errors::{CipherError, CipherErrors};
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

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.chars().count()
    }
}

impl Default for Caesar {
    fn default() -> Self {
        Self { shift: 0, alphabet: String::from(LATIN_UPPER) }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => (v + self.shift) % self.alphabet_len(),
                None => return Err(CipherError::invalid_input_char(s))
            };
            let char = match self.val_to_char(n) {
                Some(c) => c,
                None => return Err(CipherError::invalid_input_char(s))
            };
            out.push(char)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => (v + self.alphabet_len() - self.shift) % self.alphabet_len(),
                None => return Err(CipherError::invalid_input_char(s))
            };
            let char = match self.val_to_char(n) {
                Some(c) => c,
                None => return Err(CipherError::invalid_input_char(s))
            };
            out.push(char)
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        let length = self.alphabet.len();
        self.shift = rng.gen_range(0..length);
    }

    fn get_input_alphabet(&mut self) -> &String {
        &mut self.alphabet
    }

    fn get_output_alphabet(&mut self) -> &String {
        &mut self.alphabet
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn validate_settings(&self) -> Result<(),CipherErrors> {
        if self.shift > self.alphabet_len() {
            return Err(CipherErrors::new(vec![CipherError::Key(String::from("key value is incorrect"))]))
        }
        Ok(())
    }
}





#[cfg(test)]
mod caesar_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "WKHTXLFNEURZQIRAMXPSVRYHUWKHODCBGRJ";

    #[test]
    fn encrypt_test() {
        let cipher = Caesar::new(3,LATIN_UPPER);
        assert_eq!(cipher.encrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = Caesar::new(3,LATIN_UPPER);
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}