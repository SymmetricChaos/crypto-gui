use crate::errors::Error;
use eframe::epaint::ahash::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

pub enum BinaryToTextMode {
    Bytes,
    Hex,
    Utf8,
}

lazy_static! {
    pub static ref IS_HEX_BYTES: Regex = Regex::new(r"^([0-9a-f][0-9a-f])*$").unwrap();
    pub static ref HEX: HashMap<String, u8> = (0..255).map(|n| (format!("{:02x}", n), n)).collect();
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Error> {
    let mut text: String = hex.split_whitespace().collect();
    text.make_ascii_lowercase();
    if !IS_HEX_BYTES.is_match(&text) {
        return Err(Error::Input("not valid hex bytes".into()));
    } else {
        let mut out = Vec::new();
        for i in 0..(text.len() / 2) {
            let lo = i * 2;
            out.push(HEX[&text[lo..lo + 2]])
        }
        Ok(out)
    }
}

pub trait BinaryToText {
    // Encode some literal bytes
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, Error>;
    // Take a hex string, convert it to bytes, and then encode it
    fn encode_hex(&self, hex: &str) -> Result<String, Error>;
    // Encode some literal UTF-8 text
    fn encode_utf8(&self, text: &str) -> Result<String, Error>;
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
