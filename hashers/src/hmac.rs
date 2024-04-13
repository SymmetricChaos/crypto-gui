use crate::{errors::HasherError, sha2::sha256::Sha2_256, traits::ClassicHasher};
use utils::byte_formatting::ByteFormat;

pub struct Hmac {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key_format: ByteFormat,
    pub key: Vec<u8>,
    pub block_size: usize,
    pub hasher: Box<dyn ClassicHasher>,
}

impl Default for Hmac {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key_format: ByteFormat::Hex,
            key: Vec::new(),
            block_size: 64,
            hasher: Box::new(Sha2_256::default()),
        }
    }
}

impl Hmac {
    pub fn key_from_str(&mut self, key_str: &str) -> Result<(), HasherError> {
        let bytes = self
            .key_format
            .text_to_bytes(key_str)
            .map_err(|_| HasherError::general("byte format error"))?;
        self.key = bytes;
        Ok(())
    }
}

impl ClassicHasher for Hmac {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        // Compress the key if necessary
        let k = if self.key.len() > self.block_size {
            let mut k = self.hasher.hash(&self.key);
            k.truncate(self.block_size);
            k
        } else {
            self.key.clone()
        };
        // XOR the key into the outer padding and the inner padding
        let mut o_key: Vec<u8> = vec![0x5c; self.block_size];
        for (i, byte) in k.iter().enumerate() {
            o_key[i] ^= byte;
        }
        let mut i_key: Vec<u8> = vec![0x36; self.block_size];
        for (i, byte) in k.iter().enumerate() {
            i_key[i] ^= byte;
        }

        i_key.extend_from_slice(bytes);
        let inner = self.hasher.hash(&i_key);
        o_key.extend_from_slice(&inner);
        self.hasher.hash(&o_key)
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod hmac_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Hmac::default();

        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.key_format = ByteFormat::Utf8;

        hasher.key_from_str("key").unwrap();

        assert_eq!(
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8",
            hasher
                .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
                .unwrap()
        );
    }
}
