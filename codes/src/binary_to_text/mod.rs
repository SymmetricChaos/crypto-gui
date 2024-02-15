pub mod ascii85;
pub mod base16;
pub mod base32;
pub mod base64;
pub mod basex;
pub mod numeric;
pub mod pgp_words;
pub mod skey;

use std::{fs::read, path::PathBuf};

use utils::byte_formatting::hex_to_bytes;

use crate::errors::CodeError;

pub trait BinaryToText {
    // Encode some literal bytes
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError>;

    // Take a hex string, convert it to bytes, and then encode it
    fn encode_hex(&self, hex: &str) -> Result<String, CodeError> {
        let bytes = hex_to_bytes(hex).map_err(|_| CodeError::input("not valid hexcode"))?;
        self.encode_bytes(&bytes)
    }

    // Encode some literal UTF-8 text
    fn encode_utf8(&self, text: &str) -> Result<String, CodeError> {
        self.encode_bytes(text.as_bytes())
    }

    fn encode_file(&self, path: Option<PathBuf>) -> Result<String, CodeError> {
        if path.is_none() {
            return Err(CodeError::input("no file stored"));
        }
        let bytes = &read(path.as_ref().unwrap()).unwrap()[..];
        self.encode_bytes(bytes)
    }
}
