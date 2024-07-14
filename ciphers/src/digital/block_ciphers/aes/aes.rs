use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{
    digital::block_ciphers::block_cipher::{
        none_padding, BlockCipher, BCMode, BCPadding,
    },
    Cipher, CipherError,
};

use super::{
    aes_functions::{
        add_round_key, inv_mix_columns, inv_shift_rows, inv_sub_bytes, mix_columns, rot_word,
        shift_rows, sub_bytes, sub_key_to_bytes, transpose_state,
    },
    sbox::sub_word,
};

pub struct Aes128 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; Self::KEY_WORDS],
    round_keys: [[u8; 16]; Self::ROUNDS],
    pub ctr: u128,
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Aes128 {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            key: [0; Self::KEY_WORDS],
            round_keys: [[0u8; 16]; Self::ROUNDS],
            ctr: 0,
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Aes128 {
    pub const BLOCKSIZE: u32 = 16;
    pub const ROUNDS: usize = 11;
    pub const KEY_WORDS: usize = 4;

    // Create the round keys
    pub fn ksa(&mut self) {
        let rc: [u32; 10] = [
            0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000,
            0x80000000, 0x1b000000, 0x36000000,
        ];

        // During expansion actions are on words of 32-bits
        let mut round_keys: Vec<[u32; 4]> = Vec::with_capacity(Self::ROUNDS);

        // First subkey is the user supplied key
        round_keys.push(self.key);

        for round in 1..Self::ROUNDS {
            let mut k = [0_u32; 4];

            let rotated = rot_word(round_keys[round - 1][3]);
            let subbed = sub_word(rotated);
            let xored = subbed ^ rc[round - 1];

            k[0] = xored ^ round_keys[round - 1][0];
            k[1] = k[0] ^ round_keys[round - 1][1];
            k[2] = k[1] ^ round_keys[round - 1][2];
            k[3] = k[2] ^ round_keys[round - 1][3];

            round_keys.push(k)
        }
        self.round_keys = round_keys
            .into_iter()
            .map(|k| sub_key_to_bytes(k))
            .collect_vec()
            .try_into()
            .unwrap();
    }
}

impl BlockCipher<16> for Aes128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        transpose_state(bytes);
        // Initial round key
        add_round_key(bytes, &self.round_keys[0]);

        // Main rounds
        for i in 1..(Self::ROUNDS - 1) {
            sub_bytes(bytes);
            shift_rows(bytes);
            mix_columns(bytes);
            add_round_key(bytes, &self.round_keys[i]);
        }

        // Finalization round
        sub_bytes(bytes);
        shift_rows(bytes);
        add_round_key(bytes, &self.round_keys[Self::ROUNDS - 1]);
        transpose_state(bytes);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        transpose_state(bytes);
        // Initial round key
        add_round_key(bytes, &self.round_keys[Self::ROUNDS - 1]);

        // Main rounds
        for i in (1..(Self::ROUNDS - 1)).rev() {
            inv_shift_rows(bytes);
            inv_sub_bytes(bytes);
            add_round_key(bytes, &self.round_keys[i]);
            inv_mix_columns(bytes);
        }

        // Finalization round
        inv_shift_rows(bytes);
        inv_sub_bytes(bytes);
        add_round_key(bytes, &self.round_keys[0]);
        transpose_state(bytes);
    }

    fn set_mode(&mut self, mode: BCMode) {
        self.mode = mode
    }

    fn set_padding(&mut self, padding: BCPadding) {
        self.padding = padding
    }
}

impl Cipher for Aes128 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.mode.padded() {
            self.padding.add_padding(&mut bytes, Self::BLOCKSIZE)?;
        }

        match self.mode {
            BCMode::Ecb => self.encrypt_ecb(&mut bytes),
            BCMode::Ctr => self.encrypt_ctr(&mut bytes, self.ctr.to_be_bytes()),
            BCMode::Cbc => self.encrypt_cbc(&mut bytes, self.iv.to_be_bytes()),
        };
        Ok(self.output_format.byte_slice_to_text(&bytes))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.mode.padded() {
            if self.padding == BCPadding::None {
                none_padding(&mut bytes, Self::BLOCKSIZE)?
            };
        }

        match self.mode {
            BCMode::Ecb => self.decrypt_ecb(&mut bytes),
            BCMode::Ctr => self.decrypt_ctr(&mut bytes, self.ctr.to_be_bytes()),
            BCMode::Cbc => self.decrypt_cbc(&mut bytes, self.iv.to_be_bytes()),
        };

        if self.mode.padded() {
            self.padding.strip_padding(&mut bytes, Self::BLOCKSIZE)?;
        }

        Ok(self.output_format.byte_slice_to_text(&bytes))
    }
}

#[cfg(test)]
mod aes_tests {

    use super::*;

