use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

pub struct Crypt {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Crypt {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for Crypt {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    crate::hash_bytes_from_string! {}
}
