use crate::{digital::block_ciphers::block_cipher::BCMode, errors::CipherError, Cipher};
use utils::byte_formatting::ByteFormat;

use super::{padded_bytes_to_u64_be, AsconState};

pub struct Ascon80pq {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub associated_data: Vec<u8>,
    pub subkeys: [u64; 3],
    pub nonce: [u64; 2],
}

impl Default for Ascon80pq {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            associated_data: Default::default(),
            subkeys: Default::default(),
            nonce: Default::default(),
        }
    }
}

impl Ascon80pq {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn mode(mut self, mode: BCMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn ksa(&mut self, bytes: [u8; 20]) {
        self.subkeys[0] = 0;
        self.subkeys[0] |= (bytes[0] as u64) << 24;
        self.subkeys[0] |= (bytes[1] as u64) << 16;
        self.subkeys[0] |= (bytes[2] as u64) << 8;
        self.subkeys[0] |= (bytes[3] as u64) << 0;

        utils::byte_formatting::fill_u64s_be(&mut self.subkeys[1..], &bytes[4..]);
    }

    pub fn with_key(mut self, key: [u8; 20]) -> Self {
        self.ksa(key);
        self
    }

    pub fn nonce(&mut self, bytes: [u8; 16]) {
        utils::byte_formatting::fill_u64s_be(&mut self.nonce, &bytes);
    }

    pub fn with_nonce(mut self, key: [u8; 16]) -> Self {
        self.nonce(key);
        self
    }

    pub fn ad(&mut self, ad: &[u8]) {
        self.associated_data = ad.to_owned();
    }

    pub fn with_ad(mut self, ad: &[u8]) -> Self {
        self.associated_data = ad.to_owned();
        self
    }

    pub fn ad_str(&mut self, ad: &str) {
        self.associated_data = ByteFormat::Hex
            .text_to_bytes(ad)
            .expect("bytes must be given as hex");
    }

    pub fn with_ad_str(mut self, ad: &str) -> Self {
        self.associated_data = ByteFormat::Hex
            .text_to_bytes(ad)
            .expect("bytes must be given as hex");
        self
    }

    fn b_round(&self, state: &mut AsconState) {
        state.rounds_6()
    }

    fn xor_into_state(&self, state: &mut AsconState, bytes: &[u8]) {
        state[0] ^= padded_bytes_to_u64_be(bytes);
    }

    fn absorb_ad(&self, state: &mut AsconState, rate: usize) {
        if !self.associated_data.is_empty() {
            let mut adlen = self.associated_data.len();
            let mut ptr = 0;
            // Absorb full blocks
            while adlen >= rate {
                self.xor_into_state(state, &self.associated_data[ptr..ptr + rate]);
                self.b_round(state);
                ptr += rate;
                adlen -= rate;
            }
            // Absorb the last padded blcok
            self.xor_into_state(state, &self.associated_data[ptr..]);
            self.b_round(state)
        }
    }

    fn finalize(&self, state: &mut AsconState) {
        state[1] ^= (self.subkeys[0] << 32) | (self.subkeys[1] >> 32);
        state[2] ^= (self.subkeys[1] << 32) | (self.subkeys[2] >> 32);
        state[3] ^= self.subkeys[2] << 32;
        state.rounds_12();
        state[3] ^= self.subkeys[1];
        state[4] ^= self.subkeys[2];
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = AsconState::ascon_80pq(self.subkeys, self.nonce);
        let rate = 8;

        // Absorb associated data if it is provided
        self.absorb_ad(&mut state, rate);

        // Flip the last bit, this is described as domain separation
        state[4] ^= 1;

        // Encrypt the plaintext treating the last block specially
        let mut mlen = bytes.len();
        let mut ptr = 0;
        let mut ctext = Vec::new();
        // Absorb full blocks
        while mlen >= rate {
            self.xor_into_state(&mut state, &bytes[ptr..ptr + rate]);
            ctext.extend(state[0].to_be_bytes());
            ptr += rate;
            mlen -= rate;
            self.b_round(&mut state)
        }
        // Absorb the last padded block
        self.xor_into_state(&mut state, &bytes[ptr..]);
        ctext.extend(state[0].to_be_bytes());
        ctext.truncate(bytes.len());

        // Finalize and create the authentication tag
        self.finalize(&mut state);
        ctext.extend(state[3].to_be_bytes());
        ctext.extend(state[4].to_be_bytes());

        ctext
    }

    pub fn decrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        if bytes.len() < 16 {
            return Err(CipherError::general(
                "authentication failed, message too short",
            ));
        }

        let mut state = AsconState::ascon_80pq(self.subkeys, self.nonce);
        let rate = 8;

        // Absorb associated data if it is provided
        self.absorb_ad(&mut state, rate);

        // Flip the last bit, this is described as domain separation
        state[4] ^= 1;

        // Split off the tag then decrypt the ciphertext
        let (message, tag) = bytes.split_at(bytes.len() - 16);
        let mut ptext = Vec::new();
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= rate {
            let c = u64::from_be_bytes(message[ptr..ptr + rate].try_into().unwrap());
            let p = state[0] ^ c;
            ptext.extend(p.to_be_bytes());
            state[0] = c;
            ptr += rate;
            mlen -= rate;
            self.b_round(&mut state)
        }
        // Decrypt and absorb the last block. This is
        let c = padded_bytes_to_u64_be(&message[ptr..]);
        let p = state[0] ^ c;
        ptext.extend(p.to_be_bytes());
        ptext.truncate(bytes.len() - 16);
        self.xor_into_state(&mut state, &p.to_be_bytes()[0..mlen]);

        // Finalize and check tag
        self.finalize(&mut state);

        let mut t: [u8; 16] = [0; 16];
        t[0..8].copy_from_slice(&state[3].to_be_bytes());
        t[8..16].copy_from_slice(&state[4].to_be_bytes());

        if t == tag {
            Ok(ptext)
        } else {
            // println!("{:02x?}", ptext);
            Err(CipherError::general("authentication failed"))
        }
    }
}

