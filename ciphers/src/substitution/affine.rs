use crate::{traits::Cipher};
use utils::{math_functions::mul_inv, preset_alphabet::Alphabet, vecstring::VecString};
use utils::errors::GeneralError;
pub struct Affine {
    pub add_key: usize,
    pub mul_key: usize,
    pub alphabet: VecString,
}

impl Affine {
    fn encrypt_char(&self, c: char) -> Result<char, GeneralError> {
        let mut pos = self
            .alphabet
            .get_pos(c)
            .ok_or_else(|| GeneralError::invalid_input_char(c))?;
        pos *= self.mul_key;
        pos += self.add_key;
        pos %= self.alphabet_len();
        Ok(*self.alphabet.get_char(pos).unwrap())
    }

    fn decrypt_char(&self, c: char, mul_key_inv: usize) -> Result<char, GeneralError> {
        let mut pos = self
            .alphabet
            .get_pos(c)
            .ok_or_else(|| GeneralError::invalid_input_char(c))?;
        pos += self.alphabet_len() - self.add_key;
        pos *= mul_key_inv;
        pos %= self.alphabet_len();
        Ok(*self.alphabet.get_char(pos).unwrap())
    }

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(alphabet);
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }

    pub fn find_mul_inverse(&self) -> Result<usize, GeneralError> {
        match mul_inv(&self.mul_key, &self.alphabet.chars().count()) {
            Some(n) => {
                match usize::try_from(n) {
                    Ok(n) => Ok(n),
                    Err(e) => Err(GeneralError::key(e)),
                }
            }  
            None => Err(GeneralError::key("the multiplicative key of an Affine Cipher cannot share any factors with the length of the alphabet"))
        }
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
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        self.find_mul_inverse()?;
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            out.push(self.encrypt_char(c)?);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let mul_inv = self.find_mul_inverse()?;
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            out.push(self.decrypt_char(c, mul_inv)?);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod affine_tests {
    use super::*;

    const PTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CTEXT: &'static str = "UMXFZRNBIKVJQCVOWZLAPVEXKUMXGDYTSVH";

    #[test]
    fn encrypt_test() {
        let mut cipher = Affine::default();
        cipher.add_key = 3;
        cipher.mul_key = 5;
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Affine::default();
        cipher.add_key = 3;
        cipher.mul_key = 5;
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
