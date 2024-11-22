use crate::{errors::CodeError, traits::Code};
use num::Integer;
use utils::byte_formatting::ByteFormat;

// To be used in a more complex encoding scheme.
// u64 allows recording a single repetition that takes up 18 exabytes and thus should
// avoid ever overflowing
// pub fn u64_leb128(n: u64) -> Vec<u8> {
//     if n == 0 {
//         return vec![0];
//     }
//     let mut n = n;
//     let mut out = Vec::with_capacity(8);
//     while n != 0 {
//         let mut b = (n as u8) & 0x7f;
//         n = n >> 7; 
//         if n != 0 {
//             b |= 0x80;
//         }
//         out.push(b);
//     }
//     out
// }

// pub fn leb128_to_u64<T: AsRef<[u8]>>(v: T) -> u64 {
//     let mut out = 0;
//     let mut shift = 0;
//     for byte in v.as_ref() {
//         out |= ((byte & 0x7f) as u64) << shift;
//         shift += 7;
//     }
//     out
// }

fn bytes_to_rle_one_byte(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut cur = bytes[0];
    let mut ctr = 0_u8;
    for byte in bytes {
        if ctr == 255 {
            out.push(cur);
            out.push(ctr);
            ctr = 0;
        } else if *byte != cur {
            out.push(cur);
            out.push(ctr);
            cur = *byte;
            ctr = 0;
        }
        ctr += 1;
    }
    out.push(cur);
    out.push(ctr);

    out
}

fn rle_to_bytes_one_byte(bytes: &[u8]) -> Vec<u8> {
    if !bytes.len().is_even() {
        panic!("the rle must be an even number of bytes")
    }
    let mut out = Vec::new();
    for chunk in bytes.chunks(2) {
        let byte = chunk[0];
        let ctr = chunk[1];
        for _ in 0..ctr {
            out.push(byte);
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RleMethod {
    OneByte,
    Leb128,
}

pub struct RunLengthEncodingBytes {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub method: RleMethod,
}

impl Default for RunLengthEncodingBytes {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            method: RleMethod::OneByte,
        }
    }
}

impl RunLengthEncodingBytes {

    fn compress(&self, bytes: &[u8]) -> Vec<u8> {
        match self.method {
            RleMethod::OneByte => bytes_to_rle_one_byte(bytes),
            RleMethod::Leb128 => todo!(),
        }
    }

    fn decompress(&self, bytes: &[u8]) -> Vec<u8> {
        match self.method {
            RleMethod::OneByte => rle_to_bytes_one_byte(bytes),
            RleMethod::Leb128 => todo!(),
        }
    }

}

impl Code for RunLengthEncodingBytes {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CodeError::input("invalid input bytes"))?;

        Ok(self.output_format.byte_slice_to_text(&self.compress(&bytes)))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CodeError::input("invalid input bytes"))?;

        if !bytes.len().is_even() {
            return Err(CodeError::input("the rle must be an even number of bytes"));
        }

        Ok(self.output_format.byte_slice_to_text(&self.decompress(&bytes)))
    }
}

#[cfg(test)]
mod rle_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ffffffffababab0000000000";
    const ENCODEDTEXT: &'static str = "ff04ab030005";

    #[test]
    fn check_overflow() {
        let bytes = vec![0_u8; 300];
        let rle = vec![0, 255, 0, 45];
        assert_eq!(rle, bytes_to_rle_one_byte(&bytes));
        assert_eq!(bytes, rle_to_bytes_one_byte(&rle));
    }

    #[test]
    fn encode_test() {
        let code = RunLengthEncodingBytes::default();
        assert_eq!(ENCODEDTEXT, code.encode(PLAINTEXT).unwrap())
    }

    #[test]
    fn decode_test() {
        let code = RunLengthEncodingBytes::default();
        assert_eq!(PLAINTEXT, code.decode(ENCODEDTEXT).unwrap())
    }
}
