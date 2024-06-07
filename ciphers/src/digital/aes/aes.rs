use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{
    digital::{bit_padding, none_padding, strip_bit_padding, BlockCipherMode, BlockCipherPadding},
    Cipher, CipherError,
};

use super::{
    multiplication::{mul11, mul13, mul14, mul2, mul3, mul9},
    sbox::{inv_sbox, sbox, sub_word},
};

// Rotate a 32-bit word by 8-bits
fn rot_word(n: u32) -> u32 {
    n.rotate_left(8)
}

// When the subkeys are created by the key scheduke they are [u32;4] but need to be [u8;16] to xor into the block
fn sub_key_to_bytes(key: [u32; 4]) -> [u8; 16] {
    key.into_iter()
        .map(|w| w.to_be_bytes())
        .flatten()
        .collect_vec()
        .try_into()
        .unwrap()
}

// The internal state of AES is shown as a grid of bytes in column major order.
// This swaps array positions to transpose the bytes and put them in this order
// A (faster?) alternative would be to change the block transformation instead
fn transpose_state(state: &mut [u8; 16]) {
    for (idx, orig) in [(1, 4), (2, 8), (3, 12), (6, 9), (7, 13), (11, 14)].into_iter() {
        state.swap(orig, idx)
    }
}

// Perform the SBOX substititon to all bytes in the state
pub fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state {
        *byte = sbox(*byte)
    }
}

// Inverse of above
pub fn inv_sub_bytes(state: &mut [u8; 16]) {
    for byte in state {
        *byte = inv_sbox(*byte)
    }
}

// Shift each row the state
pub fn shift_rows(state: &mut [u8; 16]) {
    state[4..8].rotate_left(1);
    state[8..12].rotate_left(2);
    state[12..16].rotate_left(3);
}

// Inverse of above
pub fn inv_shift_rows(state: &mut [u8; 16]) {
    state[4..8].rotate_right(1);
    state[8..12].rotate_right(2);
    state[12..16].rotate_right(3);
}

// Mix each column using an invertible matrix multiplication
pub fn mix_columns(state: &mut [u8; 16]) {
    mix_column(state, [0, 4, 8, 12]);
    mix_column(state, [1, 5, 9, 13]);
    mix_column(state, [2, 6, 10, 14]);
    mix_column(state, [3, 7, 11, 15]);
}

// Inverse of above
pub fn inv_mix_columns(state: &mut [u8; 16]) {
    inv_mix_column(state, [0, 4, 8, 12]);
    inv_mix_column(state, [1, 5, 9, 13]);
    inv_mix_column(state, [2, 6, 10, 14]);
    inv_mix_column(state, [3, 7, 11, 15]);
}

// Perform the matrix multiplication.
// The scalar additions are XOR
// The scalar multiplications looked up from tables
pub fn mix_column(state: &mut [u8; 16], idxs: [usize; 4]) {
    let a = state[idxs[0]];
    let b = state[idxs[1]];
    let c = state[idxs[2]];
    let d = state[idxs[3]];
    state[idxs[0]] = mul2(a) ^ mul3(b) ^ c ^ d;
    state[idxs[1]] = a ^ mul2(b) ^ mul3(c) ^ d;
    state[idxs[2]] = a ^ b ^ mul2(c) ^ mul3(d);
    state[idxs[3]] = mul3(a) ^ b ^ c ^ mul2(d);
}

// Inverse of above
pub fn inv_mix_column(state: &mut [u8; 16], idxs: [usize; 4]) {
    let a = state[idxs[0]];
    let b = state[idxs[1]];
    let c = state[idxs[2]];
    let d = state[idxs[3]];
    state[idxs[0]] = mul14(a) ^ mul11(b) ^ mul13(c) ^ mul9(d);
    state[idxs[1]] = mul9(a) ^ mul14(b) ^ mul11(c) ^ mul13(d);
    state[idxs[2]] = mul13(a) ^ mul9(b) ^ mul14(c) ^ mul11(d);
    state[idxs[3]] = mul11(a) ^ mul13(b) ^ mul9(c) ^ mul14(d);
}

// XOR the round key into the state column by column
// This operation is its own inverse
pub fn add_round_key(state: &mut [u8; 16], round_key: &[u8; 16]) {
    // Key is added column by column
    for (idx, key) in [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15]
        .into_iter()
        .zip(round_key)
    {
        state[idx] ^= key
    }
}

pub fn print_aes_state(state: &[u8; 16]) {
    for line in state.chunks_exact(4) {
        println!("{:02x?}", line)
    }
}

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
            mode: BlockCipherMode::Ecb,
            padding: BlockCipherPadding::None,
        }
    }
}

impl Aes128 {
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

        for input in bytes.chunks(16) {
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

        match self.padding {
            BlockCipherPadding::None => none_padding(&mut input, 16)?,
            BlockCipherPadding::Bit => bit_padding(&mut input, 16),
        };

        for block in input.chunks_exact_mut(16) {
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

        if self.padding == BlockCipherPadding::None {
            if bytes.len() % 16 != 0 {
                return Err(CipherError::input(
                    "input must have a length in bytes that is a multiple of 16",
                ));
            }
        }

        let mut input = bytes.to_vec();

        for block in input.chunks_exact_mut(16) {
            Self::decrypt_block(block.try_into().unwrap(), &round_keys);
        }

        match self.padding {
            BlockCipherPadding::None => none_padding(&mut input, 16)?,
            BlockCipherPadding::Bit => strip_bit_padding(&mut input),
        };

        Ok(input)
    }
}

impl Cipher for Aes128 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = match self.mode {
            BlockCipherMode::Ecb => self.encrypt_ecb(&mut bytes)?,
            BlockCipherMode::Ctr => self.encrypt_ctr(&mut bytes)?,
            BlockCipherMode::Cbc => todo!(),
        };
        Ok(self.output_format.byte_slice_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = match self.mode {
            BlockCipherMode::Ecb => self.decrypt_ecb(&mut bytes)?,
            BlockCipherMode::Ctr => self.decrypt_ctr(&mut bytes)?,
            BlockCipherMode::Cbc => todo!(),
        };
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
        cipher.padding = BlockCipherPadding::Bit;
        cipher.mode = BlockCipherMode::Ecb;
        cipher.input_format = ByteFormat::Utf8;
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
