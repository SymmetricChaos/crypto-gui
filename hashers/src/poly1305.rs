use crate::{errors::HasherError, traits::ClassicHasher};
use crypto_bigint::U192;
use utils::byte_formatting::ByteFormat;

pub struct Poly1305 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u8; 16],
}

impl Default for Poly1305 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key: [0; 16],
        }
    }
}

impl Poly1305 {
    //const MODULUS: U192 = 2 ** 130 - 5;
}

impl ClassicHasher for Poly1305 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Padding
        while input.len() != 16 {
            input.push(0x00)
        }

        let mut coefs = Vec::with_capacity(input.len() / 16);

        // Message is taken 16 bytes at a time.
        for block in input.chunks_exact(16) {
            let mut block = block.to_vec();
            block.push(0x01);
            coefs.push(U192::from_be_slice(&block));
        }

        todo!()
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
mod md5_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Poly1305::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        assert_eq!(
            "d41d8cd98f00b204e9800998ecf8427e",
            hasher.hash_bytes_from_string("").unwrap()
        );
    }
}
