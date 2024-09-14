use crate::traits::ClassicHasher;
use strum::EnumIter;
use utils::byte_formatting::ByteFormat;

use super::AsconState;

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum Variant {
    AsconMac,
    AsconPrf,
    AsconPrfShort,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AsconMac => write!(f, "Ascon-MAC"),
            Self::AsconPrf => write!(f, "Ascon-PRF"),
            Self::AsconPrfShort => write!(f, "Ascon-PRFshort"),
        }
    }
}

impl Variant {
    pub fn initialize(&self, key: [u64; 2], hash_len: u64) -> AsconState {
        assert!(hash_len <= 128);
        match self {
            Variant::AsconMac => {
                AsconState::initialize_full([0x80808c0000000080, key[0], key[1], 0, 0])
            }
            Variant::AsconPrf => {
                AsconState::initialize_full([0x80808c0000000000, key[0], key[1], 0, 0])
            }
            Variant::AsconPrfShort => {
                AsconState::initialize_full([0x80808c0000000000 ^ hash_len, key[0], key[1], 0, 0])
            }
        }
    }
}

pub struct Ascon {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub hash_len: u64,
    pub key: [u64; 2],
    pub variant: Variant,
}

impl Default for Ascon {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            key: [0, 0],
            variant: Variant::AsconPrf,
        }
    }
}

impl Ascon {
    pub fn ascon_prf() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            key: [0, 0],
            variant: Variant::AsconPrf,
        }
    }
    pub fn ascon_prfshort(hash_len: u64) -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len,
            key: [0, 0],
            variant: Variant::AsconPrfShort,
        }
    }
    pub fn ascon_mac() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            key: [0, 0],
            variant: Variant::AsconMac,
        }
    }
}

impl ClassicHasher for Ascon {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = self.variant.initialize(self.key, self.hash_len);
        state.absorb_12(&bytes);
        state.squeeze_12(self.hash_len as usize)
    }

    crate::hash_bytes_from_string! {}
}

// #[cfg(test)]
// mod ascon_tests {
//     use super::*;

//     #[test]
//     fn test_initialization_hash() {
//         assert_eq!(
//             AsconState::initialize(0x00400c0000000100).0,
//             Variant::AsconHash.initialize().0
//         )
//     }

//     #[test]
//     fn test_initialization_xof() {
//         assert_eq!(
//             AsconState::initialize(0x00400c0000000000).0,
//             Variant::AsconXof.initialize().0
//         )
//     }
// }

// crate::basic_hash_tests!(
//     Ascon::ascon_hash(), ascon_hash_0, "",
//     "7346bc14f036e87ae03d0997913088f5f68411434b3cf8b54fa796a80d251f91";
//     Ascon::ascon_hash(), ascon_hash_1, "00",
//     "8dd446ada58a7740ecf56eb638ef775f7d5c0fd5f0c2bbbdfdec29609d3c43a2";
//     Ascon::ascon_hash(), ascon_hash_2, "0001",
//     "f77ca13bf89146d3254f1cfb7eddba8fa1bf162284bb29e7f645545cf9e08424";
//     Ascon::ascon_hash(), ascon_hash_7, "000102030405",
//     "9c52142852beb6654907cc23cc5b171075d411ca80082aafd7dd0d09ba0bba1d";
//     Ascon::ascon_hash(), ascon_hash_8, "00010203040506",
//     "dd409ccc0c60cd7f474c0beed1e1cd48140ad45d5136dc5fda5ebe283df8d3f6";
//     Ascon::ascon_hash(), ascon_hash_1025, "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
//     "2eb89744de7f9a6f47d53db756bb2f67b127da96762a1c47a5d7bfc1f7273f5c";
// );
