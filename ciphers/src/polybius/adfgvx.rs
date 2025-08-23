use super::PolybiusSquare;
use crate::traits::Cipher;
use crate::transposition::Columnar;
use utils::{errors::GeneralError, preset_alphabet::Alphabet};

pub enum AdfgvxMode {
    Short,
    Long,
}

pub struct Adfgvx {
    pub mode: AdfgvxMode,
    polybius: PolybiusSquare,
    columnar: Columnar,
}

impl Adfgvx {
    pub fn alphabet(&self) -> &'static str {
        match self.mode {
            AdfgvxMode::Short => Alphabet::BasicLatinNoJ.slice(),
            AdfgvxMode::Long => Alphabet::Alphanumeric.slice(),
        }
    }

    pub fn assign_mode(&mut self, mode: AdfgvxMode) {
        self.mode = mode;
    }

    pub fn assign_polybius_key(&mut self, key: &str) {
        self.polybius.assign_key(key, self.alphabet())
    }

    pub fn assign_columnar_key(
        &mut self,
        key: &str,
    ) -> Result<(), utils::text_functions::StringRankError> {
        self.columnar.assign_key(key, Alphabet::BasicLatin.into())
    }

    pub fn show_polybius_grid(&self) -> String {
        self.polybius.show_grid()
    }
}

impl Default for Adfgvx {
    fn default() -> Self {
        let polybius = PolybiusSquare::default();

        Self {
            mode: AdfgvxMode::Short,
            polybius,
            columnar: Columnar::default(),
        }
    }
}

impl Cipher for Adfgvx {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let poly_text = self.polybius.encrypt(text)?;
        let colm_text = self.columnar.encrypt(&poly_text)?;
        Ok(colm_text)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let colm_text = self.columnar.decrypt(text)?;
        let poly_text = self.polybius.decrypt(&colm_text)?;
        Ok(poly_text)
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
        cipher.assign_polybius_key("KEYWORKFORUSEINTEST");
        cipher.assign_columnar_key("SOMEWORD").unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT1);
    }

    #[test]
    fn decrypt_test_adfgx() {
        let mut cipher = Adfgvx::default();
        cipher.assign_polybius_key("KEYWORKFORUSEINTEST");
        cipher.assign_columnar_key("SOMEWORD").unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_adfgvx() {
        let mut cipher = Adfgvx::default();
        cipher.mode = AdfgvxMode::Long;
        cipher.assign_polybius_key("57This9Should0Mix2Words");
        cipher.assign_columnar_key("SOMEWORD").unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn decrypt_test_adfgvx() {
        let mut cipher = Adfgvx::default();
        cipher.mode = AdfgvxMode::Long;
        cipher.assign_polybius_key("57This9Should0Mix2Words");
        cipher.assign_columnar_key("SOMEWORD").unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT2).unwrap(), PLAINTEXT);
    }
}
