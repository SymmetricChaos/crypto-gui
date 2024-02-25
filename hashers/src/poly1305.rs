use crate::{errors::HasherError, traits::ClassicHasher};
use crypto_bigint::U192;
use num::{pow::Pow, BigUint, FromPrimitive, One};
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
    //const MODULUS: BigUint = 2 ** 130 - 5;
}

impl ClassicHasher for Poly1305 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        // Calculated the prime modulus but simpler to initialize via byte array
        //let modulus = BigUint::from_i32(2).unwrap().pow(130_u32) - BigUint::from_i32(5).unwrap();
        //println!("{:0x?}", modulus.to_bytes_be());
        let modulus = BigUint::from_bytes_be(&[
            0x03_u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xfb,
        ]);

        let key = BigUint::from_bytes_le(&self.key);
        let blocks = bytes.chunks_exact(16);
        let mut accumulator = BigUint::one();

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

        // Message is taken 16 bytes at a time.
        for block in blocks {
            let mut block = block.to_vec();
            block.insert(0, 0x01);
            accumulator += BigUint::from_bytes_le(&block);
            accumulator *= &key;
            accumulator %= &modulus;
        }

        // Final step
        if last_block.len() != 0 {
            accumulator += BigUint::from_bytes_le(&last_block);
            accumulator *= &key;
            accumulator %= &modulus;
        }

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
mod md5_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Poly1305::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        //https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-00#section-4
        hasher.key = [
            116, 104, 105, 115, 32, 105, 115, 32, 51, 50, 45, 98, 121, 116, 101, 32, 107, 101, 121,
            32, 102, 111, 114, 32, 80, 111, 108, 121, 49, 51, 48, 53,
        ];
        assert_eq!(
            "49ec78090e481ec6c26b33b91ccc0307",
            hasher
                .hash_bytes_from_string(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                )
                .unwrap()
        );
    }
}
