use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

pub struct Fletcher16 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Fletcher16 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for Fletcher16 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut c0: u16 = 0;
        let mut c1: u16 = 0;

        for byte in bytes {
            c0 = (c0 + *byte as u16) % 255;
            c1 = (c1 + c0 as u16) % 255;
        }

        vec![c1 as u8, c0 as u8]
    }

    crate::hash_bytes_from_string! {}
}

pub struct Fletcher32 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Fletcher32 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for Fletcher32 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut c0: u32 = 0;
        let mut c1: u32 = 0;

        for byte in bytes {
            c0 = (c0 + *byte as u32) % 65535;
            c1 = (c1 + c0 as u32) % 65535;
        }

        [c1 as u16, c0 as u16]
            .into_iter()
            .flat_map(|w| w.to_be_bytes())
            .collect()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1,
    Fletcher16::default(),
    "abcde",
    "c8f0";

    test2,
    Fletcher16::default(),
    "abcdef",
    "2057";

    test3,
    Fletcher16::default(),
    "abcdefgh",
    "0627";
);
