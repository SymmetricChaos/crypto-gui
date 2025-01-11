use super::BinaryToText;
use crate::{errors::CodeError, traits::Code};
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
        let mut row = String::with_capacity(76);
        for byte in bytes {
            match byte {
                9 => row.push('\t'),
                32..=60 => row.push(*byte as char),
                62..=126 => row.push(*byte as char),
                _ => {
                    if row.len() > 72 {
                        out.extend(row.drain(..));
                        out.push_str("=\r\n");
                    }
                    push_quote_byte(*byte, &mut row);
                }
            }
            if row.len() >= 75 {
                out.extend(row.drain(..75));
                out.push_str("=\r\n");
            }
        }
        out.push_str(&row);
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
        for line in text.split("=\r\n") {}

        todo!()
    }
}

#[cfg(test)]
mod quoted_printable_tests {
    use super::*;

    // the thin space character has to be escaped for some reason
    const TEXT: &str = "\t—\u{2009}Antoine de Saint-Exupéry, Citadelle (1948)";
    const LONG_TEXT: &str = "J'interdis aux marchands de vanter trop leurs marchandises. Car ils se font vite pédagogues et t'enseignent comme but ce qui n'est par essence qu'un moyen, et te trompant ainsi sur la route à suivre les voilà bientôt qui te dégradent, car si leur musique est vulgaire ils te fabriquent pour te la vendre une âme vulgaire.";

    #[test]
    fn encode() {
        let code = QuotedPrintable::default();
        assert_eq!(
            "\t=E2=80=94=E2=80=89Antoine de Saint-Exup=C3=A9ry, Citadelle (1948)",
            code.encode_bytes(&ByteFormat::Utf8.text_to_bytes(TEXT).unwrap())
                .unwrap()
        );
    }

    #[test]
    fn encode_mulitline() {
        let code = QuotedPrintable::default();
        assert_eq!(
            "J'interdis aux marchands de vanter trop leurs marchandises. Car ils se font=\r
 vite p=C3=A9dagogues et t'enseignent comme but ce qui n'est par essence qu=\r
'un moyen, et te trompant ainsi sur la route =C3=A0 suivre les voil=C3=A0 b=\r
ient=C3=B4t qui te d=C3=A9gradent, car si leur musique est vulgaire ils te =\r
fabriquent pour te la vendre une =C3=A2me vulgaire.",
            code.encode_bytes(&ByteFormat::Utf8.text_to_bytes(LONG_TEXT).unwrap())
                .unwrap()
        );
    }
}
