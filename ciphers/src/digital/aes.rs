use utils::byte_formatting::ByteFormat;

use crate::{Cipher, CipherError};

pub struct Aes {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
}

impl Default for Aes {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
        }
    }
}

impl Aes {
    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "input length must be a multiple of 64 bits",
            ));
        };

        let mut out = Vec::with_capacity(bytes.len());

        Ok(out)
    }

    pub fn decrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "input length must be a multiple of 64 bits",
            ));
        };

        let mut out = Vec::with_capacity(bytes.len());

        Ok(out)
    }
}

impl Cipher for Aes {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.encrypt_bytes(&mut bytes)?;
        Ok(self.output_format.byte_slice_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.decrypt_bytes(&mut bytes)?;
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod aes_tests {

    use super::*;
}
