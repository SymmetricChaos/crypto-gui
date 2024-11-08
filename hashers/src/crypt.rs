use std::ops::Shl;

use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;
pub struct CryptDes {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub salt: u32, // only 12 bits used
}

impl Default for CryptDes {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: 0,
        }
    }
}

impl CryptDes {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for CryptDes {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut key: u64 = 0;
        for i in 0..8 {
            key = key << 8;
            if let Some(byte) = bytes.get(i) {
                key |= *byte as u64
            }
        }

        todo!()
    }

    crate::hash_bytes_from_string! {}
}

// crate::basic_hash_tests!(
//     test1,
//     Crypt::default(),
//     "INPUT",
//     "OUTPUT";
// );
