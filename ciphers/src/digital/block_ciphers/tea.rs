use crate::impl_block_cipher;
use utils::byte_formatting::{overwrite_bytes, u32_pair_to_u8_array, ByteFormat};

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

const DELTA: u32 = 0x9e3779b9;

pub struct Tea {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; 4],
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Tea {
    fn default() -> Self {
        Self {
            key: [0, 1, 2, 3],
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Tea {}

impl BlockCipher<8> for Tea {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
            *elem = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        let mut sum: u32 = 0;
        for _ in 0..32 {
            sum = sum.wrapping_add(DELTA);
            v[0] = v[0].wrapping_add(
                ((v[1] << 4).wrapping_add(self.key[0]))
                    ^ (v[1].wrapping_add(sum))
                    ^ ((v[1] >> 5).wrapping_add(self.key[1])),
            );
            v[1] = v[1].wrapping_add(
                ((v[0] << 4).wrapping_add(self.key[2]))
                    ^ (v[0].wrapping_add(sum))
                    ^ ((v[0] >> 5).wrapping_add(self.key[3])),
            );
        }
        overwrite_bytes(bytes, &u32_pair_to_u8_array(v));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
            *elem = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        let mut sum: u32 = 0xC6EF3720;
        for _ in 0..32 {
            v[1] = v[1].wrapping_sub(
                ((v[0] << 4).wrapping_add(self.key[2]))
                    ^ (v[0].wrapping_add(sum))
                    ^ ((v[0] >> 5).wrapping_add(self.key[3])),
            );
            v[0] = v[0].wrapping_sub(
                ((v[1] << 4).wrapping_add(self.key[0]))
                    ^ (v[1].wrapping_add(sum))
                    ^ ((v[1] >> 5).wrapping_add(self.key[1])),
            );
            sum = sum.wrapping_sub(DELTA);
        }
        overwrite_bytes(bytes, &u32_pair_to_u8_array(v));
    }
}

impl_block_cipher!(Tea, 8);

#[cfg(test)]
mod tea_tests {

    use crate::Cipher;

    use super::*;

    #[test]
    fn encrypt_decrypt_ecb() {
        let ptext = "01020304050607080102030405060708";
        let mut cipher = Tea::default();
        cipher.mode = BCMode::Ecb;
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    #[test]
    fn encrypt_decrypt_ctr() {
        let ptext = "01020304050607080102030405060708";
        let mut cipher = Tea::default();
        cipher.mode = BCMode::Ctr;
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
