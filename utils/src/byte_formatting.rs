use base64::prelude::*;
use itertools::Itertools;
use num::traits::ToBytes;
use paste::paste;
use strum::{Display, EnumIter};

pub const IS_HEX_BYTES: std::cell::LazyCell<regex::Regex> =
    std::cell::LazyCell::new(|| regex::Regex::new(r"^(?:[0-9a-fA-F]{2})+$").unwrap());
pub const IS_BINARY_BYTES: std::cell::LazyCell<regex::Regex> =
    std::cell::LazyCell::new(|| regex::Regex::new(r"^(?:[0-1]{8})+$").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexToBytesError;

// A string containing hex characters converted into bytes
// Bytes are read as pairs of characters from left to right, only an even number of characters are accepted
// "DEADBEEF" -> [222, 173, 190, 239]
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, HexToBytesError> {
    let mut text: String = hex.split_ascii_whitespace().collect();
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

macro_rules! text_to_and_slice_to {
    ($($t:ty),+ $(,)?) => {
        paste! {
            $(
            pub fn [<text_to_ $t _be>](&self, text: &str) -> Result<Vec<$t>, ByteFormatError> {
                let bytes = self.text_to_bytes(text)?;
                let n = (<$t>::BITS / 8) as usize;
                if bytes.len() % n != 0 {
                    Err(ByteFormatError("number of bytes must be a muliple of the type size in bytes"))
                } else {
                    Ok(bytes
                        .chunks_exact(n )
                        .map(|p| $t::from_be_bytes(p.try_into().unwrap()))
                        .collect_vec())
                }
            }

            pub fn [<text_to_ $t _le>](&self, text: &str) -> Result<Vec<$t>, ByteFormatError> {
                let bytes = self.text_to_bytes(text)?;
                let n = (<$t>::BITS / 8) as usize;
                if bytes.len() % n != 0 {
                    Err(ByteFormatError("number of bytes must be a muliple of the type size in bytes"))
                } else {
                    Ok(bytes
                        .chunks_exact(n)
                        .map(|p| $t::from_le_bytes(p.try_into().unwrap()))
                        .collect_vec())
                }
            }

            pub fn [<$t _slice_to_text_be>]<T: AsRef<[$t]>>(&self, nums: T) -> String {
                self.byte_slice_to_text(
                    nums.as_ref()
                        .iter()
                        .flat_map(|n| n.to_be_bytes())
                        .collect_vec(),
                )
            }

            pub fn[<$t _slice_to_text_le>]<T: AsRef<[$t]>>(&self, nums: T) -> String {
                self.byte_slice_to_text(
                    nums.as_ref()
                        .iter()
                        .flat_map(|n| n.to_le_bytes())
                        .collect_vec(),
                )
            }
        )+
        }
    };
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
    #[strum(default)]
    Hex,
    Base64,
    Binary,
    #[strum(to_string = "Text (UTF-8)")]
    Utf8,
}

impl ByteFormat {
    pub fn text_to_bytes(&self, text: &str) -> Result<Vec<u8>, ByteFormatError> {
        if text.len() == 0 {
            return Ok(Vec::new());
        }
        match self {
            ByteFormat::Hex => {
                hex_to_bytes(text).map_err(|_| ByteFormatError("expected hexadecimal"))
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

    text_to_and_slice_to!(u16, i16, u32, i32, u64, i64, u128, i128);

    pub fn text_to_u8(&self, text: &str) -> Result<Vec<u8>, ByteFormatError> {
        self.text_to_bytes(text)
    }

    pub fn text_to_i8(&self, text: &str) -> Result<Vec<i8>, ByteFormatError> {
        self.text_to_bytes(text)
            .map(|v| v.into_iter().map(|n| n as i8).collect_vec())
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
            "deadbeef",
            ByteFormat::Hex.byte_slice_to_text(&[222, 173, 190, 239])
        );
    }
}

macro_rules! fillers_and_makers {
    ($t: ty, $w: literal) => {
        paste! {
                /// Use bytes to fill the target with the type. Panics if target cannot be not exactly filled. Big-endian.
                pub fn [<fill_ $t s_be>]<T: AsRef<[u8]>>(target: &mut [$t], bytes: T) {
                    for (elem, chunk) in target.iter_mut().zip_eq(bytes.as_ref().chunks_exact($w)) {
                        *elem = <$t>::from_be_bytes(chunk.try_into().unwrap());
                    }
                }

                /// Use bytes to make an array filled with the type. Panics if the array cannot be exactly filled. Big-endian.
                pub fn [<make_ $t s_be>]<const N: usize>(bytes: &[u8]) -> [$t; N] {
                    let mut out = [0; N];
                    for (elem, chunk) in out.iter_mut().zip_eq(bytes.chunks_exact($w)) {
                        *elem = <$t>::from_be_bytes(chunk.try_into().unwrap());
                    }
                    out
                }

                /// Take a slice of the type and filled the target with bytes. Panics if the target cannot be exactly filled. Big-endian.
                pub fn [<$t s_to_bytes_be>]<T: AsRef<[$t]>, S: AsMut<[u8]>>(mut target: S, words: T) {
                    for (chunk, word) in target.as_mut().chunks_exact_mut($w).zip_eq(words.as_ref()) {
                        chunk.copy_from_slice(&word.to_be_bytes());
                    }
                }

                /// Use bytes to fill the target with the type. Panics if target cannot be not exactly filled. Little-endian.
                pub fn [<fill_ $t s_le>]<T: AsRef<[u8]>>(target: &mut [$t], bytes: T) {
                    for (elem, chunk) in target.iter_mut().zip_eq(bytes.as_ref().chunks_exact($w)) {
                        *elem = <$t>::from_le_bytes(chunk.try_into().unwrap());
                    }
                }

                /// Use bytes to make an array filled with the type. Panics if the array cannot be exactly filled. Little-endian.
                pub fn [<make_ $t s_le>]<const N: usize>(bytes: &[u8]) -> [$t; N] {
                    let mut out = [0; N];
                    for (elem, chunk) in out.iter_mut().zip_eq(bytes.chunks_exact($w)) {
                        *elem = <$t>::from_le_bytes(chunk.try_into().unwrap());
                    }
                    out
                }

                /// Take a slice of the type and filled the target with bytes. Panics if the target cannot be exactly filled. Little-endian.
                pub fn [<$t s_to_bytes_le>]<T: AsRef<[$t]>, S: AsMut<[u8]>>(mut target: S, words: T) {
                    for (chunk, word) in target.as_mut().chunks_exact_mut($w).zip_eq(words.as_ref()) {
                        chunk.copy_from_slice(&word.to_le_bytes());
                    }
                }
        }
    };
}

// Creates these functions
// fill_Ns_be()
// make_Ns_be()
// Ns_to_bytes_be()
// fill_Ns_le()
// make_Ns_le()
// Ns_to_bytes_le()

fillers_and_makers!(u16, 2);
fillers_and_makers!(i16, 2);
fillers_and_makers!(u32, 4);
fillers_and_makers!(i32, 4);
fillers_and_makers!(u64, 8);
fillers_and_makers!(i64, 8);
fillers_and_makers!(u128, 16);
fillers_and_makers!(i128, 16);

/// If target is longer it is only partially overwritten. If target is shorter the extra source is ignored.
pub fn overwrite_bytes<T: AsMut<[u8]>, S: AsRef<[u8]>>(mut target: T, source: S) {
    for (t, s) in target.as_mut().iter_mut().zip(source.as_ref()) {
        *t = *s
    }
}

// Overwrites target with source and panic if lengths are not equal.
pub fn overwrite_bytes_strict<T: AsMut<[u8]>, S: AsRef<[u8]>>(mut target: T, source: S) {
    for (t, s) in target.as_mut().iter_mut().zip_eq(source.as_ref()) {
        *t = *s
    }
}

/// If target is longer it is only partially XORed. If target is shorter the extra source is ignored.
pub fn xor_into_bytes<T: AsMut<[u8]>, S: AsRef<[u8]>>(mut target: T, source: S) {
    for (t, s) in target.as_mut().iter_mut().zip(source.as_ref()) {
        *t ^= *s
    }
}

// XORs source into target panics if lengths are not equal.
pub fn xor_into_bytes_strict<T: AsMut<[u8]>, S: AsRef<[u8]>>(mut target: T, source: S) {
    for (t, s) in target.as_mut().iter_mut().zip_eq(source.as_ref()) {
        *t ^= *s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_with_u64s() {
        let b = b"SHA3\0\0\0\0";
        let n: [u64; 1] = make_u64s_le(b);
        println!("{:02x?}", b);
        println!("{:016x?}", n);

        let b: [u8; 8] = [0, 0, 0, 0, 0, 0, 1, 0];
        let n: [u64; 1] = make_u64s_le(&b);
        println!("{:02x?}", b);
        println!("{:016x?}", n);
    }
}
