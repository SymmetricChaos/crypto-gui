use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crate::impl_cipher_for_block_cipher;
use utils::byte_formatting::{u32_pair_to_u8_array, ByteFormat};

const DELTA: u32 = 0x9e3779b9;

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

impl Xtea {}

impl BlockCipher<8> for Xtea {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
            *elem = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        let mut sum: u32 = 0;
        for _ in 0..32 {
            v[0] = v[0].wrapping_add(
                (v[1] << 4)
                    ^ (v[1] >> 5).wrapping_add(v[1])
                    ^ sum.wrapping_add(self.key[(sum % 4) as usize]),
            );
            sum = sum.wrapping_add(DELTA);
            v[1] = v[1].wrapping_add(
                (v[0] << 4)
                    ^ (v[0] >> 5).wrapping_add(v[0])
                    ^ sum.wrapping_add(self.key[((sum >> 11) % 4) as usize]),
            );
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
            v[1] = v[1].wrapping_sub(
                (v[0] << 4)
                    ^ (v[0] >> 5).wrapping_add(v[0])
                    ^ sum.wrapping_add(self.key[((sum >> 11) % 4) as usize]),
            );
            sum = sum.wrapping_sub(DELTA);
            v[0] = v[0].wrapping_sub(
                (v[1] << 4)
                    ^ (v[1] >> 5).wrapping_add(v[1])
                    ^ sum.wrapping_add(self.key[(sum % 4) as usize]),
            );
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
