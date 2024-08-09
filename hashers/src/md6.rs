use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

#[derive(Debug, Clone)]
pub struct Md6 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Md6 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Md6 {}

impl ClassicHasher for Md6 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod md6_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Md6::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        assert_eq!("", hasher.hash_bytes_from_string("").unwrap());
        assert_eq!("", hasher.hash_bytes_from_string("a").unwrap());
    }
}
