use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

const IV: [u32; 8] = [
    0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e,
];

const TJ15: u32 = 0x79cc4519;
const TJ63: u32 = 0x7a879d8a;

fn ff(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (x & z) | (y & z)
    }
}

fn gg(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (!x & z)
    }
}

fn p0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}

fn p1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}

pub struct Sm3 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Sm3 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for Sm3 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Padding and appending length is identical to MD4 and MD5
        // Length in bits before padding
        let b_len = (input.len().wrapping_mul(8)) as u64;

        // Step 1.Padding
        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length in bits is 448 mod 512
        // equivalently until the length in bytes is 56 mod 64
        while (input.len() % 64) != 56 {
            input.push(0)
        }

        // Step 2. Append length
        for b in b_len.to_be_bytes() {
            input.push(b)
        }

        todo!()
    }

    crate::hash_bytes_from_string! {}
}
