use crate::{Cipher, CipherError};
use utils::byte_formatting::ByteFormat;

// https://cr.yp.to/snuffle/salsafamily-20071225.pdf
pub struct ChaCha {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; 8],
    pub nonce: [u32; 2],
    pub rounds: u8,
}

impl Default for ChaCha {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            key: [
                0x04030201, 0x08070605, 0x0c0b0a09, 0x100f0e0d, 0x14131211, 0x18171615, 0x1c1b1a19,
                0x201f1e1d,
            ],
            nonce: [0x01040103, 0x06020905],
            rounds: 20,
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

    pub fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(16);

        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(12);

        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(8);

        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(7);
    }

    pub fn double_round(state: &mut [u32; 16]) {
        Self::quarter_round(state, 0, 4, 8, 12);
        Self::quarter_round(state, 1, 5, 9, 13);
        Self::quarter_round(state, 2, 6, 10, 14);
        Self::quarter_round(state, 3, 7, 11, 15);
        Self::quarter_round(state, 0, 5, 10, 15);
        Self::quarter_round(state, 1, 6, 11, 12);
        Self::quarter_round(state, 2, 7, 8, 13);
        Self::quarter_round(state, 3, 4, 9, 14);
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut ctr = 0_u64;
        let mut out = Vec::new();
        let mut state = [
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
            0x00000000,
            0x00000000,
            self.nonce[0],
            self.nonce[1],
        ];

        for block in bytes.chunks(64) {
            // Mix the counter into the state
            state[12] = ctr as u32; // low bits, "as" cast truncates
            state[13] = (ctr >> 32) as u32; // high bits

            // Temporary state
            let mut t_state = state.clone();

            // Only ChaCha20, ChaCha12, and ChaCha8 are official but any number is usable
            for _round in 0..self.rounds / 2 {
                Self::double_round(&mut t_state);
            }
            if self.rounds % 2 == 1 {
                Self::quarter_round(&mut t_state, 0, 4, 8, 12);
                Self::quarter_round(&mut t_state, 1, 5, 9, 13);
                Self::quarter_round(&mut t_state, 2, 6, 10, 14);
                Self::quarter_round(&mut t_state, 3, 7, 11, 15);
            }

            // XOR the current state into the temporary state
            for (i, word) in t_state.iter_mut().enumerate() {
                *word = word.wrapping_add(state[i])
            }

            // Create a byte stream
            let key_steam = t_state.iter().flat_map(|w| w.to_le_bytes());

            for (input_byte, key_byte) in block.iter().zip(key_steam) {
                out.push(*input_byte ^ key_byte)
            }

            ctr += 1;
        }

        out
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
    fn state_test() {
        let cipher = ChaCha::default();
        let state = cipher.create_state(7);
        let mut t_state = state.clone();

        assert_eq!(
            t_state,
            [
                0x61707865, 0x04030201, 0x08070605, 0x0c0b0a09, 0x100f0e0d, 0x3320646e, 0x01040103,
                0x06020905, 0x00000007, 0x00000000, 0x79622d32, 0x14131211, 0x18171615, 0x1c1b1a19,
                0x201f1e1d, 0x6b206574
            ]
        );

        for _ in 0..10 {
            ChaCha::double_round(&mut t_state);
        }

        assert_eq!(
            t_state,
            [
                0x58318d3e, 0x0292df4f, 0xa28d8215, 0xa1aca723, 0x697a34c7, 0xf2f00ba8, 0x63e9b0a1,
                0x27250e3a, 0xb1c7f1f3, 0x62066edc, 0x66d3ccf1, 0xb0365cf3, 0x091ad09e, 0x64f0c40f,
                0xd60d95ea, 0x00be78c9
            ]
        );

        for (i, word) in t_state.iter_mut().enumerate() {
            *word = word.wrapping_add(state[i])
        }

        assert_eq!(
            t_state,
            [
                0xb9a205a3, 0x0695e150, 0xaa94881a, 0xadb7b12c, 0x798942d4, 0x26107016, 0x64edb1a4,
                0x2d27173f, 0xb1c7f1fa, 0x62066edc, 0xe035fa23, 0xc4496f04, 0x2131e6b3, 0x810bde28,
                0xf62cb407, 0x6bdede3d
            ]
        );
    }
}
