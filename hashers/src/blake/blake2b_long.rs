use utils::byte_formatting::ByteFormat;

use crate::{
    blake::{Blake2b, Blake2bStateful},
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
        assert!(self.hash_len > 1, "hash_len cannot be 0 bytes");
        assert!(
            self.key.len() <= 64,
            "the length of the key cannot be more than 64 bytes"
        );

        let mut msg = (self.hash_len as u32).to_le_bytes().to_vec();
        msg.extend_from_slice(bytes);

        // For short output the length is concatenated with the message and then hashed directly with Blake2b
        if self.hash_len <= 64 {
            let mut h = Blake2bStateful::init(&self.key, self.hash_len as u64);
            h.update(bytes);
            return h.finalize();
        }

        let mut hasher = Blake2b::default().hash_len(64).key(&self.key);
        let mut v = hasher.hash(&msg);
        let mut out = Vec::with_capacity(self.hash_len);
        let mut ctr = self.hash_len;

        while ctr > 32 {
            // Take 32 bytes of the temporary value then hash the whole vector
            // This is presumably related to length extension type attacks
            out.extend_from_slice(&v[0..32]);
            ctr -= 32;
            v = hasher.hash(&v);
        }

        // Final bytes change the hash length of Blake2b, which alters its state, so truncation is not used
        hasher.hash_len = ctr;
        v = hasher.hash(&v);
        out.extend_from_slice(&v);

        out
    }

    crate::hash_bytes_from_string! {}
}
