use itertools::Itertools;
use rand::{thread_rng, RngCore};
use utils::{byte_formatting::ByteFormat, errors::GeneralError};

use crate::Cipher;

pub struct XorSecretSplitting {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,

    pub n_splits: u32,
    pub sep: String,
}

impl Default for XorSecretSplitting {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,

            n_splits: 3,
            sep: String::from(","),
        }
    }
}

impl XorSecretSplitting {
    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<Vec<u8>> {
        assert!(self.n_splits >= 2);

        let mut rng = thread_rng();
        let mut splits = vec![vec![0; bytes.len()]; self.n_splits as usize];

        // The secret is placed arbitrarily
        splits[0] = bytes.to_vec();

        // Each other split is filled with random bytes
        for i in 1..self.n_splits {
            rng.fill_bytes(&mut splits[i as usize]);
        }

        // Each other split is XORed into the secret
        for i in 1..self.n_splits {
            for j in 0..bytes.len() {
                splits[0][j] ^= splits[i as usize][j]
            }
        }

        splits
    }

    pub fn decrypt_splits(&self, splits: &Vec<Vec<u8>>) -> Vec<u8> {
        for b in splits.iter() {
            assert_eq!(b.len(), splits[0].len())
        }

        let mut out = vec![0; splits[0].len()];

        for split in splits {
            for (i, byte) in split.iter().enumerate() {
                out[i] ^= *byte
            }
        }

        out
    }
}

impl Cipher for XorSecretSplitting {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        if self.n_splits < 2 {
            return Err(GeneralError::state(
                "secret splitting requires at least two shares",
            ));
        }

        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| GeneralError::input("byte format error"))?;
        let out = self.encrypt_bytes(&bytes);
        Ok(out
            .iter()
            .map(|split| self.output_format.byte_slice_to_text(split))
            .join(&self.sep))
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let splits = text.split(&self.sep).collect_vec();
        let mut split_bytes = Vec::new();
        for split in splits {
            split_bytes.push(
                self.input_format
                    .text_to_bytes(split.trim())
                    .map_err(|_| GeneralError::input("byte format error"))?,
            );
        }
        for b in split_bytes.iter() {
            if b.len() != split_bytes[0].len() {
                return Err(GeneralError::input("all splits must be of equal length"));
            }
        }
        let out = self.decrypt_splits(&split_bytes);

        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod secret_splitting_tests {

    use super::*;

    #[test]
    fn test_split_unsplit() {
        let cipher = XorSecretSplitting::default();
        let ptext = "0a1b2c3d4e5f";
        let ctext = cipher.encrypt(&ptext).unwrap();
        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, dtext);
    }
}
