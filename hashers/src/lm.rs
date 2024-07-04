use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

const WORD: &'static str = "KGS!@#$%";

pub struct Lm {
    pub output_format: ByteFormat,
}

impl Default for Lm {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for Lm {
    /// This should not be called directly as LM is not meant to encrypt arbitrary bytes
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        if !text.is_ascii() {
            return Err(HasherError::general(
                "LM hash accepts only ASCII characters",
            ));
        }

        if text.chars().count() > 14 {
            return Err(HasherError::general(
                "LM hash cannot accept a password longer than 14 characters",
            ));
        };

        let mut input = text.to_uppercase();
        while input.len() < 14 {
            input.push('\0')
        }

        let mut bytes = ByteFormat::Utf8
            .text_to_bytes(&input)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}
