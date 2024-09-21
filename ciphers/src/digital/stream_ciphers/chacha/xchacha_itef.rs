use crate::{Cipher, CipherError};
use utils::byte_formatting::{fill_u32s_le, ByteFormat};

use super::ChaChaState;

pub struct XChaChaItef {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u32; 8],
    pub nonce: [u32; 6],
    pub rounds: u8,
    pub ctr: u32,
}

impl Default for XChaChaItef {
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

impl XChaChaItef {
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

    pub fn create_state(&self, ctr: u32) -> [u32; 16] {
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
            ctr,
            0x00000000,
            self.nonce[4],
            self.nonce[5],
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

    // Create a key_stream with the specified number of blocks and with the counter started at a particular value
    pub fn key_stream_with_ctr(&self, blocks: u32, ctr: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity((blocks * 64) as usize);
        let mut key_stream = [0; 64];
        let mut state = ChaChaState::new(self.create_state(ctr));

        for _ in 0..blocks {
            self.block_function(&mut state, &mut key_stream);
            out.extend(key_stream);
            state[12] = state[12].wrapping_add(1);
        }

        out
    }

    // // Encrypt a message with the counter started at a particular value
    pub fn encrypt_bytes_with_ctr(&self, bytes: &[u8], ctr: u32) -> Vec<u8> {
        let mut out = Vec::new();
        let mut key_stream = [0; 64];
        let mut state = ChaChaState(self.create_state(ctr));

        for block in bytes.chunks(64) {
            self.block_function(&mut state, &mut key_stream);
            for (input_byte, key_byte) in block.iter().zip(key_stream) {
                out.push(*input_byte ^ key_byte)
            }
            state[12] = state[12].wrapping_add(1);
        }

        out
    }

    // Encrypt a message with the counter started at the stored value
    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        self.encrypt_bytes_with_ctr(bytes, self.ctr)
    }
}

crate::impl_cipher_for_stream_cipher!(XChaChaItef);

#[cfg(test)]
mod xchacha_tests {

    use super::*;

    #[test]
    fn synthetic_key() {
        // https://datatracker.ietf.org/doc/html/draft-arciszewski-xchacha#section-2.2.1
        let mut cipher = XChaChaItef::default();
        cipher.key = [
            0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
            0x1f1e1d1c,
        ];
        cipher.nonce = [0x09000000, 0x4a000000, 0x00000000, 0x27594131, 0, 0];

        assert_eq!(
            [
                0x82413b42, 0x27b27bfe, 0xd30e4250, 0x8a877d73, 0xa0f9e4d5, 0x8a74a853, 0xc12ec413,
                0x26d3ecdc
            ],
            cipher.synthetic_key()
        )
    }

    #[test]
    fn key_stream_test() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let cipher = XChaChaItef::default().with_key_and_nonce(
            [
                0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d,
                0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b,
                0x9c, 0x9d, 0x9e, 0x9f,
            ],
            [
                0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d,
                0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x58,
            ],
        );

        let key_stream = cipher.key_stream_with_ctr(5, 1);

        assert_eq!(key_stream[0..304], ByteFormat::Hex.text_to_bytes("29624b4b1b140ace53740e405b2168540fd7d630c1f536fecd722fc3cddba7f4cca98cf9e47e5e64d115450f9b125b54449ff76141ca620a1f9cfcab2a1a8a255e766a5266b878846120ea64ad99aa479471e63befcbd37cd1c22a221fe462215cf32c74895bf505863ccddd48f62916dc6521f1ec50a5ae08903aa259d9bf607cd8026fba548604f1b6072d91bc91243a5b845f7fd171b02edc5a0a84cf28dd241146bc376e3f48df5e7fee1d11048c190a3d3deb0feb64b42d9c6fdeee290fa0e6ae2c26c0249ea8c181f7e2ffd100cbe5fd3c4f8271d62b15330cb8fdcf00b3df507ca8c924f7017b7e712d15a2eb5c50484451e54e1b4b995bd8fdd94597bb94d7af0b2c04df10ba0890899ed9293a0f55b8bafa999264035f1d4fbe7fe0aafa109a62372027e50e10cdfecca127").unwrap());
    }

    #[test]
    fn encrypt_test() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7

        let cipher = XChaChaItef::default().with_key_and_nonce(
            [
                0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d,
                0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b,
                0x9c, 0x9d, 0x9e, 0x9f,
            ],
            [
                0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d,
                0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x58,
            ],
        );

        let ptext = b"The dhole (pronounced \"dole\") is also known as the Asiatic wild dog, red dog, and whistling dog. It is about the size of a German shepherd but looks more like a long-legged fox. This highly elusive and skilled jumper is classified with wolves, coyotes, jackals, and foxes in the taxonomic family Canidae.";
        let ctext = cipher.encrypt_bytes_with_ctr(ptext, 1);

        assert_eq!(ctext, ByteFormat::Hex.text_to_bytes("7d0a2e6b7f7c65a236542630294e063b7ab9b555a5d5149aa21e4ae1e4fbce87ecc8e08a8b5e350abe622b2ffa617b202cfad72032a3037e76ffdcdc4376ee053a190d7e46ca1de04144850381b9cb29f051915386b8a710b8ac4d027b8b050f7cba5854e028d564e453b8a968824173fc16488b8970cac828f11ae53cabd20112f87107df24ee6183d2274fe4c8b1485534ef2c5fbc1ec24bfc3663efaa08bc047d29d25043532db8391a8a3d776bf4372a6955827ccb0cdd4af403a7ce4c63d595c75a43e045f0cce1f29c8b93bd65afc5974922f214a40b7c402cdb91ae73c0b63615cdad0480680f16515a7ace9d39236464328a37743ffc28f4ddb324f4d0f5bbdc270c65b1749a6efff1fbaa09536175ccd29fb9e6057b307320d316838a9c71f70b5b5907a66f7ea49aadc409").unwrap());
    }
}
