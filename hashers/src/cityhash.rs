use utils::byte_formatting::ByteFormat;

// Prime numbers
const P0: u64 = 0xc3a5c85c97cb3127;
const P1: u64 = 0xb492b66fbe98f273;
const P2: u64 = 0x9ae16a3b2f90404f;

// Constants from Murmur3
const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;

fn fmix(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x85ebca6b);
    x ^= x >> 13;
    x = x.wrapping_mul(0xc2b2ae35);
    x ^= x >> 16;
    x
}

pub struct CityHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for CityHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}
