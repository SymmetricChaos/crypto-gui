use super::Cipher;
use crate::{errors::CipherError};
use crate::text_types::{PresetAlphabet::*};
use crate::math_functions::mul_inv;
use rand::{prelude::ThreadRng, Rng};

pub struct Affine {
    pub add_key: usize,
    pub mul_key: usize,
    alphabet: String,
}

impl Affine {
    fn encrypt_char(&self, c: char) -> char {
        let mut pos = self.alphabet.chars().position(|x| x == c).unwrap();
        pos *= self.mul_key;
        pos += self.add_key;
        pos %= self.alphabet_len();
        self.alphabet.chars().nth(pos).unwrap()
    }
 
    fn decrypt_char(&self, c: char, mul_key_inv: usize) -> char {
        let mut pos = self.alphabet.chars().position(|x| x == c).unwrap();
        pos += self.alphabet_len() - self.add_key;
        pos *= mul_key_inv;
        pos %= self.alphabet_len();
        self.alphabet.chars().nth(pos).unwrap()
    }
 
    pub fn alphabet_len(&self) -> usize {
        self.alphabet.chars().count()
    }

    pub fn length(&self) -> usize {
        self.alphabet.chars().count()
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
                return Err(CipherError::invalid_input_char(c))
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
            alphabet: String::from(BasicLatin),
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
        let out = text.chars().map(|s| self.decrypt_char(s, mul_inv)).collect();
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        let length = self.alphabet.len();
        self.add_key = rng.gen_range(0..length);
        loop {
            let mul = rng.gen_range(1..length);
            if mul_inv(mul, self.length()).is_some() {
                self.mul_key = mul;
                break;
            };
        }
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
    
    fn validate_settings(&self) -> Result<(), CipherError> {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default();
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
