use utils::byte_formatting::ByteFormat;

use crate::{Cipher, CipherError};

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

pub struct Feal {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    subkeys: [u16; 16],
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Feal {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            subkeys: Default::default(),
            iv: 0,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Feal {
    pub fn ksa(&mut self, key: u64) {}

    pub fn encrypt_block(&self, block: u64) {}
    pub fn decrypt_block(&self, block: u64) {}
}

impl BlockCipher<8> for Feal {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        todo!()
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        todo!()
    }

    fn set_mode(&mut self, mode: BCMode) {
        self.mode = mode
    }

    fn set_padding(&mut self, padding: BCPadding) {
        self.padding = padding
    }
}

impl Cipher for Feal {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}
