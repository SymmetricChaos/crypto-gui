use crate::{errors::CipherError, traits::Cipher};
use utils::preset_alphabet::PresetAlphabet;
use utils::vecstring::VecString;

pub struct Caesar {
    pub shift: i32,
    pub alphabet: VecString,
    pub alphabet_string: String,
}

impl Caesar {
    fn encrypt_char(&self, c: char) -> Result<char, CipherError> {
        self.alphabet
            .get_shifted_char(c, self.shift)
            .ok_or(CipherError::invalid_input_char(c))
    }

    fn decrypt_char(&self, c: char) -> Result<char, CipherError> {
        self.alphabet
            .get_shifted_char(c, -self.shift)
            .ok_or(CipherError::invalid_input_char(c))
    }

    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }

    pub fn control_alphabet(&mut self) -> &mut String {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
        &mut self.alphabet_string
    }
}

impl Default for Caesar {
    fn default() -> Self {
        Self {
            shift: 0,
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
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

    // fn randomize(&mut self) {
    //     self.shift = get_global_rng().gen_range(0..self.alphabet.len()) as i32;
    // }
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
