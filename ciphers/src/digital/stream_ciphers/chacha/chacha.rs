use crate::{Cipher, CipherError};
use utils::byte_formatting::{fill_u32s_le, ByteFormat};

use super::ChaChaState;

pub struct ChaCha {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u32; 8],
    pub nonce: [u32; 2],
    pub rounds: u8,
    pub ctr: u64,
}

impl Default for ChaCha {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,

            // default for key and nonce taken from test vector here: https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
            key: [
                0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
                0x1f1e1d1c,
            ],
            nonce: [0x03020100, 0x07060504],
            rounds: 20,
            ctr: 0,
        }
    }
}

impl ChaCha {
    pub fn key_and_nonce(&mut self, key: [u8; 32], nonce: [u8; 16]) {
        fill_u32s_le(&mut self.key, &key);
        fill_u32s_le(&mut self.nonce, &nonce);
    }

    pub fn with_key_and_nonce(mut self, key: [u8; 32], nonce: [u8; 16]) -> Self {
        self.key_and_nonce(key, nonce);
        self
    }

    pub fn create_state(&self, ctr: u64) -> [u32; 16] {
        [
            0x61707865,
            0x3320646e,
            0x79622d32,
            0x6b206574,
            self.key[0],
            self.key[1],
            self.key[2],
            self.key[3],
            self.key[4],
            self.key[5],
            self.key[6],
            self.key[7],
            ctr as u32,
            (ctr >> 32) as u32,
            self.nonce[0],
            self.nonce[1],
        ]
    }

    pub fn block_function(&self, state: &mut ChaChaState, block: &mut [u8; 64], ctr: u64) {
        // Mix the counter into the state
        state[12] = ctr as u32; // low bits, "as" cast truncates
        state[13] = (ctr >> 32) as u32; // high bits

        // Temporary state
        let mut t_state = state.clone();

        // Only ChaCha20, ChaCha12, and ChaCha8 are official but any number is usable
        for _round in 0..self.rounds / 2 {
            t_state.double_round();
        }
        if self.rounds % 2 == 1 {
            t_state.column_round();
        }

        // Add the current state into the temporary state
        for (i, word) in t_state.0.iter_mut().enumerate() {
            *word = word.wrapping_add(state[i])
        }

        // Create a byte stream
        for (i, b) in t_state.0.iter().flat_map(|w| w.to_le_bytes()).enumerate() {
            block[i] = b
        }
    }

    // Create a key_stream with the specified number of blocks and with the counter started at a particular value
    pub fn key_stream_with_ctr(&self, blocks: u64, ctr: u64) -> Vec<u8> {
        let mut ctr = ctr;
        let mut out = Vec::with_capacity((blocks * 64) as usize);
        let mut key_stream = [0; 64];
        let mut state = ChaChaState::new(self.create_state(ctr));

        for _ in 0..blocks {
            self.block_function(&mut state, &mut key_stream, ctr);
            out.extend(key_stream);
            ctr = ctr.wrapping_add(1);
        }

        out
    }

    // Encrypt a message with the counter started at a particular value
    pub fn encrypt_bytes_with_ctr(&self, bytes: &[u8], ctr: u64) -> Vec<u8> {
        let mut ctr = ctr;
        let mut out = Vec::new();
        let mut key_stream = [0; 64];
        let mut state = ChaChaState::new(self.create_state(ctr));

        for block in bytes.chunks(64) {
            // Insert the counter into the state
            self.block_function(&mut state, &mut key_stream, ctr);
            out.extend(key_stream);
            ctr = ctr.wrapping_add(1);

            // XOR the keystream into the message bytes
            for (input_byte, key_byte) in block.iter().zip(key_stream) {
                out.push(*input_byte ^ key_byte)
            }
        }

        out
    }

    // Encrypt a message with the counter started at a particular value
    pub fn encrypt_bytes_with_ctr_mut(&self, bytes: &mut [u8], ctr: u64) {
        let mut ctr = ctr;
        let mut key_stream = [0; 64];
        let mut state = ChaChaState::new(self.create_state(ctr));

        for block in bytes.chunks_mut(64) {
            // Insert the counter into the state
            self.block_function(&mut state, &mut key_stream, ctr);
            ctr = ctr.wrapping_add(1);

            // XOR the keystream into the message bytes
            for (input_byte, key_byte) in block.iter_mut().zip(key_stream) {
                *input_byte ^= key_byte
            }
        }
    }

    // Encrypt a message with the counter started at the stored value
    pub fn encrypt_bytes(&self, bytes: &mut [u8]) {
        self.encrypt_bytes_with_ctr_mut(bytes, self.ctr)
    }
}

crate::impl_cipher_for_stream_cipher!(ChaCha);

#[cfg(test)]
mod chacha_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "0102030405060708";
        let cipher = ChaCha::default();

        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    #[test]
    fn key_stream_test_empty() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let mut cipher = ChaCha::default();
        cipher.key = [0, 0, 0, 0, 0, 0, 0, 0];
        cipher.nonce = [0, 0];

        let mut key_stream = [0u8; 64];
        cipher.encrypt_bytes(&mut key_stream);

        assert_eq!(
            key_stream.to_vec(),
            ByteFormat::Hex.text_to_bytes("76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586").unwrap()
        );
    }

    #[test]
    fn key_stream_test_default() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let cipher = ChaCha::default();

        let mut key_stream = [0u8; 256];
        cipher.encrypt_bytes(&mut key_stream);

        assert_eq!(
            key_stream.to_vec(),
            ByteFormat::Hex.text_to_bytes("f798a189f195e66982105ffb640bb7757f579da31602fc93ec01ac56f85ac3c134a4547b733b46413042c9440049176905d3be59ea1c53f15916155c2be8241a38008b9a26bc35941e2444177c8ade6689de95264986d95889fb60e84629c9bd9a5acb1cc118be563eb9b3a4a472f82e09a7e778492b562ef7130e88dfe031c79db9d4f7c7a899151b9a475032b63fc385245fe054e3dd5a97a5f576fe064025d3ce042c566ab2c507b138db853e3d6959660996546cc9c4a6eafdc777c040d70eaf46f76dad3979e5c5360c3317166a1c894c94a371876a94df7628fe4eaaf2ccb27d5aaae0ad7ad0f9d4b6ad3b54098746d4524d38407a6deb3ab78fab78c9").unwrap()
        );
    }
}
