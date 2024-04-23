use crate::{Cipher, CipherError};
use utils::byte_formatting::ByteFormat;

pub struct Salsa20 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; 8],
    pub nonce: [u32; 2],
    pub rounds: u8,
}

impl Default for Salsa20 {
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

impl Salsa20 {
    pub fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        state[b] ^= (state[a].wrapping_add(state[d])).rotate_left(7);
        state[c] ^= (state[b].wrapping_add(state[a])).rotate_left(9);
        state[d] ^= (state[c].wrapping_add(state[b])).rotate_left(13);
        state[a] ^= (state[d].wrapping_add(state[c])).rotate_left(18);
    }

    // Acts on columns
    pub fn odd_round(state: &mut [u32; 16]) {
        Self::quarter_round(state, 0, 4, 8, 12);
        Self::quarter_round(state, 5, 9, 13, 1);
        Self::quarter_round(state, 10, 14, 2, 6);
        Self::quarter_round(state, 15, 3, 7, 11);
    }

    // Acts on rows
    pub fn even_round(state: &mut [u32; 16]) {
        Self::quarter_round(state, 0, 1, 2, 3);
        Self::quarter_round(state, 5, 6, 7, 4);
        Self::quarter_round(state, 10, 11, 8, 9);
        Self::quarter_round(state, 15, 12, 13, 14);
    }

    pub fn double_round(state: &mut [u32; 16]) {
        Self::odd_round(state);
        Self::even_round(state);
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut ctr = 0_u64;
        let mut out = Vec::new();
        let mut state = [
            0x61707865,
            self.key[0],
            self.key[1],
            self.key[2],
            self.key[3],
            0x3320646e,
            self.nonce[0],
            self.nonce[1],
            0x00000000,
            0x00000000,
            0x79622d32,
            self.key[4],
            self.key[5],
            self.key[6],
            self.key[7],
            0x6b206574,
        ];

        for block in bytes.chunks(64) {
            // Mix the counter into the state
            state[8] = ctr as u32;
            state[9] = (ctr >> 32) as u32;

            // Temporary state
            let mut t_state = state.clone();

            // Only Salsa20/20, Salsa20/12, and Salsa20/8 are official but any number is usable
            for _round in 0..self.rounds / 2 {
                Self::double_round(&mut t_state);
            }
            if self.rounds % 2 == 1 {
                Self::odd_round(&mut t_state)
            }

            // XOR the current state into the temporary state
            for (i, word) in t_state.iter_mut().enumerate() {
                *word ^= state[i]
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

impl Cipher for Salsa20 {
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
mod salsa20_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "0102030405060708";
        let cipher = Salsa20::default();
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
