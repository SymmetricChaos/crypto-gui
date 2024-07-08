use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{
    digital::block_ciphers::{
        bit_padding, none_padding, strip_bit_padding, BlockCipherMode, BlockCipherPadding,
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
    pub ctr: u128,
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for Aes128 {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            key: [0; Self::KEY_WORDS],
            ctr: 0,
            mode: BlockCipherMode::default(),
            padding: BlockCipherPadding::default(),
        }
    }
}

impl Aes128 {
    pub const BLOCKSIZE: u32 = 16;
    pub const ROUNDS: usize = 11;
    pub const KEY_WORDS: usize = 4;

    // Create the round keys
    pub fn key_schedule(&self) -> Vec<[u32; 4]> {
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

        round_keys
    }

    pub fn encrypt_block(block: &mut [u8; 16], round_keys: &Vec<[u8; 16]>) {
        transpose_state(block);
        // Initial round key
        add_round_key(block, &round_keys[0]);

        // Main rounds
        for i in 1..(Self::ROUNDS - 1) {
            sub_bytes(block);
            shift_rows(block);
            mix_columns(block);
            add_round_key(block, &round_keys[i]);
        }

        // Finalization round
        sub_bytes(block);
        shift_rows(block);
        add_round_key(block, &round_keys[Self::ROUNDS - 1]);
        transpose_state(block);
    }

    pub fn decrypt_block(block: &mut [u8; 16], round_keys: &Vec<[u8; 16]>) {
        transpose_state(block);
        // Initial round key
        add_round_key(block, &round_keys[Self::ROUNDS - 1]);

        // Main rounds
        for i in (1..(Self::ROUNDS - 1)).rev() {
            inv_shift_rows(block);
            inv_sub_bytes(block);
            add_round_key(block, &round_keys[i]);
            inv_mix_columns(block);
        }

        // Finalization round
        inv_shift_rows(block);
        inv_sub_bytes(block);
        add_round_key(block, &round_keys[0]);
        transpose_state(block);
    }

    // Encrypt bytes in CTR mode
    pub fn encrypt_ctr(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let mut counter = self.ctr;
        let round_keys = self
            .key_schedule()
            .into_iter()
            .map(|k| sub_key_to_bytes(k))
            .collect_vec();

        let mut out = Vec::with_capacity(bytes.len());

        for input in bytes.chunks(Self::BLOCKSIZE as usize) {
            let mut state: [u8; 16] = counter.to_be_bytes().try_into().unwrap();
            Self::encrypt_block(&mut state, &round_keys);
            for (i, k) in input.into_iter().zip(state.into_iter()) {
                out.push(*i ^ k)
            }
            counter = counter.wrapping_add(1);
        }

        Ok(out)
    }

    // CTR mode is reciprocal
    pub fn decrypt_ctr(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        self.encrypt_ctr(bytes)
    }

    // Encrypt bytes in ECB mode
    pub fn encrypt_ecb(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let round_keys = self
            .key_schedule()
            .into_iter()
            .map(|k| sub_key_to_bytes(k))
            .collect_vec();

        let mut input = bytes.to_vec();

        for block in input.chunks_exact_mut(Self::BLOCKSIZE as usize) {
            Self::encrypt_block(block.try_into().unwrap(), &round_keys);
        }

        Ok(input)
    }

    // Decrypt bytes in ECB mode
    pub fn decrypt_ecb(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let round_keys = self
            .key_schedule()
            .into_iter()
            .map(|k| sub_key_to_bytes(k))
            .collect_vec();

        let mut input = bytes.to_vec();

        for block in input.chunks_exact_mut(Self::BLOCKSIZE as usize) {
            Self::decrypt_block(block.try_into().unwrap(), &round_keys);
        }

        Ok(input)
    }
}

