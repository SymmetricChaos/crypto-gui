// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use std::ops::BitXor;

use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

// 64-bit primes
const P0: u64 = 0xc3a5c85c97cb3127;
const P1: u64 = 0xb492b66fbe98f273;
const P2: u64 = 0x9ae16a3b2f90404f;

fn fetch_u64(bytes: &[u8], p: usize) -> u64 {
    u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap())
}

fn hash64_0_to_16(bytes: &[u8]) -> u64 {
    todo!()
}

fn hash64_17_to_32(bytes: &[u8]) -> u64 {
    todo!()
}

fn hash64_33_to_64(bytes: &[u8]) -> u64 {
    todo!()
}

fn hash32_65(bytes: &[u8]) -> u64 {
    todo!()
}

pub struct CityHash64 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for CityHash64 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
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
}

impl ClassicHasher for CityHash64 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            0..=16 => hash64_0_to_16(bytes),
            17..=32 => hash64_17_to_32(bytes),
            33..=64 => hash64_33_to_64(bytes),
            _ => hash64_65(bytes),
        }
        .to_be_bytes()
        .to_vec()
    }

    crate::hash_bytes_from_string! {}
}
