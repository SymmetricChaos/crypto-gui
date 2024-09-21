use crate::{Cipher, CipherError};
use utils::byte_formatting::{fill_u32s_le, ByteFormat};

use super::ChaChaState;

pub struct XChaCha {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u32; 8],
    pub nonce: [u32; 6],
    pub rounds: u8,
    pub ctr: u64,
}

impl Default for XChaCha {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key: [0, 0, 0, 0, 0, 0, 0, 0],
            nonce: [0, 0, 0, 0, 0, 0],
            rounds: 20,
            ctr: 1,
        }
    }
}

impl XChaCha {
    pub fn key_and_nonce(&mut self, key: [u8; 32], nonce: [u8; 24]) {
        fill_u32s_le(&mut self.key, &key);
        fill_u32s_le(&mut self.nonce, &nonce);
    }

    pub fn with_key_and_nonce(mut self, key: [u8; 32], nonce: [u8; 24]) -> Self {
        self.key_and_nonce(key, nonce);
        self
    }

    pub fn synthetic_key(&self) -> [u32; 8] {
        let mut state = ChaChaState::new([
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
            self.nonce[0],
            self.nonce[1],
            self.nonce[2],
            self.nonce[3],
        ]);

        for _ in 0..10 {
            state.double_round();
        }

        [
            state[0], state[1], state[2], state[3], state[12], state[13], state[14], state[15],
        ]
    }

    pub fn create_state(&self, ctr: u64) -> [u32; 16] {
        let k = self.synthetic_key();
        [
            0x61707865,
            0x3320646e,
            0x79622d32,
            0x6b206574,
            k[0],
            k[1],
            k[2],
            k[3],
            k[4],
            k[5],
            k[6],
            k[7],
            ctr as u32,
            (ctr >> 32) as u32,
            self.nonce[4],
            self.nonce[5],
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

    // Encrypt a message with the counter started at the stored value
    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        self.encrypt_bytes_with_ctr(bytes, self.ctr)
    }
}

crate::impl_cipher_for_stream_cipher!(XChaCha);
