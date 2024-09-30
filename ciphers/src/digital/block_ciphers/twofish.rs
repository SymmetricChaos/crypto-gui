use utils::byte_formatting::{fill_u32s_le, u32s_to_bytes_le, ByteFormat};

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

fn f() {
    todo!()
}

fn g(n: u32) {
    let mut x = n.to_le_bytes();

    todo!()
}

pub struct TwoFish128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u128,
    pub subkeys: [u32; 40],
    sboxes: [[u32; 256]; 4],
}

impl Default for TwoFish128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: BCMode::default(),
            padding: BCPadding::default(),
            iv: 0,
            subkeys: [0; 40],
            sboxes: [[0; 256]; 4],
        }
    }
}

crate::block_cipher_builders! {TwoFish128}

impl TwoFish128 {
    pub fn sbox(&self, n: u32, i: usize) -> u32 {
        self.sboxes[i][n as usize]
    }

    pub fn ksa_u32(&mut self, key: [u32; 8]) {}

    pub fn ksa(&mut self, bytes: [u8; 32]) {}

    pub fn with_key(mut self, bytes: [u8; 32]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<16> for TwoFish128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = [0; 4];
        fill_u32s_le(&mut block, bytes);

        u32s_to_bytes_le(bytes, &block);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = [0; 4];
        fill_u32s_le(&mut block, bytes);

        u32s_to_bytes_le(bytes, &block);
    }
}
