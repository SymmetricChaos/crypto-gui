use lazy_static::lazy_static;
use std::{collections::HashMap, fs::read, path::PathBuf};

use super::Code;
use crate::errors::Error;

const B64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

lazy_static! {
    static ref B64_MAP: HashMap<u8, u8> = {
        let mut m = HashMap::with_capacity(64);
        for (pos, chr) in B64.chars().enumerate() {
            m.insert(pos as u8, chr as u8);
        }
        m
    };
    static ref B64_MAP_INV: HashMap<u8, u8> = {
        let mut m = HashMap::with_capacity(64);
        for (pos, chr) in B64.chars().enumerate() {
            m.insert(chr as u8, pos as u8);
        }
        m.insert('=' as u8, 0);
        m
    };
}

fn encode_b64_remainder(chunk: &[u8], out: &mut Vec<u8>) {
    if chunk.len() == 2 {
        let s1 = chunk[0] >> 2;
        let s2 = ((chunk[0] << 4) & 0x3F) ^ (chunk[1] >> 4);
        let s3 = (chunk[1] << 2) & 0x3F;
        out.push(B64_MAP[&s1]);
        out.push(B64_MAP[&s2]);
        out.push(B64_MAP[&s3]);
        out.push(0x3D);
    } else if chunk.len() == 1 {
        let s1 = chunk[0] >> 2;
        let s2 = (chunk[0] << 4) & 0x3F;
        out.push(B64_MAP[&s1]);
        out.push(B64_MAP[&s2]);
        out.push(0x3D);
        out.push(0x3D);
    } else {
        return ();
    }
}

// Make it possible to encode an aribtrary file
pub struct Base64 {
    pub file: Option<PathBuf>,
}

impl Default for Base64 {
    fn default() -> Self {
        Self { file: None }
    }
}

impl Base64 {
    pub fn encode_file(&self) -> Result<String, Error> {
        if self.file.is_none() {
            return Err(Error::input("no file stored"));
        }
        let bytes = &read(self.file.as_ref().unwrap()).unwrap()[..];

        let encoded = Base64::encode_raw(bytes);
        Ok(String::from_utf8(encoded).unwrap())
    }

    pub fn encode_raw(input: &[u8]) -> Vec<u8> {
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

            out.push(B64_MAP[&s1]);
            out.push(B64_MAP[&s2]);
            out.push(B64_MAP[&s3]);
            out.push(B64_MAP[&s4]);
        }
        encode_b64_remainder(rem, &mut out);
        out
    }

    pub fn decode_raw(input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity((input.len() / 4) * 3);
        let chunks = input.chunks_exact(4);
        let padding_len = {
            let l0 = input.iter().nth_back(0).unwrap() == &0x3D;
            let l1 = input.iter().nth_back(1).unwrap() == &0x3D;
            l0 as usize + l1 as usize
        };
        for chunk in chunks {
            let s1 = B64_MAP_INV[&chunk[0]];
            let s2 = B64_MAP_INV[&chunk[1]];
            let s3 = B64_MAP_INV[&chunk[2]];
            let s4 = B64_MAP_INV[&chunk[3]];

            // shift s1 left twice to leave two bits at the bottom, shift s2 right twice to put the top two bits on the bottom, XOR together
            let o1 = (s1 << 2) ^ (s2 >> 4);
            // shift s2 left four to leave four at the bottom, shift s3 right two times to put the top four bits on the bottom, XOR together
            let o2 = (s2 << 4) ^ (s3 >> 2);
            // shift s3 left six to leave four at the bottom, shift s3 right two times to put the top four bits on the bottom, XOR together
            let o3 = (s3 << 6) ^ s4;

            out.push(o1);
            out.push(o2);
            out.push(o3);
        }
        for _ in 0..padding_len {
            out.pop();
        }
        out
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (String, char)> + '_ {
        (0..64u8).map(|x| (format!("{:06b}", x), B64_MAP[&x] as char))
    }
}

impl Code for Base64 {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let b = Base64::encode_raw(text.as_bytes());
        Ok(String::from_utf8(b).unwrap())
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let b = Base64::decode_raw(text.as_bytes());
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
