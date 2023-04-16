use super::Code;
use crate::{
    errors::Error,
    text_aux::{text_functions::bimap_from_iter, PresetAlphabet},
};
use bimap::BiMap;
use lazy_static::lazy_static;
use std::{fs::read, path::PathBuf};

lazy_static! {
    pub static ref B64_MAP: BiMap<u8, u8> = bimap_from_iter(
        PresetAlphabet::Base64
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
}

fn decode_byte(n: &u8) -> Result<&u8, Error> {
    if n == &0x3D {
        Ok(&0)
    } else {
        B64_MAP
            .get_by_right(&n)
            .ok_or_else(|| Error::invalid_input_char(*n as char))
    }
}

fn encode_b64_remainder(chunk: &[u8], out: &mut Vec<u8>) {
    if chunk.len() == 2 {
        let s1 = chunk[0] >> 2;
        let s2 = ((chunk[0] << 4) & 0x3F) ^ (chunk[1] >> 4);
        let s3 = (chunk[1] << 2) & 0x3F;
        out.push(*B64_MAP.get_by_left(&s1).unwrap());
        out.push(*B64_MAP.get_by_left(&s2).unwrap());
        out.push(*B64_MAP.get_by_left(&s3).unwrap());
        out.push(0x3D);
    } else if chunk.len() == 1 {
        let s1 = chunk[0] >> 2;
        let s2 = (chunk[0] << 4) & 0x3F;
        out.push(*B64_MAP.get_by_left(&s1).unwrap());
        out.push(*B64_MAP.get_by_left(&s2).unwrap());
        out.push(0x3D);
        out.push(0x3D);
    } else {
        return ();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Direct,
    Binary,
    Octal,
    Decimal,
    Hex,
}

// Make it possible to encode an aribtrary file
pub struct Base64 {
    pub file: Option<PathBuf>,
    pub mode: DisplayMode,
}

impl Default for Base64 {
    fn default() -> Self {
        Self {
            file: None,
            mode: DisplayMode::Direct,
        }
    }
}

impl Base64 {
    pub fn encode_file(&self) -> Result<String, Error> {
        if self.file.is_none() {
            return Err(Error::input("no file stored"));
        }
        let bytes = &read(self.file.as_ref().unwrap()).unwrap()[..];

        let encoded = Base64::encode_bytes(bytes);
        Ok(String::from_utf8(encoded).unwrap())
    }

    // All u8 values are covered so this is not fallible
    pub fn encode_bytes(input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity((input.len() / 3) * 4);
        let chunks = input.chunks_exact(3);
        let rem = chunks.remainder();

        for chunk in chunks {
            // turn the three bytes into four sextets
            // shr chunk[0] twice to keep only the top six bits
            let s1 = chunk[0] >> 2;
            // shl chunk[0] 4 times to put the bottom top 2 bits on top, mask the top two bits, then shr[1] 4 times to put the top four bits on the bottom, XOR together
            let s2 = ((chunk[0] << 4) & 0x3F) ^ (chunk[1] >> 4);
            // shl chunk[1] 2 times to leave two bits open at the bottom, mask the top two bits, shr chunk[2] 6 times to put the bottom two bits on the bottom XOR together
            let s3 = ((chunk[1] << 2) & 0x3F) ^ (chunk[2] >> 6);
            // mask the top two bits of chunk[2]
            let s4 = chunk[2] & 0x3F;

            // Encode the four sextets as four ASCII bytes
            out.push(*B64_MAP.get_by_left(&s1).unwrap());
            out.push(*B64_MAP.get_by_left(&s2).unwrap());
            out.push(*B64_MAP.get_by_left(&s3).unwrap());
            out.push(*B64_MAP.get_by_left(&s4).unwrap());
        }
        encode_b64_remainder(rem, &mut out);
        out
    }

    // Only ASCII values for u8 are allowed so this is fallible
    pub fn decode_bytes(input: &[u8]) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity((input.len() / 4) * 3);
        let chunks = input.chunks_exact(4);
        let padding_len = {
            let l0 = input.iter().nth_back(0).unwrap() == &0x3D;
            let l1 = input.iter().nth_back(1).unwrap() == &0x3D;
            l0 as usize + l1 as usize
        };
        for chunk in chunks {
            let s1 = *decode_byte(&chunk[0])?;
            let s2 = *decode_byte(&chunk[1])?;
            let s3 = *decode_byte(&chunk[2])?;
            let s4 = *decode_byte(&chunk[3])?;

            // shift s1 left twice to leave two bits at the bottom, shift s2 right four times to put the top two bits on the bottom, XOR together
            let o1 = (s1 << 2) ^ (s2 >> 4);
            // shift s2 left four to leave four at the bottom, shift s3 right two times to put the top four bits on the bottom, XOR together
            let o2 = (s2 << 4) ^ (s3 >> 2);
            // shift s3 left six to leave four at the bottom, XOR together with s4
            let o3 = (s3 << 6) ^ s4;

            out.push(o1);
            out.push(o2);
            out.push(o3);
        }
        for _ in 0..padding_len {
            out.pop();
        }
        Ok(out)
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (String, char)> + '_ {
        (0..64u8).map(|x| {
            (
                format!("{:06b}", x),
                *B64_MAP.get_by_left(&x).unwrap() as char,
            )
        })
    }
}

impl Code for Base64 {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let b = Base64::encode_bytes(text.as_bytes());
        Ok(String::from_utf8(b).unwrap())
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let b = Base64::decode_bytes(text.as_bytes())?;
        Ok(String::from_utf8(b).unwrap())
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
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

    #[test]
    fn encode_test() {
        let code = Base64::default();
        assert_eq!(code.encode(PLAINTEXT0).unwrap(), CIPHERTEXT0);
        assert_eq!(code.encode(PLAINTEXT1).unwrap(), CIPHERTEXT1);
        assert_eq!(code.encode(PLAINTEXT2).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn deode_test() {
        let code = Base64::default();
        assert_eq!(code.decode(CIPHERTEXT0).unwrap(), PLAINTEXT0);
        assert_eq!(code.decode(CIPHERTEXT1).unwrap(), PLAINTEXT1);
        assert_eq!(code.decode(CIPHERTEXT2).unwrap(), PLAINTEXT2);
    }
}
