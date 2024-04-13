use crate::{errors::HasherError, traits::ClassicHasher};
use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

pub enum HmacHasher {
    Md4,
    Md5,
    Sha2_224,
    Sha2_256,
    Sha2_364,
    Sha2_512,
}

pub struct Hmac {
    input_format: ByteFormat,
    output_format: ByteFormat,
    key_format: ByteFormat,
    key: Vec<u8>,
    block_size: usize,
    hasher: Box<dyn ClassicHasher>,
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
        let k = if self.key.len() > self.block_size {
            let mut k = self.hasher.hash(&self.key);
            k.truncate(self.block_size);
            k
        } else {
            self.key.clone()
        };
        let mut o_key: Vec<u8> = vec![0x5c; self.block_size]
            .into_iter()
            .zip(k.iter())
            .map(|(a, b)| a ^ *b)
            .collect_vec();
        let mut i_key: Vec<u8> = vec![0x36; self.block_size]
            .into_iter()
            .zip(k.iter())
            .map(|(a, b)| a ^ *b)
            .collect_vec();

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
