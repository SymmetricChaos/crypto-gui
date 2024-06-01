use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, sha2::Sha2, traits::ClassicHasher};

pub struct Pbkdf1 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    prf: Box<dyn ClassicHasher>,
    salt: Vec<u8>,
    iterations: u32,
    output_length: u64, // size of the output in bytes
}

impl Default for Pbkdf1 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            prf: Box::new(Sha2::default()),
            salt: Vec::new(),
            iterations: Default::default(),
            output_length: 64,
        }
    }
}

impl Pbkdf1 {
    pub fn hash_block(&self, bytes: &[u8], block_num: u32) -> Vec<u8> {
        let mut bytes = bytes.to_vec();
        bytes.extend(self.salt.iter());
        bytes.extend(block_num.to_be_bytes());

        let mut out = self.prf.hash(&bytes);

        for _ in 1..self.iterations {
            let t = self.prf.hash(&out);
            for (target, new) in out.iter_mut().zip_eq(t.iter()) {
                *target ^= new
            }
        }

        out
    }
}

impl ClassicHasher for Pbkdf1 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();

        let mut block_num = 1;
        while out.len() < self.output_length as usize {
            out.extend(self.hash_block(bytes, block_num));
            block_num += 1;
        }

        out.truncate(self.output_length as usize);
        out
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
