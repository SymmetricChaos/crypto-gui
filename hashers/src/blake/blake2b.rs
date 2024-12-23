use utils::byte_formatting::ByteFormat;

use crate::traits::{ClassicHasher, StatefulHasher};

use super::Blake2bStateful;

// https://eprint.iacr.org/2012/351.pdf

#[derive(Debug, Clone)]

pub struct Blake2b {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: Vec<u8>,    // optional key, length from 0 to 64 bytes
    pub hash_len: usize, // length of output in bytes, 1 to 64
}

impl Default for Blake2b {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: Vec::new(),
            hash_len: 32, // default to 256 bits
        }
    }
}

impl Blake2b {
    pub fn with_hash_len(mut self, hash_len: usize) -> Self {
        assert!(hash_len > 1 && hash_len <= 64);
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

impl ClassicHasher for Blake2b {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut h = Blake2bStateful::init(&self.key, self.hash_len as u64);
        h.update(bytes);
        h.finalize()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    empty_hash_len_64, Blake2b::default().with_hash_len(64), "",
    "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce";
    abc_hash_len_64, Blake2b::default().with_hash_len(64), "abc",
    "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923";
    keyed_hash_len_64,
    Blake2b::default()
        .input(ByteFormat::Hex)
        .with_hash_len(64)
        .with_key(ByteFormat::Hex.text_to_bytes("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f").unwrap()),
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfe",
    "142709d62e28fcccd0af97fad0f8465b971e82201dc51070faa0372aa43e92484be1c1e73ba10906d5d1853db6a4106e0a7bf9800d373d6dee2d46d62ef2a461";
);
