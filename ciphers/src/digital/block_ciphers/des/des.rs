use crate::{
    digital::block_ciphers::{
        block_cipher::{none_padding, BlockCipher, BlockCipherMode, BlockCipherPadding},
        des::des_functions::*,
    },
    Cipher, CipherError,
};
use utils::byte_formatting::ByteFormat;

pub struct Des {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    subkeys: [u64; 16],
    pub ctr: u64,
    pub iv: u64,
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for Des {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            subkeys: [0; 16],
            ctr: 0,
            iv: 0,
            mode: BlockCipherMode::default(),
            padding: BlockCipherPadding::default(),
        }
    }
}

impl Des {
    pub const BLOCKSIZE: u32 = 8;

    // Key Scheduling Algorithm (key generation)
    pub fn ksa(&mut self, key: u64) -> Result<(), CipherError> {
        self.subkeys = des_ksa(key)?;
        Ok(())
    }
}

impl BlockCipher<8> for Des {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let block = u64::from_be_bytes(bytes.try_into().unwrap());
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter() {
            b = round(b, *key);
        }
        let f = final_permutation((b << 32) | (b >> 32));
        for (plaintext, ciphertext) in bytes.iter_mut().zip(f.to_be_bytes().iter()) {
            *plaintext = *ciphertext
        }
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let block = u64::from_be_bytes(bytes.try_into().unwrap());
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter().rev() {
            b = round(b, *key);
        }
        let f = final_permutation((b << 32) | (b >> 32));
        for (ciphertext, plaintext) in bytes.iter_mut().zip(f.to_be_bytes().iter()) {
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

impl Cipher for Des {
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
mod des_tests {

    use super::*;

    // #[test]
    // fn test_encrypt_decrypt_block() {
    //     let mut cipher = Des::default();
    //     cipher.ksa(0x0123456789ABCDEF).unwrap();

    //     let cblock = cipher.encrypt_block(0x4E6F772069732074);
    //     assert_eq!(cblock, 0x3FA40E8A984D4815);

    //     let dblock = cipher.decrypt_block(0x3FA40E8A984D4815);
    //     assert_eq!(dblock, 0x4E6F772069732074);
    // }

    #[test]
    fn test_encrypt_decrypt_ecb() {
        let mut cipher = Des::default();
        cipher.ksa(0x0123456789ABCDEF).unwrap();
        cipher.mode = BlockCipherMode::Ecb;
        cipher.padding = BlockCipherPadding::None;

        const PTEXT: &'static str = "4e6f772069732074";
        const CTEXT: &'static str = "3fa40e8a984d4815";

        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(CTEXT, ctext);

        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(PTEXT, dtext);
    }

    #[test]
    fn test_encrypt_decrypt_ctr() {
        let mut cipher = Des::default();
        cipher.ksa(0x0123456789ABCDEF).unwrap();
        cipher.mode = BlockCipherMode::Ctr;

        const PTEXT: &'static str = "4e6f772069732074";

        let ctext = cipher.encrypt(PTEXT).unwrap();

        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(PTEXT, dtext);
    }
}