    #[test]
    fn test_shift_rows() {
        let mut state = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let output = [0, 1, 2, 3, 5, 6, 7, 4, 10, 11, 8, 9, 15, 12, 13, 14];
        // print_aes_state(&state);
        shift_rows(&mut state);
        // print_aes_state(&state);
        assert_eq!(state, output)
    }

    #[test]
    fn test_mix_cols() {
        let mut state = [
            0xdb, 0x01, 0xf2, 0xd4, 0x13, 0x01, 0xa, 0xd4, 0x53, 0x01, 0x22, 0xd4, 0x45, 0x01,
            0x5c, 0xd5,
        ];
        let original_state = state.clone();
        let output = [
            0x8e, 0x01, 0x9f, 0xd5, 0x4d, 0x01, 0xdc, 0xd5, 0xa1, 0x01, 0x58, 0xd7, 0xbc, 0x01,
            0x9d, 0xd6,
        ];
        // print_aes_state(&state);
        mix_columns(&mut state);
        // print_aes_state(&state);
        assert_eq!(state, output);
        inv_mix_columns(&mut state);
        // print_aes_state(&state);
        assert_eq!(state, original_state);
    }

    // #[test]
    // fn test_key_schedule_1() {
    //     let mut cipher = Aes128::default();

    //     // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
    //     cipher.key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
    //     let test_sub_keys: [[u32; 4]; 3] = [
    //         [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c],
    //         [0xa0fafe17, 0x88542cb1, 0x23a33939, 0x2a6c7605],
    //         [0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f],
    //     ];
    //     cipher.key_schedule();
    //     let sub_keys = cipher.round_keys;

    //     println!("{:08x?} {:02x?}", test_sub_keys[0], sub_keys[0]);
    //     println!("{:08x?} {:02x?}", test_sub_keys[1], sub_keys[1]);
    //     println!("{:08x?} {:02x?}", test_sub_keys[2], sub_keys[2]);
    // }

    // #[test]
    // fn test_key_schedule_2() {
    //     let mut cipher = Aes128::default();

    //     // https://github.com/kaapomoi/key-expander/blob/master/src/lib.rs
    //     cipher.key = [0x0, 0x0, 0x0, 0x1];
    //     let test_sub_keys: [[u32; 4]; 3] = [
    //         [0x0, 0x0, 0x0, 0x1],
    //         [0x62637c63, 0x62637c63, 0x62637c63, 0x62637c62],
    //         [0x9b73d6c9, 0xf910aaaa, 0x9b73d6c9, 0xf910aaab],
    //     ];
    //     cipher.key_schedule();
    //     let sub_keys = cipher.round_keys;

    //     println!("{:08x?} {:02x?}", test_sub_keys[0], sub_keys[0]);
    //     println!("{:08x?} {:02x?}", test_sub_keys[1], sub_keys[1]);
    //     println!("{:08x?} {:02x?}", test_sub_keys[2], sub_keys[2]);
    // }

    #[test]
    fn test_encypt_decrypt_ctr() {
        let mut cipher = Aes128::default();
        cipher.mode = BCMode::Ctr;

        cipher.input_format = ByteFormat::Utf8;
        let ptext = "The quick brown fox.";
        let ctext = cipher.encrypt(ptext).unwrap();
        cipher.input_format = ByteFormat::Hex;
        cipher.output_format = ByteFormat::Utf8;
        let decrypt = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, decrypt);
    }

    #[test]
    fn test_encypt_decrypt_ecb() {
        let mut cipher = Aes128::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::Bit;

        cipher.input_format = ByteFormat::Utf8;
        cipher.output_format = ByteFormat::Hex;
        let ptext = "The quick brown fox.";
        let ctext = cipher.encrypt(ptext).unwrap();
        cipher.input_format = ByteFormat::Hex;
        cipher.output_format = ByteFormat::Utf8;
        let decrypt = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, decrypt);
    }

    #[test]
    fn test_encypt() {
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
        let mut cipher = Aes128::default();
        cipher.key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
        cipher.ksa();
        let mut state = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        cipher.encrypt_block(&mut state);
        transpose_state(&mut state);
        assert_eq!(
            [
                0x39, 0x02, 0xdc, 0x19, 0x25, 0xdc, 0x11, 0x6a, 0x84, 0x09, 0x85, 0x0b, 0x1d, 0xfb,
                0x97, 0x32
            ],
            state
        );
    }

    #[test]
    fn test_ctr_mode() {
        let mut ptext = ByteFormat::Hex
            .text_to_bytes("6bc1bee22e409f96e93d7e117393172a")
            .unwrap();
        let ctext = ByteFormat::Hex
            .text_to_bytes("874d6191b620e3261bef6864990db6ce")
            .unwrap();
        let mut cipher = Aes128::default();
        cipher.ctr = 0xf0f1f2f3f4f5f6f7f8f9fafbfcfdfeff;
        cipher.key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
        cipher.ksa();
        cipher.encrypt_ctr(&mut ptext, cipher.ctr.to_be_bytes());
        assert_eq!(ctext, ptext);
    }
}
