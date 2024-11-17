use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::CodeError, traits::Code};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeEncoding {
    Utf8,
    Utf16,
    Utf32,
}

pub struct Unicode {
    pub encoding: UnicodeEncoding,
    pub mode: ByteFormat,
}

impl Unicode {
    fn utf8_encode(&self, text: &str) -> Result<String, CodeError> {
        Ok(self.mode.byte_iter_to_text(text.bytes()))
    }

    fn utf16_encode(&self, text: &str) -> Result<String, CodeError> {
        Ok(self
            .mode
            .u16_slice_to_text_be(text.encode_utf16().collect_vec()))
    }

    fn utf32_encode(&self, text: &str) -> Result<String, CodeError> {
        Ok(self
            .mode
            .u32_slice_to_text_be(text.chars().map(|c| u32::from(c)).collect_vec()))
    }

    fn utf8_decode(&self, text: &str) -> Result<String, CodeError> {
        let v = self
            .mode
            .text_to_bytes(text)
            .map_err(|e| CodeError::Input(e.to_string()))?;

        String::from_utf8(v).map_err(|e| CodeError::Input(e.to_string()))
    }

    fn utf16_decode(&self, text: &str) -> Result<String, CodeError> {
        let v = self
            .mode
            .text_to_u16_be(text)
            .map_err(|e| CodeError::Input(e.to_string()))?;

        String::from_utf16(&v).map_err(|e| CodeError::Input(e.to_string()))
    }

    fn utf32_decode(&self, text: &str) -> Result<String, CodeError> {
        Ok(self
            .mode
            .text_to_u32_be(text)
            .map_err(|e| CodeError::Input(e.to_string()))?
            .into_iter()
            .map(|n| char::from_u32(n).unwrap_or('�'))
            .collect())
    }
}

impl Default for Unicode {
    fn default() -> Self {
        Unicode {
            encoding: UnicodeEncoding::Utf8,
            mode: ByteFormat::Binary,
        }
    }
}

impl Code for Unicode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.encoding {
            UnicodeEncoding::Utf8 => self.utf8_encode(text),
            UnicodeEncoding::Utf16 => self.utf16_encode(text),
            UnicodeEncoding::Utf32 => self.utf32_encode(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        match self.encoding {
            UnicodeEncoding::Utf8 => self.utf8_decode(text),
            UnicodeEncoding::Utf16 => self.utf16_decode(text),
            UnicodeEncoding::Utf32 => self.utf32_decode(text),
        }
    }
}

#[cfg(test)]
mod unicode_tests {
    use super::*;

    const PLAINTEXT: &'static str = "The 素早い καφέ 🦊 ｊｕｍｐｓ over the lazy 🐶.";

    #[test]
    fn encrypt_decrypt() {
        let mut code = Unicode::default();

        for encoding in [
            UnicodeEncoding::Utf8,
            UnicodeEncoding::Utf16,
            UnicodeEncoding::Utf32,
        ] {
            code.encoding = encoding;

            for mode in [ByteFormat::Binary, ByteFormat::Hex, ByteFormat::Base64] {
                code.mode = mode;
                let encoded = code
                    .encode(PLAINTEXT)
                    .expect(&format!("encoding {:?} {:?} CodeError", encoding, mode));
                let decoded = code
                    .decode(&encoded)
                    .expect(&format!("decoding{:?} {:?} CodeError", encoding, mode));
                if decoded != PLAINTEXT {
                    panic!(
                        "decoded {:?} {:?} not equivalent to plaintext",
                        encoding, mode
                    )
                }
            }
        }
    }
}
