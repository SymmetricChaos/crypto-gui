use crate::{errors::CipherError, traits::Cipher};
use utils::preset_alphabet::Alphabet;
use utils::vecstring::VecString;

pub struct Caesar {
    pub shift: i32,
    pub alphabet: VecString,
}

impl Caesar {
    fn encrypt_char(&self, c: char) -> Result<char, CipherError> {
        let p = self
            .alphabet
            .get_pos(c)
            .ok_or(CipherError::invalid_input_char(c))?;
        Ok(*self.alphabet.get_char_offset(p, self.shift).unwrap())
    }

    fn decrypt_char(&self, c: char) -> Result<char, CipherError> {
        let p = self
            .alphabet
            .get_pos(c)
            .ok_or(CipherError::invalid_input_char(c))?;
        Ok(*self.alphabet.get_char_offset(p, -self.shift).unwrap())
    }

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(alphabet);
    }
}

impl Default for Caesar {
    fn default() -> Self {
        Self {
            shift: 0,
            alphabet: VecString::from(Alphabet::BasicLatin),
        }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut vec = Vec::new();
        for c in text.chars() {
            vec.push(self.encrypt_char(c)?)
        }
        Ok(vec.into_iter().collect())
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut vec = Vec::new();
        for c in text.chars() {
            vec.push(self.decrypt_char(c)?)
        }
        Ok(vec.into_iter().collect())
    }
}

#[cfg(test)]
mod caesar_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "WKHTXLFNEURZQIRAMXPSVRYHUWKHODCBGRJ";

    #[test]
    fn encrypt_test() {
        let mut cipher = Caesar::default();
        cipher.shift = 3;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Caesar::default();
        cipher.shift = 3;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
