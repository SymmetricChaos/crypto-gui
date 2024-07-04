use crate::errors::HasherError;

use super::des_functions::{des_ksa, final_permutation, initial_permutation, round};

pub struct Des {
    subkeys: [u64; 16],
}

impl Default for Des {
    fn default() -> Self {
        Self { subkeys: [0; 16] }
    }
}

impl Des {
    // Key Scheduling Algorithm (key generation)
    pub fn ksa(&mut self, key: u64) -> Result<(), HasherError> {
        self.subkeys = des_ksa(key)?;
        Ok(())
    }

    pub fn encrypt_block(&self, block: u64) -> u64 {
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }

}
