use utils::byte_formatting::ByteFormat;

use super::super::block_cipher::{BCMode, BCPadding, BlockCipher};

pub struct Khufu {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u64,
    pub round_keys: [u32; 16],
    pub mode: BCMode,
    pub padding: BCPadding,
}

crate::block_cipher_builders! {Khufu, u64}

impl Default for Khufu {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            round_keys: [0; 16],
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Khufu {
    pub fn ksa(&mut self, bytes: [u8; 64]) {}

    pub fn with_key(mut self, bytes: [u8; 64]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<8> for Khufu {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        utils::byte_formatting::fill_u32s_be(&mut v, &bytes);

        for key in self.round_keys {}
        utils::byte_formatting::u32s_to_bytes_be(bytes, &v);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        utils::byte_formatting::fill_u32s_be(&mut v, &bytes);

        for key in self.round_keys {}
        utils::byte_formatting::u32s_to_bytes_be(bytes, &v);
    }

    crate::block_cipher_getters!();
}

// crate::impl_cipher_for_block_cipher!(Khufu, 8);

// #[cfg(test)]
// mod khufu_tests {

//     use super::*;

// }

// crate::test_block_cipher!(

// );
