use crate::{
    digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher},
    impl_cipher_for_block_cipher,
};
use utils::byte_formatting::ByteFormat;

use super::{
    aes_functions::{
        add_round_key, inv_mix_columns, inv_shift_rows, inv_sub_bytes, mix_columns, rot_word,
        shift_rows, sub_bytes, sub_key_slice_to_bytes, transpose_state,
    },
    sbox::sub_word,
};

pub const ROUNDS: usize = 14; // Nr
pub const KEY_WORDS: usize = 8; // Nk
pub const COLUMNS: usize = 4; // Nb

pub struct Aes256 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; KEY_WORDS],
    round_keys: [[u8; 16]; ROUNDS + 1],
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Aes256 {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            key: [0; KEY_WORDS],
            round_keys: [[0u8; 16]; ROUNDS + 1],
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Aes256 {
    // Create the round keys
    pub fn ksa(&mut self) {
        let rc: [u32; 10] = [
            0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000,
            0x80000000, 0x1b000000, 0x36000000,
        ];

        // During expansion actions are on words of 32-bits
        let mut round_keys: Vec<u32> = Vec::new();

        round_keys.extend_from_slice(&self.key);

        for i in KEY_WORDS..((ROUNDS + 1) * COLUMNS) {
            let mut t = round_keys[i - 1];
            if i % KEY_WORDS == 0 {
                t = sub_word(rot_word(t)) ^ rc[(i / KEY_WORDS) - 1];
            } else if KEY_WORDS > 6 && i % KEY_WORDS == 4 {
                t = sub_word(t);
            }
            round_keys.push(round_keys[i - KEY_WORDS] ^ t);
        }

        for (i, chunk) in round_keys.chunks(4).enumerate() {
            self.round_keys[i] = sub_key_slice_to_bytes(chunk)
        }
    }
}

impl BlockCipher<16> for Aes256 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        transpose_state(bytes);
        // Initial round key
        add_round_key(bytes, &self.round_keys[0]);

        // Main rounds
        for i in 1..ROUNDS {
            sub_bytes(bytes);
            shift_rows(bytes);
            mix_columns(bytes);
            add_round_key(bytes, &self.round_keys[i]);
        }

        // Finalization round
        sub_bytes(bytes);
        shift_rows(bytes);
        add_round_key(bytes, &self.round_keys[ROUNDS]);
        transpose_state(bytes);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        transpose_state(bytes);
        // Initial round key
        add_round_key(bytes, &self.round_keys[ROUNDS]);

        // Main rounds
        for i in (1..ROUNDS).rev() {
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
}

impl_cipher_for_block_cipher!(Aes256, 16);

#[cfg(test)]
mod aes256_tests {

    use super::*;

    #[test]
    #[ignore]
    fn test_key_schedule_1() {
        let mut cipher = Aes256::default();

        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
        cipher.key = [
            0x603deb10, 0x15ca71be, 0x2b73aef0, 0x857d7781, 0x1f352c07, 0x3b6108d7, 0x2d9810a3,
            0x0914dff4,
        ];
        let test_sub_keys: [[u32; 4]; 3] = [
            [0x603deb10, 0x15ca71be, 0x2b73aef0, 0x857d7781],
            [0x1f352c07, 0x3b6108d7, 0x2d9810a3, 0x0914dff4],
            [0x9ba35411, 0x8e6925af, 0xa51a8b5f, 0x2067fcde],
        ];
        cipher.ksa();
        let sub_keys = cipher.round_keys;

        println!("{:08x?} {:02x?}", test_sub_keys[0], sub_keys[0]);
        println!("{:08x?} {:02x?}", test_sub_keys[1], sub_keys[1]);
        println!("{:08x?} {:02x?}", test_sub_keys[2], sub_keys[2]);
    }
}
