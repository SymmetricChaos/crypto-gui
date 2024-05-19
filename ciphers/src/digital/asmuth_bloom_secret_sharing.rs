use rand::{thread_rng, Rng};

use crate::{Cipher, CipherError};

// https://arxiv.org/pdf/1901.02802

pub struct AMSecretSharing {
    pub shares: u64,
    pub threshold: u64,
    pub k: u64,
    pub n: u64,
    pub random_shares: bool,
}

impl Default for AMSecretSharing {
    fn default() -> Self {
        Self {
            shares: 3,
            threshold: 3,
            k: 3,
            n: 4,
            random_shares: true,
        }
    }
}

impl AMSecretSharing {}

impl Cipher for AMSecretSharing {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let secret =
            u64::from_str_radix(text, 10).map_err(|e| CipherError::Input(e.to_string()))?;

        // to be filled with an increasing sequence of pairwise coprime numbers
        let mut sequence: Vec<u64> = Vec::with_capacity((self.n + 1) as usize);

        if sequence[0] >= secret {
            return Err(CipherError::input(
                "secret must be less than the first term of the sequence",
            ));
        }

        let mut rng = thread_rng();

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
