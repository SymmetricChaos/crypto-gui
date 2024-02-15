use super::BinaryToText;
use crate::errors::CodeError;
use crate::traits::Code;
use bimap::BiMap;
use lazy_static::lazy_static;
use utils::byte_formatting::{bytes_to_hex, ByteFormat};
// use std::fs::read;
// use std::path::PathBuf;
use utils::preset_alphabet::Alphabet;
use utils::text_functions::bimap_from_iter;

const MASK: u8 = 0b00111111;
const PAD: u8 = '=' as u8;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum B64Variant {
    Standard,
    UrlSafe,
}

lazy_static! {
    pub static ref B64_MAP: BiMap<u8, u8> = bimap_from_iter(
        Alphabet::Base64
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
    pub static ref B64_URLSAFE_MAP: BiMap<u8, u8> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
}

pub struct Base64 {
    // pub file: Option<PathBuf>,
    pub use_padding: bool,
    pub mode: ByteFormat,
    pub variant: B64Variant,
}

impl Default for Base64 {
    fn default() -> Self {
        Self {
            // file: None,
            use_padding: true,
            mode: ByteFormat::Utf8,
            variant: B64Variant::Standard,
        }
    }
}

impl Base64 {
    pub fn map(&self) -> &BiMap<u8, u8> {
        match self.variant {
            B64Variant::Standard => &B64_MAP,
            B64Variant::UrlSafe => &B64_URLSAFE_MAP,
        }
    }

    pub fn chars_codes(&self) -> impl Iterator<Item = (String, char)> + '_ {
        (0..64u8).map(|x| {
            (
                format!("{:06b}", x),
                *self.map().get_by_left(&x).unwrap() as char,
            )
        })
    }

    // pub fn encode_file(&self) -> Result<String, CodeError> {
    //     if self.file.is_none() {
    //         return Err(CodeError::input("no file stored"));
    //     }
    //     let bytes = &read(self.file.as_ref().unwrap()).unwrap()[..];
    //     self.encode_bytes(bytes)
    // }
}

impl BinaryToText for Base64 {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
        let mut out = Vec::with_capacity((bytes.len() / 3) * 4 + 1);
        let map = self.map();
        let mut buffer = 0_u32;
        let mut bits_in_use = 0;
        let mut bytes = bytes.iter();

        loop {
            // If less than 6 bits are being used get the next byte
            if bits_in_use < 6 {
                match bytes.next() {
                    // If it exists put it into the buffer
                    Some(n) => {
                        buffer = buffer << 8;
                        buffer = buffer ^ (*n as u32);
                        bits_in_use += 8
                    }
                    // Otherwise normal encoding is done
                    None => break,
                };
            }
            // Get the five highest USED bits in the buffer and map them
            let n = ((buffer >> (bits_in_use - 6)) as u8) & MASK;
            out.push(*map.get_by_left(&n).unwrap());
            bits_in_use -= 6;
        }

        if bits_in_use != 0 {
            // If padding is used continue shifting in 0 bytes until we reach 0 bits in use (a multiple of 40)
            // The only differene is that the 00000 word is now PAD instead of A
            if self.use_padding {
                while bits_in_use != 0 {
                    if bits_in_use < 6 {
                        buffer = buffer << 8;
                        bits_in_use += 8;
                    }
                    let n = ((buffer >> (bits_in_use - 6)) as u8) & MASK;
                    if n == 0 {
                        out.push(PAD)
                    } else {
                        out.push(*map.get_by_left(&n).unwrap());
                    }
                    bits_in_use -= 6;
                }
            } else {
                if bits_in_use < 6 {
                    buffer = buffer << 8;
                    bits_in_use += 8;
                }
                let n = ((buffer >> (bits_in_use - 6)) as u8) & MASK;
                if n == 0 {
                    out.push(PAD)
                } else {
                    out.push(*map.get_by_left(&n).unwrap());
                }
            }
        }

        Ok(String::from_utf8(out).unwrap())
    }
}

impl Code for Base64 {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.mode {
            ByteFormat::Hex => self.encode_hex(text),
            ByteFormat::Utf8 => self.encode_utf8(text),
            ByteFormat::Base64 => todo!(),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::with_capacity((text.len() / 4) * 3 + 1);
        let mut buffer = 0_u32;
        let mut bits_in_use = 0;
        let map = self.map();
        // Detect and remove padding then map each character to its bitstring
        let mut bytes = text
            .bytes()
            .take_while(|n| n != &PAD)
            .filter(|b| b.is_ascii_whitespace())
            .map(|n| {
                map.get_by_right(&n)
                    .ok_or_else(|| CodeError::invalid_input_char(n as char))
            });
        loop {
            if bits_in_use < 8 {
                buffer = buffer << 6;
                if let Some(n) = bytes.next() {
                    buffer = buffer ^ (*n? & MASK) as u32;
                    bits_in_use += 6
                } else {
                    break;
                }
            } else {
                let n = (buffer >> (bits_in_use - 8)) as u8;
                out.push(n);
                bits_in_use -= 8;
            }
        }
        match self.mode {
            ByteFormat::Hex => Ok(bytes_to_hex(&out)),
            ByteFormat::Utf8 => String::from_utf8(out).map_err(|e| CodeError::Input(e.to_string())),
            ByteFormat::Base64 => todo!(),
        }
    }
}

#[cfg(test)]
mod base64_tests {
    use super::*;

    const PLAINTEXT0: &'static str = "Many hands make light work.";
    const PLAINTEXT1: &'static str = "Many hands make light work";
    const PLAINTEXT2: &'static str = "Many hands make light woA";

    const CIPHERTEXT0: &'static str = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu";
    const CIPHERTEXT1: &'static str = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcms=";
    const CIPHERTEXT2: &'static str = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvQQ==";

    const CIPHERTEXT0_NOPAD: &'static str = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu";
    const CIPHERTEXT1_NOPAD: &'static str = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcms";
    const CIPHERTEXT2_NOPAD: &'static str = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvQQ";

    #[test]
    fn encode_test() {
        let code = Base64::default();
        assert_eq!(code.encode(PLAINTEXT0).unwrap(), CIPHERTEXT0);
        assert_eq!(code.encode(PLAINTEXT1).unwrap(), CIPHERTEXT1);
        assert_eq!(code.encode(PLAINTEXT2).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn encode_test_nopad() {
        let mut code = Base64::default();
        code.use_padding = false;
        assert_eq!(code.encode(PLAINTEXT0).unwrap(), CIPHERTEXT0_NOPAD);
        assert_eq!(code.encode(PLAINTEXT1).unwrap(), CIPHERTEXT1_NOPAD);
        assert_eq!(code.encode(PLAINTEXT2).unwrap(), CIPHERTEXT2_NOPAD);
    }

    #[test]
    fn decode_test() {
        let code = Base64::default();
        assert_eq!(code.decode(CIPHERTEXT0).unwrap(), PLAINTEXT0);
        assert_eq!(code.decode(CIPHERTEXT1).unwrap(), PLAINTEXT1);
        assert_eq!(code.decode(CIPHERTEXT2).unwrap(), PLAINTEXT2);
    }

    #[test]
    fn decode_test_nopad() {
        let code = Base64::default();
        assert_eq!(code.decode(CIPHERTEXT0_NOPAD).unwrap(), PLAINTEXT0);
        assert_eq!(code.decode(CIPHERTEXT1_NOPAD).unwrap(), PLAINTEXT1);
        assert_eq!(code.decode(CIPHERTEXT2_NOPAD).unwrap(), PLAINTEXT2);
    }
}
