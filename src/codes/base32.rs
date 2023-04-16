use super::Code;
use crate::{errors::Error, text_aux::text_functions::bimap_from_iter};
use bimap::BiMap;
use lazy_static::lazy_static;
use std::{fs::read, path::PathBuf};

// Mask to set top three bits to zero
const MASK: u8 = 0b00011111;

const PAD: u8 = '=' as u8;

const BASE32_ALPHA: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const WORD_SAFE_BASE32: &'static str = "23456789CFGHJMPQRVWXcfghjmpqrvwx";

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
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum B32Variant {
    Rfc4648,
    WordSafe,
}

// Make it possible to encode an aribtrary file
pub struct Base32 {
    pub file: Option<PathBuf>,
    pub variant: B32Variant,
    pub use_padding: bool,
}

impl Default for Base32 {
    fn default() -> Self {
        Self {
            file: None,
            variant: B32Variant::Rfc4648,
            use_padding: true,
        }
    }
}

impl Base32 {
    pub fn map(&self) -> &BiMap<u8, u8> {
        match self.variant {
            B32Variant::Rfc4648 => &B32_MAP,
            B32Variant::WordSafe => &B32_WORD_SAFE_MAP,
        }
    }

    pub fn encode_file(&self) -> Result<String, Error> {
        if self.file.is_none() {
            return Err(Error::input("no file stored"));
        }
        let bytes = &read(self.file.as_ref().unwrap()).unwrap()[..];

        let encoded = self.encode_byte_stream(bytes);
        Ok(String::from_utf8(encoded).unwrap())
    }

    fn encode_byte_stream(&self, input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity((input.len() / 5) * 8);
        let map = self.map();
        let mut buffer = 0_u32;
        let mut bits_in_use = 0;
        let mut bytes = input.iter();

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

        // If padding is used continue shifting in 0 bytes until we reach 0 bits in use (a multiple of 40)
        // The only differene is that the 00000 word is now PAD instead of A
        if bits_in_use != 0 {
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

        out
    }

    fn decode_byte_stream(&self, input: &[u8]) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity((input.len() / 8) * 5);
        let needed_padding = 8 - (input.len() % 8);
        let mut buffer = 0_u32;
        let mut bits_in_use = 0;
        let mut bytes = input.iter();
        loop {
            if bits_in_use < 5 {
                buffer = buffer << 5;
                match bytes.next() {
                    Some(n) => {
                        buffer = buffer ^ (*n & MASK) as u32;
                        bits_in_use += 5
                    }
                    None => break,
                };
            }
            let n = ((buffer >> (bits_in_use - 5)) as u8) & MASK;
            dbg!(n as char);
            out.push(*self.decode_byte(&n).unwrap());
            bits_in_use -= 5;
        }
        Ok(out)
    }

    pub fn decode_bytes(&self, input: &[u8]) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity((input.len() / 8) * 5);
        let extra_bytes = input.len() % 8;

        let chunks = input.chunks_exact(8);
        let padding_len = {
            match input.iter().filter(|b| b == &&PAD).count() {
                0 => 0,
                1 => 1,
                3 => 2,
                4 => 3,
                6 => 4,
                n => {
                    return Err(Error::Input(format!(
                        "valid Base32 cannot have {} padding bytes",
                        n
                    )))
                }
            }
        };
        for chunk in chunks {
            // Turn the eight bytes into five bytes
            let s1 = *self.decode_byte(&chunk[0])?;
            let s2 = *self.decode_byte(&chunk[1])?;
            let s3 = *self.decode_byte(&chunk[2])?;
            let s4 = *self.decode_byte(&chunk[3])?;
            let s5 = *self.decode_byte(&chunk[4])?;
            let s6 = *self.decode_byte(&chunk[5])?;
            let s7 = *self.decode_byte(&chunk[6])?;
            let s8 = *self.decode_byte(&chunk[7])?;

            // BITSHIFT HECK (no masks needed as top 3 bits always zero)
            let o1 = (s1 << 3) ^ (s2 >> 2);
            let o2 = (s2 << 6) ^ (s3 << 1) ^ (s4 >> 4);
            let o3 = (s4 << 4) ^ (s5 >> 1);
            let o4 = (s5 << 7) ^ (s6 << 2) ^ (s7 >> 3);
            let o5 = (s7 << 5) ^ s8;
            out.push(o1);
            out.push(o2);
            out.push(o3);
            out.push(o4);
            out.push(o5);
        }

        for _ in 0..padding_len {
            out.pop();
        }
        Ok(out)
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (String, char)> + '_ {
        (0..32u8).map(|x| {
            (
                format!("{:05b}", x),
                *self.map().get_by_left(&x).unwrap() as char,
            )
        })
    }

    fn decode_byte(&self, n: &u8) -> Result<&u8, Error> {
        if n == &PAD {
            Ok(&0)
        } else {
            self.map()
                .get_by_right(&n)
                .ok_or_else(|| Error::invalid_input_char(*n as char))
        }
    }
}

impl Code for Base32 {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let b = self.encode_byte_stream(text.as_bytes());
        Ok(String::from_utf8(b).unwrap())
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let b = self.decode_bytes(text.as_bytes())?;
        Ok(String::from_utf8(b).unwrap())
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod base64_tests {
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
}
