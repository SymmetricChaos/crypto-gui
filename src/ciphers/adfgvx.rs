use rand::prelude::ThreadRng;
use crate::{ciphers::{Polybius,Columnar}, errors::CipherError};
use crate::text_types::{PresetAlphabet::*, PresetAlphabet};
use super::Cipher;
 
pub struct ADFGVX {
    pub polybius: Polybius,
    pub columnar: Columnar
}
 
impl ADFGVX {
    pub fn set_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoJ => {
                self.polybius.set_alphabet(mode);
                self.polybius.set_labels(String::from("ADFGX"));
            }
            BasicLatinWithDigits => {
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
        polybius.set_alphabet(BasicLatinNoJ);
        polybius.set_labels(String::from("ADFGX"));

        Self{
              polybius,
              columnar: Columnar::default()
        }
    }
}
 
impl Cipher for ADFGVX {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let poly_text = self.polybius.encrypt(text)?;
        let colm_text = self.columnar.encrypt(&poly_text)?;
        Ok(colm_text)
    }
 
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let colm_text = self.columnar.decrypt(text)?;
        let poly_text = self.polybius.decrypt(&colm_text)?;
        Ok(poly_text)
    }
 
    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.polybius.randomize(rng);
        self.columnar.randomize(rng);
    }
 
    fn get_input_alphabet(&self) -> &String {
        &self.polybius.get_input_alphabet()
    }
 
    fn get_output_alphabet(&self) -> &String {
        &self.polybius.get_labels()
    }
 
    fn get_mut_input_alphabet(&mut self) -> &mut String {
        unimplemented!("ADFGX and ADFGVX ciphers use historically accurate alphabets that should not be changed")
    }
 
    fn get_mut_output_alphabet(&mut self) -> &mut String {
        unimplemented!("ADFGX and ADFGVX ciphers use historically accurate alphabets that should not be changed")
    }
 
    fn validate_settings(&self) -> Result<(), CipherError> {
        todo!()
    }
}



#[cfg(test)]
mod adfgvx_tests {
    use super::*;

    const PLAINTEXT: &'static str =   "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    const CIPHERTEXT1: &'static str = "GDXXFAAXFGDAXGGAGDDGDGFGAFGXDFGFDAGAXDFXXXGAAFFFXDXDXFGGDAFXDGGAFDGGFA";
    const CIPHERTEXT2: &'static str = "FDGGFAAVDFXXFFDAFDDFAGFGAFDFDDFAXXGVVVXGVVAAAFFFGDVDVFFFAGDDAGGAFDGFDA";

    #[test]
    fn encrypt_test_adfgx() {
        let mut cipher = ADFGVX::default();
        cipher.polybius.set_key("KEYWORKFORUSEINTEST");
        cipher.columnar.set_key("SOMEWORD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT1);
    }

    #[test]
    fn decrypt_test_adfgx() {
        let mut cipher = ADFGVX::default();
        cipher.polybius.set_key("KEYWORKFORUSEINTEST");
        cipher.columnar.set_key("SOMEWORD");
        assert_eq!(cipher.decrypt(CIPHERTEXT1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_adfgvx() {
        let mut cipher = ADFGVX::default();
        cipher.set_alphabet(PresetAlphabet::BasicLatinWithDigits);
        cipher.polybius.set_key("57This9Should0Mix2Words");
        cipher.columnar.set_key("SOMEWORD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn decrypt_test_adfgvx() {
        let mut cipher = ADFGVX::default();
        cipher.set_alphabet(PresetAlphabet::BasicLatinWithDigits);
        cipher.polybius.set_key("57This9Should0Mix2Words");
        cipher.columnar.set_key("SOMEWORD");
        assert_eq!(cipher.decrypt(CIPHERTEXT2).unwrap(), PLAINTEXT);
    }
}
