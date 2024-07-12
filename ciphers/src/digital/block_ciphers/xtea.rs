use utils::byte_formatting::{u32_pair_to_u8_array, ByteFormat};

use crate::{Cipher, CipherError};

use super::block_cipher::{none_padding, BlockCipher, BlockCipherMode, BlockCipherPadding};

pub struct Xtea {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; 4],
    pub ctr: u64,
    pub iv: u64,
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for Xtea {
    fn default() -> Self {
        Self {
            key: [0, 1, 2, 3],
            ctr: 0,
            iv: 0,
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            mode: BlockCipherMode::default(),
            padding: BlockCipherPadding::default(),
        }
    }
}

impl Xtea {
    const DELTA: u32 = 0x9e3779b9;
    const BLOCKSIZE: u32 = 8;
}

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
            sum = sum.wrapping_add(Self::DELTA);
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
            sum = sum.wrapping_sub(Self::DELTA);
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

    fn set_mode(&mut self, mode: BlockCipherMode) {
        self.mode = mode
    }

    fn set_padding(&mut self, padding: BlockCipherPadding) {
        self.padding = padding
    }
}

impl Cipher for Xtea {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.mode.padded() {
            self.padding.add_padding(&mut bytes, Self::BLOCKSIZE)?;
        }

        match self.mode {
            BlockCipherMode::Ecb => self.encrypt_ecb(&mut bytes),
            BlockCipherMode::Ctr => self.encrypt_ctr(&mut bytes, self.ctr.to_be_bytes()),
            BlockCipherMode::Cbc => self.encrypt_cbc(&mut bytes, self.iv.to_be_bytes()),
        };
        Ok(self.output_format.byte_slice_to_text(&bytes))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.mode.padded() {
            if self.padding == BlockCipherPadding::None {
                none_padding(&mut bytes, Self::BLOCKSIZE)?
            };
        }

        match self.mode {
            BlockCipherMode::Ecb => self.decrypt_ecb(&mut bytes),
            BlockCipherMode::Ctr => self.decrypt_ctr(&mut bytes, self.ctr.to_be_bytes()),
            BlockCipherMode::Cbc => self.decrypt_cbc(&mut bytes, self.iv.to_be_bytes()),
        };

        if self.mode.padded() {
            self.padding.strip_padding(&mut bytes, Self::BLOCKSIZE)?;
        }

        Ok(self.output_format.byte_slice_to_text(&bytes))
    }
}

#[cfg(test)]
mod xtea_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "0102030405060708";
        let cipher = Xtea::default();
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
