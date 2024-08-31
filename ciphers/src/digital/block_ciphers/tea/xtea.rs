use crate::{
    digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher},
    impl_cipher_for_block_cipher,
};
use utils::byte_formatting::{u32_pair_to_u8_array, ByteFormat};

pub fn mx_e(a: u32, b: u32, sum: u32, k: u32) -> u32 {
    b.wrapping_add((((a << 4) ^ (a >> 5)).wrapping_add(a) & sum).wrapping_add(k))
}
pub fn mx_d(a: u32, b: u32, sum: u32, k: u32) -> u32 {
    b.wrapping_sub((((a << 4) ^ (a >> 5)).wrapping_add(a) & sum).wrapping_add(k))
}

pub struct Xtea {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u32; 4],
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Xtea {
    fn default() -> Self {
        Self {
            key: [0, 1, 2, 3],
            iv: 0,
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Xtea {
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        utils::byte_formatting::fill_u32s_be(&mut self.key, &bytes);
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<8> for Xtea {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        utils::byte_formatting::fill_u32s_be(&mut v, bytes);
        let mut sum: u32 = 0;
        for _ in 0..32 {
            v[0] = mx_e(v[1], v[0], sum, self.key[(sum & 3) as usize]);
            sum = sum.wrapping_add(super::DELTA);
            v[1] = mx_e(v[0], v[1], sum, self.key[(sum >> 11 & 3) as usize]);
        }
        for (plaintext, ciphertext) in bytes.iter_mut().zip(u32_pair_to_u8_array(v).iter()) {
            *plaintext = *ciphertext
        }
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        utils::byte_formatting::fill_u32s_be(&mut v, bytes);
        let mut sum: u32 = 0xC6EF3720;
        for _ in 0..32 {
            v[1] = mx_d(v[0], v[1], sum, self.key[(sum >> 11 & 3) as usize]);
            sum = sum.wrapping_sub(super::DELTA);
            v[0] = mx_d(v[1], v[0], sum, self.key[(sum & 3) as usize]);
        }
        for (ciphertext, plaintext) in bytes.iter_mut().zip(u32_pair_to_u8_array(v).iter()) {
            *ciphertext = *plaintext
        }
    }
}

impl_cipher_for_block_cipher!(Xtea, 8);

#[cfg(test)]
mod xtea_tests {

    use crate::Cipher;

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "0102030405060708";
        let cipher = Xtea::default().with_key([0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]);
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
