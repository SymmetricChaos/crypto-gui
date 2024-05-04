use super::chacha::ChaCha;
use crate::{Cipher, CipherError};
use num::{BigUint, Zero};

// https://datatracker.ietf.org/doc/html/rfc8439
pub struct ChaCha20Poly1305 {
    pub cipher: ChaCha,
    pub associated_data: Vec<u8>,
}

impl Default for ChaCha20Poly1305 {
    fn default() -> Self {
        Self {
            cipher: ChaCha::default(),
            associated_data: Vec::new(),
        }
    }
}

impl ChaCha20Poly1305 {
    fn create_tag(&self, encrypted_bytes: &[u8]) -> Vec<u8> {
        // The r key will be restricted within the hash invocation
        let keys: ([u8; 16], [u8; 16]) = {
            let v = self.cipher.encrypt_bytes(&[0; 32]);
            (v[0..16].try_into().unwrap(), v[16..].try_into().unwrap())
        };
        let inputs = self.tag_input(encrypted_bytes);
        self.hash(&inputs, keys.0, keys.1)
    }

    // Hash the *encrypted* message, associated data, and padding
    fn tag_input(&self, encrypted_bytes: &[u8]) -> Vec<u8> {
        let mut concat = self.associated_data.clone();
        while concat.len() % 16 != 0 {
            concat.push(0x00);
        }
        concat.extend_from_slice(&encrypted_bytes);
        while concat.len() % 16 != 0 {
            concat.push(0x00);
        }
        concat.extend_from_slice(&(self.associated_data.len() as u64).to_le_bytes());
        concat.extend_from_slice(&(encrypted_bytes.len() as u64).to_le_bytes());
        concat
    }

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
            accumulator += BigUint::from_bytes_be(&block);
            accumulator *= &key;
            accumulator %= &modulus;
        }

        // Final step
        if last_block.len() != 0 {
            accumulator += BigUint::from_bytes_be(&last_block);
            accumulator *= &key;

            accumulator %= &modulus;
        }

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
        // Create encrypted bytes
        let bytes = self
            .cipher
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let encrypted_bytes = self.cipher.encrypt_bytes_with_ctr(&bytes, 1);

        // The r key is restricted within the hash invocation
        // Put the tag first for simplicity when decoding
        let mut tag = self.create_tag(&encrypted_bytes);
        tag.extend_from_slice(&encrypted_bytes);

        Ok(self.cipher.output_format.byte_slice_to_text(&tag))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let message = self
            .cipher
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        // Split the tag and the encrypted message
        let (message_tag, encrypted_bytes) = message.split_at(16);

        if message_tag != self.create_tag(&encrypted_bytes) {
            return Err(CipherError::input("message failed authentication"));
        }

        // ChaCha is reciprocal
        let decrypted_bytes = self.cipher.encrypt_bytes_with_ctr(&encrypted_bytes, 1);
        Ok(self
            .cipher
            .output_format
            .byte_slice_to_text(&decrypted_bytes))
    }
}

#[cfg(test)]
mod chacha_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "01020304050607080910111213141516";
        let cipher = ChaCha20Poly1305::default();

        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
