use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

#[derive(Debug, Clone)]
pub struct Gost {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Gost {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for Gost {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    crate::hash_bytes_from_string! {}
}
