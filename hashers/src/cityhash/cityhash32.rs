// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

use super::helpers::{hash32_0_to_4, hash32_13_to_24, hash32_25, hash32_5_to_12};

pub struct CityHash32 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for CityHash32 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl CityHash32 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for CityHash32 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            0..=4 => hash32_0_to_4(bytes),
            5..=12 => hash32_5_to_12(bytes),
            13..=24 => hash32_13_to_24(bytes),
            _ => hash32_25(bytes),
        }
        .to_be_bytes()
        .to_vec()
    }

    crate::hash_bytes_from_string! {}
}
