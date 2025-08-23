use crate::traits::Cipher;
use utils::errors::GeneralError;
use utils::preset_alphabet::Alphabet;
use utils::vecstring::VecString;

pub struct Alberti {
    pub fixed_alphabet: VecString,
    pub moving_alphabet: VecString,
}

impl Default for Alberti {
    fn default() -> Self {
        Self {
            fixed_alphabet: VecString::from(Alphabet::BasicLatin),
            moving_alphabet: VecString::from(Alphabet::BasicLatin.string().to_ascii_lowercase()),
        }
    }
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
        let position = self.fixed_alphabet.get_pos(symbol).unwrap();
        *self
            .moving_alphabet
            .get_char_offset(position, index as i32)
            .unwrap()
    }

    // Unwrap justified by checks made in decrypt()
    fn decrypt_char(&self, symbol: char, index: usize) -> char {
        let position = self.moving_alphabet.get_pos(symbol).unwrap();
        *self
            .fixed_alphabet
            .get_char_offset(position, -(index as i32))
            .unwrap()
    }

    pub fn alphabet_len(&self) -> usize {
        self.fixed_alphabet.chars().count()
    }

    fn validate_settings(&self) -> Result<(), GeneralError> {
        if self.fixed_alphabet.len() != self.moving_alphabet.len() {
            return Err(GeneralError::alphabet("alphabets must be of equal length"));
        }
        Ok(())
    }
}

impl Cipher for Alberti {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        self.validate_settings()?;
        let mut index = 0;
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.fixed_alphabet.contains(s) {
                out.push(self.encrypt_char(s, index));
            } else if self.moving_alphabet.contains(s) {
                index = self
                    .moving_alphabet
                    .get_pos(s)
                    .ok_or(GeneralError::invalid_input_char(s))?;
                out.push(*self.fixed_alphabet.get_char(index).unwrap());
            } else {
                return Err(GeneralError::invalid_input_char(s));
            }
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        self.validate_settings()?;
        let mut index = 0;
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.moving_alphabet.contains(s) {
                out.push(self.decrypt_char(s, index));
            } else if self.fixed_alphabet.contains(s) {
                index = self
                    .fixed_alphabet
                    .get_pos(s)
                    .ok_or(GeneralError::invalid_input_char(s))?;
                out.push(*self.moving_alphabet.get_char(index).unwrap());
            } else {
                return Err(GeneralError::invalid_input_char(s));
            }
        }
        Ok(out)
    }
}

// impl Display for Alberti {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut out = self.fixed_alphabet.to_string();
//         out.push('\n');
//         out.push_str(&self.moving_alphabet.to_string()[self.start_index..]);
//         out.push_str(&self.moving_alphabet.to_string()[0..self.start_index]);
//         write!(f, "{}", out)
//     }
// }

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
    fn encrypt_test_2() {
        let mut cipher = Alberti::default();
        cipher.assign_fixed_alphabet("ABCDEFGILMNOPQRSTVXZ1234");
        cipher.assign_moving_alphabet("acegklnprtuz&xysomqihfdb");
        assert_eq!(
            cipher.encrypt("gLAGVER2RAySIFARA").unwrap(),
            "DzgthpmamgRlfiyky"
        );
    }

    #[test]
    fn decrypt_test() {
        let cipher = Alberti::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
