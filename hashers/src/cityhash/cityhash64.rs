// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

use super::helpers::{hash128_64, hash64_0_to_16, hash64_17_to_32, hash64_33_to_64, hash64_65, P2};

pub struct CityHash64 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub seeds: Option<[u64; 2]>,
}

impl Default for CityHash64 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            seeds: None,
        }
    }
}

impl CityHash64 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seeds = Some([P2, seed]);
        self
    }

    pub fn with_seeds(mut self, seed0: u64, seed1: u64) -> Self {
        self.seeds = Some([seed0, seed1]);
        self
    }
}

impl ClassicHasher for CityHash64 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let h = match bytes.len() {
            0..=16 => hash64_0_to_16(bytes),
            17..=32 => hash64_17_to_32(bytes),
            33..=64 => hash64_33_to_64(bytes),
            _ => hash64_65(bytes),
        };
        if let Some([s0, s1]) = self.seeds {
            hash128_64(h.wrapping_sub(s0), s1)
        } else {
            h
        }
        .to_be_bytes()
        .to_vec()
    }

    crate::hash_bytes_from_string! {}
}
