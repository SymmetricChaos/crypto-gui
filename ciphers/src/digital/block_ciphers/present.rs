use utils::byte_formatting::ByteFormat;

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

pub struct Present {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u64,
    pub round_keys: [u32; 31],
    pub mode: BCMode,
    pub padding: BCPadding,
}

crate::block_cipher_builders! {Present, u64}

impl Default for Present {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            round_keys: [0; 31],
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Present {
    pub fn ksa(&mut self, bytes: [u8; 16]) {}

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<16> for Present {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];

        for i in 0..31 {}
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];

        for i in 0..31 {}
    }
}

// crate::impl_cipher_for_block_cipher!(Present, 8);

// #[cfg(test)]
// mod present_tests {

//     use super::*;

// }

// crate::test_block_cipher!(

// );
