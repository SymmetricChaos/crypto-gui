use std::io::Read;

use itertools::Itertools;

use crate::traits::ClassicHasher;

// https://eprint.iacr.org/2012/351.pdf

pub struct Blake2b {
    pub key: Vec<u8>,    // optional key, length from 0 to 64 bytes
    pub hash_len: usize, // length of output in bytes, 1 to 64
}

impl Default for Blake2b {
    fn default() -> Self {
        Self {
            key: Vec::new(),
            hash_len: 16,
        }
    }
}

impl Blake2b {
    // Initialization vector, sqrt of the first eight primes
    const IV: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    pub fn compress() {}
}

impl ClassicHasher for Blake2b {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        if self.hash_len > 64 {
            panic!("hash_len cannot be greater than 64 as there are only 64 bytes of state")
        }

        let mut state = Self::IV.clone();
        // Key length and hash length are mixed into the state, this ensures identical inputs don't resemble each other when these inputs are varied
        let mixer: u64 = 0x01010000 ^ ((self.key.len() as u64) << 8) ^ self.hash_len as u64;
        state[0] ^= mixer;

        let mut bytes_taken = 0;
        let mut bytes_remaining = bytes.len() + 128;

        let mut key = self.key.clone();
        while key.len() != 128 {
            key.push(0);
        }

        // compress the key

        // compress the data in 128 byte chunks, excluding the last chunk

        // compress the last chunk, padding with zeroes if it is too short

        state
            .iter()
            .map(|x| x.to_be_bytes())
            .flatten()
            .take(self.hash_len)
            .collect_vec()
    }
}

#[cfg(test)]
mod blake2b_tests {
    use super::*;
}
