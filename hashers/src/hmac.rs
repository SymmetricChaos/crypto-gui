use crate::{
    errors::HasherError,
    md4::Md4,
    md5::Md5,
    sha::{
        sha256::{Sha2_224, Sha2_256},
        // sha512::{Sha2_384, Sha2_512},
        Sha1,
    },
    traits::{ClassicHasher, KeyedHasher},
};
use strum::{Display, EnumIter, VariantNames};
use utils::byte_formatting::ByteFormat;

pub struct Hmac {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key_format: ByteFormat,
    pub key: Vec<u8>,
    pub hasher: Box<dyn ClassicHasher>,
}
impl Default for Hmac {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key_format: ByteFormat::Hex,
            key: Vec::new(),
            hasher: Box::new(Sha2_256::default()),
        }
    }
}
impl Hmac {
    pub const BLOCK_SIZE: usize = 64; // Only hashers with a block size of 64 are allowed

    pub fn key_from_str(&mut self, key_str: &str) -> Result<(), HasherError> {
        let bytes = self
            .key_format
            .text_to_bytes(key_str)
            .map_err(|_| HasherError::general("byte format error"))?;
        self.key = bytes;
        Ok(())
    }
}
impl ClassicHasher for Hmac {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let k = if self.key.len() > Self::BLOCK_SIZE {
            let mut k = self.hasher.hash(&self.key);
            k.truncate(Self::BLOCK_SIZE);
            k
        } else {
            self.key.clone()
        };
        let mut o_key: Vec<u8> = vec![0x5c; Self::BLOCK_SIZE];
        utils::byte_formatting::xor_into_bytes(&mut o_key, &k);
        let mut i_key: Vec<u8> = vec![0x36; Self::BLOCK_SIZE];
        utils::byte_formatting::xor_into_bytes(&mut i_key, &k);
        i_key.extend_from_slice(bytes);
        let inner = self.hasher.hash(&i_key);
        o_key.extend_from_slice(&inner);
        self.hasher.hash(&o_key)
    }
    crate::hash_bytes_from_string! {}
}
impl KeyedHasher for Hmac {
    fn set_salt(&mut self, _bytes: &[u8]) {
        unimplemented!("HMAC does not accept a salt argument")
    }
    fn set_key(&mut self, bytes: &[u8]) {
        self.key = bytes.to_vec();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, VariantNames)]
#[strum(serialize_all = "UPPERCASE")]
pub enum SelectHmac {
    Sha0,
    Sha1,
    Md4,
    Md5,
    Sha224,
    Sha256,
    // Sha384,
    // Sha512,
}

impl SelectHmac {
    pub fn new(&self) -> Box<dyn ClassicHasher> {
        match self {
            SelectHmac::Sha0 => Box::new(Sha1::sha0()), // SHA0 is treated as a variant of SHA1 as they are nearly identical
            SelectHmac::Sha1 => Box::new(Sha1::default()),
            SelectHmac::Md4 => Box::new(Md4::default()),
            SelectHmac::Md5 => Box::new(Md5::default()),
            SelectHmac::Sha224 => Box::new(Sha2_224::default()),
            SelectHmac::Sha256 => Box::new(Sha2_256::default()),
            // SelectHmac::Sha384 => Box::new(Sha2_384::default()),
            // SelectHmac::Sha512 => Box::new(Sha2_512::default()),
        }
    }
}

#[cfg(test)]
mod hmac_tests {

    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Hmac::default();

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

        // let mut hasher = HmacSha512::default();

        // hasher.input_format = ByteFormat::Utf8;
        // hasher.output_format = ByteFormat::Hex;
        // hasher.key_format = ByteFormat::Utf8;

        // hasher.key_from_str("key").unwrap();

        // assert_eq!(
        //     "b42af09057bac1e2d41708e48a902e09b5ff7f12ab428a4fe86653c73dd248fb82f948a549f7b791a5b41915ee4d1ec3935357e4e2317250d0372afa2ebeeb3a",
        //     hasher
        //         .hash_bytes_from_string("The quick brown fox jumps over the lazy dog")
        //         .unwrap()
        // );
    }
}
