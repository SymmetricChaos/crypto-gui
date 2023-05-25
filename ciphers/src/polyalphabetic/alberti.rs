use crate::{errors::CipherError, traits::Cipher};
use std::fmt::Display;
use utils::preset_alphabet::PresetAlphabet;
use utils::vecstring::VecString;

pub struct Alberti {
    pub fixed_alphabet: VecString,
    pub moving_alphabet: VecString,
    pub start_index: usize,
}

impl Alberti {
    pub fn assign_fixed_alphabet(&mut self, alphabet: &str) {
        self.fixed_alphabet = VecString::unique_from(alphabet);
    }

    pub fn assign_moving_alphabet(&mut self, alphabet: &str) {
        self.moving_alphabet = VecString::unique_from(alphabet);
    }

    // Unwrap justified by checks made in encrypt()
    fn encrypt_char(&self, symbol: char, index: usize) -> char {
        let position = self.fixed_alphabet.get_pos_of(symbol).unwrap();
        self.moving_alphabet
            .get_char_offset(position, index as i32)
            .unwrap()
    }

    // Unwrap justified by checks made in decrypt()
    fn decrypt_char(&self, symbol: char, index: usize) -> char {
        let position = self.moving_alphabet.get_pos_of(symbol).unwrap();
        self.fixed_alphabet
            .get_char_offset(position, -(index as i32))
            .unwrap()
    }

    pub fn alphabet_len(&self) -> usize {
        self.fixed_alphabet.chars().count()
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if self.fixed_alphabet.len() != self.moving_alphabet.len() {
            return Err(CipherError::alphabet("alphabets must be of equal length"));
        }
        Ok(())
    }
}

impl Cipher for Alberti {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let mut index = self.start_index.clone();
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.fixed_alphabet.contains(s) {
                out.push(self.encrypt_char(s, index));
            } else if self.moving_alphabet.contains(s) {
                index = self
                    .moving_alphabet
                    .get_pos_of(s)
                    .ok_or(CipherError::invalid_input_char(s))?;
                out.push(self.fixed_alphabet.get_char_at(index).unwrap());
            } else {
                return Err(CipherError::invalid_input_char(s));
            }
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        let mut index = self.start_index.clone();
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.moving_alphabet.contains(s) {
                out.push(self.decrypt_char(s, index));
            } else if self.fixed_alphabet.contains(s) {
                index = self
                    .fixed_alphabet
                    .get_pos_of(s)
                    .ok_or(CipherError::invalid_input_char(s))?;
                out.push(self.moving_alphabet.get_char_at(index).unwrap());
            } else {
                return Err(CipherError::invalid_input_char(s));
            }
        }
        Ok(out)
    }
}

impl Default for Alberti {
    fn default() -> Self {
        Self {
            fixed_alphabet: VecString::from(PresetAlphabet::BasicLatin),

            moving_alphabet: VecString::from(
                PresetAlphabet::BasicLatin.string().to_ascii_lowercase(),
            ),
            start_index: 0,
        }
    }
}

impl Display for Alberti {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.fixed_alphabet.to_string();
        out.push('\n');
        out.push_str(&self.moving_alphabet.to_string()[self.start_index..]);
        out.push_str(&self.moving_alphabet.to_string()[0..self.start_index]);
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod alberti_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUItCKBReOWNFOsXJUMPStOVERTiHELAZYDnOG";
    const CIPHERTEXT: &'static str = "thequiTvdukEsarjsSpbmehkThoxkmIpmtihglNbt";

    #[test]
    fn encrypt_test() {
        let cipher = Alberti::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = Alberti::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
