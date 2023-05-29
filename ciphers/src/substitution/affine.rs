use crate::{errors::CipherError, traits::Cipher};
use utils::{math_functions::mul_inv, preset_alphabet::Alphabet, vecstring::VecString};

pub struct Affine {
    pub add_key: usize,
    pub mul_key: usize,
    pub alphabet: VecString,
}

impl Affine {
    fn encrypt_char(&self, c: char) -> char {
        let mut pos = self.alphabet.get_pos(c).unwrap();
        pos *= self.mul_key;
        pos += self.add_key;
        pos %= self.alphabet_len();
        *self.alphabet.get_char(pos).unwrap()
    }

    fn decrypt_char(&self, c: char, mul_key_inv: usize) -> char {
        let mut pos = self.alphabet.get_pos(c).unwrap();
        pos += self.alphabet_len() - self.add_key;
        pos *= mul_key_inv;
        pos %= self.alphabet_len();
        *self.alphabet.get_char(pos).unwrap()
    }

    pub fn set_alphabet(&mut self) {}

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(alphabet);
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }

    pub fn find_mul_inverse(&self) -> Result<usize, CipherError> {
        match mul_inv(self.mul_key, self.alphabet.chars().count()) {
            Some(n) => Ok(n),
            None => Err(CipherError::key("The multiplicative key of an Affine Cipher cannot share any factors with the length of the alphabet"))
        }
    }

    pub fn check_input(&self, text: &str) -> Result<(), CipherError> {
        for c in text.chars() {
            if !self.alphabet.contains(c) {
                return Err(CipherError::invalid_input_char(c));
            }
        }
        Ok(())
    }
}

impl Default for Affine {
    fn default() -> Self {
        Self {
            add_key: 0,
            mul_key: 1,
            alphabet: VecString::from(Alphabet::BasicLatin),
        }
    }
}

impl Cipher for Affine {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_input(text)?;
        // The inverse is not used but it must exist
        self.find_mul_inverse()?;
        let out = text.chars().map(|s| self.encrypt_char(s)).collect();
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_input(text)?;
        let mul_inv = self.find_mul_inverse()?;
        let out = text
            .chars()
            .map(|s| self.decrypt_char(s, mul_inv))
            .collect();
        Ok(out)
    }
}

#[cfg(test)]
mod affine_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "UMXFZRNBIKVJQCVOWZLAPVEXKUMXGDYTSVH";

    #[test]
    fn encrypt_test() {
        let mut cipher = Affine::default();
        cipher.add_key = 3;
        cipher.mul_key = 5;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Affine::default();
        cipher.add_key = 3;
        cipher.mul_key = 5;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
