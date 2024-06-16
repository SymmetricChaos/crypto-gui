use utils::byte_formatting::ByteFormat;

use super::{BlockCipherMode, BlockCipherPadding};

pub struct Idea {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    // subkeys: [u16; 16],
    pub ctr: u64,
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for Idea {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            // subkeys: Default::default(),
            ctr: 0,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Idea {
    pub fn ksa(&mut self, key: u64) {}

    pub fn encrypt_block(&self, block: u64) {}
    pub fn decrypt_block(&self, block: u64) {}
}
