use rand::prelude::ThreadRng;

use crate::{ciphers::{Polybius,Columnar}, text_functions::PresetAlphabet, errors::CipherError};
use super::Cipher;
 
pub struct ADFGVX {
    polybius: Polybius,
    columnar: Columnar
}
 
impl ADFGVX {
    pub fn set_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            PresetAlphabet::EnglishNoJ => {
                self.polybius.set_alphabet(mode);
                self.polybius.set_labels(String::from("ADFGX"));
            }
            PresetAlphabet::EnglishWithDigits => {
                self.polybius.set_alphabet(mode);
                self.polybius.set_labels(String::from("ADFGVX"));
            }
            _ => ()
        }
    }
}
 
impl Default for ADFGVX {
    fn default() -> Self {
        let mut polybius = Polybius::default();
        polybius.set_alphabet(PresetAlphabet::EnglishNoJ);
        polybius.set_labels(String::from("ADFGVX"));

        Self{
              polybius,
              columnar: Columnar::default()
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
 
    fn get_input_alphabet(&self) -> &String {
        &self.polybius.get_input_alphabet()
    }
 
    fn get_output_alphabet(&self) -> &String {
        &self.polybius.get_labels()
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