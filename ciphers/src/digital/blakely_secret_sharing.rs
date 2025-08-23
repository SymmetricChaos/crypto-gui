use rand::{thread_rng, Rng};
use utils::errors::GeneralError;

use crate::Cipher;

// https://arxiv.org/pdf/1901.02802

pub struct BlakelySecretSharing {
    pub shares: u64,
    pub threshold: u64,
    pub modulus: u64,
    pub random_shares: bool,
}

impl Default for BlakelySecretSharing {
    fn default() -> Self {
        Self {
            shares: 3,
            threshold: 3,
            modulus: 73,
            random_shares: true,
        }
    }
}

impl BlakelySecretSharing {}

impl Cipher for BlakelySecretSharing {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let secret = u64::from_str_radix(text, 10).map_err(|e| GeneralError::input(e))?;

        let mut rng = thread_rng();

        let x = rng.gen_range(0..self.modulus);

        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
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
