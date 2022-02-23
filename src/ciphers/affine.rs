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
    fn char_to_val(&self, c: char) -> Option<usize> {
        self.alphabet.chars().position(|x| x == c)
    }

    fn val_to_char(&self, v: usize) -> Option<char> {
        self.alphabet.chars().nth(v)
    }

    pub fn length(&self) -> usize {
        self.alphabet.chars().count()
    }

    pub fn find_inverse(&self) -> Option<usize> {
        mul_inv(self.mul_key, self.alphabet.chars().count())
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
        let symbols = text.chars();
        let mut out = String::with_capacity(text.len());
        match self.find_inverse() {
            Some(n) => n,
            None => return Err(CipherError::key("The multiplicative key of an Affine Cipher cannot share any factors with the length of the alphabet"))
        };
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => (v * self.mul_key + self.add_key) % self.length(),
                None => return Err(CipherError::invalid_input_char(s)),
            };
            // Unwrap is justified because the modulo operation forces n to be a valid index
            out.push(self.val_to_char(n).unwrap())
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let symbols = text.chars();
        let mut out = String::with_capacity(text.len());
        let mki = match self.find_inverse() {
            Some(n) => n,
            None => return Err(CipherError::key("The multiplicative key of an Affine Cipher cannot share any factors with the length of the alphabet"))
        };
        for s in symbols {
            let val = self.char_to_val(s);
            let n = match val {
                Some(v) => ((v + self.length() - self.add_key) * mki) % self.length(),
                None => return Err(CipherError::invalid_input_char(s)),
            };
            // Unwrap is justified because the modulo operation forces n to be a valid index
            out.push(self.val_to_char(n).unwrap())
        }
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

    fn get_output_alphabet(&self) -> &String {
        &self.alphabet
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        todo!()
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
