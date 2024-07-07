use itertools::Itertools;
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

use crate::{Cipher, CipherError};

pub struct SecretSplitting {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub n_splits: u32,
    pub sep: String,
}

impl Default for SecretSplitting {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            n_splits: 3,
            sep: String::from(","),
        }
    }
}

impl SecretSplitting {
    fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<Vec<u8>> {
        assert!(self.n_splits >= 2);

        let mut rng = thread_rng();
        let mut splits = vec![vec![0; self.n_splits as usize]; bytes.len()];

        // The secret is placed arbitrarily
        splits[0] = bytes.to_vec();

        // Each other split is filled with random bytes
        for i in 1..self.n_splits {
            rng.fill_bytes(&mut splits[i as usize]);
        }

        // Each other split is XORed into the secret
        for i in 1..self.n_splits {
            for j in 0..self.n_splits {
                splits[0][j as usize] ^= splits[i as usize][j as usize]
            }
        }

        splits
    }

    fn decrypt_bytes(&self, bytes: &Vec<Vec<u8>>) -> Vec<u8> {
        for b in bytes.iter() {
            assert_eq!(b.len(), bytes[0].len())
        }

        let mut out = vec![0; bytes[0].len()];

        for split in bytes {
            for (i, byte) in split.iter().enumerate() {
                out[i] ^= *byte
            }
        }

        out
    }
}

impl Cipher for SecretSplitting {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        if self.n_splits < 2 {
            return Err(CipherError::state(
                "secret splitting requires at least two shares",
            ));
        }

        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.encrypt_bytes(&bytes);
        Ok(out
            .iter()
            .map(|split| self.output_format.byte_slice_to_text(split))
            .join(&self.sep))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let splits = text.split(&self.sep).collect_vec();
        let mut split_bytes = Vec::new();
        for split in splits {
            split_bytes.push(
                self.input_format
                    .text_to_bytes(split.trim())
                    .map_err(|_| CipherError::input("byte format error"))?,
            );
        }
        for b in split_bytes.iter() {
            if b.len() != split_bytes[0].len() {
                return Err(CipherError::input("all splits must be of equal length"));
            }
        }
        let out = self.decrypt_bytes(&split_bytes);

        Ok(self.output_format.byte_slice_to_text(&out))
    }
}
