use crypto_bigint::U1024;
use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

pub struct Vsh {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub n: U1024,
}

impl Default for Vsh {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            n: Default::default(),
        }
    }
}

impl ClassicHasher for Vsh {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut x = U1024::ONE;

        todo!()
    }

    crate::hash_bytes_from_string! {}
}
