use crate::{
    errors::HasherError,
    md4::Md4,
    md5::Md5,
    sha::{
        sha1::Sha1,
        sha256::{Sha2_224, Sha2_256},
        sha512::{Sha2_384, Sha2_512},
        Sha0,
    },
    traits::{ClassicHasher, KeyedHasher},
};
use strum::{Display, EnumIter, VariantNames};
use utils::byte_formatting::ByteFormat;

macro_rules! hmac {
    ($name: ident, $hash_function: ty, $block_size: literal) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub key_format: ByteFormat,
            pub key: Vec<u8>,
            pub hasher: $hash_function,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Utf8,
                    output_format: ByteFormat::Hex,
                    key_format: ByteFormat::Hex,
                    key: Vec::new(),
                    hasher: <$hash_function>::default(),
                }
            }
        }

        impl $name {
            pub const BLOCK_SIZE: usize = $block_size;

            pub fn block_size(&self) -> usize {
                $block_size
            }

            pub fn key_from_str(&mut self, key_str: &str) -> Result<(), HasherError> {
                let bytes = self
                    .key_format
                    .text_to_bytes(key_str)
                    .map_err(|_| HasherError::general("byte format error"))?;
                self.key = bytes;
                Ok(())
            }
        }

        impl ClassicHasher for $name {
            fn hash(&self, bytes: &[u8]) -> Vec<u8> {
                // Compress the key if necessary
                let k = if self.key.len() > Self::BLOCK_SIZE {
                    let mut k = self.hasher.hash(&self.key);
                    k.truncate(Self::BLOCK_SIZE);
                    k
                } else {
                    self.key.clone()
                };
                // XOR the key into the outer padding and the inner padding
                let mut o_key: Vec<u8> = vec![0x5c; Self::BLOCK_SIZE];
                for (i, byte) in k.iter().enumerate() {
                    o_key[i] ^= byte;
                }
                let mut i_key: Vec<u8> = vec![0x36; Self::BLOCK_SIZE];
                for (i, byte) in k.iter().enumerate() {
                    i_key[i] ^= byte;
                }

                i_key.extend_from_slice(bytes);
                let inner = self.hasher.hash(&i_key);
                o_key.extend_from_slice(&inner);
                self.hasher.hash(&o_key)
            }

            crate::hash_bytes_from_string! {}
        }

        impl KeyedHasher for $name {
            fn set_salt(&mut self, _bytes: &[u8]) {
                unimplemented!("HMAC does not accept a salt argument")
            }

            fn set_key(&mut self, bytes: &[u8]) {
                self.key = bytes.to_vec();
            }
        }
    };
}

hmac!(HmacSha0, Sha0, 64);
hmac!(HmacSha1, Sha1, 64);
hmac!(HmacMd4, Md4, 64);
hmac!(HmacMd5, Md5, 64);
hmac!(HmacSha224, Sha2_224, 64);
hmac!(HmacSha256, Sha2_256, 64);
hmac!(HmacSha384, Sha2_384, 128);
hmac!(HmacSha512, Sha2_512, 128);

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, VariantNames)]
#[strum(serialize_all = "UPPERCASE")]
pub enum SelectHmac {
    Sha0,
    Sha1,
    Md4,
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

impl SelectHmac {
    pub fn new(&self) -> Box<dyn KeyedHasher> {
        match self {
            SelectHmac::Sha0 => Box::new(HmacSha0::default()),
            SelectHmac::Sha1 => Box::new(HmacSha1::default()),
            SelectHmac::Md5 => Box::new(HmacMd5::default()),
            SelectHmac::Md4 => Box::new(HmacMd4::default()),
            SelectHmac::Sha224 => Box::new(HmacSha224::default()),
            SelectHmac::Sha256 => Box::new(HmacSha256::default()),
            SelectHmac::Sha384 => Box::new(HmacSha384::default()),
            SelectHmac::Sha512 => Box::new(HmacSha512::default()),
        }
    }
}

#[cfg(test)]
mod hmac_tests {

    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = HmacSha256::default();

        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.key_format = ByteFormat::Utf8;

        hasher.key_from_str("key").unwrap();

        assert_eq!(
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8",
            hasher
                .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
                .unwrap()
        );

        let mut hasher = HmacSha512::default();

        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.key_format = ByteFormat::Utf8;

        hasher.key_from_str("key").unwrap();

        assert_eq!(
            "b42af09057bac1e2d41708e48a902e09b5ff7f12ab428a4fe86653c73dd248fb82f948a549f7b791a5b41915ee4d1ec3935357e4e2317250d0372afa2ebeeb3a",
            hasher
                .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
                .unwrap()
        );
    }
}
