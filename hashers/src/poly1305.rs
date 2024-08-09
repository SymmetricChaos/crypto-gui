use crate::{errors::HasherError, traits::ClassicHasher};
use num::{BigUint, Zero};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone)]
pub struct Poly1305 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key_r: [u8; 16], // point at which the polynomial is evaluated
    pub key_s: [u8; 16], // nonce that is added at the end
}

impl Default for Poly1305 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key_r: [0; 16],
            key_s: [0; 16],
        }
    }
}

impl Poly1305 {
    pub fn restrict_key_r(&mut self) {
        for i in [3, 7, 11, 15] {
            // The top four bits must be 0
            self.key_r[i] &= 0b00001111;
        }
        for i in [4, 8, 12] {
            // The lower two bits must be 0
            self.key_r[i] &= 0b11111100;
        }
    }

    pub fn key_r_from_string(&mut self, s: &str) -> Result<(), HasherError> {
        if s.len() != 32 {
            return Err(HasherError::key(
                "key must be given as exactly 32 hex digits",
            ));
        } else {
            if let Ok(v) = ByteFormat::Hex.text_to_bytes(s) {
                self.key_r = v.try_into().expect("failed to convert Vec<u8> to [u8; 32]");
                self.restrict_key_r();
            } else {
                return Err(HasherError::key(
                    "key must be given as exactly 32 hex digits",
                ));
            }
        }
        Ok(())
    }

    pub fn key_s_from_string(&mut self, s: &str) -> Result<(), HasherError> {
        if s.len() != 32 {
            return Err(HasherError::key(
                "key must be given as exactly 32 hex digits",
            ));
        } else {
            if let Ok(v) = ByteFormat::Hex.text_to_bytes(s) {
                self.key_s = v.try_into().expect("failed to convert Vec<u8> to [u8; 32]");
            } else {
                return Err(HasherError::key(
                    "key must be given as exactly 32 hex digits",
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

        let key = BigUint::from_bytes_le(&self.key_r);
        // println!("keyr: {}", key.to_str_radix(16));
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
            // println!("main: {:02x?}", &block);
            // println!("main: {}", BigUint::from_bytes_be(&block).to_str_radix(16));
            accumulator += BigUint::from_bytes_be(&block);
            accumulator *= &key;
            accumulator %= &modulus;
        }

        // Final step
        if last_block.len() != 0 {
            // println!("last: {:02x?}", &last_block);
            // println!(
            //     "last: {}",
            //     BigUint::from_bytes_be(&last_block).to_str_radix(16)
            // );
            accumulator += BigUint::from_bytes_be(&last_block);
            accumulator *= &key;

            accumulator %= &modulus;
        }
        // println!("m(r): {}", accumulator.to_str_radix(16));

        accumulator += BigUint::from_bytes_le(&self.key_s);

        let mut out = accumulator.to_bytes_le();
        while out.len() < 16 {
            out.push(0x00);
        }

        out[0..16].to_vec()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod poly1305_tests {
    use super::*;

    // https://cr.yp.to/mac/poly1305-20050329.pdf
    #[test]
    fn test_chunks_1() {
        let mut hasher = Poly1305::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        hasher
            .key_r_from_string("12976a08c4426d0ce8a82407c4f48207")
            .unwrap();
        hasher
            .key_s_from_string("80f8c20aa71202d1e29179cbcb555a57")
            .unwrap();
        /*
        keyr: 782f4c40724a8e80c6d42c4086a9712
        main: 1d1944d37edcb4227341e7f4a721208ab
        main: 1f0fa9144c0f2309881b3455d79b8c636
        main: 167cb3431faa0e4c3b218808be4620c99
        last: 1f91b5c0921cbc461d994c958e183fa
        m(r): 0c3c4f37c464bbd44306c9f8502ea5bd1
        */
        assert_eq!(
            "5154ad0d2cb26e01274fc51148491f1b",
            hasher.hash_bytes_from_string("ab0812724a7f1e342742cbed374d94d136c6b8795d45b3819830f2c04491faf0990c62e48b8018b2c3e4a0fa3134cb67fa83e158c994d961c4cb21095c1bf9").unwrap()
        )
    }

    #[test]
    fn test_chunks_2() {
        let mut hasher = Poly1305::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        hasher
            .key_r_from_string("851fc40c3467ac0be05cc20404f3f700")
            .unwrap();
        hasher
            .key_s_from_string("580b3b0f9447bb1e69d095b5928b6dbc")
            .unwrap();
        /*
        keyr: f7f30404c25ce00bac67340cc41f85
        main: 1f6f3
        m(r): 321e58e25a69d7f8f27060770b3f8bb9c
        */
        assert_eq!(
            "f4c633c3044fc145f84f335cb81953de",
            hasher.hash_bytes_from_string("f3f6").unwrap()
        )
    }
}
