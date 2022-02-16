use rand::prelude::ThreadRng;

use crate::{ciphers::{Polybius,Columnar}, text_functions::PresetAlphabet, errors::CipherError};
use super::Cipher;
 
pub struct ADFGVX {
    alphabet: String,
    labels: String,
    polybius_key: String,
    columnar_key: String
}
 
impl ADFGVX {
    pub fn set_alphabet(&mut self, alpha: PresetAlphabet) {
        match alpha {
            PresetAlphabet::EnglishNoJ => {
                self.alphabet = String::from(alpha);
                self.labels = String::from("ADFGX");
            }
            PresetAlphabet::EnglishWithDigits => {
                self.alphabet = String::from(alpha);
                self.labels = String::from("ADFGVX");
            }
            _ => ()
        }
    }
}
 
impl Default for ADFGVX {
    fn default() -> Self {
        Self{ alphabet: String::from(PresetAlphabet::EnglishNoJ), 
              labels: String::from("ADFGVX"),
              polybius_key: String::new(),
              columnar_key: String::new()
        }
    }
}
 
impl Cipher for ADFGVX {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!("Set up Polybius Square and Columnar Transposition then execute in order")
    }
 
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!("Set up Columnar Transposition and Polybius Square then execute in order")
    }
 
    fn randomize(&mut self, rng: &mut ThreadRng) {
        todo!()
    }
 
    fn get_input_alphabet(&mut self) -> &String {
        &self.alphabet
    }
 
    fn get_output_alphabet(&mut self) -> &String {
        &self.labels
    }
 
    fn get_mut_input_alphabet(&mut self) -> &mut String {
        unimplemented!("ADFGX and ADFGVX ciphers use historically accurate alphabets")
    }
 
    fn get_mut_output_alphabet(&mut self) -> &mut String {
        unimplemented!("ADFGX and ADFGVX ciphers use historically accurate alphabets")
    }
 
    fn validate_settings(&self) -> Result<(), crate::errors::CipherErrors> {
        todo!()
    }
}