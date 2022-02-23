use rand::prelude::ThreadRng;
use crate::{ciphers::{Polybius,Columnar}, errors::CipherError};
use crate::text_types::{PresetAlphabet::*};
use super::Cipher;
 
pub struct B64 {
    pub polybius: Polybius,
    pub columnar1: Columnar,
    pub columnar2: Columnar,
}
 
impl Default for B64 {
    fn default() -> Self {
        let mut polybius = Polybius::default();
        polybius.set_alphabet(Base64);
 
        Self{
              polybius,
              columnar1: Columnar::default(),
              columnar2: Columnar::default(),
        }
    }
}
 
impl Cipher for B64 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let t1 = self.polybius.encrypt(text)?;
        let t2 = self.columnar1.encrypt(&t1)?;
        let t3 = self.columnar2.encrypt(&t2)?;
        let t4 = self.polybius.decrypt(&t3)?;
        Ok(t4)
    }
 
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let t1 = self.polybius.encrypt(text)?;
        let t2 = self.columnar2.decrypt(&t1)?;
        let t3 = self.columnar1.decrypt(&t2)?;
        let t4 = self.polybius.decrypt(&t3)?;
        Ok(t4)
    }
 
    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.polybius.randomize(rng);
        self.columnar1.randomize(rng);
        self.columnar2.randomize(rng);
    }
 
    fn get_input_alphabet(&self) -> &String {
        &self.polybius.get_input_alphabet()
    }
 
    fn get_output_alphabet(&self) -> &String {
        &self.polybius.get_labels()
    }
 
    fn get_mut_input_alphabet(&mut self) -> &mut String {
        unimplemented!("The B64 cipher uses a fixed alphabet")
    }
 
    fn get_mut_output_alphabet(&mut self) -> &mut String {
        unimplemented!("The B64 cipher uses a fixed alphabet")
    }
 
    fn validate_settings(&self) -> Result<(), CipherError> {
        unreachable!("B64 actually has no settings. why did you call this method?")
    }
}

#[cfg(test)]
mod b64_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "hMzRT3BBKyRgKfOgJBQ6DoRwaRCI1UD4MQF";

    #[test]
    fn encrypt_test() {
        let mut cipher = B64::default();
        cipher.polybius.set_key("ENCRYPTION");
        cipher.columnar1.set_key("NOVELTY");
        cipher.columnar2.set_key("SHUFFLE");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = B64::default();
        cipher.polybius.set_key("ENCRYPTION");
        cipher.columnar1.set_key("NOVELTY");
        cipher.columnar2.set_key("SHUFFLE");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }

}
