use base64::prelude::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use num::traits::ToBytes;
use regex::Regex;
use strum::{Display, EnumIter};

lazy_static! {
    pub static ref IS_HEX_BYTES: Regex = Regex::new(r"^(?:[0-9a-fA-F]{2})+$").unwrap();
    pub static ref IS_BINARY_BYTES: Regex = Regex::new(r"^(?:[0-1]{8})+$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexToBytesError;

// A string containing hex characters converted into bytes
// Bytes are read as pairs of characters from left to right, only an even number of characters are accepted
// "DEADBEEF" -> [222, 173, 190, 239]
pub fn hex_to_bytes_ltr(hex: &str) -> Result<Vec<u8>, HexToBytesError> {
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
pub fn hex_to_bytes_rtl(hex: &str) -> Result<Vec<u8>, HexToBytesError> {
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

pub fn bytes_to_hex<T: AsRef<[u8]>>(bytes: T) -> String {
    bytes.as_ref().iter().map(|b| format!("{:02x}", b)).join("")
}

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

pub fn bytes_to_bitstring<T: AsRef<[u8]>>(bytes: T) -> String {
    bytes.as_ref().iter().map(|b| format!("{:08b}", b)).join("")
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ByteFormatError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, Display)]
pub enum ByteFormat {
    #[strum(to_string = "Hexadecimal")]
    Hex,
    #[strum(to_string = "Text (UTF-8)")]
    Utf8,
    Base64,
    Binary,
}

impl ByteFormat {
    pub fn text_to_bytes(&self, text: &str) -> Result<Vec<u8>, ByteFormatError> {
        if text.len() == 0 {
            return Ok(Vec::new());
        }
        match self {
            ByteFormat::Hex => hex_to_bytes_ltr(text).map_err(|_| ByteFormatError),
            ByteFormat::Utf8 => Ok(text.as_bytes().to_owned()),
            ByteFormat::Base64 => BASE64_STANDARD.decode(text).map_err(|_| ByteFormatError),
            ByteFormat::Binary => bitstring_to_bytes(text).map_err(|_| ByteFormatError),
        }
    }

    pub fn text_to_u16_be(&self, text: &str) -> Result<Vec<u16>, ByteFormatError> {
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

    pub fn text_to_u16_le(&self, text: &str) -> Result<Vec<u16>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 2 != 0 {
            Err(ByteFormatError)
        } else {
            Ok(bytes
                .chunks_exact(2)
                .map(|p| u16::from_le_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_u32_be(&self, text: &str) -> Result<Vec<u32>, ByteFormatError> {
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

    pub fn text_to_u32_le(&self, text: &str) -> Result<Vec<u32>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 4 != 0 {
            Err(ByteFormatError)
        } else {
            Ok(bytes
                .chunks_exact(4)
                .map(|p| u32::from_le_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_u64_be(&self, text: &str) -> Result<Vec<u64>, ByteFormatError> {
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

    pub fn text_to_u64_le(&self, text: &str) -> Result<Vec<u64>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 8 != 0 {
            Err(ByteFormatError)
        } else {
            Ok(bytes
                .chunks_exact(8)
                .map(|p| u64::from_le_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn byte_slice_to_text<T: AsRef<[u8]>>(&self, bytes: T) -> String {
        match self {
            ByteFormat::Hex => bytes_to_hex(bytes),
            ByteFormat::Utf8 => String::from_utf8_lossy(bytes.as_ref()).to_string(),
            ByteFormat::Base64 => BASE64_STANDARD.encode(bytes),
            ByteFormat::Binary => bytes
                .as_ref()
                .iter()
                .map(|byte| format!("{:08b}", byte))
                .collect(),
        }
    }

    pub fn byte_iter_to_text(&self, bytes: impl Iterator<Item = u8>) -> String {
        match self {
            ByteFormat::Hex => bytes.map(|byte| format!("{:02x}", byte)).collect(),
            ByteFormat::Utf8 => String::from_utf8_lossy(&bytes.collect_vec()).to_string(),
            ByteFormat::Base64 => BASE64_STANDARD.encode(&bytes.collect_vec()),
            ByteFormat::Binary => bytes.map(|byte| format!("{:08b}", byte)).collect(),
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
        // assert_eq!(
        //     vec![239, 190, 173, 222],
        //     ByteFormat::HexLe.text_to_bytes("DEADBEEF").unwrap()
        // );

        assert_eq!(
            "deadbeef",
            ByteFormat::Hex.byte_slice_to_text(&[222, 173, 190, 239])
        );
        // assert_eq!(
        //     "deadbeef",
        //     ByteFormat::HexLe.byte_slice_to_text(&[239, 190, 173, 222])
        // );
    }
}

pub fn u64_to_u32_pair(n: u64) -> [u32; 2] {
    [(n >> 32) as u32, n as u32]
}

pub fn u32_pair_to_u64(n: [u32; 2]) -> u64 {
    (n[0] as u64) << 32 | n[1] as u64
}

pub fn u32_pair_to_u8_array(s: [u32; 2]) -> [u8; 8] {
    let a = s[0].to_be_bytes();
    let b = s[1].to_be_bytes();
    let mut out = [0; 8];
    for i in 0..4 {
        out[i] = a[i];
        out[i + 4] = b[i];
    }

    out
}

pub fn u64_pair_to_u8_array(s: [u64; 2]) -> [u8; 16] {
    let a = s[0].to_be_bytes();
    let b = s[1].to_be_bytes();
    let mut out = [0; 16];
    for i in 0..8 {
        out[i] = a[i];
        out[i + 8] = b[i];
    }

    out
}

pub fn u32_4_to_u8_16(s: [u32; 4]) -> [u8; 16] {
    let a = s[0].to_be_bytes();
    let b = s[1].to_be_bytes();
    let c = s[2].to_be_bytes();
    let d = s[3].to_be_bytes();
    let mut out = [0; 16];
    for i in 0..4 {
        out[i] = a[i];
        out[i + 4] = b[i];
        out[i + 8] = c[i];
        out[i + 12] = d[i];
    }
    out
}

pub fn fill_u16s_be(target: &mut [u16], bytes: &[u8]) {
    for (elem, chunk) in target.iter_mut().zip_eq(bytes.chunks_exact(2)) {
        *elem = u16::from_be_bytes(chunk.try_into().unwrap());
    }
}

pub fn fill_u16s_le(target: &mut [u16], bytes: &[u8]) {
    for (elem, chunk) in target.iter_mut().zip_eq(bytes.chunks_exact(2)) {
        *elem = u16::from_le_bytes(chunk.try_into().unwrap());
    }
}

pub fn fill_u32s_be(target: &mut [u32], bytes: &[u8]) {
    for (elem, chunk) in target.iter_mut().zip_eq(bytes.chunks_exact(4)) {
        *elem = u32::from_be_bytes(chunk.try_into().unwrap());
    }
}

pub fn fill_u32s_le(target: &mut [u32], bytes: &[u8]) {
    for (elem, chunk) in target.iter_mut().zip_eq(bytes.chunks_exact(4)) {
        *elem = u32::from_le_bytes(chunk.try_into().unwrap());
    }
}

pub fn fill_u64s_be(target: &mut [u64], bytes: &[u8]) {
    for (elem, chunk) in target.iter_mut().zip_eq(bytes.chunks_exact(8)) {
        *elem = u64::from_be_bytes(chunk.try_into().unwrap());
    }
}

pub fn fill_u64s_le(target: &mut [u64], bytes: &[u8]) {
    for (elem, chunk) in target.iter_mut().zip_eq(bytes.chunks_exact(8)) {
        *elem = u64::from_le_bytes(chunk.try_into().unwrap());
    }
}

pub fn u32s_to_bytes_be(target: &mut [u8], words: &[u32]) {
    for i in 0..words.len() {
        let bytes = words[i].to_be_bytes();
        for j in 0..4 {
            target[(i * 4) + j] = bytes[j];
        }
    }
}

pub fn u32s_to_bytes_le(target: &mut [u8], words: &[u32]) {
    for i in 0..words.len() {
        let bytes = words[i].to_le_bytes();
        for j in 0..4 {
            target[(i * 4) + j] = bytes[j];
        }
    }
}

pub fn u64s_to_bytes_be(target: &mut [u8], words: &[u64]) {
    for i in 0..words.len() {
        let bytes = words[i].to_be_bytes();
        for j in 0..8 {
            target[(i * 8) + j] = bytes[j];
        }
    }
}

pub fn u64s_to_bytes_le(target: &mut [u8], words: &[u64]) {
    for i in 0..words.len() {
        let bytes = words[i].to_le_bytes();
        for j in 0..8 {
            target[(i * 8) + j] = bytes[j];
        }
    }
}

pub fn overwrite_bytes(target: &mut [u8], source: &[u8]) {
    for (t, s) in target.iter_mut().zip_eq(source.iter()) {
        *t = *s
    }
}

pub fn xor_into_bytes(target: &mut [u8], source: &[u8]) {
    for (t, s) in target.iter_mut().zip_eq(source.iter()) {
        *t ^= *s
    }
}
