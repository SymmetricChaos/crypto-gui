use utils::byte_formatting::ByteFormat;

use crate::{
    digital::block_ciphers::{BlockCipherMode, BlockCipherPadding},
    CipherError,
};

use super::des_functions::*;

pub struct TripleDes {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    subkeys: [[u64; 16]; 3],
    pub ctr: u64,
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for TripleDes {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            subkeys: [[0; 16]; 3],
            ctr: 0,
            mode: BlockCipherMode::default(),
            padding: BlockCipherPadding::default(),
        }
    }
}

impl TripleDes {
    pub fn ksa(&mut self, key1: u64, key2: u64, key3: u64) -> Result<(), CipherError> {
        for (i, key) in [key1, key2, key3].into_iter().enumerate() {
            test_des_key(key)?;
            des_ksa(&mut self.subkeys[i], key);
        }
        Ok(())
    }

    pub fn encrypt_block(&self, block: u64) -> u64 {
        // Encrypt with K3
        let mut b = initial_permutation(block);
        for key in self.subkeys[3].iter() {
            b = round(b, *key);
        }
        b = final_permutation((b << 32) | (b >> 32));
        // Decrypt with K2
        b = initial_permutation(b);
        for key in self.subkeys[1].iter().rev() {
            b = round(b, *key);
        }
        b = final_permutation((b << 32) | (b >> 32));
        // Encrypt with K1
        b = initial_permutation(b);
        for key in self.subkeys[0].iter() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }

    pub fn decrypt_block(&self, block: u64) -> u64 {
        // Decrypt with K1
        let mut b = initial_permutation(block);
        for key in self.subkeys[0].iter().rev() {
            b = round(b, *key);
        }
        b = final_permutation((b << 32) | (b >> 32));
        // Encrypt with K2
        b = initial_permutation(b);
        for key in self.subkeys[1].iter() {
            b = round(b, *key);
        }
        b = final_permutation((b << 32) | (b >> 32));
        // Decrypt with K3
        b = initial_permutation(b);
        for key in self.subkeys[2].iter().rev() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }
}
