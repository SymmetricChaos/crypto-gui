use utils::byte_formatting::ByteFormat;

use crate::Cipher;

use super::block_cipher::{BlockCipher, BCMode, BCPadding};

struct Rc6 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub rounds: usize,
    pub state: Vec<u32>,
    pub ctr: u64,
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Rc6 {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            rounds: 20,
            state: Default::default(),
            ctr: Default::default(),
            iv: Default::default(),
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl BlockCipher<16> for Rc6 {
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

impl Cipher for Rc6 {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }
}
