use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

pub struct Adler32 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Adler32 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for Adler32 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut a: u16 = 1;
        let mut b: u16 = 0;

        for byte in bytes {
            a = a.wrapping_add(*byte as u16);
            b = b.wrapping_add(a);
        }

        [b, a].into_iter().flat_map(|w| w.to_be_bytes()).collect()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1,
    Adler32::default(),
    "Wikipedia",
    "11e60398";
);
