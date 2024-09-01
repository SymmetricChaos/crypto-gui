use utils::byte_formatting::{fill_u64s_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

pub struct Simon {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Simon {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Simon {}

impl BlockCipher<16> for Simon {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        todo!()
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        todo!()
    }
}

// crate::impl_cipher_for_block_cipher!(Simon, 16);
