use super::BinaryToText;
use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use lazy_static::lazy_static;
use num::Integer;
use utils::byte_formatting::ByteFormat;
use utils::text_functions::bimap_from_iter;

const ASCII85_BTOA: &'static str =
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstu";

const ASCII85_IPV6: &'static str =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";

const ASCII85_ZEROQM: &'static str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.-:+=^!/*?&<>()[]{}@%$#";

lazy_static! {
    pub static ref ASCII85_BTOA_MAP: BiMap<u8, u8> = bimap_from_iter(
        ASCII85_BTOA
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
    pub static ref ASCII85_IPV6_MAP: BiMap<u8, u8> = bimap_from_iter(
        ASCII85_IPV6
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
    pub static ref ASCII85_ZEROQM_MAP: BiMap<u8, u8> = bimap_from_iter(
        ASCII85_ZEROQM
            .chars()
            .enumerate()
            .map(|(n, c)| (n as u8, c as u8))
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Ascii85Variant {
    Adobe,
    Btoa,
    Ipv6,
    ZeroQm,
}

// Make it possible to encode an aribtrary file
pub struct Ascii85 {
    pub variant: Ascii85Variant,
    pub mode: ByteFormat,
}

impl Default for Ascii85 {
    fn default() -> Self {
        Self {
            variant: Ascii85Variant::Btoa,
            mode: ByteFormat::Utf8,
        }
    }
}

impl Ascii85 {
    pub fn map(&self) -> &BiMap<u8, u8> {
        match self.variant {
            Ascii85Variant::Adobe => &ASCII85_BTOA_MAP, // uses the same map as BTOA.
            Ascii85Variant::Btoa => &ASCII85_BTOA_MAP,
            Ascii85Variant::Ipv6 => &ASCII85_IPV6_MAP,
            Ascii85Variant::ZeroQm => &ASCII85_ZEROQM_MAP,
        }
    }

    pub fn chars_codes(&self) -> impl Iterator<Item = (String, char)> + '_ {
        (0..85u8).map(|x| {
            (
                format!("{x: <2}"),
                *self.map().get_by_left(&x).unwrap() as char,
            )
        })
    }
}

impl BinaryToText for Ascii85 {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
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
            if self.variant == Ascii85Variant::Btoa {
                if buffer == 0x20202020 {
                    out.push('y' as u8);
                    continue;
                }
            }

            if self.variant == Ascii85Variant::Btoa || self.variant == Ascii85Variant::Adobe {
                if used_bytes == 5 && buffer == 0 {
                    out.push('z' as u8);
                    continue;
                }
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
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.mode {
            ByteFormat::Hex => self.encode_hex(text),
            ByteFormat::Utf8 => self.encode_utf8(text),
            ByteFormat::Base64 => self.encode_base64(text),
            ByteFormat::Binary => self.encode_bits(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out: Vec<u8> = Vec::new();
        let mut chars = text.chars().filter(|c| !c.is_whitespace()).peekable();
        let map = self.map();

        loop {
            // Break if done
            if chars.peek().is_none() {
                break;
            }

            // Btoa and Adobe optimize runs of null bytes
            if self.variant == Ascii85Variant::Btoa || self.variant == Ascii85Variant::Adobe {
                if *chars.peek().unwrap() == 'z' {
                    out.extend_from_slice(&[0, 0, 0, 0]);
                    chars.next(); // remove the 'z'
                    continue;
                }
            }

            // Btoa optimizes runs of spaces
            if self.variant == Ascii85Variant::Btoa {
                if *chars.peek().unwrap() == 'y' {
                    out.extend_from_slice(&[0x20, 0x20, 0x20, 0x20]);
                    chars.next(); // remove the 'y'
                    continue;
                }
            }

            // If those are handled we fill the buffer algebraically
            let mut buffer = 0_u32;

            let mut used_chars = 4;
            for i in (0..5).rev() {
                match chars.next() {
                    Some(byte) => {
                        buffer += *map
                            .get_by_right(&(byte as u8))
                            .ok_or_else(|| CodeError::invalid_input_char(byte as char))?
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
        Ok(self.mode.byte_slice_to_text(out))
    }
}

#[cfg(test)]
mod ascii85_tests {
    use super::*;

    const TESTS: [(&'static str, &'static str); 8] = [
        ("Man is d", "9jqo^BlbD-"),      // multiple blocks
        ("Man ", "9jqo^"),               // single block
        ("Man", "9jqo"),                 // partial
        ("Ma", "9jn"),                   // partial
        ("M", "9`"),                     // partial
        ("    ", "y"),                   // special
        ("\0\0\0\0", "z"),               // special
        ("abcd    efgh", "@:E_WyAS,Rg"), // special in contex
    ];

    #[test]
    fn encode_test() {
        let code = Ascii85::default();
        for (ptext, ctext) in TESTS {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
    }

    #[test]
    fn decode_test() {
        let code = Ascii85::default();
        for (ptext, ctext) in TESTS {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
    }

    #[test]
    fn decode_test_errs() {
        let code = Ascii85::default();
        // Fail on character that is always invalid
        assert_eq!(
            code.decode("abdc}").unwrap_err(),
            CodeError::Input("invalid character `}`, alphabets are case sensitive".into())
        );
        // Fail on z if not found at the start of a chunk
        assert_eq!(
            code.decode("azg}").unwrap_err(),
            CodeError::Input("invalid character `z`, alphabets are case sensitive".into())
        );
        // Fail on y if not found at the start of a chunk
        assert_eq!(
            code.decode("agy{").unwrap_err(),
            CodeError::Input("invalid character `y`, alphabets are case sensitive".into())
        );
    }
}
