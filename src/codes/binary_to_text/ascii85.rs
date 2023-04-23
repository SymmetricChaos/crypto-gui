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
    pub use_padding: bool,
}

impl Default for Ascii85 {
    fn default() -> Self {
        Self {
            file: None,
            mode: BinaryToTextMode::Utf8,
            use_padding: true,
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
            // Push four bytes into buffer
            // Push PAD ('\0') into the buffer if bytes run out, count the number of PAD bytes
            // If buffer == 0x0 and no padding was used, push 'z' to out
            // If buffer == 0x20202020, push 'y' to
            // Otherwise divide by 85 and take remainder four times, map those values to characters
            // read those characters in reverse order, stopping after 4-(pad count)

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
        todo!();
        // match self.mode {
        //     BinaryToTextMode::Hex => bytes_to_hex(&out),
        //     BinaryToTextMode::Utf8 => {
        //         String::from_utf8(out).map_err(|e| Error::Input(e.to_string()))
        //     }
        // }
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod ascii85_tests {
    use super::*;

    const PLAINTEXT0: &'static str = "Man ";
    const PLAINTEXT1: &'static str = "Man";
    const PLAINTEXT2: &'static str = "Ma";
    const PLAINTEXT3: &'static str = "M";

    const CIPHERTEXT0: &'static str = "9jqo^";
    const CIPHERTEXT1: &'static str = "9jqo";
    const CIPHERTEXT2: &'static str = "9jn";
    const CIPHERTEXT3: &'static str = "9`";

    #[test]
    fn encode_test() {
        let code = Ascii85::default();
        assert_eq!(code.encode(PLAINTEXT0).unwrap(), CIPHERTEXT0);
        assert_eq!(code.encode(PLAINTEXT1).unwrap(), CIPHERTEXT1);
        assert_eq!(code.encode(PLAINTEXT2).unwrap(), CIPHERTEXT2);
        assert_eq!(code.encode(PLAINTEXT3).unwrap(), CIPHERTEXT3);
    }

    #[test]
    fn decode_test() {
        let code = Ascii85::default();
        assert_eq!(code.decode(CIPHERTEXT0).unwrap(), PLAINTEXT0);
        assert_eq!(code.decode(CIPHERTEXT1).unwrap(), PLAINTEXT1);
        assert_eq!(code.decode(CIPHERTEXT2).unwrap(), PLAINTEXT2);
        assert_eq!(code.decode(CIPHERTEXT3).unwrap(), PLAINTEXT3);
    }
}
