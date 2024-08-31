use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::{u32_pair_to_u8_array, ByteFormat};

pub struct Xxtea {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u32; 4],
    pub iv: Vec<u32>,
    pub block_words: u32,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Xxtea {
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

impl Xxtea {
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        utils::byte_formatting::fill_u32s_be(&mut self.key, &bytes);
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn mx(y: u32, z: u32, sum: u32, p: u32, e: u32, k: &[u32; 4]) -> u32 {
        (z >> 5 ^ y << 2) + (y >> 3 ^ z << 4) ^ (sum ^ y) + (k[(p & 3 ^ e) as usize] ^ z)
    }

    pub fn encrypt_bytes(&self, bytes: &mut [u8]) {
        let mut n = bytes.len() / 4;
    }
}

// #[cfg(test)]
// mod xxtea_tests {

//     use crate::Cipher;

//     use super::*;

//     #[test]
//     fn encrypt_decrypt_test() {
//         let ptext = "0102030405060708";
//         let cipher = Xxtea::default();
//         let ctext = cipher.encrypt(ptext).unwrap();
//         assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
//     }
// }
