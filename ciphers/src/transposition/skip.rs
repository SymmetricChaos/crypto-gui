use itertools::Itertools;
use num::Integer;
use utils::errors::GeneralError;

use crate::Cipher;

pub struct SkipCipher {
    pub initial: usize,
    pub skip: usize,
}

impl Default for SkipCipher {
    fn default() -> Self {
        Self {
            initial: 6,
            skip: 5,
        }
    }
}

impl SkipCipher {
    /// Panics if skip == 0
    pub fn new(initial: usize, skip: usize) -> Self {
        assert!(skip > 0);
        Self { initial, skip }
    }
}

impl Cipher for SkipCipher {
    fn encrypt(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        let cs = text.chars().collect_vec();
        if cs.len().gcd(&self.skip) != 1 {
            return Err(GeneralError::input(
                "input text must have a length that is coprime to the skip size",
            ));
        }
        let mut out = String::with_capacity(cs.len());
        let mut idx = self.initial % cs.len();
        for _ in 0..cs.len() {
            out.push(cs[idx]);
            idx = (idx + self.skip) % cs.len();
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CTEXT: &'static str = "TEUCBONOJMSVRHLZDGHQIKRWFXUPOETEAYO";

    #[test]
    fn encrypt_test() {
        let cipher = SkipCipher::new(0, 2);
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = SkipCipher::new(0, 2);
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
