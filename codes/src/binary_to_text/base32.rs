use bimap::BiMap;
use lazy_static::lazy_static;
use utils::{byte_formatting::ByteFormat, text_functions::bimap_from_iter};

use crate::{errors::CodeError, traits::Code};

use super::BinaryToText;

// Mask to set top three bits to zero
const MASK: u8 = 0b00011111;
const PAD: u8 = '=' as u8;

const BASE32_ALPHA: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const WORD_SAFE_BASE32: &'static str = "23456789CFGHJMPQRVWXcfghjmpqrvwx";
const BASE32_HEX: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUV";

lazy_static! {
    pub static ref B32_MAP: BiMap<u8, u8> = bimap_from_iter(
        BASE32_ALPHA
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
    pub static ref B32_WORD_SAFE_MAP: BiMap<u8, u8> = bimap_from_iter(
        WORD_SAFE_BASE32
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
    pub static ref B32_HEX: BiMap<u8, u8> = bimap_from_iter(
        BASE32_HEX
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum B32Variant {
    Rfc4648,
    WordSafe,
    ExtendedHex,
}

// Make it possible to encode an aribtrary file
pub struct Base32 {
    pub variant: B32Variant,
    pub mode: ByteFormat,
    pub use_padding: bool,
}

impl Default for Base32 {
    fn default() -> Self {
        Self {
            variant: B32Variant::Rfc4648,
            mode: ByteFormat::Utf8,
            use_padding: true,
        }
    }
}

impl Base32 {
    pub fn map(&self) -> &BiMap<u8, u8> {
        match self.variant {
            B32Variant::Rfc4648 => &B32_MAP,
            B32Variant::WordSafe => &B32_WORD_SAFE_MAP,
            B32Variant::ExtendedHex => &B32_HEX,
        }
    }

    pub fn chars_codes(&self) -> impl Iterator<Item = (String, char)> + '_ {
        (0..32u8).map(|x| {
            (
                format!("{:05b}", x),
                *self.map().get_by_left(&x).unwrap() as char,
            )
        })
    }
}

impl BinaryToText for Base32 {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
        let mut out = Vec::with_capacity((bytes.len() / 5) * 8);
        let map = self.map();
        let mut buffer = 0_u32;
        let mut bits_in_use = 0;
        let mut bytes = bytes.iter();

        loop {
            // If less than 5 bits are bring used get the next byte
            if bits_in_use < 5 {
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
            // Get the five highest USED bites in the buffer and map them
            let n = ((buffer >> (bits_in_use - 5)) as u8) & MASK;
            out.push(*map.get_by_left(&n).unwrap());
            bits_in_use -= 5;
        }

        if bits_in_use != 0 {
            // If padding is used continue shifting in 0 bytes until we reach 0 bits in use (a multiple of 40)
            // The only differene is that the 00000 word is now PAD instead of A
            if self.use_padding {
                while bits_in_use != 0 {
                    if bits_in_use < 5 {
                        buffer = buffer << 8;
                        bits_in_use += 8;
                    }
                    let n = ((buffer >> (bits_in_use - 5)) as u8) & MASK;
                    if n == 0 {
                        out.push(PAD)
                    } else {
                        out.push(*map.get_by_left(&n).unwrap());
                    }
                    bits_in_use -= 5;
                }
            } else {
                if bits_in_use < 5 {
                    buffer = buffer << 8;
                    bits_in_use += 8;
                }
                let n = ((buffer >> (bits_in_use - 5)) as u8) & MASK;
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

impl Code for Base32 {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.mode {
            ByteFormat::Hex => self.encode_hex(text),
            ByteFormat::Utf8 => self.encode_utf8(text),
            ByteFormat::Base64 => self.encode_base64(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::with_capacity((text.len() / 8) * 5);
        let mut buffer = 0_u32;
        let mut bits_in_use = 0;
        let map = self.map();
        // Detect and remove padding then map each character to its bitstring
        let mut bytes = text.bytes().take_while(|n| n != &PAD).map(|n| {
            map.get_by_right(&n)
                .ok_or_else(|| CodeError::invalid_input_char(n as char))
        });
        loop {
            if bits_in_use < 8 {
                buffer = buffer << 5;
                if let Some(n) = bytes.next() {
                    buffer = buffer ^ (*n? & MASK) as u32;
                    bits_in_use += 5
                } else {
                    break;
                }
            } else {
                let n = (buffer >> (bits_in_use - 8)) as u8;
                out.push(n);
                bits_in_use -= 8;
            }
        }
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

    const CIPHERTEXT0: &'static str = "JVQW46LI";
    const CIPHERTEXT1: &'static str = "JVQW46I=";
    const CIPHERTEXT2: &'static str = "JVQW4===";
    const CIPHERTEXT3: &'static str = "JVQQ====";
    const CIPHERTEXT4: &'static str = "JU======";

    const CIPHERTEXT0_NOPAD: &'static str = "JVQW46LI";
    const CIPHERTEXT1_NOPAD: &'static str = "JVQW46I";
    const CIPHERTEXT2_NOPAD: &'static str = "JVQW4";
    const CIPHERTEXT3_NOPAD: &'static str = "JVQQ";
    const CIPHERTEXT4_NOPAD: &'static str = "JU";

    #[test]
    fn encode_test() {
        let code = Base32::default();
        assert_eq!(code.encode(PLAINTEXT0).unwrap(), CIPHERTEXT0);
        assert_eq!(code.encode(PLAINTEXT1).unwrap(), CIPHERTEXT1);
        assert_eq!(code.encode(PLAINTEXT2).unwrap(), CIPHERTEXT2);
        assert_eq!(code.encode(PLAINTEXT3).unwrap(), CIPHERTEXT3);
        assert_eq!(code.encode(PLAINTEXT4).unwrap(), CIPHERTEXT4);
    }

    #[test]
    fn encode_test_nopad() {
        let mut code = Base32::default();
        code.use_padding = false;
        assert_eq!(code.encode(PLAINTEXT0).unwrap(), CIPHERTEXT0_NOPAD);
        assert_eq!(code.encode(PLAINTEXT1).unwrap(), CIPHERTEXT1_NOPAD);
        assert_eq!(code.encode(PLAINTEXT2).unwrap(), CIPHERTEXT2_NOPAD);
        assert_eq!(code.encode(PLAINTEXT3).unwrap(), CIPHERTEXT3_NOPAD);
        assert_eq!(code.encode(PLAINTEXT4).unwrap(), CIPHERTEXT4_NOPAD);
    }

    #[test]
    fn decode_test() {
        let code = Base32::default();
        assert_eq!(code.decode(CIPHERTEXT0).unwrap(), PLAINTEXT0);
        assert_eq!(code.decode(CIPHERTEXT1).unwrap(), PLAINTEXT1);
        assert_eq!(code.decode(CIPHERTEXT2).unwrap(), PLAINTEXT2);
        assert_eq!(code.decode(CIPHERTEXT3).unwrap(), PLAINTEXT3);
        assert_eq!(code.decode(CIPHERTEXT4).unwrap(), PLAINTEXT4);
    }

    #[test]
    fn decode_test_nopad() {
        let mut code = Base32::default();
        code.use_padding = false;
        assert_eq!(code.decode(CIPHERTEXT0_NOPAD).unwrap(), PLAINTEXT0);
        assert_eq!(code.decode(CIPHERTEXT1_NOPAD).unwrap(), PLAINTEXT1);
        assert_eq!(code.decode(CIPHERTEXT2_NOPAD).unwrap(), PLAINTEXT2);
        assert_eq!(code.decode(CIPHERTEXT3_NOPAD).unwrap(), PLAINTEXT3);
        assert_eq!(code.decode(CIPHERTEXT4_NOPAD).unwrap(), PLAINTEXT4);
    }
}
