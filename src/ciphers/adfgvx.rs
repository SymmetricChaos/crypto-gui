use super::Cipher;
use crate::{
    ciphers::{Columnar, PolybiusSquare},
    errors::CipherError,
    text_aux::{PresetAlphabet, PresetAlphabet::*},
};
use rand::prelude::StdRng;

pub struct ADFGVX {
    pub polybius: PolybiusSquare,
    pub columnar: Columnar,
}

impl ADFGVX {
    pub fn set_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoJ => {
                self.polybius.assign_alphabet(mode);
                self.polybius.assign_labels("ADFGX");
            }
            BasicLatinWithDigits => {
                self.polybius.assign_alphabet(mode);
                self.polybius.assign_labels("ADFGVX");
            }
            _ => (),
        }
    }
}

impl Default for ADFGVX {
    fn default() -> Self {
        let mut polybius = PolybiusSquare::default();
        polybius.assign_alphabet(BasicLatinNoJ);
        polybius.assign_labels("ADFGX");

        Self {
            polybius,
            columnar: Columnar::default(),
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

    fn randomize(&mut self, rng: &mut StdRng) {
        self.polybius.randomize(rng);
        self.columnar.randomize(rng);
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod adfgvx_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    const CIPHERTEXT1: &'static str =
        "GDXXFAAXFGDAXGGAGDDGDGFGAFGXDFGFDAGAXDFXXXGAAFFFXDXDXFGGDAFXDGGAFDGGFA";
    const CIPHERTEXT2: &'static str =
        "FDGGFAAVDFXXFFDAFDDFAGFGAFDFDDFAXXGVVVXGVVAAAFFFGDVDVFFFAGDDAGGAFDGFDA";

    #[test]
    fn encrypt_test_adfgx() {
        let mut cipher = ADFGVX::default();
        cipher.polybius.assign_key("KEYWORKFORUSEINTEST");
        cipher.columnar.assign_key("SOMEWORD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT1);
    }

    #[test]
    fn decrypt_test_adfgx() {
        let mut cipher = ADFGVX::default();
        cipher.polybius.assign_key("KEYWORKFORUSEINTEST");
        cipher.columnar.assign_key("SOMEWORD");
        assert_eq!(cipher.decrypt(CIPHERTEXT1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_adfgvx() {
        let mut cipher = ADFGVX::default();
        cipher.set_alphabet(PresetAlphabet::BasicLatinWithDigits);
        cipher.polybius.assign_key("57This9Should0Mix2Words");
        cipher.columnar.assign_key("SOMEWORD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn decrypt_test_adfgvx() {
        let mut cipher = ADFGVX::default();
        cipher.set_alphabet(PresetAlphabet::BasicLatinWithDigits);
        cipher.polybius.assign_key("57This9Should0Mix2Words");
        cipher.columnar.assign_key("SOMEWORD");
        assert_eq!(cipher.decrypt(CIPHERTEXT2).unwrap(), PLAINTEXT);
    }
}
