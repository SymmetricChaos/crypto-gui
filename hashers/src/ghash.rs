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

// Add the bytes of block to the accumulator (this is XOR in the Galois Field) then multiply by the value h
// This is used to implement Horner's Rule for evaluating a polynomial
pub fn add_mul(acc: &mut u128, block: &[u8], h: u128) {
    for (i, byte) in block.iter().enumerate() {
        *acc ^= (*byte as u128) << (15 - i) * 8
    }
    *acc = mult_gf(*acc, h);
}

#[derive(Debug, Clone)]
pub struct Ghash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub h: u128,     // usually determined by a cipher
    pub ad_len: u64, // how many bytes of input to treat as additional data
}

impl Default for Ghash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            h: 0,
            ad_len: 0,
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

    pub fn h(mut self, h: u128) -> Self {
        self.h = h;
        self
    }

    pub fn h_bytes(mut self, h: [u8; 16]) -> Self {
        self.h = u128::from_be_bytes(h);
        self
    }

    pub fn ad_len(mut self, ad_len: u64) -> Self {
        self.ad_len = ad_len;
        self
    }
}

impl ClassicHasher for Ghash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut acc: u128 = 0;

        // In an AEAD cipher the input would be treated as Addition Data and Ciphertext
        let (ad, ctext) = bytes.split_at(self.ad_len as usize);

        for block in ad.chunks(16) {
            add_mul(&mut acc, block, self.h);
        }

        for block in ctext.chunks(16) {
            add_mul(&mut acc, block, self.h);
        }

        // XOR in the length of the addition data and the length of the ciphertext
        acc ^= ((self.ad_len * 8) as u128) << 64;
        acc ^= (ctext.len() * 8) as u128;
        // One more multiplication
        acc = mult_gf(acc, self.h);

        acc.to_be_bytes().into()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod ghash_tests {
    use super::*;

    #[test]
    fn multiply() {
        let z = mult_gf(
            0x0388DACE60B6A392F328C2B971B2FE78,
            0x66E94BD4EF8A2C3B884CFA59CA342B2E,
        );
        assert_eq!(0x5E2EC746917062882C85B0685353DEB7, z);
    }
}

crate::basic_hash_tests!(
    test1,
    Ghash::default().h(0x66e94bd4ef8a2c3b884cfa59ca342b2e),
    "",
    "00000000000000000000000000000000";

    test2,
    Ghash::default().input(ByteFormat::Hex).h(0x66e94bd4ef8a2c3b884cfa59ca342b2e),
    "0388dace60b6a392f328c2b971b2fe78",
    "f38cbb1ad69223dcc3457ae5b6b0f885";

    test3,
    Ghash::default().input(ByteFormat::Hex).h(0xb83b533708bf535d0aa6e52980d53b78),
    "42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091473f5985",
    "7f1b32b81b820d02614f8895ac1d4eac";

    test4,
    Ghash::default().input(ByteFormat::Hex).h(0xb83b533708bf535d0aa6e52980d53b78).ad_len(20),
    "feedfacedeadbeeffeedfacedeadbeefabaddad242831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091",
    "698e57f70e6ecc7fd9463b7260a9ae5f";
);
