use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

pub struct BsdChecksum {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for BsdChecksum {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl BsdChecksum {}

impl ClassicHasher for BsdChecksum {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut out = 0_u16;
        for byte in bytes {
            out = out.rotate_right(1);
            out = out.wrapping_add(*byte as u16);
        }
        out.to_le_bytes().to_vec()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod bsd_tests {
    use super::*;

    #[test]
    fn test() {
        let mut hasher = BsdChecksum::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
    }
}
