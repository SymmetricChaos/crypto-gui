use utils::byte_formatting::ByteFormat;

use crate::{
    blake::Blake2bStateful,
    traits::{ClassicHasher, StatefulHasher},
};

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
        assert!(
            self.key.len() <= 64,
            "the length of the key cannot be more than 64 bytes"
        );

        // Incorporate the length of the output and the bytes

        // For short output just finalize and return the bytes
        if self.hash_len <= 64 {
            let mut h = Blake2bStateful::init(&self.key, self.hash_len as u64);
            h.update(&(self.hash_len as u32).to_le_bytes());
            h.update(bytes);
            return h.finalize();
        }

        let mut h = Blake2bStateful::init(&self.key, 64);
        h.update(&(self.hash_len as u32).to_le_bytes());
        h.update(bytes);
        let mut out = Vec::with_capacity(self.hash_len);
        let mut ctr = self.hash_len;
        let mut v = h.finalize();

        while ctr > 32 {
            // Take 32 bytes of the temporary value then hash the whole vector
            // This is presumably related to length extension type attacks
            out.extend_from_slice(&v[0..32]);
            ctr -= 32;
            v = Blake2bStateful::hash_512(&v)
        }

        // Final bytes change the hash length of Blake2b, which alters its state, so truncation is not used
        let mut h = Blake2bStateful::init(&[], ctr as u64);
        h.update(&v);
        out.extend_from_slice(&h.finalize());

        out
    }

    crate::hash_bytes_from_string! {}
}
