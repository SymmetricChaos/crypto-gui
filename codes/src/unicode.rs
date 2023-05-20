use itertools::Itertools;

use crate::{
    errors::CodeError,
    text_utils::functions::{u16_to_string, u32_to_string, u8_to_string, NumRep},
    traits::Code,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeEncoding {
    Utf8,
    Utf16,
    Utf32,
}

pub struct Unicode {
    pub encoding: UnicodeEncoding,
    pub mode: NumRep,
}

impl Unicode {
    fn utf8_encode(&self, text: &str) -> Result<String, CodeError> {
        Ok(text.bytes().map(|n| u8_to_string(n, self.mode)).join(" "))
    }

    fn utf16_encode(&self, text: &str) -> Result<String, CodeError> {
        Ok(text
            .encode_utf16()
            .map(|n| u16_to_string(n, self.mode))
            .join(" "))
    }

    fn utf32_encode(&self, text: &str) -> Result<String, CodeError> {
        Ok(text
            .chars()
            .map(|c| u32::from(c))
            .map(|n| u32_to_string(n, self.mode))
            .join(" "))
    }

    fn utf8_decode(&self, text: &str) -> Result<String, CodeError> {
        let chunks = text.split(" ");
        let radix = self.mode.radix();
        let mut vec = Vec::with_capacity(chunks.clone().count());

        for chunk in chunks {
            match u8::from_str_radix(chunk, radix) {
                Ok(n) => vec.push(n),
                Err(_) => {
                    return Err(CodeError::Input(format!(
                        "CodeError decoding UTF-8, unable to parse string: {}",
                        chunk
                    )))
                }
            }
        }

        String::from_utf8(vec).map_err(|e| CodeError::Input(e.to_string()))
    }

    fn utf16_decode(&self, text: &str) -> Result<String, CodeError> {
        let chunks = text.split(" ");
        let radix = self.mode.radix();
        let mut vec = Vec::with_capacity(chunks.clone().count());

        for chunk in chunks {
            match u16::from_str_radix(chunk, radix) {
                Ok(n) => vec.push(n),
                Err(_) => {
                    return Err(CodeError::Input(format!(
                        "CodeError decoding UTF-16, unable to parse string: {}",
                        chunk
                    )))
                }
            }
        }

        String::from_utf16(&vec).map_err(|e| CodeError::Input(e.to_string()))
    }

    fn utf32_decode(&self, text: &str) -> Result<String, CodeError> {
        let chunks = text.split(" ");

        let mut out = String::with_capacity(chunks.clone().count());

        let radix = self.mode.radix();

        for chunk in chunks {
            match u32::from_str_radix(chunk, radix) {
                Ok(n) => {
                    match char::from_u32(n) {
                        Some(c) => out.push(c),
                        None => {
                            return Err(CodeError::Input(format!(
                                "UTF-32 decoding CodeError, invalid input string: {}",
                                chunk
                            )))
                        }
                    };
                }
                Err(_) => {
                    return Err(CodeError::Input(format!(
                        "CodeError decoding UTF-32 unable to parse string: {}",
                        chunk
                    )))
                }
            }
        }

        Ok(out)
    }
}

impl Default for Unicode {
    fn default() -> Self {
        Unicode {
            encoding: UnicodeEncoding::Utf8,
            mode: NumRep::Binary,
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

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
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

            for mode in [
                NumRep::Binary,
                NumRep::Octal,
                NumRep::Decimal,
                NumRep::HexLower,
            ] {
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
