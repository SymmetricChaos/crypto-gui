pub mod aes;
pub mod des;
pub mod elgamal;
pub mod rc4;
pub mod rc5;
pub mod rsa;

use crate::CipherError;
use base64::prelude::*;
use utils::text_functions::hex_to_bytes;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ByteFormat {
    Hex,
    Utf8,
    Base64,
}

impl ByteFormat {
    pub fn text_to_bytes(&self, text: &str) -> Result<Vec<u8>, CipherError> {
        match self {
            ByteFormat::Hex => {
                hex_to_bytes(text).map_err(|_| CipherError::input("invalid hex digits"))
            }
            ByteFormat::Utf8 => Ok(text.as_bytes().to_owned()),
            ByteFormat::Base64 => BASE64_STANDARD
                .decode(text)
                .map_err(|_| CipherError::input("Base64 decoding error")),
        }
    }

    pub fn bytes_to_text(&self, bytes: &[u8]) -> String {
        match self {
            ByteFormat::Hex => bytes.iter().map(|byte| format!("{:02x}", byte)).collect(),
            ByteFormat::Utf8 => String::from_utf8_lossy(&bytes).to_string(),
            ByteFormat::Base64 => BASE64_STANDARD.encode(bytes),
        }
    }
}
