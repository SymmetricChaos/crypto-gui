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
pub struct ByteFormatError(&'static str);

impl std::fmt::Display for ByteFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
            ByteFormat::Hex => {
                hex_to_bytes_ltr(text).map_err(|_| ByteFormatError("expected hexadecimal"))
            }
            ByteFormat::Utf8 => Ok(text.as_bytes().to_owned()),
            ByteFormat::Base64 => BASE64_STANDARD
                .decode(text)
                .map_err(|_| ByteFormatError("expected Base64")),
            ByteFormat::Binary => {
                bitstring_to_bytes(text).map_err(|_| ByteFormatError("expected binary"))
            }
        }
    }

    pub fn text_to_u8(&self, text: &str) -> Result<Vec<u8>, ByteFormatError> {
        self.text_to_bytes(text)
    }

    pub fn text_to_i8(&self, text: &str) -> Result<Vec<i8>, ByteFormatError> {
        self.text_to_bytes(text)
            .map(|v| v.into_iter().map(|n| n as i8).collect_vec())
    }

    pub fn text_to_u16_be(&self, text: &str) -> Result<Vec<u16>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 2 != 0 {
            Err(ByteFormatError("data must be in blocks of two bytes"))
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
            Err(ByteFormatError("data must be in blocks of two bytes"))
        } else {
            Ok(bytes
                .chunks_exact(2)
                .map(|p| u16::from_le_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_i16_be(&self, text: &str) -> Result<Vec<i16>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 2 != 0 {
            Err(ByteFormatError("data must be in blocks of two bytes"))
        } else {
            Ok(bytes
                .chunks_exact(2)
                .map(|p| i16::from_be_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_i16_le(&self, text: &str) -> Result<Vec<i16>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 2 != 0 {
            Err(ByteFormatError("data must be in blocks of two bytes"))
        } else {
            Ok(bytes
                .chunks_exact(2)
                .map(|p| i16::from_le_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_u32_be(&self, text: &str) -> Result<Vec<u32>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 4 != 0 {
            Err(ByteFormatError("data must be in blocks of four bytes"))
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
            Err(ByteFormatError("data must be in blocks of four bytes"))
        } else {
            Ok(bytes
                .chunks_exact(4)
                .map(|p| u32::from_le_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_i32_be(&self, text: &str) -> Result<Vec<i32>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 4 != 0 {
            Err(ByteFormatError("data must be in blocks of four bytes"))
        } else {
            Ok(bytes
                .chunks_exact(4)
                .map(|p| i32::from_be_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_i32_le(&self, text: &str) -> Result<Vec<i32>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 4 != 0 {
            Err(ByteFormatError("data must be in blocks of four bytes"))
        } else {
            Ok(bytes
                .chunks_exact(4)
                .map(|p| i32::from_le_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_u64_be(&self, text: &str) -> Result<Vec<u64>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 8 != 0 {
            Err(ByteFormatError("data must be in blocks of eight bytes"))
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
            Err(ByteFormatError("data must be in blocks of eight bytes"))
        } else {
            Ok(bytes
                .chunks_exact(8)
                .map(|p| u64::from_le_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_i64_be(&self, text: &str) -> Result<Vec<i64>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 8 != 0 {
            Err(ByteFormatError("data must be in blocks of eight bytes"))
        } else {
            Ok(bytes
                .chunks_exact(8)
                .map(|p| i64::from_be_bytes(p.try_into().unwrap()))
                .collect_vec())
        }
    }

    pub fn text_to_i64_le(&self, text: &str) -> Result<Vec<i64>, ByteFormatError> {
        let bytes = self.text_to_bytes(text)?;

        if bytes.len() % 8 != 0 {
            Err(ByteFormatError("data must be in blocks of eight bytes"))
        } else {
            Ok(bytes
                .chunks_exact(8)
                .map(|p| i64::from_le_bytes(p.try_into().unwrap()))
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

    pub fn i32_slice_to_text_be<T: AsRef<[i32]>>(&self, nums: T) -> String {
        self.byte_slice_to_text(
            nums.as_ref()
                .iter()
                .flat_map(|n| n.to_be_bytes())
                .collect_vec(),
        )
    }

    pub fn i32_slice_to_text_le<T: AsRef<[i32]>>(&self, nums: T) -> String {
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

    pub fn i64_slice_to_text_be<T: AsRef<[i64]>>(&self, nums: T) -> String {
        self.byte_slice_to_text(
            nums.as_ref()
                .iter()
                .flat_map(|n| n.to_be_bytes())
                .collect_vec(),
        )
    }

    pub fn i64_slice_to_text_le<T: AsRef<[i64]>>(&self, nums: T) -> String {
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

macro_rules! filler_and_maker {
    ($n1: ident, $n2: ident, $n3: ident, $n4: ident,  $n5: ident,  $n6: ident, $t: ty, $w: literal) => {
        pub fn $n1(target: &mut [$t], bytes: &[u8]) {
            for (elem, chunk) in target.iter_mut().zip_eq(bytes.chunks_exact($w)) {
                *elem = <$t>::from_be_bytes(chunk.try_into().unwrap());
            }
        }

        pub fn $n2<const N: usize>(bytes: &[u8]) -> [$t; N] {
            let mut out = [0; N];
            for (elem, chunk) in out.iter_mut().zip_eq(bytes.chunks_exact($w)) {
                *elem = <$t>::from_be_bytes(chunk.try_into().unwrap());
            }
            out
        }

        pub fn $n3(target: &mut [u8], words: &[$t]) {
            for (chunk, word) in target.chunks_exact_mut($w).zip_eq(words) {
                chunk.copy_from_slice(&word.to_be_bytes());
            }
        }

        pub fn $n4(target: &mut [$t], bytes: &[u8]) {
            for (elem, chunk) in target.iter_mut().zip_eq(bytes.chunks_exact($w)) {
                *elem = <$t>::from_le_bytes(chunk.try_into().unwrap());
            }
        }

        pub fn $n5<const N: usize>(bytes: &[u8]) -> [$t; N] {
            let mut out = [0; N];
            for (elem, chunk) in out.iter_mut().zip_eq(bytes.chunks_exact($w)) {
                *elem = <$t>::from_le_bytes(chunk.try_into().unwrap());
            }
            out
        }

        pub fn $n6(target: &mut [u8], words: &[$t]) {
            for (chunk, word) in target.chunks_exact_mut($w).zip_eq(words) {
                chunk.copy_from_slice(&word.to_le_bytes());
            }
        }
    };
}

filler_and_maker!(
    fill_u16s_be,
    make_u16s_be,
    u16s_to_bytes_be,
    fill_u16s_le,
    make_u16s_le,
    u16s_to_bytes_le,
    u16,
    2
);

filler_and_maker!(
    fill_u32s_be,
    make_u32s_be,
    u32s_to_bytes_be,
    fill_u32s_le,
    make_u32s_le,
    u32s_to_bytes_le,
    u32,
    4
);

filler_and_maker!(
    fill_i32s_be,
    make_i32s_be,
    i32s_to_bytes_be,
    fill_i32s_le,
    make_i32s_le,
    i32s_to_bytes_le,
    i32,
    4
);

filler_and_maker!(
    fill_u64s_be,
    make_u64s_be,
    u64s_to_bytes_be,
    fill_u64s_le,
    make_u64s_le,
    u64s_to_bytes_le,
    u64,
    8
);

filler_and_maker!(
    fill_i64s_be,
    make_i64s_be,
    i64s_to_bytes_be,
    fill_i64s_le,
    make_i64s_le,
    i64s_to_bytes_le,
    i64,
    8
);

filler_and_maker!(
    fill_u128s_be,
    make_u128s_be,
    u128s_to_bytes_be,
    fill_u128s_le,
    make_u128s_le,
    u128s_to_bytes_le,
    u128,
    16
);

/// If target is longer it is only partially overwritten. If target is shorter the extra source is ignored.
pub fn overwrite_bytes(target: &mut [u8], source: &[u8]) {
    for (t, s) in target.iter_mut().zip(source.iter()) {
        *t = *s
    }
}

/// If target is longer it is only partially XORed. If target is shorter the extra source is ignored.
pub fn xor_into_bytes(target: &mut [u8], source: &[u8]) {
    for (t, s) in target.iter_mut().zip(source.iter()) {
        *t ^= *s
    }
}
