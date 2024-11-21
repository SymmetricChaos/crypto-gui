use crate::{errors::CodeError, traits::Code};
use num::Integer;
use utils::byte_formatting::ByteFormat;

fn bytes_to_rle(bytes: &[u8]) -> Vec<u8> {
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

fn rle_to_bytes(bytes: &[u8]) -> Vec<u8> {
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

pub struct RunLengthEncoding {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for RunLengthEncoding {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Code for RunLengthEncoding {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CodeError::input("invalid input bytes"))?;

        Ok(self.output_format.byte_slice_to_text(&bytes_to_rle(&bytes)))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CodeError::input("invalid input bytes"))?;

        if !bytes.len().is_even() {
            return Err(CodeError::input("the rle must be an even number of bytes"));
        }

        Ok(self.output_format.byte_slice_to_text(&rle_to_bytes(&bytes)))
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
        assert_eq!(rle, bytes_to_rle(&bytes));
        assert_eq!(bytes, rle_to_bytes(&rle));
    }

    #[test]
    fn encode_test() {
        let code = RunLengthEncoding::default();
        assert_eq!(ENCODEDTEXT, code.encode(PLAINTEXT).unwrap())
    }

    #[test]
    fn decode_test() {
        let code = RunLengthEncoding::default();
        assert_eq!(PLAINTEXT, code.decode(ENCODEDTEXT).unwrap())
    }
}
