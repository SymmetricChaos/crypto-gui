use std::fmt::Display;

use strum::EnumIter;
use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

use super::{
    ripemd0::RipeMd0, ripemd128::RipeMd128, ripemd160::RipeMd160, ripemd256::RipeMd256,
    ripemd320::RipeMd320,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum RipeMdVariant {
    Md0,
    Md128,
    Md160,
    Md256,
    Md320,
}

impl Display for RipeMdVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RipeMdVariant::Md0 => write!(f, "RIPEMD (original)"),
            RipeMdVariant::Md128 => write!(f, "RIPEMD-128"),
            RipeMdVariant::Md160 => write!(f, "RIPEMD-160"),
            RipeMdVariant::Md256 => write!(f, "RIPEMD-256"),
            RipeMdVariant::Md320 => write!(f, "RIPEMD-320"),
        }
    }
}

impl RipeMdVariant {
    pub fn hasher(&self) -> Box<dyn ClassicHasher> {
        match self {
            RipeMdVariant::Md0 => Box::new(RipeMd0::default()),
            RipeMdVariant::Md128 => Box::new(RipeMd128::default()),
            RipeMdVariant::Md160 => Box::new(RipeMd160::default()),
            RipeMdVariant::Md256 => Box::new(RipeMd256::default()),
            RipeMdVariant::Md320 => Box::new(RipeMd320::default()),
        }
    }
}

#[derive(Clone)]
pub struct RipeMd {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub variant: RipeMdVariant,
}

impl Default for RipeMd {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: RipeMdVariant::Md160,
        }
    }
}

impl ClassicHasher for RipeMd {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        self.variant.hasher().hash(bytes)
    }

    crate::hash_bytes_from_string! {}
}
