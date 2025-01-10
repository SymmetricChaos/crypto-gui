use super::BinaryToText;
use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

fn push_quote_byte(byte: u8, string: &mut String) {
    string.push_str(&format!("={:02X}", byte))
}

pub struct QuotedPrintable {
    pub mode: ByteFormat,
}

impl Default for QuotedPrintable {
    fn default() -> Self {
        Self {
            mode: ByteFormat::Utf8,
        }
    }
}

impl BinaryToText for QuotedPrintable {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
        let mut out = String::new();
        for byte in bytes {
            match byte {
                9 => out.push('\t'),
                32..=60 => out.push(*byte as char),
                62..=126 => out.push(*byte as char),
                _ => push_quote_byte(*byte, &mut out),
            }
        }
        Ok(out)
    }
}

impl Code for QuotedPrintable {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.mode {
            ByteFormat::Hex => self.encode_hex(text),
            ByteFormat::Utf8 => self.encode_utf8(text),
            ByteFormat::Base64 => self.encode_base64(text),
            ByteFormat::Binary => self.encode_bits(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod quoted_printable_tests {
    use super::*;

    // the thin space character has to be escaped for some reason
    const TEXT: &str = "\t—\u{2009}Antoine de Saint-Exupéry, Citadelle (1948)";

    #[test]
    fn encode() {
        let code = QuotedPrintable::default();
        assert_eq!(
            "\t=E2=80=94=E2=80=89Antoine de Saint-Exup=C3=A9ry, Citadelle (1948)",
            code.encode_bytes(&ByteFormat::Utf8.text_to_bytes(TEXT).unwrap())
                .unwrap()
        );
    }
}
