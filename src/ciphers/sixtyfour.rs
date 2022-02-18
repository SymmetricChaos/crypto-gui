use rand::prelude::ThreadRng;
use crate::{ciphers::{Polybius,Columnar}, text_functions::PresetAlphabet, errors::CipherError};
use super::Cipher;
 
pub struct SixtyFour {
    polybius: Polybius,
    columnar: Columnar
}
 
impl Default for SixtyFour {
    fn default() -> Self {
        let mut polybius = Polybius::default();
        polybius.set_alphabet(PresetAlphabet::Base64);
 
        Self{
              polybius,
              columnar: Columnar::default()
        }
    }
}
 
impl Cipher for SixtyFour {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let t1 = self.polybius.encrypt(text)?;
        let t2 = self.columnar.encrypt(&t1)?;
        let t3 = self.columnar.encrypt(&t2)?;
        let t4 = self.polybius.decrypt(&t3)?;
        Ok(t4)
    }
 
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let t1 = self.polybius.encrypt(text)?;
        let t2 = self.columnar.decrypt(&t1)?;
        let t3 = self.columnar.decrypt(&t2)?;
        let t4 = self.polybius.decrypt(&t3)?;
        Ok(t4)
    }
 
    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.polybius.randomize();
        self.columnar.randomize();
    }
 
    fn get_input_alphabet(&self) -> &String {
        &self.polybius.get_input_alphabet()
    }
 
    fn get_output_alphabet(&self) -> &String {
        &self.polybius.get_labels()
    }
 
    fn get_mut_input_alphabet(&mut self) -> &mut String {
        unimplemented!("The Sixty Four cipher uses a fixed alphabet")
    }
 
    fn get_mut_output_alphabet(&mut self) -> &mut String {
        unimplemented!("The Sixty Four cipher uses a fixed alphabet")
    }
 
    fn validate_settings(&self) -> Result<(), CipherError> {
        todo!()
    }
}