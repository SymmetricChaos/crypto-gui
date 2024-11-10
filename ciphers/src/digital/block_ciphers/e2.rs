use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::ByteFormat;

pub struct E2 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for E2 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

crate::block_cipher_builders! {E2, u128}

impl E2 {
    pub fn ksa(&mut self, bytes: [u8; 16]) {}

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<16> for E2 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        todo!()
    }
    fn decrypt_block(&self, bytes: &mut [u8]) {
        todo!()
    }
}

crate::impl_cipher_for_block_cipher!(E2, 16);

crate::test_block_cipher!(
    test_1, E2::default().with_key([0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]),
    [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
    [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
);
