use crate::{
    digital::stream_ciphers::chacha::{column_round, double_round},
    Cipher, CipherError,
};
use std::num::Wrapping;
use utils::byte_formatting::ByteFormat;

pub struct HChaCha {
    pub key: [u32; 8],
    pub nonce: [u32; 4],
}

impl HChaCha {}

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

            // default for key and nonce taken from test vector here: https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
            key: [
                0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
                0x1f1e1d1c,
            ],
            nonce: [0, 0, 0, 0, 0, 0],
            rounds: 20,
            ctr: 0,
        }
    }
}

impl XChaCha {
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

    // Create a key_stream with the specified number of blocks and with the counter started at a particular value
    pub fn key_stream_with_ctr(&self, blocks: u64, ctr: u64) -> Vec<u8> {
        let mut ctr = ctr;
        let mut out = Vec::with_capacity((blocks * 64) as usize);
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

        for _block in 0..blocks {
            // Mix the counter into the state
            state[12] = Wrapping(ctr as u32); // low bits, "as" cast truncates
            state[13] = Wrapping((ctr >> 32) as u32); // high bits

            // println!("key_stream_state: {:08x?}", state);

            // Temporary state
            let mut t_state = state.clone();

            // Only ChaCha20, ChaCha12, and ChaCha8 are official but any number is usable
            for _round in 0..self.rounds / 2 {
                double_round(&mut t_state);
            }
            if self.rounds % 2 == 1 {
                column_round(&mut t_state)
            }

            // XOR the current state into the temporary state
            for (i, word) in t_state.iter_mut().enumerate() {
                *word += state[i]
            }

            // Create a byte stream
            let key_stream = t_state.iter().flat_map(|w| w.0.to_le_bytes());

            out.extend(key_stream);

            ctr = ctr.wrapping_add(1);
        }

        out
    }

    // Encrypt a message with the counter started at a particular value
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
            // Insert the counter into the state
            state[12] = Wrapping(ctr as u32); // low bits, "as" cast truncates
            state[13] = Wrapping((ctr >> 32) as u32); // high bits

            // Temporary state
            let mut t_state = state.clone();

            // Only ChaCha20, ChaCha12, and ChaCha8 are official but any number is usable
            for _round in 0..self.rounds / 2 {
                double_round(&mut t_state);
            }
            if self.rounds % 2 == 1 {
                column_round(&mut t_state)
            }

            // XOR the current state into the temporary state
            for (i, word) in t_state.iter_mut().enumerate() {
                *word += state[i]
            }

            // Create a byte stream
            let key_steam = t_state.iter().flat_map(|w| w.0.to_le_bytes());

            // XOR the keystream into the message bytes
            for (input_byte, key_byte) in block.iter().zip(key_steam) {
                out.push(*input_byte ^ key_byte)
            }

            ctr = ctr.wrapping_add(1);
        }

        out
    }

    // Encrypt a message with the counter started at the stored value
    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        self.encrypt_bytes_with_ctr(bytes, self.ctr)
    }
}

crate::impl_cipher_for_stream_cipher!(XChaCha);

#[cfg(test)]
mod chacha_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "0102030405060708";
        let cipher = XChaCha::default();

        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    #[test]
    fn state_test_empty() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let mut cipher = XChaCha::default();
        cipher.key = [0, 0, 0, 0, 0, 0, 0, 0];
        cipher.nonce = [0, 0, 0, 0, 0, 0];

        let key_stream = cipher.encrypt_bytes(&[0u8; 64]);

        assert_eq!(
            key_stream,
            ByteFormat::Hex.text_to_bytes("76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586").unwrap()
        );
    }

    #[test]
    fn state_test() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let cipher = XChaCha::default();
        let key_stream = cipher.encrypt_bytes(&[0u8; 256]);

        assert_eq!(
            key_stream,
            ByteFormat::Hex.text_to_bytes("f798a189f195e66982105ffb640bb7757f579da31602fc93ec01ac56f85ac3c134a4547b733b46413042c9440049176905d3be59ea1c53f15916155c2be8241a38008b9a26bc35941e2444177c8ade6689de95264986d95889fb60e84629c9bd9a5acb1cc118be563eb9b3a4a472f82e09a7e778492b562ef7130e88dfe031c79db9d4f7c7a899151b9a475032b63fc385245fe054e3dd5a97a5f576fe064025d3ce042c566ab2c507b138db853e3d6959660996546cc9c4a6eafdc777c040d70eaf46f76dad3979e5c5360c3317166a1c894c94a371876a94df7628fe4eaaf2ccb27d5aaae0ad7ad0f9d4b6ad3b54098746d4524d38407a6deb3ab78fab78c9").unwrap()
        );
    }
}
