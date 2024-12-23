use utils::byte_formatting::ByteFormat;

use crate::traits::{ClassicHasher, StatefulHasher};

use super::Blake2sStateful;

// https://eprint.iacr.org/2012/351.pdf

#[derive(Debug, Clone)]
pub struct Blake2s {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: Vec<u8>,    // optional key, length from 0 to 32 bytes
    pub hash_len: usize, // length of output in bytes, 1 to 32
}

impl Default for Blake2s {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: Vec::new(),
            hash_len: 16, // default to 128 bits
        }
    }
}

impl Blake2s {
    pub fn with_hash_len(mut self, hash_len: usize) -> Self {
        assert!(hash_len > 1 && hash_len <= 32);
        self.hash_len = hash_len;
        self
    }

    pub fn with_key<T: AsRef<[u8]>>(mut self, key: T) -> Self {
        assert!(key.as_ref().len() <= 32);
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

impl ClassicHasher for Blake2s {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut h = Blake2sStateful::init(&self.key, self.hash_len as u32);
        h.update(bytes);
        h.finalize()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    empty_hash_len_32, Blake2s::default().with_hash_len(32), "",
    "69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9";
    hash_8_len_32, Blake2s::default().with_hash_len(32).input(ByteFormat::Hex), "0001020304050607",
    "c7e887b546623635e93e0495598f1726821996c2377705b93a1f636f872bfa2d";
    keyed_hash_len_32,Blake2s::default()
        .input(ByteFormat::Hex)
        .with_hash_len(32)
        .with_key(ByteFormat::Hex.text_to_bytes("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").unwrap()),
        "000102030405060708090a0b0c0d0e0f",
    "19ba234f0a4f38637d1839f9d9f76ad91c8522307143c97d5f93f69274cec9a7";
);
