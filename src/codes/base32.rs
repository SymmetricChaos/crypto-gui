use super::Code;
use crate::{errors::Error, text_aux::text_functions::bimap_from_iter};
use bimap::BiMap;
use lazy_static::lazy_static;
use std::{fs::read, path::PathBuf};

const MASK: u8 = 0b00011111;
const PAD: u8 = '=' as u8;

const BASE32_ALPHA: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

lazy_static! {
    pub static ref B32_MAP: BiMap<u8, u8> = bimap_from_iter(
        BASE32_ALPHA
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
}

fn decode_byte(n: &u8) -> Result<&u8, Error> {
    if n == &PAD {
        Ok(&0)
    } else {
        B32_MAP
            .get_by_right(&n)
            .ok_or_else(|| Error::invalid_input_char(*n as char))
    }
}

fn encode_b32_remainder(chunk: &[u8], out: &mut Vec<u8>) {
    if chunk.len() == 4 {
        let s1 = (chunk[0] >> 3) & MASK;
        let s2 = ((chunk[0] << 2) & MASK) ^ ((chunk[1] >> 6) & MASK);
        let s3 = (chunk[1] >> 1) & MASK;
        let s4 = ((chunk[1] << 4) & MASK) ^ ((chunk[2] >> 4) & MASK);
        let s5 = ((chunk[2] << 1) & MASK) ^ ((chunk[3] >> 7) & MASK);
        let s6 = (chunk[3] >> 2) & MASK;
        let s7 = (chunk[3] << 3) & MASK;
        out.push(*B32_MAP.get_by_left(&s1).unwrap());
        out.push(*B32_MAP.get_by_left(&s2).unwrap());
        out.push(*B32_MAP.get_by_left(&s3).unwrap());
        out.push(*B32_MAP.get_by_left(&s4).unwrap());
        out.push(*B32_MAP.get_by_left(&s5).unwrap());
        out.push(*B32_MAP.get_by_left(&s6).unwrap());
        out.push(*B32_MAP.get_by_left(&s7).unwrap());
        out.push(PAD);
    } else if chunk.len() == 3 {
        let s1 = (chunk[0] >> 3) & MASK;
        let s2 = ((chunk[0] << 2) & MASK) ^ ((chunk[1] >> 6) & MASK);
        let s3 = (chunk[1] >> 1) & MASK;
        let s4 = ((chunk[1] << 4) & MASK) ^ ((chunk[2] >> 4) & MASK);
        let s5 = (chunk[2] << 1) & MASK;
        out.push(*B32_MAP.get_by_left(&s1).unwrap());
        out.push(*B32_MAP.get_by_left(&s2).unwrap());
        out.push(*B32_MAP.get_by_left(&s3).unwrap());
        out.push(*B32_MAP.get_by_left(&s4).unwrap());
        out.push(*B32_MAP.get_by_left(&s5).unwrap());
        out.push(PAD);
        out.push(PAD);
        out.push(PAD);
    } else if chunk.len() == 2 {
        let s1 = (chunk[0] >> 3) & MASK;
        let s2 = ((chunk[0] << 2) & MASK) ^ ((chunk[1] >> 6) & MASK);
        let s3 = (chunk[1] >> 1) & MASK;
        let s4 = (chunk[1] << 4) & MASK;
        out.push(*B32_MAP.get_by_left(&s1).unwrap());
        out.push(*B32_MAP.get_by_left(&s2).unwrap());
        out.push(*B32_MAP.get_by_left(&s3).unwrap());
        out.push(*B32_MAP.get_by_left(&s4).unwrap());
        out.push(PAD);
        out.push(PAD);
        out.push(PAD);
        out.push(PAD);
    } else if chunk.len() == 1 {
        let s1 = (chunk[0] >> 3) & MASK;
        let s2 = (chunk[0] << 2) & MASK;
        out.push(*B32_MAP.get_by_left(&s1).unwrap());
        out.push(*B32_MAP.get_by_left(&s2).unwrap());
        out.push(PAD);
        out.push(PAD);
        out.push(PAD);
        out.push(PAD);
        out.push(PAD);
        out.push(PAD);
    } else {
        return ();
    }
}

// Make it possible to encode an aribtrary file
pub struct Base32 {
    pub file: Option<PathBuf>,
}

impl Default for Base32 {
    fn default() -> Self {
        Self { file: None }
    }
}

impl Base32 {
    pub fn encode_file(&self) -> Result<String, Error> {
        if self.file.is_none() {
            return Err(Error::input("no file stored"));
        }
        let bytes = &read(self.file.as_ref().unwrap()).unwrap()[..];

        let encoded = Base32::encode_bytes(bytes);
        Ok(String::from_utf8(encoded).unwrap())
    }

    // All u8 values are covered so this is not fallible
    pub fn encode_bytes(input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity((input.len() / 5) * 8);
        let chunks = input.chunks_exact(5);
        let rem = chunks.remainder();

        for chunk in chunks {
            // turn the five bytes into eight pentets
            // BITSHIFT HELL
            let s1 = (chunk[0] >> 3) & MASK;
            let s2 = ((chunk[0] << 2) & MASK) ^ ((chunk[1] >> 6) & MASK);
            let s3 = (chunk[1] >> 1) & MASK;
            let s4 = ((chunk[1] << 4) & MASK) ^ ((chunk[2] >> 4) & MASK);
            let s5 = ((chunk[2] << 1) & MASK) ^ ((chunk[3] >> 7) & MASK);
            let s6 = (chunk[3] >> 2) & MASK;
            let s7 = ((chunk[3] << 3) & MASK) ^ ((chunk[4] >> 5) & MASK);
            let s8 = chunk[4] & MASK;

            out.push(*B32_MAP.get_by_left(&s1).unwrap());
            out.push(*B32_MAP.get_by_left(&s2).unwrap());
            out.push(*B32_MAP.get_by_left(&s3).unwrap());
            out.push(*B32_MAP.get_by_left(&s4).unwrap());
            out.push(*B32_MAP.get_by_left(&s5).unwrap());
            out.push(*B32_MAP.get_by_left(&s6).unwrap());
            out.push(*B32_MAP.get_by_left(&s7).unwrap());
            out.push(*B32_MAP.get_by_left(&s8).unwrap());
        }
        encode_b32_remainder(rem, &mut out);
        out
    }

    // Only ASCII values for u8 are allowed so this is fallible
    pub fn decode_bytes(input: &[u8]) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity((input.len() / 8) * 5);
        if input.len() % 8 != 0 {
            return Err(Error::input(
                "not valid Base32, input's length in bytes must be a multiple of 8",
            ));
        }
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
            let s1 = *decode_byte(&chunk[0])?;
            let s2 = *decode_byte(&chunk[1])?;
            let s3 = *decode_byte(&chunk[2])?;
            let s4 = *decode_byte(&chunk[3])?;
            let s5 = *decode_byte(&chunk[4])?;
            let s6 = *decode_byte(&chunk[5])?;
            let s7 = *decode_byte(&chunk[6])?;
            let s8 = *decode_byte(&chunk[7])?;

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
                *B32_MAP.get_by_left(&x).unwrap() as char,
            )
        })
    }
}

impl Code for Base32 {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let b = Base32::encode_bytes(text.as_bytes());
        Ok(String::from_utf8(b).unwrap())
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let b = Base32::decode_bytes(text.as_bytes())?;
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
    fn deode_test() {
        let code = Base32::default();
        assert_eq!(code.decode(CIPHERTEXT0).unwrap(), PLAINTEXT0);
        assert_eq!(code.decode(CIPHERTEXT1).unwrap(), PLAINTEXT1);
        assert_eq!(code.decode(CIPHERTEXT2).unwrap(), PLAINTEXT2);
        assert_eq!(code.decode(CIPHERTEXT3).unwrap(), PLAINTEXT3);
        assert_eq!(code.decode(CIPHERTEXT4).unwrap(), PLAINTEXT4);
    }
}
