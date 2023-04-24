pub mod ascii85;
pub mod base32;
pub mod base64;
pub mod pgp_words;
pub mod skey;

use crate::errors::Error;
use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BinaryToTextMode {
    Hex,
    Utf8,
}

lazy_static! {
    pub static ref IS_HEX_BYTES: Regex = Regex::new(r"^([0-9a-f][0-9a-f])*$").unwrap();
    pub static ref HEX: BiMap<String, u8> = (0..255).map(|n| (format!("{:02x}", n), n)).collect();
}

// A string containing hex characters converted into bytes
// "DEADBEEF" -> [222, 173, 190, 239]
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Error> {
    let mut text: String = hex.split_whitespace().collect();
    text.make_ascii_lowercase();
    if !IS_HEX_BYTES.is_match(&text) {
        return Err(Error::Input("not valid hex bytes".into()));
    } else {
        let mut out = Vec::new();
        for i in 0..(text.len() / 2) {
            let lo = i * 2;
            out.push(*HEX.get_by_left(&text[lo..lo + 2]).unwrap())
        }
        Ok(out)
    }
}

// Convert bytes into a string containing hex code representing them
pub fn bytes_to_hex(bytes: &[u8]) -> Result<String, Error> {
    Ok(bytes
        .into_iter()
        .map(|b| HEX.get_by_right(b).unwrap())
        .join(""))
}

pub trait BinaryToText {
    // Encode some literal bytes
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, Error>;

    // Take a hex string, convert it to bytes, and then encode it
    fn encode_hex(&self, hex: &str) -> Result<String, Error> {
        let bytes = hex_to_bytes(hex)?;
        self.encode_bytes(&bytes)
    }

    // Encode some literal UTF-8 text
    fn encode_utf8(&self, text: &str) -> Result<String, Error> {
        self.encode_bytes(text.as_bytes())
    }

    // fn encode_file(&self, path: Option<PathBuf>) -> Result<String, Error> {
    //     if path.is_none() {
    //         return Err(Error::input("no file stored"));
    //     }
    //     let bytes = &read(path.as_ref().unwrap()).unwrap()[..];
    //     self.encode_bytes(bytes)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_convert() {
        let hex = "   01  2 0";
        assert_eq!(hex_to_bytes(hex).unwrap(), vec![1, 32]);
    }
}
