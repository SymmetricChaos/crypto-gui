use crate::{
    errors::HasherError,
    md4::Md4,
    md5::Md5,
    sha::{
        sha0::Sha0,
        sha256::{Sha2_224, Sha2_256},
        Sha1, Sha2_384, Sha2_512,
    },
    traits::ClassicHasher,
};
use strum::{Display, EnumIter, VariantNames};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, VariantNames)]
#[strum(serialize_all = "UPPERCASE")]
pub enum HmacVariant {
    Sha0,
    Sha1,
    Md4,
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

impl HmacVariant {
    pub fn block_size(&self) -> usize {
        match self {
            Self::Sha0 => 64,
            Self::Sha1 => 64,
            Self::Md4 => 64,
            Self::Md5 => 64,
            Self::Sha224 => 64,
            Self::Sha256 => 64,
            Self::Sha384 => 128,
            Self::Sha512 => 128,
        }
    }

    pub fn hasher(&self) -> Box<dyn ClassicHasher> {
        match self {
            Self::Sha0 => Box::new(Sha0::default()),
            Self::Sha1 => Box::new(Sha1::default()),
            Self::Md4 => Box::new(Md4::default()),
            Self::Md5 => Box::new(Md5::default()),
            Self::Sha224 => Box::new(Sha2_224::default()),
            Self::Sha256 => Box::new(Sha2_256::default()),
            Self::Sha384 => Box::new(Sha2_384::default()),
            Self::Sha512 => Box::new(Sha2_512::default()),
        }
    }
}

pub struct Hmac {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: Vec<u8>,
    pub variant: HmacVariant,
}
impl Default for Hmac {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: Vec::new(),
            variant: HmacVariant::Sha256,
        }
    }
}
impl Hmac {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn variant(mut self, variant: HmacVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn key(mut self, key: Vec<u8>) -> Self {
        self.key = key;
        self
    }

    pub fn key_from_str(mut self, format: ByteFormat, key_str: &str) -> Self {
        let bytes = format.text_to_bytes(key_str).expect("invalid key string");
        self.key = bytes;
        self
    }

    // For changing the key interactively
    pub fn set_key(&mut self, key: &[u8]) {
        self.key = key.to_vec();
    }

    // Falliable method for changing the key from a string interactively
    pub fn set_key_from_str(
        &mut self,
        format: ByteFormat,
        key_str: &str,
    ) -> Result<(), HasherError> {
        let bytes = format
            .text_to_bytes(key_str)
            .map_err(|_| HasherError::general("byte format error"))?;
        self.key = bytes;
        Ok(())
    }
}

impl ClassicHasher for Hmac {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let hasher = self.variant.hasher();
        let block_size = self.variant.block_size();

        let k = if self.key.len() > block_size {
            let mut k = hasher.hash(&self.key);
            k.truncate(block_size);
            k
        } else {
            self.key.clone()
        };

        let mut o_key: Vec<u8> = vec![0x5c; block_size];
        utils::byte_formatting::xor_into_bytes(&mut o_key, &k);
        let mut i_key: Vec<u8> = vec![0x36; block_size];
        utils::byte_formatting::xor_into_bytes(&mut i_key, &k);
        i_key.extend_from_slice(bytes);
        let inner = hasher.hash(&i_key);
        o_key.extend_from_slice(&inner);
        hasher.hash(&o_key)
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1, Hmac::default().key_from_str(ByteFormat::Utf8, "key"), "The quick brown fox jumps over the lazy dog", "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8";
);
