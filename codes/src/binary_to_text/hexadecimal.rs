use super::BinaryToText;
use crate::traits::Code;
use itertools::Itertools;
use utils::{byte_formatting::ByteFormat, errors::GeneralError};

crate::lazy_regex!(IS_BASE16, r"^([0-9A-F][0-9A-F])*$");

pub struct Hexadecimal {
    pub mode: ByteFormat,
    pub upper: bool,
}

impl Default for Hexadecimal {
    fn default() -> Self {
        Self {
            mode: ByteFormat::Utf8,
            upper: true,
        }
    }
}

impl BinaryToText for Hexadecimal {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, GeneralError> {
        if self.upper {
            Ok(bytes.iter().map(|b| format!("{b:02X}")).collect())
        } else {
            Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
        }
    }
}

impl Code for Hexadecimal {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        match self.mode {
            ByteFormat::Hex => self.encode_hex(text),
            ByteFormat::Utf8 => self.encode_utf8(text),
            ByteFormat::Base64 => self.encode_base64(text),
            ByteFormat::Binary => self.encode_bits(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        if !IS_BASE16.is_match(&text.to_uppercase()) {
            return Err(GeneralError::input(
                "provided text is not valid Hexadecimal",
            ));
        }

        let out = text
            .to_uppercase()
            .chars()
            .chunks(2)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .map(|s| u8::from_str_radix(&s, 16).expect("invalid codes are caught by regex first"))
            .collect_vec();

        Ok(self.mode.byte_slice_to_text(out))
    }
}

#[cfg(test)]
mod base32_tests {
    use super::*;

    const PLAINTEXT0: &'static str = "Manyh";
    const PLAINTEXT1: &'static str = "Many";
    const PLAINTEXT2: &'static str = "Man";
    const PLAINTEXT3: &'static str = "Ma";
    const PLAINTEXT4: &'static str = "M";

    const CODETEXT0: &'static str = "4D616E7968";
    const CODETEXT1: &'static str = "4D616E79";
    const CODETEXT2: &'static str = "4D616E";
    const CODETEXT3: &'static str = "4D61";
    const CODETEXT4: &'static str = "4D";

    #[test]
    fn encode_test() {
        let code = Hexadecimal::default();
        assert_eq!(code.encode(PLAINTEXT0).unwrap(), CODETEXT0);
        assert_eq!(code.encode(PLAINTEXT1).unwrap(), CODETEXT1);
        assert_eq!(code.encode(PLAINTEXT2).unwrap(), CODETEXT2);
        assert_eq!(code.encode(PLAINTEXT3).unwrap(), CODETEXT3);
        assert_eq!(code.encode(PLAINTEXT4).unwrap(), CODETEXT4);
    }

    #[test]
    fn decode_test() {
        let code = Hexadecimal::default();
        assert_eq!(code.decode(CODETEXT0).unwrap(), PLAINTEXT0);
        assert_eq!(code.decode(CODETEXT1).unwrap(), PLAINTEXT1);
        assert_eq!(code.decode(CODETEXT2).unwrap(), PLAINTEXT2);
        assert_eq!(code.decode(CODETEXT3).unwrap(), PLAINTEXT3);
        assert_eq!(code.decode(CODETEXT4).unwrap(), PLAINTEXT4);
    }
}
