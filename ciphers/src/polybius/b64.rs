use super::PolybiusSquare;
use crate::transposition::Columnar;
use crate::{errors::CipherError, traits::Cipher};
use utils::preset_alphabet::Alphabet;

pub struct B64 {
    polybius: PolybiusSquare,
    columnar1: Columnar,
    columnar2: Columnar,
}

impl Default for B64 {
    fn default() -> Self {
        Self {
            polybius: PolybiusSquare::default(),
            columnar1: Columnar::default(),
            columnar2: Columnar::default(),
        }
    }
}

impl B64 {
    pub fn assign_polybius_key(&mut self, key: &str) {
        self.polybius.assign_key(key, Alphabet::Base64.slice());
    }

    pub fn assign_columnar_key_1(
        &mut self,
        key: &str,
    ) -> Result<(), utils::functions::StringRankError> {
        self.columnar1.assign_key(key, Alphabet::Base64.slice())
    }

    pub fn assign_columnar_key_2(
        &mut self,
        key: &str,
    ) -> Result<(), utils::functions::StringRankError> {
        self.columnar2.assign_key(key, Alphabet::Base64.slice())
    }

    pub fn polybius_grid(&self) -> String {
        self.polybius.show_grid()
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
}

#[cfg(test)]
mod b64_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "hMzRT3BBKyRgKfOgJBQ6DoRwaRCI1UD4MQF";

    #[test]
    fn encrypt_test() {
        let mut cipher = B64::default();
        cipher.assign_polybius_key("ENCRYPTION");
        cipher.assign_columnar_key_1("NOVELTY").unwrap();
        cipher.assign_columnar_key_2("SHUFFLE").unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = B64::default();
        cipher.assign_polybius_key("ENCRYPTION");
        cipher.assign_columnar_key_1("NOVELTY").unwrap();
        cipher.assign_columnar_key_2("SHUFFLE").unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
