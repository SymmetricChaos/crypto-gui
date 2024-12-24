use crate::{ascon::unpadded_bytes_128, errors::HasherError, traits::ClassicHasher};
use strum::EnumIter;
use utils::byte_formatting::ByteFormat;

use super::AsconState;

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum Variant {
    AsconMac,
    AsconMaca,
    AsconPrf,
    AsconPrfa,
    AsconPrfShort,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AsconMac => write!(f, "Ascon-MAC"),
            Self::AsconMaca => write!(f, "Ascon-MACa"),
            Self::AsconPrf => write!(f, "Ascon-PRF"),
            Self::AsconPrfa => write!(f, "Ascon-PRFa"),
            Self::AsconPrfShort => write!(f, "Ascon-PRFshort"),
        }
    }
}

impl Variant {
    pub fn initialize(&self, key: [u64; 2], hash_bytes: u64, message: &[u8]) -> AsconState {
        match self {
            Variant::AsconMac => {
                assert!(
                    hash_bytes <= 16,
                    "Ascon-MAC must have a hash length of 128 bits or less"
                );
                AsconState::initialize_full([0x80808c0000000080, key[0], key[1], 0, 0])
            }
            Variant::AsconMaca => {
                assert!(
                    hash_bytes <= 16,
                    "Ascon-MAC must have a hash length of 128 bits or less"
                );
                AsconState::initialize_full([0x80808c0400000080, key[0], key[1], 0, 0])
            }
            Variant::AsconPrf => {
                AsconState::initialize_full([0x80808c0000000000, key[0], key[1], 0, 0])
            }
            Variant::AsconPrfa => {
                AsconState::initialize_full([0x80808c0400000000, key[0], key[1], 0, 0])
            }
            Variant::AsconPrfShort => {
                assert!(
                    hash_bytes <= 16,
                    "Ascon-PRFshort must have a hash length of 128 bits or less"
                );
                assert!(
                    message.len() <= 16,
                    "Ascon-PRFshort must have a message length of 128 bits or less"
                );
                let iv = u64::from_be_bytes([
                    0x80,
                    (message.len() * 8) as u8,
                    0x4C,
                    (hash_bytes * 8) as u8,
                    0,
                    0,
                    0,
                    0,
                ]);
                let [a, b] = unpadded_bytes_128(&message);
                AsconState::initialize_full([iv, key[0], key[1], a, b])
            }
        }
    }
}

pub struct AsconMac {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub hash_len: u64,
    pub key: [u64; 2],
    pub variant: Variant,
}

impl Default for AsconMac {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 16,
            key: [0, 0],
            variant: Variant::AsconPrf,
        }
    }
}

impl AsconMac {
    pub fn ascon_prf() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 16,
            key: [0, 0],
            variant: Variant::AsconPrf,
        }
    }
    pub fn ascon_prfa() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 16,
            key: [0, 0],
            variant: Variant::AsconPrfa,
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
            hash_len: 16,
            key: [0, 0],
            variant: Variant::AsconMac,
        }
    }
    pub fn ascon_maca() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 16,
            key: [0, 0],
            variant: Variant::AsconMaca,
        }
    }

    pub fn ksa(&mut self, bytes: [u8; 16]) {
        utils::byte_formatting::fill_u64s_be(&mut self.key, &bytes);
    }

    pub fn with_key(mut self, key: [u8; 16]) -> Self {
        self.ksa(key);
        self
    }
}

impl ClassicHasher for AsconMac {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        if self.variant == Variant::AsconPrfShort && bytes.len() > 16 {
            panic!("Ascon-PRFshort can only be used with inputs of 128 bits or less")
        }
        let mut state = self.variant.initialize(self.key, self.hash_len, bytes);
        match self.variant {
            Variant::AsconMac | Variant::AsconPrf => {
                state.absorb_256_prf(&bytes, 8);
                state.squeeze_128_prf(self.hash_len as usize, 12)
            }
            Variant::AsconMaca | Variant::AsconPrfa => {
                state.absorb_320_prf(&bytes, 8);
                state.squeeze_128_prf(self.hash_len as usize, 12)
            }
            Variant::AsconPrfShort => state.squeeze_128_prfshort(self.hash_len as usize, self.key),
        }
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, crate::errors::HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| crate::errors::HasherError::general("byte format error"))?;
        if self.variant == Variant::AsconPrfShort && bytes.len() > 16 {
            return Err(HasherError::general(
                "Ascon-PRFshort can only be used with inputs of 128 bits or less",
            ));
        }
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

