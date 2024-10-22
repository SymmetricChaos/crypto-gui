use utils::byte_formatting::ByteFormat;
use super::block_cipher::{BCMode, BCPadding};

pub struct Kasumi {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Kasumi {
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

crate::block_cipher_builders! {Kasumi, u64}