use utils::byte_formatting::ByteFormat;

use super::super::block_cipher::{BCMode, BCPadding, BlockCipher};

pub struct Khafre {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u64,
    pub round_keys: [u32; 16],
    pub mode: BCMode,
    pub padding: BCPadding,
}

crate::block_cipher_builders! {Khafre, u64}

impl Default for Khafre {
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

impl Khafre {
    pub fn ksa(&mut self, bytes: [u8; 64]) {}

    pub fn with_key(mut self, bytes: [u8; 64]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<8> for Khafre {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        utils::byte_formatting::fill_u32s_be(&mut v, bytes);
        for key in self.round_keys {}
        utils::byte_formatting::u32s_to_bytes_be(bytes, &v);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        utils::byte_formatting::fill_u32s_be(&mut v, bytes);
        for key in self.round_keys {}
        utils::byte_formatting::u32s_to_bytes_be(bytes, &v);
    }
}

// crate::impl_cipher_for_block_cipher!(Khafre, 8);

// #[cfg(test)]
// mod khafre_tests {

//     use super::*;

// }

// crate::test_block_cipher!(

// );