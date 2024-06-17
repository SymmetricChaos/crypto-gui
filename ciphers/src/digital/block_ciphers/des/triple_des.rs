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
    pub fn ksa(&mut self, keys: [u64; 3]) -> Result<(), CipherError> {
        let mut temp = [[0u64; 16]; 3];
        for (i, key) in keys.into_iter().enumerate() {
            temp[i] = des_ksa(key)?;
        }
        self.subkeys = temp;
        Ok(())
    }

    fn encrypt_with_subkey(&self, block: u64, i: usize) -> u64 {
        let mut b = initial_permutation(block);
        for key in self.subkeys[i].iter() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }

    fn decrypt_with_subkey(&self, block: u64, i: usize) -> u64 {
        let mut b = initial_permutation(block);
        for key in self.subkeys[i].iter().rev() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }

    pub fn encrypt_block(&self, block: u64) -> u64 {
        let b = self.encrypt_with_subkey(block, 2);
        let b = self.decrypt_with_subkey(b, 1);
        self.encrypt_with_subkey(b, 0)
    }

    pub fn decrypt_block(&self, block: u64) -> u64 {
        let b = self.decrypt_with_subkey(block, 0);
        let b = self.encrypt_with_subkey(b, 1);
        self.decrypt_with_subkey(b, 2)
    }
}
