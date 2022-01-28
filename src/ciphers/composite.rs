use std::collections::VecDeque;

use rand::prelude::ThreadRng;
use super::Cipher;
use crate::text_functions::{LATIN_UPPER, string_pairs};
use crate::errors::CipherError;


pub struct Composite {
    ciphers: Vec<dyn Cipher>,
}

impl Composite {

    fn validate_key(&self) -> Result<(),CipherError> {
        todo!()
    }

    fn validate_input(&self, text: &str) -> Result<(),CipherError> {
        todo!()
    }


}

impl Default for Composite {
    fn default() -> Self {
        todo!()
    }
}

impl Cipher for Composite {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        todo!()
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        todo!()
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
}