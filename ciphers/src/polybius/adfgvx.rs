use super::PolybiusSquare;
use crate::traits::Cipher;
use crate::transposition::Columnar;
use utils::{errors::GeneralError, preset_alphabet::Alphabet};

pub enum AdfgvxMode {
    Short,
    Long,
}

pub struct Adfgvx {
    mode: AdfgvxMode,
    polybius: PolybiusSquare,
    columnar: Columnar,
}

impl Adfgvx {
    /// The longer version with the 36 letter Alphanumeric alphabet
    pub fn new_adfgvx() -> Result<Self, GeneralError> {
        Ok(Self {
            mode: AdfgvxMode::Long,
            polybius: PolybiusSquare::new(Alphabet::Alphanumeric.slice(), "ADFGVX", false)?,
            columnar: Columnar::default(),
        })
    }

    /// The shorter version with the 25 letter alphabet (J removed)
    pub fn new_adfgx() -> Result<Self, GeneralError> {
        Ok(Self {
            mode: AdfgvxMode::Short,
            polybius: PolybiusSquare::new(Alphabet::BasicLatinNoJ.slice(), "ADFGX", false)?,
            columnar: Columnar::default(),
        })
    }

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
        Self::new_adfgx().unwrap()
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

    const PTEXT: &'static str = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    const CTEXT1: &'static str =
        "DAGGADDFFXADFXDFXGDDDDAFFADFGAXGAGADXXXXFFXDXAAXXAAAFDFFDDDGGAFAAGXGXG";
    const CTEXT2: &'static str =
        "ADFVXDDXVXDFFVFGVFFVFVGADGFFVGXVDFADXVVAGVAGVGDVGFFVAVDGFAVGFFDGGDVFXF";

    #[test]
    fn encrypt_test_adfgx() {
        let mut cipher = Adfgvx::new_adfgx().unwrap();
        cipher.assign_polybius_key("KEYWORKFORUSEINTEST");
        cipher.assign_columnar_key("SOMEWORD").unwrap();
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT1);
    }

    #[test]
    fn decrypt_test_adfgx() {
        let mut cipher = Adfgvx::new_adfgx().unwrap();
        cipher.assign_polybius_key("KEYWORKFORUSEINTEST");
        cipher.assign_columnar_key("SOMEWORD").unwrap();
        assert_eq!(cipher.decrypt(CTEXT1).unwrap(), PTEXT);
    }

    #[test]
    fn encrypt_test_adfgvx() {
        let mut cipher = Adfgvx::new_adfgvx().unwrap();
        cipher.assign_polybius_key("57This9Should0Mix2Words");
        cipher.assign_columnar_key("SOMEWORD").unwrap();
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT2);
    }

    #[test]
    fn decrypt_test_adfgvx() {
        let mut cipher = Adfgvx::new_adfgvx().unwrap();
        cipher.assign_polybius_key("57This9Should0Mix2Words");
        cipher.assign_columnar_key("SOMEWORD").unwrap();
        assert_eq!(cipher.decrypt(CTEXT2).unwrap(), PTEXT);
    }
}
