use utils::byte_formatting::ByteFormat;

use crate::{blake::Blake2b, traits::ClassicHasher};

// https://eprint.iacr.org/2012/351.pdf

// Identical to Blake2b but allowing a hash of any length. This variant is specific to Argon2.
#[derive(Debug, Clone)]
pub struct Blake2bLong {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: Vec<u8>,    // optional key, length from 0 to 64 bytes
    pub hash_len: usize, // length of output in bytes, greater than 1
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
    pub fn hash_len(mut self, hash_len: usize) -> Self {
        assert!(hash_len > 1);
        self.hash_len = hash_len;
        self
    }

    pub fn key<T: AsRef<[u8]>>(mut self, key: T) -> Self {
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
        assert!(self.hash_len > 1, "hash_len cannot be 0 bytes");
        assert!(
            self.key.len() <= 64,
            "the length of the key cannot be more than 64 bytes"
        );

        // Short circuit for short inputs
        if self.hash_len <= 64 {
            return Blake2b::default()
                .hash_len(self.hash_len)
                .key(&self.key)
                .hash(bytes);
        }

        let hasher = Blake2b::default().hash_len(64).key(&self.key);
        let mut temp = hasher.hash(bytes);
        let mut out = Vec::with_capacity(self.hash_len);

        while out.len() < self.hash_len {
            // Take 32 bytes of the temporary value then hash the whole vector
            // By using only half of the output length extension is as difficult as a preimage attack
            out.extend_from_slice(&temp[0..32]);
            temp = hasher.hash(&temp);
        }
        out.truncate(self.hash_len);

        out
    }

    crate::hash_bytes_from_string! {}
}
