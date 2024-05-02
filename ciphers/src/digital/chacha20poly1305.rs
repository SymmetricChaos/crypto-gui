use super::chacha::ChaCha;
use crate::{Cipher, CipherError};
use num::{BigUint, Zero};

// https://datatracker.ietf.org/doc/html/rfc8439
pub struct ChaCha20Poly1305 {
    pub cipher: ChaCha,
}

impl Default for ChaCha20Poly1305 {
    fn default() -> Self {
        Self {
            cipher: ChaCha::default(),
        }
    }
}

impl ChaCha20Poly1305 {
    fn hash(&self, bytes: &[u8], key_r: [u8; 16], key_s: [u8; 16]) -> Vec<u8> {
        // Prime modulus (2**130 - 5) initialized from array
        let modulus = BigUint::from_bytes_be(&[
            0x03_u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xfb,
        ]);

        // Restrict key_r, the point where the polynomial is evaluated
        let mut key_r = key_r;
        for i in [3, 7, 11, 15] {
            key_r[i] &= 0b11110000;
        }
        for i in [4, 8, 12] {
            key_r[i] &= 0b00000011;
        }

        let key = BigUint::from_bytes_le(&key_r);
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
            //println!("main: {:02x?}", &block);
            // println!("main: {}", BigUint::from_bytes_be(&block).to_str_radix(16));
            accumulator += BigUint::from_bytes_be(&block);
            accumulator *= &key;
            accumulator %= &modulus;
        }

        // Final step
        if last_block.len() != 0 {
            //println!("last: {:02x?}", &last_block);
            // println!(
            //     "last: {}",
            //     BigUint::from_bytes_be(&last_block).to_str_radix(16)
            // );
            accumulator += BigUint::from_bytes_be(&last_block);
            accumulator *= &key;

            accumulator %= &modulus;
        }
        // println!("m(r): {}", accumulator.to_str_radix(16));

        accumulator += BigUint::from_bytes_le(&key_s);

        let mut out = accumulator.to_bytes_le();
        while out.len() < 16 {
            out.push(0x00);
        }

        out[0..16].to_vec()
    }
}

impl Cipher for ChaCha20Poly1305 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let keys: ([u8; 16], [u8; 16]) = {
            let v = self.cipher.encrypt_bytes(&[0; 32]);
            (v[0..16].try_into().unwrap(), v[16..].try_into().unwrap())
        };

        // Create encrypted bytes
        let mut bytes = self
            .cipher
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.cipher.encrypt_bytes_with_ctr(&bytes, 1);

        // Hash the encrypted message
        // The r key is restricted within the hash invocation
        let tag = self.hash(&bytes, keys.0, keys.1);
        bytes.extend_from_slice(&tag);

        Ok(self.cipher.output_format.byte_slice_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // Remove the tag and check that it is valid
        todo!();

        // Decrypt the encrypted portion
        todo!();
    }
}
