use crate::traits::ClassicHasher;
use strum::EnumIter;
use utils::byte_formatting::ByteFormat;

use super::AsconState;

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum Variant {
    AsconHash,
    AsconHasha,
    AsconXof,
    AsconXofa,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AsconHash => write!(f, "Ascon-Hash"),
            Self::AsconHasha => write!(f, "Ascon-Hasha"),
            Self::AsconXof => write!(f, "Ascon-XOF"),
            Self::AsconXofa => write!(f, "Ascon-XOFa"),
        }
    }
}

impl Variant {
    pub fn initialize(&self) -> AsconState {
        match self {
            Variant::AsconHash => AsconState([
                0xee9398aadb67f03d,
                0x8bb21831c60f1002,
                0xb48a92db98d5da62,
                0x43189921b8f8e3e8,
                0x348fa5c9d525e140,
            ]),
            Variant::AsconHasha => AsconState([
                0x01470194fc6528a6,
                0x738ec38ac0adffa7,
                0x2ec8e3296c76384c,
                0xd6f6a54d7f52377d,
                0xa13c42a223be8d87,
            ]),
            Variant::AsconXof => AsconState([
                0xb57e273b814cd416,
                0x2b51042562ae2420,
                0x66a3a7768ddf2218,
                0x5aad0a7a8153650c,
                0x4f3e0e32539493b6,
            ]),
            Variant::AsconXofa => AsconState([
                0x44906568b77b9832,
                0xcd8d6cae53455532,
                0xf7b5212756422129,
                0x246885e1de0d225b,
                0xa8cb5ce33449973f,
            ]),
        }
    }
}

pub struct AsconMac {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub hash_len: usize,
    pub variant: Variant,
}

impl Default for AsconMac {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            variant: Variant::AsconHash,
        }
    }
}

impl AsconMac {
    pub fn ascon_hash() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            variant: Variant::AsconHash,
        }
    }
    pub fn ascon_hasha() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            variant: Variant::AsconHasha,
        }
    }
    pub fn ascon_xof(hash_len: usize) -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len,
            variant: Variant::AsconXof,
        }
    }
    pub fn ascon_xofa(hash_len: usize) -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len,
            variant: Variant::AsconXofa,
        }
    }
}

impl ClassicHasher for AsconMac {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = self.variant.initialize();
        match self.variant {
            Variant::AsconHash | Variant::AsconXof => {
                state.absorb_64_hash(&bytes, 12);
                state.squeeze_64_hash(self.hash_len, 12)
            }
            Variant::AsconHasha | Variant::AsconXofa => {
                state.absorb_64_hash(&bytes, 8);
                state.squeeze_64_hash(self.hash_len, 8)
            }
        }
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod ascon_tests {
    use super::*;

    #[test]
    fn test_initialization_hash() {
        assert_eq!(
            AsconState::initialize(0x00400c0000000100).0,
            Variant::AsconHash.initialize().0
        )
    }

    #[test]
    fn test_initialization_xof() {
        assert_eq!(
            AsconState::initialize(0x00400c0000000000).0,
            Variant::AsconXof.initialize().0
        )
    }
}

pub const INPUT_1: &'static str = "";
pub const INPUT_2: &'static str = "00";
pub const INPUT_9: &'static str = "0001020304050607";
pub const INPUT_1025: &'static str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";

crate::basic_hash_tests!(
    ascon_hash_1, AsconMac::ascon_hash(), INPUT_1,
    "7346bc14f036e87ae03d0997913088f5f68411434b3cf8b54fa796a80d251f91";
    ascon_hash_2, AsconMac::ascon_hash(), INPUT_2,
    "8dd446ada58a7740ecf56eb638ef775f7d5c0fd5f0c2bbbdfdec29609d3c43a2";
    ascon_hash_9, AsconMac::ascon_hash(), INPUT_9,
    "f4c6a44b29915d3d57cf928a18ec6226bb8dd6c1136acd24965f7e7780cd69cf";
    ascon_hash_1025, AsconMac::ascon_hash(), INPUT_1025,
    "2eb89744de7f9a6f47d53db756bb2f67b127da96762a1c47a5d7bfc1f7273f5c";

    ascon_xof_1, AsconMac::ascon_xof(32), INPUT_1,
    "5d4cbde6350ea4c174bd65b5b332f8408f99740b81aa02735eaefbcf0ba0339e";
    ascon_xof_2, AsconMac::ascon_xof(32), INPUT_2,
    "b2edbb27ac8397a55bc83d137c151de9ede048338fe907f0d3629e717846fedc";
    ascon_xof_1025, AsconMac::ascon_xof(32), INPUT_1025,
    "675b6da0d02ddd65042b7487bdefce06a4be090662ed39a703ad802c977a4b3b";


    ascon_hasha_1, AsconMac::ascon_hasha(), INPUT_1,
    "aecd027026d0675f9de7a8ad8ccf512db64b1edcf0b20c388a0c7cc617aaa2c4";
    ascon_hasha_2, AsconMac::ascon_hasha(), INPUT_2,
    "5a55f0367763d334a3174f9c17fa476eb9196a22f10daf29505633572e7756e4";
    ascon_hasha_1025, AsconMac::ascon_hasha(), INPUT_1025,
    "14f6a0c1e5751733955b820ca67bc89bb7eb7014c88caeb5f380d75eed484fe9";

    ascon_xofa_1, AsconMac::ascon_xofa(32), INPUT_1,
    "7c10dffd6bb03be262d72fbe1b0f530013c6c4eadaabde278d6f29d579e3908d";
    ascon_xofa_1025, AsconMac::ascon_xofa(32), INPUT_1025,
    "8096e9bb573ea6b2c1d7acac7fb9d9f8f6c89e52a63b1b129037fd4fcc913ffb";
);
