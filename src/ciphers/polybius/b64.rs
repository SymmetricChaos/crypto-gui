use crate::{
    ciphers::{transposition::Columnar, Cipher},
    errors::Error,
    text_aux::PresetAlphabet::*,
};

use super::PolybiusSquare;

pub struct B64 {
    pub polybius: PolybiusSquare,
    pub columnar1: Columnar,
    pub columnar2: Columnar,
}

impl Default for B64 {
    fn default() -> Self {
        let mut polybius = PolybiusSquare::default();
        polybius.assign_alphabet(Base64);

        Self {
            polybius,
            columnar1: Columnar::default(),
            columnar2: Columnar::default(),
        }
    }
}

impl Cipher for B64 {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let t1 = self.polybius.encrypt(text)?;
        let t2 = self.columnar1.encrypt(&t1)?;
        let t3 = self.columnar2.encrypt(&t2)?;
        let t4 = self.polybius.decrypt(&t3)?;
        Ok(t4)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let t1 = self.polybius.encrypt(text)?;
        let t2 = self.columnar2.decrypt(&t1)?;
        let t3 = self.columnar1.decrypt(&t2)?;
        let t4 = self.polybius.decrypt(&t3)?;
        Ok(t4)
    }

    fn randomize(&mut self) {
        self.polybius.randomize();
        self.columnar1.randomize();
        self.columnar2.randomize();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod b64_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "hMzRT3BBKyRgKfOgJBQ6DoRwaRCI1UD4MQF";

    #[test]
    fn encrypt_test() {
        let mut cipher = B64::default();
        cipher.polybius.assign_key("ENCRYPTION");
        cipher.columnar1.assign_key("NOVELTY");
        cipher.columnar2.assign_key("SHUFFLE");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = B64::default();
        cipher.polybius.assign_key("ENCRYPTION");
        cipher.columnar1.assign_key("NOVELTY");
        cipher.columnar2.assign_key("SHUFFLE");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
