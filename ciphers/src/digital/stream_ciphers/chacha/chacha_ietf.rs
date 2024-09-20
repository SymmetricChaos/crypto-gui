use crate::{Cipher, CipherError};

use utils::byte_formatting::{fill_u32s_le, ByteFormat};

use super::ChaChaState;

pub struct ChaChaIetf {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u32; 8],
    pub nonce: [u32; 3],
    pub rounds: u8,
    pub ctr: u32,
}

impl Default for ChaChaIetf {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,

            // default for key and nonce taken from test vector here: https://datatracker.ietf.org/doc/html/rfc8439#section-2.3.2
            key: [
                0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
                0x1f1e1d1c,
            ],
            nonce: [0x09000000, 0x4a000000, 0x000000],
            rounds: 20,
            ctr: 0,
        }
    }
}

impl ChaChaIetf {
    pub fn key_and_nonce(&mut self, key: [u8; 32], nonce: [u8; 20]) {
        fill_u32s_le(&mut self.key, &key);
        fill_u32s_le(&mut self.nonce, &nonce);
    }

    pub fn with_key_and_nonce(mut self, key: [u8; 32], nonce: [u8; 20]) -> Self {
        self.key_and_nonce(key, nonce);
        self
    }

    pub fn create_state(&self, ctr: u32) -> [u32; 16] {
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
            ctr,
            self.nonce[0],
            self.nonce[1],
            self.nonce[2],
        ]
    }

    pub fn block_function(&self, state: &mut ChaChaState, block: &mut [u8; 64]) {
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

    pub fn key_stream_with_ctr(&self, blocks: u64, ctr: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity((blocks * 64) as usize);
        let mut key_stream = [0; 64];
        let mut state = ChaChaState(self.create_state(ctr));

        for _ in 0..blocks {
            self.block_function(&mut state, &mut key_stream);
            out.extend(key_stream);

            // Increment
            state[12] = state[12].wrapping_add(1);
        }

        out
    }

    pub fn encrypt_bytes_with_ctr(&self, bytes: &[u8], ctr: u32) -> Vec<u8> {
        let mut out = Vec::new();
        let mut key_stream = [0; 64];
        let mut state = ChaChaState(self.create_state(ctr));

        for block in bytes.chunks(64) {
            self.block_function(&mut state, &mut key_stream);
            for (input_byte, key_byte) in block.iter().zip(key_stream) {
                out.push(*input_byte ^ key_byte)
            }

            // Increment
            state[12] = state[12].wrapping_add(1);
        }

        out
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        self.encrypt_bytes_with_ctr(bytes, self.ctr)
    }
}

crate::impl_cipher_for_stream_cipher!(ChaChaIetf);

#[cfg(test)]
mod chacha_ietf_tests {

    use super::*;

    #[test]
    fn keystream_test() {
        // https://datatracker.ietf.org/doc/html/rfc8439#section-2.3.2
        let mut cipher = ChaChaIetf::default();
        cipher.ctr = 1;

        let key_stream = cipher.encrypt_bytes(&[0u8; 64]);

        assert_eq!(
            key_stream,
            ByteFormat::Hex.text_to_bytes("10f1e7e4d13b5915500fdd1fa32071c4c7d1f4c733c068030422aa9ac3d46c4ed2826446079faa0914c2d705d98b02a2b5129cd1de164eb9cbd083e8a2503c4e").unwrap()
        );
    }

    #[test]
    fn encrypt_test() {
        // https://datatracker.ietf.org/doc/html/rfc8439#section-2.3.2
        const PTEXT: &'static str = "Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
        let mut cipher = ChaChaIetf::default();
        cipher.input_format = ByteFormat::Utf8;
        cipher.key = [
            0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
            0x1f1e1d1c,
        ];
        cipher.nonce = [0, 0x4a000000, 0];
        cipher.ctr = 1;

        let ctext = cipher.encrypt_bytes(PTEXT.as_bytes());
        assert_eq!(
            ctext,
            ByteFormat::Hex.text_to_bytes("6e2e359a2568f98041ba0728dd0d6981e97e7aec1d4360c20a27afccfd9fae0bf91b65c5524733ab8f593dabcd62b3571639d624e65152ab8f530c359f0861d807ca0dbf500d6a6156a38e088a22b65e52bc514d16ccf806818ce91ab77937365af90bbf74a35be6b40b8eedf2785e42874d").unwrap()
        );
    }
}