impl Cipher for Aes128 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        // Add padding if needed
        if self.mode.padded() {
            match self.padding {
                BlockCipherPadding::None => none_padding(&mut bytes, Self::BLOCKSIZE)?,
                BlockCipherPadding::Bit => bit_padding(&mut bytes, Self::BLOCKSIZE),
            };
        }

        let out = match self.mode {
            BlockCipherMode::Ecb => self.encrypt_ecb(&mut bytes)?,
            BlockCipherMode::Ctr => self.encrypt_ctr(&mut bytes)?,
            BlockCipherMode::Cbc => return Err(CipherError::state("CBC mode not implemented")),
        };
        Ok(self.output_format.byte_slice_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        // Graceful error if ciphertext is the wrong size
        if self.mode.padded() {
            if self.padding == BlockCipherPadding::None {
                none_padding(&mut bytes, Self::BLOCKSIZE)?
            };
        }

        let mut out = match self.mode {
            BlockCipherMode::Ecb => self.decrypt_ecb(&mut bytes)?,
            BlockCipherMode::Ctr => self.decrypt_ctr(&mut bytes)?,
            BlockCipherMode::Cbc => return Err(CipherError::state("CBC mode not implemented")),
        };

        // Remove padding if needed
        if self.mode.padded() {
            match self.padding {
                BlockCipherPadding::None => none_padding(&mut out, Self::BLOCKSIZE)?,
                BlockCipherPadding::Bit => strip_bit_padding(&mut out)?,
            };
        }

        Ok(self.output_format.byte_slice_to_text(&out))
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

    #[test]
    fn test_key_schedule_1() {
        let mut cipher = Aes128::default();

        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
        cipher.key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
        let test_sub_keys: [[u32; 4]; 3] = [
            [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c],
            [0xa0fafe17, 0x88542cb1, 0x23a33939, 0x2a6c7605],
            [0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f],
        ];
        let sub_keys = cipher.key_schedule();

        // for k in sub_keys.iter() {
        //     println!("{:08x?}", k)
        // }

        assert_eq!(test_sub_keys[0], sub_keys[0]);
        assert_eq!(test_sub_keys[1], sub_keys[1]);
        assert_eq!(test_sub_keys[2], sub_keys[2]);
    }

    #[test]
    fn test_key_schedule_2() {
        let mut cipher = Aes128::default();

        // https://github.com/kaapomoi/key-expander/blob/master/src/lib.rs
        cipher.key = [0x0, 0x0, 0x0, 0x1];
        let test_sub_keys: [[u32; 4]; 3] = [
            [0x0, 0x0, 0x0, 0x1],
            [0x62637c63, 0x62637c63, 0x62637c63, 0x62637c62],
            [0x9b73d6c9, 0xf910aaaa, 0x9b73d6c9, 0xf910aaab],
        ];
        let sub_keys = cipher.key_schedule();

        // for k in sub_keys.iter() {
        //     println!("{:08x?}", k)
        // }

        assert_eq!(test_sub_keys[0], sub_keys[0]);
        assert_eq!(test_sub_keys[1], sub_keys[1]);
        assert_eq!(test_sub_keys[2], sub_keys[2]);
    }

    #[test]
    fn test_encypt_decrypt_ctr() {
        let mut cipher = Aes128::default();
        cipher.mode = BlockCipherMode::Ctr;

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
        cipher.mode = BlockCipherMode::Ecb;
        cipher.padding = BlockCipherPadding::Bit;

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
        let round_keys = cipher
            .key_schedule()
            .into_iter()
            .map(|k| sub_key_to_bytes(k))
            .collect_vec();
        let mut state = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        Aes128::encrypt_block(&mut state, &round_keys);
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
        let ptext = ByteFormat::Hex
            .text_to_bytes("6bc1bee22e409f96e93d7e117393172a")
            .unwrap();
        let ctext = ByteFormat::Hex
            .text_to_bytes("874d6191b620e3261bef6864990db6ce")
            .unwrap();
        let mut cipher = Aes128::default();
        cipher.ctr = 0xf0f1f2f3f4f5f6f7f8f9fafbfcfdfeff;
        cipher.key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
        assert_eq!(ctext, cipher.encrypt_ctr(&ptext).unwrap())
    }
}
