use utils::text_functions::hex_to_bytes;

use crate::{Cipher, CipherError};

use super::{InputFormat, OutputFormat};

pub struct Aes {
    pub output_format: OutputFormat,
    pub input_format: InputFormat,
}

impl Default for Aes {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Hex,
            input_format: InputFormat::Hex,
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
        let mut bytes = match self.input_format {
            InputFormat::Hex => {
                hex_to_bytes(text).map_err(|_| CipherError::input("not valid hexcode"))?
            }
            InputFormat::Utf8 => text.bytes().collect(),
        };
        let out = self.encrypt_bytes(&mut bytes)?;
        match self.output_format {
            OutputFormat::Hex => Ok(out.iter().map(|byte| format!("{:02x}", byte)).collect()),
            OutputFormat::Utf8 => Ok(String::from_utf8_lossy(&out).to_string()),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}

#[cfg(test)]
mod aes_tests {

    use super::*;
}
