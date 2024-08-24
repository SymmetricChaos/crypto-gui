use super::auxiliary::haval_arrays::{D, K2, K3, K4, K5};
use utils::byte_formatting::ByteFormat;

pub struct Haval {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub rounds: u32,
    pub output_length: u32,
}

impl Default for Haval {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rounds: 3,
            output_length: 16,
        }
    }
}
