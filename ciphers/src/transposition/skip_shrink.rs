use itertools::Itertools;
use num::Integer;
use utils::errors::GeneralError;

use crate::Cipher;

pub struct SkipShrink {
    pub initial: usize,
    pub skip: usize,
}

impl Default for SkipShrink {
    fn default() -> Self {
        Self {
            initial: 6,
            skip: 5,
        }
    }
}

impl SkipShrink {
    pub fn new(initial: usize, skip: usize) -> Self {
        Self { initial, skip }
    }
}

impl Cipher for SkipShrink {
    fn encrypt(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        let mut cs = text.chars().collect_vec();
        let mut out = String::with_capacity(cs.len());
        let mut idx = self.initial % cs.len();
        for _ in 1..cs.len() {
            out.push(cs.remove(idx));
            idx = (idx + self.skip) % cs.len();
        }
        out.push(*cs.last().unwrap());
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
        let cipher = SkipShrink::new(0, 1);
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = SkipShrink::new(0, 1);
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
