use crate::traits::ClassicHasher;
use std::num::Wrapping;
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone)]
pub struct OneAtATime {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for OneAtATime {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl OneAtATime {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for OneAtATime {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut hash = Wrapping(0_u32);
        for byte in bytes.into_iter() {
            hash += *byte as u32;
            hash += hash << 10;
            hash ^= hash >> 6;
        }
        hash += hash << 3;
        hash ^= hash >> 11;
        hash += hash << 15;
        hash.0.to_be_bytes().to_vec()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod jenkins_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let hasher = OneAtATime::default();
        assert_eq!("ca2e9442", hasher.hash_bytes_from_string("a").unwrap());
        assert_eq!(
            "519e91f5",
            hasher
                .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
                .unwrap()
        );
    }
}
