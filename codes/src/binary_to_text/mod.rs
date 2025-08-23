pub mod ascii85;
pub mod base32;
pub mod base64;
pub mod basex;
pub mod bytewords;
pub mod hexadecimal;
pub mod intel_hex;
pub mod numeric;
pub mod pgp_words;
pub mod quoted_printable;
pub mod skey;

use utils::{
    byte_formatting::{hex_to_bytes, ByteFormat},
    errors::GeneralError,
};

pub trait BinaryToText {
    // Encode some literal bytes
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, GeneralError>;

    // Take a hex string, convert it to bytes, and then encode it
    fn encode_hex(&self, hex: &str) -> Result<String, GeneralError> {
        let bytes = hex_to_bytes(hex).map_err(|_| GeneralError::input("not valid hexcode"))?;
        self.encode_bytes(&bytes)
    }

    fn encode_base64(&self, text: &str) -> Result<String, GeneralError> {
        let bytes = ByteFormat::Base64
            .text_to_bytes(text)
            .map_err(|_| GeneralError::input("not valid Base64"))?;
        self.encode_bytes(&bytes)
    }

    // Encode some literal UTF-8 text
    fn encode_utf8(&self, text: &str) -> Result<String, GeneralError> {
        self.encode_bytes(text.as_bytes())
    }

    // Encode some string of characters representing bits
    fn encode_bits(&self, text: &str) -> Result<String, GeneralError> {
        let bytes = ByteFormat::Binary
            .text_to_bytes(text)
            .map_err(|_| GeneralError::input("not valid binary"))?;
        self.encode_bytes(&bytes)
    }
}
