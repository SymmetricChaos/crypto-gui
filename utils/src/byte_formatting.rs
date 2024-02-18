use base64::prelude::*;
use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref IS_HEX_BYTES: Regex = Regex::new(r"^(?:[0-9a-fA-F]{2})+$").unwrap();
    pub static ref HEX: BiMap<String, u8> = (0..255).map(|n| (format!("{:02x}", n), n)).collect();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexToBytesError;

// A string containing hex characters converted into bytes
// "DEADBEEF" -> [222, 173, 190, 239]
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, HexToBytesError> {
    let mut text: String = hex.lines().collect();
    text = text.to_ascii_lowercase();
    if !IS_HEX_BYTES.is_match(&text) {
        return Err(HexToBytesError);
    } else {
        let mut out = Vec::new();
        for i in 0..(text.len() / 2) {
            let lo = i * 2;
            out.push(*HEX.get_by_left(&text[lo..lo + 2]).unwrap())
        }
        Ok(out)
    }
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.into_iter().map(|b| format!("{:02x}", b)).join("")
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ByteFormatError;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ByteFormat {
    Hex,
    Utf8,
    Base64,
}

impl ByteFormat {
    pub fn text_to_bytes(&self, text: &str) -> Result<Vec<u8>, ByteFormatError> {
        if text.len() == 0 {
            return Ok(Vec::new());
        }
        match self {
            ByteFormat::Hex => hex_to_bytes(text).map_err(|_| ByteFormatError),
            ByteFormat::Utf8 => Ok(text.as_bytes().to_owned()),
            ByteFormat::Base64 => BASE64_STANDARD.decode(text).map_err(|_| ByteFormatError),
        }
    }

    pub fn text_to_u16(&self, text: &str) -> Result<Vec<u16>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 2 != 0 {
            Err(ByteFormatError)
        } else {
            Ok(bytes
                .chunks_exact(2)
                .map(|p| u16::from_be_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_u32(&self, text: &str) -> Result<Vec<u32>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 4 != 0 {
            Err(ByteFormatError)
        } else {
            Ok(bytes
                .chunks_exact(4)
                .map(|p| u32::from_be_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_u64(&self, text: &str) -> Result<Vec<u64>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 8 != 0 {
            Err(ByteFormatError)
        } else {
            Ok(bytes
                .chunks_exact(8)
                .map(|p| u64::from_be_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn byte_slice_to_text<T: AsRef<[u8]>>(&self, bytes: T) -> String {
        match self {
            ByteFormat::Hex => bytes
                .as_ref()
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
            ByteFormat::Utf8 => String::from_utf8_lossy(bytes.as_ref()).to_string(),
            ByteFormat::Base64 => BASE64_STANDARD.encode(bytes),
        }
    }

    pub fn byte_iter_to_text(&self, bytes: impl Iterator<Item = u8>) -> String {
        match self {
            ByteFormat::Hex => bytes.map(|byte| format!("{:02x}", byte)).collect(),
            ByteFormat::Utf8 => String::from_utf8_lossy(&bytes.collect_vec()).to_string(),
            ByteFormat::Base64 => BASE64_STANDARD.encode(&bytes.collect_vec()),
        }
    }

    pub fn u16_slice_to_text_be<T: AsRef<[u16]>>(&self, nums: T) -> String {
        self.byte_slice_to_text(
            nums.as_ref()
                .iter()
                .flat_map(|n| n.to_be_bytes())
                .collect_vec(),
        )
    }

    pub fn u16_slice_to_text_le<T: AsRef<[u16]>>(&self, nums: T) -> String {
        self.byte_slice_to_text(
            nums.as_ref()
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect_vec(),
        )
    }

    pub fn u32_slice_to_text_be<T: AsRef<[u32]>>(&self, nums: T) -> String {
        self.byte_slice_to_text(
            nums.as_ref()
                .iter()
                .flat_map(|n| n.to_be_bytes())
                .collect_vec(),
        )
    }

    pub fn u32_slice_to_text_le<T: AsRef<[u32]>>(&self, nums: T) -> String {
        self.byte_slice_to_text(
            nums.as_ref()
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect_vec(),
        )
    }

    pub fn u64_slice_to_text_be<T: AsRef<[u64]>>(&self, nums: T) -> String {
        self.byte_slice_to_text(
            nums.as_ref()
                .iter()
                .flat_map(|n| n.to_be_bytes())
                .collect_vec(),
        )
    }

    pub fn u64_slice_to_text_le<T: AsRef<[u64]>>(&self, nums: T) -> String {
        self.byte_slice_to_text(
            nums.as_ref()
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect_vec(),
        )
    }
}
