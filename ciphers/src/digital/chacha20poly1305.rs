use super::chacha::ChaCha;
use crate::{Cipher, CipherError};
use num::{BigUint, Zero};

// https://datatracker.ietf.org/doc/html/rfc8439
pub struct ChaCha20Poly1305 {
    pub cipher: ChaCha,
    pub associated_data: Vec<u8>,
    pub ctr: u64,
}

impl Default for ChaCha20Poly1305 {
    fn default() -> Self {
        Self {
            cipher: ChaCha::default(),
            associated_data: Vec::new(),
            ctr: 0,
        }
    }
}

impl ChaCha20Poly1305 {
    fn create_tag(&self, encrypted_bytes: &[u8]) -> Vec<u8> {
        // The r key will be restricted within the hash invocation
        let mut keys: ([u8; 16], [u8; 16]) = {
            let v = self
                .cipher
                .encrypt_bytes_with_ctr(&[0; 32], self.ctr.rotate_left(32));
            (v[0..16].try_into().unwrap(), v[16..].try_into().unwrap())
        };

        // Restrict key_r, the point where the polynomial is evaluated
        //  r[3], r[7], r[11], and r[15] are required to have their top four bits clear (be smaller than 16)
        for i in [3, 7, 11, 15] {
            keys.0[i] &= 0b00001111;
        }
        // r[4], r[8], and r[12] are required to have their bottom two bits clear (be divisible by 4)
        for i in [4, 8, 12] {
            keys.0[i] &= 0b11111100;
        }
        // Reverse the bytes
        // keys.0.reverse();
        // keys.1.reverse();

        // println!("key r: {:02x?}", keys.0);
        // println!("key s: {:02x?}", keys.1);

        let inputs = self.tag_input(encrypted_bytes);
        self.hash(&inputs, keys.0, keys.1)
    }

    // Hash the *encrypted* message, associated data, and padding
    fn tag_input(&self, encrypted_bytes: &[u8]) -> Vec<u8> {
        let mut input = self.associated_data.clone();
        while input.len() % 16 != 0 {
            input.push(0x00);
        }
        input.extend_from_slice(&encrypted_bytes);
        while input.len() % 16 != 0 {
            input.push(0x00);
        }
        input.extend_from_slice(&(self.associated_data.len() as u64).to_le_bytes());
        input.extend_from_slice(&(encrypted_bytes.len() as u64).to_le_bytes());

        for line in input.chunks(16) {
            println!("{:02x?}", line);
        }

        input
    }

    // We expect key_r to be correctly clamped
    fn hash(&self, bytes: &[u8], key_r: [u8; 16], key_s: [u8; 16]) -> Vec<u8> {
        // Prime modulus (2**130 - 5) initialized from array
        let modulus = BigUint::from_bytes_be(&[
            0x03_u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xfb,
        ]);

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

        // println!("{:02x?}", &out[0..16]);

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
        let encrypted_bytes = self
            .cipher
            .encrypt_bytes_with_ctr(&bytes, (self.ctr + 1).rotate_left(32));

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
        let decrypted_bytes = self
            .cipher
            .encrypt_bytes_with_ctr(&encrypted_bytes, (self.ctr + 1).rotate_left(32));
        Ok(self
            .cipher
            .output_format
            .byte_slice_to_text(&decrypted_bytes))
    }
}

#[cfg(test)]
mod chacha_tests {

    use itertools::Itertools;
    use utils::byte_formatting::ByteFormat;

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "01020304050607080910111213141516";
        let cipher = ChaCha20Poly1305::default();

        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    #[test]
    fn encrypt_test() {
        // https://datatracker.ietf.org/doc/html/rfc8439#section-2.8.2
        let ptext = "Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
        let aad = vec![
            0x50, 0x51, 0x52, 0x53, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
        ];
        let mut cipher = ChaCha20Poly1305::default();
        cipher.cipher.input_format = ByteFormat::Utf8;
        cipher.associated_data = aad;
        cipher.cipher.key = [
            0x80818283_u32,
            0x84858687,
            0x88898a8b,
            0x8c8d8e8f,
            0x90919293,
            0x94959697,
            0x98999a9b,
            0x9c9d9e9f,
        ]
        .iter()
        .map(|n| n.to_be())
        .collect_vec()
        .try_into()
        .unwrap();
        cipher.cipher.nonce = [0x40414243_u32, 0x44454647]
            .iter()
            .map(|n| n.to_be())
            .collect_vec()
            .try_into()
            .unwrap();
        cipher.ctr = 7;

        // Checked that these are correct
        // let ptext_bytes = ByteFormat::Utf8.text_to_bytes(&ptext).unwrap();
        // println!("ptext_bytes: {:02x?}", ptext_bytes);

        // This does not match
        let key_stream = cipher.cipher.key_stream_with_ctr(2, 7 << 32);
        println!("key_stream: {:02x?}", key_stream);

        let mut ctext = cipher.encrypt(ptext).unwrap();
        let rem = ctext.split_off(32);

        println!("{ctext}");
        println!("{rem}");
    }
}
