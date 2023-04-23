use crate::codes::Code;
use crate::{errors::Error, text_aux::text_functions::bimap_from_iter};
use bimap::BiMap;
use lazy_static::lazy_static;
use num::Integer;
use std::path::PathBuf;

use super::{bytes_to_hex, BinaryToText, BinaryToTextMode};

const ASCII85_BTOA: &'static str =
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstu";

// const ASCII85_IPV6: &'static str =
//     "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";

lazy_static! {
    pub static ref ASCII85_BTOA_MAP: BiMap<u8, u8> = bimap_from_iter(
        ASCII85_BTOA
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
}

// Make it possible to encode an aribtrary file
pub struct Ascii85 {
    pub file: Option<PathBuf>,
    pub mode: BinaryToTextMode,
}

impl Default for Ascii85 {
    fn default() -> Self {
        Self {
            file: None,
            mode: BinaryToTextMode::Utf8,
        }
    }
}

impl Ascii85 {
    pub fn map(&self) -> &BiMap<u8, u8> {
        &ASCII85_BTOA_MAP
    }

    // pub fn chars_codes(&mut self) -> impl Iterator<Item = (String, char)> + '_ {
    //     (0..32u8).map(|x| {
    //         (
    //             format!("{:05b}", x),
    //             *self.map().get_by_left(&x).unwrap() as char,
    //         )
    //     })
    // }
}

impl BinaryToText for Ascii85 {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, Error> {
        let mut out = Vec::with_capacity((bytes.len() / 5) * 8);
        let map = self.map();
        let mut buffer = 0_u32;
        let mut bytes = bytes.iter().peekable();

        loop {
            // Break if done
            if bytes.peek().is_none() {
                break;
            }

            // Fill buffer and count padding
            // Nothing is XORed in for pad bytes because '\0' is the all zero byte
            let mut used_bytes = 5;
            for _ in 0..4 {
                buffer <<= 8;
                match bytes.next() {
                    Some(byte) => buffer ^= *byte as u32,
                    None => {
                        used_bytes -= 1;
                    }
                }
            }

            if buffer == 0x20202020 {
                out.push('y' as u8);
                continue;
            }

            if used_bytes == 5 && buffer == 0 {
                out.push('z' as u8);
                continue;
            }

            let mut chars = [0; 5];
            for i in 0..5 {
                let (quot, rem) = buffer.div_rem(&85);
                chars[i] = *map.get_by_left(&(rem as u8)).unwrap();
                buffer = quot;
            }

            for c in chars.into_iter().rev().take(used_bytes) {
                out.push(c)
            }
        }

        Ok(String::from_utf8(out).unwrap())
    }
}

impl Code for Ascii85 {
    fn encode(&self, text: &str) -> Result<String, Error> {
        match self.mode {
            BinaryToTextMode::Hex => self.encode_hex(text),
            BinaryToTextMode::Utf8 => self.encode_utf8(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out: Vec<u8> = Vec::new();
        let mut chars = text.chars().filter(|c| !c.is_whitespace()).peekable();
        let map = self.map();

        loop {
            // Break if done
            if chars.peek().is_none() {
                break;
            }

            // Handle special 'z' and 'y' characters
            if *chars.peek().unwrap() == 'z' {
                out.extend_from_slice(&[0, 0, 0, 0]);
                chars.next(); // remove the 'z'
                continue;
            }
            if *chars.peek().unwrap() == 'y' {
                out.extend_from_slice(&[0x20, 0x20, 0x20, 0x20]);
                chars.next(); // remove the 'y'
                continue;
            }

            // If those are handled we fill the buffer algebraically
            let mut buffer = 0_u32;

            let mut used_chars = 4;
            for i in (0..5).rev() {
                match chars.next() {
                    Some(byte) => {
                        buffer += *map
                            .get_by_right(&(byte as u8))
                            .ok_or_else(|| Error::invalid_input_char(byte as char))?
                            as u32
                            * 85_u32.pow(i)
                    }
                    None => {
                        used_chars -= 1;
                        buffer += 84 * 85_u32.pow(i);
                    }
                }
            }

            // Extract the used bytes from the buffer
            for b in buffer.to_le_bytes().into_iter().rev().take(used_chars) {
                out.push(b)
            }
        }
        match self.mode {
            BinaryToTextMode::Hex => bytes_to_hex(&out),
            BinaryToTextMode::Utf8 => {
                String::from_utf8(out).map_err(|e| Error::Input(e.to_string()))
            }
        }
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod ascii85_tests {
    use super::*;

    const TEST_TEXT: [(&'static str, &'static str); 7] = [
        ("Man is d", "9jqo^BlbD-"),
        ("Man ", "9jqo^"),
        ("Man", "9jqo"),
        ("Ma", "9jn"),
        ("M", "9`"),
        ("    ", "y"),
        ("\0\0\0\0", "z"),
    ];

    #[test]
    fn encode_test() {
        let code = Ascii85::default();
        for (ptext, ctext) in TEST_TEXT {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
    }

    #[test]
    fn decode_test() {
        let code = Ascii85::default();
        for (ptext, ctext) in TEST_TEXT {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
    }
}
