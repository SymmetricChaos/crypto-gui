use super::chacha_ietf::ChaChaIetf;
use crate::Cipher;
use num::{BigUint, Zero};
use utils::errors::GeneralError;

// https://datatracker.ietf.org/doc/html/rfc8439
pub struct ChaCha20Poly1305 {
    pub cipher: ChaChaIetf,
    pub associated_data: Vec<u8>,
    pub ctr: u32,
}

impl Default for ChaCha20Poly1305 {
    fn default() -> Self {
        Self {
            cipher: ChaChaIetf::default(),
            associated_data: Vec::new(),
            ctr: 0,
        }
    }
}

impl ChaCha20Poly1305 {
    fn create_tag(&self, encrypted_bytes: &[u8]) -> Vec<u8> {
        // The r key will be restricted within the hash invocation
        let mut keys: ([u8; 16], [u8; 16]) = {
            let v = self.cipher.encrypt_bytes_with_ctr(&[0; 32], self.ctr);
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

        input
    }

    // We expect key_r to be correctly clamped
    fn hash(&self, bytes: &[u8], key_r: [u8; 16], key_s: [u8; 16]) -> Vec<u8> {
        // Prime modulus (2**130 - 5) initialized from array
        let modulus = BigUint::from_bytes_be(&[
            0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
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

        out[0..16].to_vec()
    }
}

impl Cipher for ChaCha20Poly1305 {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        // Create encrypted bytes
        let bytes = self
            .cipher
            .input_format
            .text_to_bytes(text)
            .map_err(|_| GeneralError::input("byte format error"))?;
        let mut encrypted_bytes = self.cipher.encrypt_bytes_with_ctr(&bytes, self.ctr + 1);

        // The r key is restricted within the hash invocation
        // Put the tag first for simplicity when decoding
        let tag = self.create_tag(&encrypted_bytes);
        encrypted_bytes.extend_from_slice(&tag);

        Ok(self
            .cipher
            .output_format
            .byte_slice_to_text(&encrypted_bytes))
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let message = self
            .cipher
            .input_format
            .text_to_bytes(text)
            .map_err(|_| GeneralError::input("byte format error"))?;

        if message.len() < 16 {
            return Err(GeneralError::input("authentication tag is missing"));
        }

        // Split the tag and the encrypted message
        let (encrypted_bytes, message_tag) = message.split_at(message.len() - 16);

        if message_tag != self.create_tag(&encrypted_bytes) {
            return Err(GeneralError::input("message failed authentication"));
        }

        // ChaCha is reciprocal
        let decrypted_bytes = self
            .cipher
            .encrypt_bytes_with_ctr(&encrypted_bytes, self.ctr + 1);
        Ok(self
            .cipher
            .output_format
            .byte_slice_to_text(&decrypted_bytes))
    }
}

#[cfg(test)]
mod chacha20_poly1305_tests {

    use itertools::Itertools;
    use utils::byte_formatting::ByteFormat;

    use super::*;

    const PTEXT: &'static str = "Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
    const AAD: [u8; 12] = [
        0x50, 0x51, 0x52, 0x53, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
    ];

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "01020304050607080910111213141516";
        let cipher = ChaCha20Poly1305::default();

        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    // #[test]
    // fn key_stream_test() {
    //     let mut cipher = ChaCha20Poly1305::default();
    //     cipher.cipher.input_format = ByteFormat::Utf8;
    //     cipher.associated_data = AAD.to_vec();
    //     cipher.cipher.key = [
    //         0x80818283_u32,
    //         0x84858687,
    //         0x88898a8b,
    //         0x8c8d8e8f,
    //         0x90919293,
    //         0x94959697,
    //         0x98999a9b,
    //         0x9c9d9e9f,
    //     ]
    //     .iter()
    //     .map(|n| n.to_be())
    //     .collect_vec()
    //     .try_into()
    //     .unwrap();
    //     cipher.cipher.nonce = [0x07000000_u32, 0x40414243, 0x44454647]
    //         .iter()
    //         .map(|n| n.to_be())
    //         .collect_vec()
    //         .try_into()
    //         .unwrap();
    //     cipher.ctr = 0;

    //     let key_stream = cipher.cipher.key_stream_with_ctr(2, 0);
    //     println!("key_stream: {:02x?}", key_stream);
    // }

    #[test]
    fn encrypt_test() {
        // https://datatracker.ietf.org/doc/html/rfc8439#section-2.8.2

        let mut cipher = ChaCha20Poly1305::default();
        cipher.cipher.input_format = ByteFormat::Utf8;
        cipher.associated_data = AAD.to_vec();
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
        cipher.cipher.nonce = [0x07000000_u32, 0x40414243, 0x44454647]
            .iter()
            .map(|n| n.to_be())
            .collect_vec()
            .try_into()
            .unwrap();
        cipher.ctr = 0;

        let ctext = cipher.encrypt(PTEXT).unwrap();
        let (ctext, tag) = ctext.split_at(228);

        // Remaining errors are caused by the second block of ciphertext not encrypting the same as in the test vector

        assert_eq!(tag, "1ae10b594f09e26a7e902ecbd0600691");
        assert_eq!(ctext, "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d63dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b3692ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc3ff4def08e4b7a9de576d26586cec64b6116");
    }
}
