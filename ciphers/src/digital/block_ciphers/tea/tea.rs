use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::{fill_u32s_be, make_u32s_be, u32s_to_bytes_be, ByteFormat};

pub fn mx_e(a: u32, b: u32, sum: u32, k1: u32, k2: u32) -> u32 {
    b.wrapping_add((a << 4).wrapping_add(k1) ^ (a.wrapping_add(sum)) ^ (a >> 5).wrapping_add(k2))
}
pub fn mx_d(a: u32, b: u32, sum: u32, k1: u32, k2: u32) -> u32 {
    b.wrapping_sub((a << 4).wrapping_add(k1) ^ (a.wrapping_add(sum)) ^ (a >> 5).wrapping_add(k2))
}

pub struct Tea {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub subkeys: [u32; 4],
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Tea {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            subkeys: [0, 0, 0, 0],
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

crate::block_cipher_builders! {Tea, u64}

impl Tea {
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        fill_u32s_be(&mut self.subkeys, &bytes);
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<8> for Tea {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = make_u32s_be::<2>(bytes);
        let mut sum: u32 = 0;
        for _ in 0..32 {
            sum = sum.wrapping_add(super::DELTA);
            v[0] = mx_e(v[1], v[0], sum, self.subkeys[0], self.subkeys[1]);
            v[1] = mx_e(v[0], v[1], sum, self.subkeys[2], self.subkeys[3]);
        }
        u32s_to_bytes_be(bytes, &v);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = make_u32s_be::<2>(bytes);
        let mut sum: u32 = 0xC6EF3720;

        for _ in 0..32 {
            v[1] = mx_d(v[0], v[1], sum, self.subkeys[2], self.subkeys[3]);
            v[0] = mx_d(v[1], v[0], sum, self.subkeys[0], self.subkeys[1]);
            sum = sum.wrapping_sub(super::DELTA);
        }
        u32s_to_bytes_be(bytes, &v);
    }
}

crate::impl_cipher_for_block_cipher!(Tea, 8);

#[cfg(test)]
mod tea_tests {

    use crate::Cipher;

    use super::*;

    #[test]
    fn encrypt_decrypt_ecb() {
        let ptext = "01020304050607080102030405060708";
        let mut cipher = Tea::default().with_key([0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]);
        cipher.mode = BCMode::Ecb;
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    #[test]
    fn encrypt_decrypt_ctr() {
        let ptext = "01020304050607080102030405060708";
        let mut cipher = Tea::default().with_key([0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]);
        cipher.mode = BCMode::Ctr;
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
