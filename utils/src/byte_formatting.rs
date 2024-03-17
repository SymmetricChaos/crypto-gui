use base64::prelude::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref IS_HEX_BYTES: Regex = Regex::new(r"^(?:[0-9a-fA-F]{2})+$").unwrap();
    pub static ref IS_BINARY_BYTES: Regex = Regex::new(r"^(?:[0-1]{8})+$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexToBytesError;

// A string containing hex characters converted into bytes
// Bytes are read as pairs of characters from left to right, only an even number of characters are accepted
// "DEADBEEF" -> [222, 173, 190, 239]
pub fn hex_to_bytes_be(hex: &str) -> Result<Vec<u8>, HexToBytesError> {
    let mut text: String = hex.lines().collect();
    text = text.to_ascii_lowercase();
    if !IS_HEX_BYTES.is_match(&text) {
        return Err(HexToBytesError);
    } else {
        let mut out = Vec::new();
        for i in 0..(text.len() / 2) {
            let lo = i * 2;
            out.push(u8::from_str_radix(&text[lo..lo + 2], 16).unwrap())
        }
        Ok(out)
    }
}

// A string containing hex characters converted into bytes
// Bytes are read as pairs of characters from right to left, only an even number of characters are accepted
// "DEADBEEF" -> [239, 190, 173, 222]
pub fn hex_to_bytes_le(hex: &str) -> Result<Vec<u8>, HexToBytesError> {
    let mut text: String = hex.lines().collect();
    text = text.to_ascii_lowercase();
    if !IS_HEX_BYTES.is_match(&text) {
        return Err(HexToBytesError);
    } else {
        let mut out = Vec::new();
        for i in (0..(text.len() / 2)).rev() {
            let lo = i * 2;
            out.push(u8::from_str_radix(&text[lo..lo + 2], 16).unwrap())
        }
        Ok(out)
    }
}

pub fn bytes_to_hex_be(bytes: &[u8]) -> String {
    bytes.into_iter().map(|b| format!("{:02x}", b)).join("")
}

pub fn bytes_to_hex_le(bytes: &[u8]) -> String {
    bytes
        .into_iter()
        .rev()
        .map(|b| format!("{:02x}", b))
        .join("")
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct OctToBytesError;

// pub fn oct_to_bytes(hex: &str) -> Result<Vec<u8>, OctToBytesError> {
//     let mut text: String = hex.lines().collect();
//     text = text.to_ascii_lowercase();
//     if !IS_OCT_BYTES.is_match(&text) {
//         return Err(OctToBytesError);
//     } else {
//         let mut out = Vec::new();
//         for i in 0..(text.len() / 3) {
//             let lo = i * 3;
//             out.push(u8::from_str_radix(&text[lo..lo + 3], 8).unwrap())
//         }
//         Ok(out)
//     }
// }

// pub fn bytes_to_oct(bytes: &[u8]) -> String {
//     bytes.into_iter().map(|b| format!("{:03o}", b)).join("")
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitstringToBytesError;

pub fn bitstring_to_bytes(hex: &str) -> Result<Vec<u8>, BitstringToBytesError> {
    let mut text: String = hex.lines().collect();
    text = text.to_ascii_lowercase();
    if !IS_BINARY_BYTES.is_match(&text) {
        return Err(BitstringToBytesError);
    } else {
        let mut out = Vec::new();
        for i in 0..(text.len() / 8) {
            let lo = i * 8;
            out.push(u8::from_str_radix(&text[lo..lo + 8], 2).unwrap())
        }
        Ok(out)
    }
}

pub fn bytes_to_bitstring(bytes: &[u8]) -> String {
    bytes.into_iter().map(|b| format!("{:08b}", b)).join("")
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ByteFormatError;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ByteFormat {
    Hex,
    HexLe,
    Utf8,
    Base64,
    // Oct,
    Bit,
}

impl ByteFormat {
    pub fn text_to_bytes(&self, text: &str) -> Result<Vec<u8>, ByteFormatError> {
        if text.len() == 0 {
            return Ok(Vec::new());
        }
        match self {
            ByteFormat::Hex => hex_to_bytes_be(text).map_err(|_| ByteFormatError),
            ByteFormat::HexLe => hex_to_bytes_le(text).map_err(|_| ByteFormatError),
            ByteFormat::Utf8 => Ok(text.as_bytes().to_owned()),
            ByteFormat::Base64 => BASE64_STANDARD.decode(text).map_err(|_| ByteFormatError),
            ByteFormat::Bit => bitstring_to_bytes(text).map_err(|_| ByteFormatError),
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
            ByteFormat::Bit => bytes
                .as_ref()
                .iter()
                .map(|byte| format!("{:08b}", byte))
                .collect(),
            ByteFormat::HexLe => bytes
                .as_ref()
                .iter()
                .rev()
                .map(|byte| format!("{:02x}", byte))
                .collect(),
        }
    }

    pub fn byte_iter_to_text(&self, bytes: impl Iterator<Item = u8>) -> String {
        match self {
            ByteFormat::Hex => bytes.map(|byte| format!("{:02x}", byte)).collect(),
            ByteFormat::HexLe => todo!("can't read all iterators backward"),
            ByteFormat::Utf8 => String::from_utf8_lossy(&bytes.collect_vec()).to_string(),
            ByteFormat::Base64 => BASE64_STANDARD.encode(&bytes.collect_vec()),
            ByteFormat::Bit => bytes.map(|byte| format!("{:08b}", byte)).collect(),
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

#[cfg(test)]
mod bit_function_tests {

    use super::*;

    #[test]
    fn hex_endianness() {
        assert_eq!(
            vec![222, 173, 190, 239],
            ByteFormat::Hex.text_to_bytes("DEADBEEF").unwrap()
        );
        assert_eq!(
            vec![239, 190, 173, 222],
            ByteFormat::HexLe.text_to_bytes("DEADBEEF").unwrap()
        );

        assert_eq!(
            "deadbeef",
            ByteFormat::Hex.byte_slice_to_text(&[222, 173, 190, 239])
        );
        assert_eq!(
            "deadbeef",
            ByteFormat::HexLe.byte_slice_to_text(&[239, 190, 173, 222])
        );
    }
}
