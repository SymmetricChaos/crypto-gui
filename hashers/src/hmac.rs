use crate::{
    errors::HasherError,
    md4::Md4,
    md5::Md5,
    sha::{
        sha256::{Sha2_224, Sha2_256},
        // sha512::{Sha2_384, Sha2_512},
        Sha1,
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
    // Sha384,
    // Sha512,
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
    pub const BLOCK_SIZE: usize = 64; // Only hashers with a block size of 64 are allowed

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

    // Construct the hash function as needed to avoid complicated storage
    pub fn inner_hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.variant {
            HmacVariant::Sha0 => Sha1::sha0()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            HmacVariant::Sha1 => Sha1::sha1()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            HmacVariant::Md4 => Md4::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            HmacVariant::Md5 => Md5::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            HmacVariant::Sha224 => Sha2_224::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            HmacVariant::Sha256 => Sha2_256::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
        }
    }
}

impl ClassicHasher for Hmac {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let k = if self.key.len() > Self::BLOCK_SIZE {
            let mut k = self.inner_hash(&self.key);
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
        let inner = self.inner_hash(&i_key);
        o_key.extend_from_slice(&inner);
        self.inner_hash(&o_key)
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    Hmac::default().key_from_str(ByteFormat::Utf8, "key"), test1, "The quick brown fox jumps over the lazy dog", "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8";
);
