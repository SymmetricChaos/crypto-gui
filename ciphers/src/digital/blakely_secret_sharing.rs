use crate::{Cipher, CipherError};

// https://arxiv.org/pdf/1901.02802

pub struct BlakelySecretSharing {
    pub shares: u64,
    pub threshold: u64,
    pub random_shares: bool,
}

impl Default for BlakelySecretSharing {
    fn default() -> Self {
        Self {
            shares: 3,
            threshold: 3,
            random_shares: true,
        }
    }
}

impl BlakelySecretSharing {}

impl Cipher for BlakelySecretSharing {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}

#[cfg(test)]
mod blakely_tests {

    #[test]
    fn encrypt_test() {}

    #[test]
    fn decrypt_test() {}
}