impl Cipher for Ascon80pq {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        Ok(self
            .output_format
            .byte_slice_to_text(&self.encrypt_bytes(&bytes)))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        Ok(self
            .output_format
            .byte_slice_to_text(&self.decrypt_bytes(&bytes)?))
    }
}

#[cfg(test)]
mod ascon_tests {

    use super::*;

    fn ascon80pq_test(ptext: &str, ad: &str, ctext: &str) {
        let cipher = Ascon80pq::default()
            .with_key([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
            ])
            .with_nonce([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_ad_str(ad);
        let otext = cipher.encrypt(ptext).unwrap();
        assert_eq!(ctext, otext, "encrypt failed");
        let otext = cipher.decrypt(ctext).unwrap();
        assert_eq!(ptext, otext, "decrypt failed");
    }

    #[test]
    fn ascon80pd_0_0() {
        ascon80pq_test("", "", "abb688efa0b9d56b33277a2c97d2146b")
    }

    #[test]
    fn ascon80pd_2_0() {
        ascon80pq_test("0001", "", "2846798d04b1e591cbcdf30dbf58d268a69a")
    }

    #[test]
    fn ascon80pd_8_0() {
        ascon80pq_test(
            "0001020304050607",
            "",
            "2846418067ce93861a484e22565f161146fb6f47913803f9",
        )
    }

    #[test]
    fn ascon80pd_0_8() {
        ascon80pq_test("", "0001020304050607", "d80b5c5c8fa97ee33d916c61772b2e23")
    }

    #[test]
    fn ascon80pd_2_2() {
        ascon80pq_test("0001", "0001", "623fff2c0fb416236e91c36d37e4f0a8f2bc")
    }

    #[test]
    fn ascon80pd_64_64() {
        ascon80pq_test(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f", "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
            "cc4e07e5fb13426effd17b0f51a6a830bf484c9651d77679971e8eb4a8edb5a00782a94c72b2b02d87dcf4af75db6996"
        )
    }
}
