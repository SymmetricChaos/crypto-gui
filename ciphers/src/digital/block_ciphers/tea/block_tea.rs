use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::ByteFormat;

pub fn mx(y: u32, z: u32, sum: u32, p: u32, e: u32, k: &[u32; 4]) -> u32 {
    (z >> 5 ^ y << 2) + (y >> 3 ^ z << 4) ^ (sum ^ y) + (k[(p & 3 ^ e) as usize] ^ z)
}

pub struct Btea {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u32; 4],
    pub iv: Vec<u32>,
    pub block_words: u32,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Btea {
    fn default() -> Self {
        Self {
            key: [0, 1, 2, 3],
            iv: vec![0, 0],
            block_words: 2,
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Btea {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn padding(mut self, padding: BCPadding) -> Self {
        self.padding = padding;
        self
    }

    pub fn mode(mut self, mode: BCMode) -> Self {
        self.mode = mode;
        self
    }

    // pub fn ksa(&mut self, bytes: [u8; 16]) {
    //     fill_u32s_be(&mut self.subkeys, &bytes);
    // }

    // pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
    //     self.ksa(bytes);
    //     self
    // }
}

// impl BlockCipher<8> for Btea {
//     fn encrypt_block(&self, bytes: &mut [u8]) {}

//     fn decrypt_block(&self, bytes: &mut [u8]) {}
// }

// crate::impl_cipher_for_block_cipher!(Btea, 8);

// #[cfg(test)]
// mod block_tea_tests {

//     use crate::Cipher;

//     use super::*;

//     #[test]
//     fn encrypt_decrypt_test() {
//         let ptext = "0102030405060708";
//         let cipher = Btea::default();
//         let ctext = cipher.encrypt(ptext).unwrap();
//         assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
//     }
// }
