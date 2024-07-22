use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crate::impl_block_cipher;
use utils::byte_formatting::ByteFormat;

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
}

impl_block_cipher!(Feal, 8);
