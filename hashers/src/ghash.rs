use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

pub fn mulx(block: [u8; 16]) -> [u8; 16] {
    let mut v = u128::from_le_bytes(block);
    let v_hi = v >> 127;

    v <<= 1;
    v ^= v_hi ^ (v_hi << 127) ^ (v_hi << 126) ^ (v_hi << 121);
    v.to_le_bytes().into()
}

#[derive(Debug, Clone)]
pub struct Ghash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: u128,
}

impl Default for Ghash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: 0,
        }
    }
}

impl Ghash {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for Ghash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    crate::hash_bytes_from_string! {}
}
