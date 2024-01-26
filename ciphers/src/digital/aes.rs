use crate::{Cipher, CipherError};

pub struct Aes {}

impl Default for Aes {
    fn default() -> Self {
        Self {}
    }
}

impl Aes {}

impl Cipher for Aes {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}

#[cfg(test)]
mod aes_tests {

    use super::*;
}
