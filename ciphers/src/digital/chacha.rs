use std::num::Wrapping;

use crate::{Cipher, CipherError};
use utils::byte_formatting::ByteFormat;

pub struct ChaCha {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; 8],
    pub nonce: [u32; 2],
    pub rounds: u8,
    pub ctr: u64,
}

impl Default for ChaCha {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
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

    pub fn quarter_round(state: &mut [Wrapping<u32>; 16], a: usize, b: usize, c: usize, d: usize) {
        state[a] += state[b];
        state[d] ^= state[a];
        state[d] = Wrapping(state[d].0.rotate_left(16));

        state[c] += state[d];
        state[b] ^= state[c];
        state[b] = Wrapping(state[b].0.rotate_left(12));

        state[a] += state[b];
        state[d] ^= state[a];
        state[d] = Wrapping(state[d].0.rotate_left(8));

        state[c] += state[d];
        state[b] ^= state[c];
        state[b] = Wrapping(state[b].0.rotate_left(7));
    }

    pub fn column_round(state: &mut [Wrapping<u32>; 16]) {
        Self::quarter_round(state, 0, 4, 8, 12);
        Self::quarter_round(state, 1, 5, 9, 13);
        Self::quarter_round(state, 2, 6, 10, 14);
        Self::quarter_round(state, 3, 7, 11, 15);
    }

    pub fn diag_round(state: &mut [Wrapping<u32>; 16]) {
        Self::quarter_round(state, 0, 5, 10, 15);
        Self::quarter_round(state, 1, 6, 11, 12);
        Self::quarter_round(state, 2, 7, 8, 13);
        Self::quarter_round(state, 3, 4, 9, 14);
    }

    pub fn double_round(state: &mut [Wrapping<u32>; 16]) {
        Self::column_round(state);
        Self::diag_round(state);
    }

    pub fn encrypt_bytes_with_ctr(&self, bytes: &[u8], ctr: u64) -> Vec<u8> {
        let mut ctr = ctr;
        let mut out = Vec::new();
        let mut state = [
            Wrapping(0x61707865),
            Wrapping(0x3320646e),
            Wrapping(0x79622d32),
            Wrapping(0x6b206574),
            Wrapping(self.key[0]),
            Wrapping(self.key[1]),
            Wrapping(self.key[2]),
            Wrapping(self.key[3]),
            Wrapping(self.key[4]),
            Wrapping(self.key[5]),
            Wrapping(self.key[6]),
            Wrapping(self.key[7]),
            Wrapping(0x00000000),
            Wrapping(0x00000000),
            Wrapping(self.nonce[0]),
            Wrapping(self.nonce[1]),
        ];

        for block in bytes.chunks(64) {
            // Mix the counter into the state
            state[12] = Wrapping(ctr as u32); // low bits, "as" cast truncates
            state[13] = Wrapping((ctr >> 32) as u32); // high bits

            // Temporary state
            let mut t_state = state.clone();

            // Only ChaCha20, ChaCha12, and ChaCha8 are official but any number is usable
            for _round in 0..self.rounds / 2 {
                Self::double_round(&mut t_state);
            }
            if self.rounds % 2 == 1 {
                Self::column_round(&mut t_state)
            }

            // XOR the current state into the temporary state
            for (i, word) in t_state.iter_mut().enumerate() {
                *word += state[i]
            }

            // Create a byte stream
            let key_steam = t_state.iter().flat_map(|w| w.0.to_le_bytes());

            for (input_byte, key_byte) in block.iter().zip(key_steam) {
                out.push(*input_byte ^ key_byte)
            }

            ctr += 1;
        }

        out
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        self.encrypt_bytes_with_ctr(bytes, 0)
    }
}

impl Cipher for ChaCha {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.encrypt_bytes(&bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }

    // Decryption is identical
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.encrypt(text)
    }
}

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
    fn state_test_empty() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let mut cipher = ChaCha::default();
        cipher.key = [0, 0, 0, 0, 0, 0, 0, 0];
        cipher.nonce = [0, 0];

        let key_stream = cipher.encrypt_bytes(&[0u8; 64]);

        assert_eq!(
            key_stream,
            ByteFormat::Hex.text_to_bytes("76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586").unwrap()
        );
    }

    #[test]
    fn state_test() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let cipher = ChaCha::default();
        let key_stream = cipher.encrypt_bytes(&[0u8; 64]);

        assert_eq!(
            key_stream,
            ByteFormat::Hex.text_to_bytes("f798a189f195e66982105ffb640bb7757f579da31602fc93ec01ac56f85ac3c134a4547b733b46413042c9440049176905d3be59ea1c53f15916155c2be8241a").unwrap()
        );
    }
}
