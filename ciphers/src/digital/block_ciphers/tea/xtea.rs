use crate::{
    digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher},
    impl_cipher_for_block_cipher,
};
use utils::byte_formatting::{u32_pair_to_u8_array, ByteFormat};

pub struct Xtea {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
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
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

pub fn mx_e(a: u32, b: u32, sum: u32, k: u32) -> u32 {
    b.wrapping_add((((a << 4) ^ (a >> 5)).wrapping_add(a) & sum).wrapping_add(k))
}
pub fn mx_d(a: u32, b: u32, sum: u32, k: u32) -> u32 {
    b.wrapping_sub((((a << 4) ^ (a >> 5)).wrapping_add(a) & sum).wrapping_add(k))
}

impl BlockCipher<8> for Xtea {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
            *elem = u32::from_be_bytes(chunk.try_into().unwrap());
        }
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
        for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
            *elem = u32::from_be_bytes(chunk.try_into().unwrap());
        }
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
        let cipher = Xtea::default();
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
