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
        if self.signed {
            let n = i64::from_str_radix(text, 10).map_err(|e| CodeError::input("invalid i64"))?;
            Ok(self.mode.byte_slice_to_text(i64_leb128(n)))
        } else {
            let n = u64::from_str_radix(text, 10).map_err(|e| CodeError::input("invalid u64"))?;
            Ok(self.mode.byte_slice_to_text(u64_leb128(n)))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod leb128_tests {
    use super::*;

    #[test]
    fn test_unsigned() {
        assert_eq!(vec![0xe5, 0x8e, 0x26], u64_leb128(624485))
    }

    #[test]
    fn test_signed() {
        assert_eq!(vec![0xc0, 0xbb, 0x78], i64_leb128(-123456))
    }
}
