use utils::byte_formatting::ByteFormat;

use crate::{errors::CodeError, traits::Code};

const MASK: u8 = 0b01111111;
const HIGH_BIT: u8 = 0b10000000;

pub fn i64_leb128(n: i64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut more = true;
    let mut n = n;
    let mut out = Vec::with_capacity(8);
    while more {
        let mut b = (n as u8) & MASK;
        n = n >> 7; // for i64 Rust makes this an arithmetic shift
        let sign_clear = ((b >> 6) & 1) == 0;
        if (n == 0 && sign_clear) || (n == -1 && !sign_clear) {
            more = false;
        } else {
            if n != 0 {
                b |= HIGH_BIT;
            }
        }
        out.push(b);
    }
    out
}

pub fn leb128_to_i64<T: AsRef<[u8]>>(v: T) -> i64 {
    let mut out = 0;
    let mut shift = 0;
    let size = 64;
    let mut bytes = v.as_ref().iter();
    let mut b: u8;
    loop {
        b = *bytes.next().unwrap();
        out |= ((b & MASK) as i64) << shift;
        shift += 7;
        if ((b >> 7) & 1) == 0 {
            break;
        }
    }
    let sign_set = ((b >> 6) & 1) == 1;
    if (shift < size) && sign_set {
        out |= !0 << shift
    }
    out
}

pub fn u64_leb128(n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut n = n;
    let mut out = Vec::with_capacity(8);
    while n != 0 {
        let mut b = (n as u8) & MASK;
        n = n >> 7; // for u64 Rust makes this a logical shift
        if n != 0 {
            b |= HIGH_BIT;
        }
        out.push(b);
    }
    out
}

pub fn leb128_to_u64<T: AsRef<[u8]>>(v: T) -> u64 {
    let mut out = 0;
    let mut shift = 0;
    for byte in v.as_ref() {
        out |= ((byte & MASK) as u64) << shift;
        shift += 7;
    }
    out
}

pub struct Leb128 {
    signed: bool,
    mode: ByteFormat,
}

impl Default for Leb128 {
    fn default() -> Self {
        Self {
            signed: false,
            mode: ByteFormat::Hex,
        }
    }
}

impl Leb128 {}

impl Code for Leb128 {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let strs = text.split(',').map(|s| s.trim());
        let mut v = Vec::new();
        if self.signed {
            for s in strs {
                let n = i64::from_str_radix(s, 10).map_err(|e| CodeError::input("invalid i64"))?;
                v.push(self.mode.byte_slice_to_text(i64_leb128(n)));
            }
        } else {
            for s in strs {
                let n = u64::from_str_radix(s, 10).map_err(|e| CodeError::input("invalid u64"))?;
                v.push(self.mode.byte_slice_to_text(u64_leb128(n)));
            }
        }
        let out = v.join(", ");
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let strs = text.split(',').map(|s| s.trim());
        let mut v = Vec::new();
        if self.signed {
            for s in strs {
                let bytes = self
                    .mode
                    .text_to_bytes(s)
                    .map_err(|_| CodeError::input("invalid bytes"))?;
                v.push(leb128_to_i64(&bytes).to_string());
            }
        } else {
            for s in strs {
                let bytes = self
                    .mode
                    .text_to_bytes(s)
                    .map_err(|_| CodeError::input("invalid bytes"))?;
                v.push(leb128_to_u64(&bytes).to_string());
            }
        }
        let out = v.join(", ");
        Ok(out)
    }
}

#[cfg(test)]
mod leb128_tests {
    use super::*;

    #[test]
    fn test_unsigned() {
        assert_eq!(vec![0xe5, 0x8e, 0x26], u64_leb128(624485));
        assert_eq!(624485, leb128_to_u64([0xe5, 0x8e, 0x26]));
    }

    #[test]
    fn test_signed() {
        assert_eq!(vec![0xc0, 0xbb, 0x78], i64_leb128(-123456));
        assert_eq!(-123456, leb128_to_i64([0xc0, 0xbb, 0x78]))
    }
}
