use crate::{errors::HasherError, traits::ClassicHasher};
use num::{BigUint, FromPrimitive, Zero};
use utils::byte_formatting::ByteFormat;

pub struct Poly1305 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u8; 32],
}

impl Default for Poly1305 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key: [0; 32],
        }
    }
}

impl Poly1305 {
    pub fn restrict_key(&mut self) {
        for i in [3, 7, 11, 15] {
            // [3, 7, 11, 15]
            // [29, 25, 21, 17]

            if self.key[i] >= 16 {
                println!("k{} = {:08b} {:02x}", i, self.key[i], self.key[i])
                // panic!("bytes 3, 7, 11, and 15 must be less than 16 (top four bits cleared)",);
            }
            self.key[i] &= 0b11110000;
        }
        for i in [4, 8, 12] {
            // [4, 8, 12]
            // [28, 24, 20]
            if self.key[i] % 4 != 0 {
                println!("k{} = {:08b} {:02x}", i, self.key[i], self.key[i])
                // panic!("bytes 4, 8, 12 must be multiplies of four (bottom two bits cleared)",);
            }
            self.key[i] &= 0b00000011;
        }
    }

    pub fn key_from_string_lossy(&mut self, s: &str) -> Result<(), HasherError> {
        if s.len() != 64 {
            return Err(HasherError::general(
                "key must be given as exactly 64 hex digits",
            ));
        } else {
            if let Ok(v) = ByteFormat::Hex.text_to_bytes(s) {
                self.key = v.try_into().expect("failed to convert Vec<u8> to [u8; 32]");
                self.restrict_key();
            } else {
                return Err(HasherError::general(
                    "key must be given as exactly 64 hex digits",
                ));
            }
        }
        Ok(())
    }
}

impl ClassicHasher for Poly1305 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        // Prime modulus (2**130 - 5) initialized from array
        let modulus = BigUint::from_bytes_be(&[
            0x03_u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xfb,
        ]);
        // let prime = BigUint::from_u32(2)
        //     .unwrap()
        //     .pow(130)
        //     .sub(BigUint::from_u32(5).unwrap());
        // assert_eq!(prime, modulus);

        let key = BigUint::from_bytes_le(&self.key);
        let blocks = bytes.chunks_exact(16);
        let mut accumulator = BigUint::zero();

        // Create and pad the last block. If the remainder is empty it is ignored.
        let mut last_block = blocks.remainder().to_vec();
        if last_block.len() != 0 {
            if last_block.len() != 16 {
                last_block.push(0x01);
            }
            while last_block.len() != 17 {
                last_block.push(0x00);
            }
        }
        last_block.reverse();

        // Message is taken 16 bytes at a time.
        for block in blocks {
            let mut block = block.to_vec();
            block.push(0x01);
            block.reverse();
            println!("main: {:02x?}", &block);
            accumulator += BigUint::from_bytes_le(&block);
            accumulator *= &key;
            accumulator %= &modulus;
        }

        // Final step
        if last_block.len() != 0 {
            println!("last: {:02x?}", &last_block);
            accumulator += BigUint::from_bytes_le(&last_block);
            accumulator *= &key;
            accumulator %= &modulus;
        }

        // Lower 16 bytes
        accumulator %= BigUint::from_u128(u128::MAX).unwrap();

        accumulator.to_bytes_le()
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod poly1305_tests {
    use super::*;

    #[test]
    fn test_zero_input() {
        let mut hasher = Poly1305::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        //https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-00#section-7
        hasher
            .key_from_string_lossy(
                "746869732069732033322d62797465206b657920666f7220506f6c7931333035",
            )
            .unwrap();

        assert_eq!(
            "49ec78090e481ec6c26b33b91ccc0307",
            hasher
                .hash_bytes_from_string(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                )
                .unwrap()
        );
    }

    #[test]
    fn test_input() {
        let mut hasher = Poly1305::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        //https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-00#section-7
        hasher
            .key_from_string_lossy(
                "746869732069732033322d62797465206b657920666f7220506f6c7931333035",
            )
            .unwrap();

        assert_eq!(
            "a6f745008f81c916a20dcc74eef2b2f0",
            hasher
                .hash_bytes_from_string("48656c6c6f20776f726c6421")
                .unwrap()
        );
    }

    //ab0812724a7f1e342742cbed374d94d136c6b8795d45b3819830f2c04491faf0990c62e48b8018b2c3e4a0fa3134cb67fa83e158c994d961c4cb21095c1bf9
    #[test]
    fn test_chunks() {
        let mut hasher = Poly1305::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        /*
        main: [01, d1, 94, 4d, 37, ed, cb, 42, 27, 34, 1e, 7f, 4a, 72, 12, 08, ab]
        main: [01, f0, fa, 91, 44, c0, f2, 30, 98, 81, b3, 45, 5d, 79, b8, c6, 36]
        main: [01, 67, cb, 34, 31, fa, a0, e4, c3, b2, 18, 80, 8b, e4, 62, 0c, 99]
        last: [00, 01, f9, 1b, 5c, 09, 21, cb, c4, 61, d9, 94, c9, 58, e1, 83, fa]
        */
        hasher
                .hash_bytes_from_string("ab0812724a7f1e342742cbed374d94d136c6b8795d45b3819830f2c04491faf0990c62e48b8018b2c3e4a0fa3134cb67fa83e158c994d961c4cb21095c1bf9").unwrap();
    }
}
