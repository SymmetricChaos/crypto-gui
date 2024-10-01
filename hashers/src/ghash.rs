use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

const R: u128 = 0xE1000000000000000000000000000000;

// Multiplication in the Galois field used for GHASH. Addition in the same is XOR.
// This implementation is not optimized at all because this project seeks clarity not real world use.
pub fn mult_gf(x: u128, y: u128) -> u128 {
    let mut out = 0;
    let mut v = x;
    for i in 0..128 {
        // Take bits of y from the left
        if y >> (127 - i) & 1 == 1 {
            out ^= v
        }
        // Check rightmost bit of v
        if v & 1 == 0 {
            v >>= 1;
        } else {
            v >>= 1;
            v ^= R;
        }
    }
    out
}

#[derive(Debug, Clone)]
pub struct Ghash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: u128,
    pub iv: [u8; 12],
}

impl Default for Ghash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: 0,
            iv: [0; 12],
        }
    }
}

impl Ghash {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn iv(mut self, iv: [u8; 12]) -> Self {
        self.iv = iv;
        self
    }

    pub fn key(mut self, key: u128) -> Self {
        self.key = key;
        self
    }

    pub fn ksa(mut self, key: [u8; 16]) -> Self {
        self.key = u128::from_be_bytes(key);
        self
    }

    pub fn padded_iv(&self) -> u128 {
        let mut padded_iv: u128 = 1;
        for i in 0..12 {
            let b = (self.iv[i] as u128) << (32 + (11 - i) * 8);
            padded_iv |= b;
        }
        padded_iv
    }
}

impl ClassicHasher for Ghash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut acc: u128 = 0;
        for block in bytes.chunks_exact(16) {
            let b = u128::from_be_bytes(block.try_into().unwrap());
            acc ^= mult_gf(b, self.key);
        }
        acc ^= self.padded_iv();
        acc.to_be_bytes().into()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod ghash_tests {
    use super::*;

    #[test]
    fn padded_iv() {
        let hasher = Ghash::default().iv([
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
        ]);
        assert_eq!(0x112233445566778899aabbcc00000001, hasher.padded_iv())
    }

    #[test]
    fn multiply() {
        let z = mult_gf(
            0x0388DACE60B6A392F328C2B971B2FE78,
            0x66E94BD4EF8A2C3B884CFA59CA342B2E,
        );
        assert_eq!(0x5E2EC746917062882C85B0685353DEB7, z);
    }
}
