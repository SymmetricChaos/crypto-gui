use utils::byte_formatting::ByteFormat;

use crate::{errors::CodeError, traits::Code};

const MASK: u8 = 0b01111111;
const HIGH_BIT: u8 = 0b10000000;

// Rust signed types are twos-complement
pub fn i128_leb128(n: i128) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut n = n;
    let mut out = Vec::with_capacity(8);
    loop {
        let b = (n as u8) & MASK;
        n = n >> 7; // for i128 Rust makes this an arithmetic shift
        let sign_clear = ((b >> 6) & 1) == 0;
        if (n == 0 && sign_clear) || (n == -1 && !sign_clear) {
            out.push(b);
            break;
        } else {
            out.push(b | HIGH_BIT);
        }
    }
    out
}

pub fn leb128_to_i128<T: AsRef<[u8]>>(v: T) -> i128 {
    let mut out = 0;
    let mut shift = 0;
    let mut bytes = v.as_ref().iter();
    let mut b: u8;
    loop {
        b = *bytes.next().unwrap();
        out |= ((b & MASK) as i128) << shift;
        shift += 7;
        if ((b >> 7) & 1) == 0 {
            break;
        }
    }
    let sign_set = ((b >> 6) & 1) == 1;
    if (shift < 128) && sign_set {
        out |= !0 << shift
    }
    out
}

pub fn u128_leb128(n: u128) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut n = n;
    let mut out = Vec::with_capacity(8);
    while n != 0 {
        let mut b = (n as u8) & MASK;
        n = n >> 7; // for u128 Rust makes this a logical shift
        if n != 0 {
            b |= HIGH_BIT;
        }
        out.push(b);
    }
    out
}

pub fn leb128_to_u128<T: AsRef<[u8]>>(v: T) -> u128 {
    let mut out = 0;
    let mut shift = 0;
    for byte in v.as_ref() {
        out |= ((byte & MASK) as u128) << shift;
        shift += 7;
    }
    out
}

pub struct Leb128 {
    pub signed: bool,
    pub byte_format: ByteFormat,
    pub spaced: bool,
}

impl Default for Leb128 {
    fn default() -> Self {
        Self {
            signed: false,
            byte_format: ByteFormat::Hex,
            spaced: true,
        }
    }
}

impl Leb128 {
    pub fn byte_format(mut self, byte_format: ByteFormat) -> Self {
        self.byte_format = byte_format;
        self
    }

    pub fn signed(mut self, signed: bool) -> Self {
        self.signed = signed;
        self
    }

    pub fn spaced(mut self, spaced: bool) -> Self {
        self.spaced = spaced;
        self
    }
}

impl Code for Leb128 {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let strs = text.split(',').map(|s| s.trim());

        let out = if self.spaced {
            let mut v = Vec::new();
            if self.signed {
                for s in strs {
                    let n = i128::from_str_radix(s, 10)
                        .map_err(|_| CodeError::input("invalid i128 encountered"))?;
                    v.push(self.byte_format.byte_slice_to_text(i128_leb128(n)));
                }
            } else {
                for s in strs {
                    let n = u128::from_str_radix(s, 10)
                        .map_err(|_| CodeError::input("invalid u128 encountered"))?;
                    v.push(self.byte_format.byte_slice_to_text(u128_leb128(n)));
                }
            }
            v.join(", ")
        } else {
            let mut v = Vec::new();
            if self.signed {
                for s in strs {
                    let n = i128::from_str_radix(s, 10)
                        .map_err(|_| CodeError::input("invalid i128 encountered"))?;
                    v.push(i128_leb128(n));
                }
            } else {
                for s in strs {
                    let n = u128::from_str_radix(s, 10)
                        .map_err(|_| CodeError::input("invalid u128 encountered"))?;
                    v.push(u128_leb128(n));
                }
            }
            self.byte_format.byte_iter_to_text(v.into_iter().flatten())
        };

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut v = Vec::new();

        if self.spaced {
            let strs = text.split(',').map(|s| s.trim());
            if self.signed {
                for s in strs {
                    let bytes = self
                        .byte_format
                        .text_to_bytes(s)
                        .map_err(|_| CodeError::input("invalid bytes"))?;
                    v.push(leb128_to_i128(&bytes).to_string());
                }
            } else {
                for s in strs {
                    let bytes = self
                        .byte_format
                        .text_to_bytes(s)
                        .map_err(|_| CodeError::input("invalid bytes"))?;
                    v.push(leb128_to_u128(&bytes).to_string());
                }
            }
        } else {
            let bytes = self
                .byte_format
                .text_to_bytes(text)
                .map_err(|_| CodeError::input("invalid bytes"))?;

            let groups = bytes.split_inclusive(|b| b >> 7 == 0);

            if self.signed {
                for group in groups {
                    v.push(leb128_to_i128(&group).to_string());
                }
            } else {
                for group in groups {
                    v.push(leb128_to_u128(&group).to_string());
                }
            }
        };

        Ok(v.join(", "))
    }
}

#[cfg(test)]
mod leb128_tests {
    use super::*;

    #[test]
    fn test_unsigned() {
        assert_eq!(vec![0xe5, 0x8e, 0x26], u128_leb128(624485));
        assert_eq!(624485, leb128_to_u128([0xe5, 0x8e, 0x26]))
    }

    #[test]
    fn test_signed() {
        assert_eq!(vec![0xc0, 0xbb, 0x78], i128_leb128(-123456));
        assert_eq!(-123456, leb128_to_i128([0xc0, 0xbb, 0x78]));
    }

    #[test]
    fn encode_decode_unsigned() {
        let x = "0, 12, 345, 6789, 10111, 213141, 5161718, 19202122, 232425262, 7282930313";
        let code = Leb128::default();
        // println!("{:02x?}",&code.encode(x).unwrap());
        assert_eq!(x, &code.decode(&code.encode(x).unwrap()).unwrap());

        let code = Leb128::default().spaced(false);
        // println!("{:02x?}",&code.encode(x).unwrap());
        assert_eq!(x, &code.decode(&code.encode(x).unwrap()).unwrap());
    }

    #[test]
    fn encode_decode_signed() {
        let x = "0, -12, 345, -6789, 10111, -213141, 5161718, -19202122, 232425262, -7282930313";
        let code = Leb128::default().signed(true);
        // println!("{:02x?}",&code.encode(x).unwrap());
        assert_eq!(x, &code.decode(&code.encode(x).unwrap()).unwrap());

        let code = Leb128::default().signed(true).spaced(false);
        // println!("{:02x?}",&code.encode(x).unwrap());
        assert_eq!(x, &code.decode(&code.encode(x).unwrap()).unwrap());
    }
}