pub const TEST_KEY: [u8; 16] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
];
pub const INPUT_1: &'static str = "";
pub const INPUT_2: &'static str = "00";
pub const INPUT_9: &'static str = "0001020304050607";
pub const INPUT_1025: &'static str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";

crate::basic_hash_tests!(
    ascon_prf_1, AsconMac::ascon_prf().with_key(TEST_KEY), INPUT_1,
    "2a766fe9a4894073bc811b19d54ac33d";
    ascon_prf_2, AsconMac::ascon_prf().with_key(TEST_KEY), INPUT_2,
    "62dcf5fd8253089b765e2cf1a0d1a4fa";
    ascon_prf_9, AsconMac::ascon_prf().with_key(TEST_KEY), INPUT_9,
    "25d813eea510ddef67d0152153c35bb8";
    ascon_prf_1025, AsconMac::ascon_prf().with_key(TEST_KEY), INPUT_1025,
    "3003aba5ab23b18d5ae5230b0c8d6af7";


    ascon_prfa_1, AsconMac::ascon_prfa().with_key(TEST_KEY), INPUT_1,
    "99fdc07ca98af6e6d282e84094cd79cf";
    ascon_prfa_2, AsconMac::ascon_prfa().with_key(TEST_KEY), INPUT_2,
    "08ae72db8e69d636b9964428dd5feb3f";
    ascon_prfa_9, AsconMac::ascon_prfa().with_key(TEST_KEY), INPUT_9,
    "55b7ed6b4eda680af96095156a8cdc87";
    ascon_prfa_1025, AsconMac::ascon_prfa().with_key(TEST_KEY), INPUT_1025,
    "66edf17a4b66dec6176db0fc7c146b89";



    ascon_prfshort_2, AsconMac::ascon_prfshort(16).with_key(TEST_KEY), "000102",
    "7715cf195fb35817ba24a4806d1173af";
    ascon_prfshort_11, AsconMac::ascon_prfshort(16).with_key(TEST_KEY), "00010203040506070809",
    "ca339213302143e914dc5684104431d4";
    ascon_prfshort_2_8, AsconMac::ascon_prfshort(8).with_key(TEST_KEY), "000102",
    "00ff4b8f834f25e1";
    ascon_prfshort_11_8, AsconMac::ascon_prfshort(8).with_key(TEST_KEY), "00010203040506070809",
    "d8c736023a30d56e";


    ascon_mac_1, AsconMac::ascon_mac().with_key(TEST_KEY), INPUT_1,
    "eb1af688825d66bf2d53e135f9323315";
    ascon_mac_2, AsconMac::ascon_mac().with_key(TEST_KEY), INPUT_2,
    "81f3c3537c5595aaa0d5780b9f88a043";
    ascon_mac_9, AsconMac::ascon_mac().with_key(TEST_KEY), INPUT_9,
    "e38a60a450275707bc69ddade9c2fb92";
    ascon_mac_1025, AsconMac::ascon_mac().with_key(TEST_KEY), INPUT_1025,
    "3f090d832d95322df4128e0e53a8ecbd";


    ascon_maca_1, AsconMac::ascon_maca().with_key(TEST_KEY), INPUT_1,
    "fddc38ec2e93f8b8524d88f6c5983d13";
    ascon_maca_2, AsconMac::ascon_maca().with_key(TEST_KEY), INPUT_2,
    "628a3773caae20b059fe89280e674735";
    ascon_maca_9, AsconMac::ascon_maca().with_key(TEST_KEY), INPUT_9,
    "c932830ced1ce26ffb53c061b26372ec";
    ascon_maca_1025, AsconMac::ascon_maca().with_key(TEST_KEY), INPUT_1025,
    "40962a720050d59e3ac61641d98733b3";
);
