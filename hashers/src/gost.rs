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
        let mut input = bytes.to_vec();

        for block in input.chunks_exact(32) {}

        let mut out = vec![0; 32];

        out
    }

    crate::hash_bytes_from_string! {}
}
