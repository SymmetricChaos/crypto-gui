use crate::Cipher;
use nalgebra::{DMatrix, DVector};
use utils::{
    errors::GeneralError, preset_alphabet::Alphabet, text_functions::string_chunks,
    vecstring::VecString,
};
// https://patents.google.com/patent/US1845947

pub struct Hill {
    pub alphabet: VecString,
    pub mat: DMatrix<usize>,
    pub mat_inv: DMatrix<usize>,
    pub key1: String,
    pub key2: String,
}

impl Default for Hill {
    fn default() -> Self {
        Self {
            alphabet: Alphabet::BasicLatin.into(),
            mat: DMatrix::from_row_slice(3, 3, &[6, 24, 1, 13, 16, 10, 20, 17, 15]),
            mat_inv: DMatrix::from_row_slice(3, 3, &[8, 5, 10, 21, 8, 21, 21, 12, 8]),
            key1: String::from("EXAMPLE"),
            key2: String::from("PASSWORDS"),
        }
    }
}

impl Hill {
    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(&alphabet);
    }

    fn cyclic_key_1(&self) -> impl Iterator<Item = usize> + '_ {
        self.key1
            .chars()
            .map(|x| self.alphabet.get_pos(x).unwrap())
            .cycle()
    }

    fn cyclic_key_2(&self) -> impl Iterator<Item = usize> + '_ {
        self.key2
            .chars()
            .map(|x| self.alphabet.get_pos(x).unwrap())
            .cycle()
    }

    fn encrypt_char(&self, c: char, k: usize) -> Result<char, GeneralError> {
        let p = self
            .alphabet
            .get_pos(c)
            .ok_or(GeneralError::invalid_input_char(c))?;
        Ok(*self.alphabet.get_char_offset(p, k as i32).unwrap())
    }

    fn decrypt_char(&self, c: char, k: usize) -> Result<char, GeneralError> {
        let p = self
            .alphabet
            .get_pos(c)
            .ok_or(GeneralError::invalid_input_char(c))?;
        Ok(*self.alphabet.get_char_offset(p, -(k as i32)).unwrap())
    }

    fn encrypt_cyclic_1(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());
        for (c, n) in text.chars().zip(self.cyclic_key_1()) {
            out.push(self.encrypt_char(c, n)?)
        }
        Ok(out)
    }

    fn decrypt_cyclic_1(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());
        for (c, n) in text.chars().zip(self.cyclic_key_1()) {
            out.push(self.decrypt_char(c, n)?)
        }
        Ok(out)
    }

    fn encrypt_cyclic_2(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());
        for (c, n) in text.chars().zip(self.cyclic_key_2()) {
            out.push(self.encrypt_char(c, n)?)
        }
        Ok(out)
    }

    fn decrypt_cyclic_2(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());
        for (c, n) in text.chars().zip(self.cyclic_key_2()) {
            out.push(self.decrypt_char(c, n)?)
        }
        Ok(out)
    }

    fn encrypt_matrix(&self, text: &str) -> Result<String, GeneralError> {
        let m = &self.mat;
        let mut out = String::new();
        for chunk in string_chunks(text, m.nrows()) {
            let column = DVector::from_iterator(
                m.nrows(),
                chunk.chars().map(|x| self.alphabet.get_pos(x).unwrap()),
            );

            let t = m * column;

            for n in t.iter() {
                out.push(*self.alphabet.get_char(n % self.alphabet.len()).unwrap());
            }
        }
        Ok(out)
    }

    fn decrypt_matrix(&self, text: &str) -> Result<String, GeneralError> {
        let m = &self.mat_inv;
        let mut out = String::new();
        for chunk in string_chunks(text, m.nrows()) {
            let column = DVector::from_iterator(
                m.nrows(),
                chunk.chars().map(|x| self.alphabet.get_pos(x).unwrap()),
            );

            let t = m * column;

            for n in t.iter() {
                out.push(*self.alphabet.get_char(n % self.alphabet.len()).unwrap());
            }
        }
        Ok(out)
    }
}

impl Cipher for Hill {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        if text.chars().count() % self.mat.nrows() != 0 {
            return Err(GeneralError::input(format!(
                "plaintext length must be a multiple of {}",
                self.mat_inv.len(),
            )));
        }
        let t = self.encrypt_cyclic_1(text)?;
        let t = self.encrypt_matrix(&t)?;
        self.encrypt_cyclic_2(&t)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        if text.chars().count() % self.mat.nrows() != 0 {
            return Err(GeneralError::input(format!(
                "ciphertext length must be a multiple of {}",
                self.mat_inv.len(),
            )));
        }
        let t = self.decrypt_cyclic_2(text)?;
        let t = self.decrypt_matrix(&t)?;
        self.decrypt_cyclic_1(&t)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn matrix_encr() {
        let cipher = Hill::default();
        assert_eq!("POH", cipher.encrypt_matrix("ACT").unwrap());
        assert_eq!("ACT", cipher.decrypt_matrix("POH").unwrap());
        assert_eq!("FIN", cipher.encrypt_matrix("CAT").unwrap());
        assert_eq!("CAT", cipher.decrypt_matrix("FIN").unwrap());
    }

    #[test]
    fn encrypt_decrypt() {
        let cipher = Hill::default();
        let ct = cipher.encrypt("THEQUICKBROWNFOXJUMPS").unwrap();
        assert_eq!("THEQUICKBROWNFOXJUMPS", cipher.decrypt(&ct).unwrap());
    }
}
