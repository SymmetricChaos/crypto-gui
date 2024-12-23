use utils::byte_formatting::ByteFormat;

use crate::{
    blake::Blake2bStateful,
    traits::{ClassicHasher, StatefulHasher},
};

use super::blake2b_long_stateful::Blake2bLongStatful;

// https://eprint.iacr.org/2012/351.pdf

// Identical to Blake2b but allowing a hash of any length. This variant is specific to Argon2.
#[derive(Debug, Clone)]
pub struct Blake2bLong {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: Vec<u8>,  // optional key, length from 0 to 64 bytes
    pub hash_len: u64, // length of output in bytes, greater than 1
}

impl Default for Blake2bLong {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: Vec::new(),
            hash_len: 32, // default to 256 bits
        }
    }
}

impl Blake2bLong {
    pub fn with_hash_len(mut self, hash_len: u64) -> Self {
        assert!(hash_len > 1);
        self.hash_len = hash_len;
        self
    }

    pub fn with_key<T: AsRef<[u8]>>(mut self, key: T) -> Self {
        assert!(key.as_ref().len() <= 64);
        self.key = key.as_ref().to_vec();
        self
    }

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for Blake2bLong {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut h = Blake2bLongStatful::init(&self.key, self.hash_len);
        h.update(bytes);
        h.finalize()
    }

    crate::hash_bytes_from_string! {}
}
