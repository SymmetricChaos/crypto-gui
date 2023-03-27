use itertools::Itertools;

use crate::{
    errors::Error,
    text_aux::bytes_as_text::{u8_to_string, ByteRep},
};

use super::Code;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeEncoding {
    Utf8,
    Utf16,
    Utf32,
}

pub struct Unicode {
    pub encoding: UnicodeEncoding,
    pub mode: ByteRep,
}

impl Unicode {
    fn utf8_encode(&self, text: &str) -> Result<String, Error> {
        Ok(text.bytes().map(|n| u8_to_string(n, self.mode)).join(" "))
    }

    fn utf16_encode(&self, text: &str) -> Result<String, Error> {
        let chunks = text.encode_utf16();
        let s = match self.mode {
            ByteRep::Binary => chunks.map(|n| (format!("{:016b}", n))).join(" "),
            ByteRep::Octal => chunks.map(|n| (format!("{:08o}", n))).join(" "),
            ByteRep::Decimal => chunks.map(|n| (format!("{}", n))).join(" "),
            ByteRep::Hex => chunks.map(|n| (format!("{:04x}", n))).join(" "),
            ByteRep::HexCap => chunks.map(|n| (format!("{:04X}", n))).join(" "),
        };
        Ok(s)
    }

    fn utf32_encode(&self, text: &str) -> Result<String, Error> {
        let chunks = text.chars().map(|c| u32::from(c));
        let s = match self.mode {
            ByteRep::Binary => chunks.map(|n| (format!("{:032b}", n))).join(" "),
            ByteRep::Octal => chunks.map(|n| (format!("{:016o}", n))).join(" "),
            ByteRep::Decimal => chunks.map(|n| (format!("{}", n))).join(" "),
            ByteRep::Hex => chunks.map(|n| (format!("{:08x}", n))).join(" "),
            ByteRep::HexCap => chunks.map(|n| (format!("{:08X}", n))).join(" "),
        };
        Ok(s)
    }

    fn utf8_decode(&self, text: &str) -> Result<String, Error> {
        let chunks = text.split(" ");
        let radix = self.mode.radix();
        let mut vec = Vec::with_capacity(chunks.clone().count());

        for chunk in chunks {
            match u8::from_str_radix(chunk, radix) {
                Ok(n) => vec.push(n),
                Err(_) => {
                    return Err(Error::Input(format!(
                        "error decoding UTF-8, unable to parse string: {}",
                        chunk
                    )))
                }
            }
        }

        String::from_utf8(vec).map_err(|e| Error::Input(e.to_string()))
    }

    fn utf16_decode(&self, text: &str) -> Result<String, Error> {
        let chunks = text.split(" ");
        let radix = self.mode.radix();
        let mut vec = Vec::with_capacity(chunks.clone().count());

        for chunk in chunks {
            match u16::from_str_radix(chunk, radix) {
                Ok(n) => vec.push(n),
                Err(_) => {
                    return Err(Error::Input(format!(
                        "error decoding UTF-16, unable to parse string: {}",
                        chunk
                    )))
                }
            }
        }

        String::from_utf16(&vec).map_err(|e| Error::Input(e.to_string()))
    }

    fn utf32_decode(&self, text: &str) -> Result<String, Error> {
        let chunks = text.split(" ");

        let mut out = String::with_capacity(chunks.clone().count());

        let radix = self.mode.radix();

        for chunk in chunks {
            match u32::from_str_radix(chunk, radix) {
                Ok(n) => {
                    match char::from_u32(n) {
                        Some(c) => out.push(c),
                        None => {
                            return Err(Error::Input(format!(
                                "UTF-32 decoding error, invalid input string: {}",
                                chunk
                            )))
                        }
                    };
                }
                Err(_) => {
                    return Err(Error::Input(format!(
                        "error decoding UTF-32 unable to parse string: {}",
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
            mode: ByteRep::Binary,
        }
    }
}

impl Code for Unicode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        match self.encoding {
            UnicodeEncoding::Utf8 => self.utf8_encode(text),
            UnicodeEncoding::Utf16 => self.utf16_encode(text),
            UnicodeEncoding::Utf32 => self.utf32_encode(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
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
    const CIPHERTEXT_UTF8: &'static str = "010101000110100001100101001000001110011110110100101000001110011010010111101010011110001110000001100001000010000011001110101110101100111010110001110011111000011011001110101011010010000011110000100111111010011010001010001000001110111110111101100010101110111110111101100101011110111110111101100011011110111110111101100100001110111110111101100100110010000001101111011101100110010101110010001000000111010001101000011001010010000001101100011000010111101001111001001000001111000010011111100100001011011000101110";

    #[test]
    fn encrypt_utf8_bits() {
        let code = Unicode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_UTF8);
    }

    #[test]
    fn encrypt_utf8_dec() {
        let mut code = Unicode::default();
        code.mode = ByteRep::Decimal;
        println!("{}", code.encode(PLAINTEXT).unwrap());
    }

    #[test]
    fn encrypt_utf8_hex() {
        let mut code = Unicode::default();
        code.mode = ByteRep::Hex;
        println!("{}", code.encode(PLAINTEXT).unwrap());
    }

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
                ByteRep::Binary,
                ByteRep::Octal,
                ByteRep::Decimal,
                ByteRep::Hex,
            ] {
                code.mode = mode;
                let encoded = code
                    .encode(PLAINTEXT)
                    .expect(&format!("encoding {:?} {:?} error", encoding, mode));
                let decoded = code
                    .decode(&encoded)
                    .expect(&format!("decoding{:?} {:?} error", encoding, mode));
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
