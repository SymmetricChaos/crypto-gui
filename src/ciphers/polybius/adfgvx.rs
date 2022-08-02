use crate::{
    ciphers::{transposition::Columnar, Cipher},
    errors::Error,
    text_aux::PresetAlphabet,
};

use super::PolybiusSquare;

pub struct Adfgvx {
    pub polybius: PolybiusSquare,
    pub columnar: Columnar,
}

impl Adfgvx {
    pub fn set_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            PresetAlphabet::BasicLatinNoJ => {
                self.polybius.assign_alphabet(mode);
                self.polybius.assign_labels("ADFGX");
            }
            PresetAlphabet::BasicLatinWithDigits => {
                self.polybius.assign_alphabet(mode);
                self.polybius.assign_labels("ADFGVX");
            }
            _ => (),
        }
    }
}

impl Default for Adfgvx {
    fn default() -> Self {
        let mut polybius = PolybiusSquare::default();
        polybius.assign_alphabet(PresetAlphabet::BasicLatinNoJ);
        polybius.assign_labels("ADFGX");

        Self {
            polybius,
            columnar: Columnar::default(),
        }
    }
}

impl Cipher for Adfgvx {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let poly_text = self.polybius.encrypt(text)?;
        let colm_text = self.columnar.encrypt(&poly_text)?;
        Ok(colm_text)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let colm_text = self.columnar.decrypt(text)?;
        let poly_text = self.polybius.decrypt(&colm_text)?;
        Ok(poly_text)
    }

    fn randomize(&mut self) {
        self.polybius.randomize();
        self.columnar.randomize();
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
        let mut cipher = Adfgvx::default();
        cipher.polybius.assign_key("KEYWORKFORUSEINTEST");
        cipher.columnar.assign_key("SOMEWORD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT1);
    }

    #[test]
    fn decrypt_test_adfgx() {
        let mut cipher = Adfgvx::default();
        cipher.polybius.assign_key("KEYWORKFORUSEINTEST");
        cipher.columnar.assign_key("SOMEWORD");
        assert_eq!(cipher.decrypt(CIPHERTEXT1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_adfgvx() {
        let mut cipher = Adfgvx::default();
        cipher.set_alphabet(PresetAlphabet::BasicLatinWithDigits);
        cipher.polybius.assign_key("57This9Should0Mix2Words");
        cipher.columnar.assign_key("SOMEWORD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn decrypt_test_adfgvx() {
        let mut cipher = Adfgvx::default();
        cipher.set_alphabet(PresetAlphabet::BasicLatinWithDigits);
        cipher.polybius.assign_key("57This9Should0Mix2Words");
        cipher.columnar.assign_key("SOMEWORD");
        assert_eq!(cipher.decrypt(CIPHERTEXT2).unwrap(), PLAINTEXT);
    }
}
