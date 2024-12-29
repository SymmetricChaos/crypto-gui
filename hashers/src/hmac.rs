use crate::{
    md4::Md4,
    md5::Md5,
    sha::{Sha0, Sha1, Sha2_224, Sha2_256, Sha2_384, Sha2_512},
    traits::StatefulHasher,
};
use strum::{Display, EnumIter, VariantNames};

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

    pub fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self {
            Self::Sha0 => Sha0::default().hash(bytes),
            Self::Sha1 => Sha1::default().hash(bytes),
            Self::Md4 => Md4::default().hash(bytes),
            Self::Md5 => Md5::default().hash(bytes),
            Self::Sha224 => Sha2_224::default().hash(bytes),
            Self::Sha256 => Sha2_256::default().hash(bytes),
            Self::Sha384 => Sha2_384::default().hash(bytes),
            Self::Sha512 => Sha2_512::default().hash(bytes),
        }
    }
}

pub struct Hmac {
    i_key: Vec<u8>,
    o_key: Vec<u8>,
    variant: HmacVariant,
}

impl Hmac {
    pub fn init(key: &[u8], variant: HmacVariant) -> Self {
        let block_size = variant.block_size();

        let k = if key.len() > block_size {
            variant.hash(&key)
        } else {
            key.to_vec()
        };

        let mut i_key: Vec<u8> = vec![0x36; block_size];
        utils::byte_formatting::xor_into_bytes(&mut i_key, &k);
        let mut o_key: Vec<u8> = vec![0x5c; block_size];
        utils::byte_formatting::xor_into_bytes(&mut o_key, &k);

        Self {
            i_key,
            o_key,
            variant,
        }
    }
}

impl StatefulHasher for Hmac {
    fn update(&mut self, bytes: &[u8]) {
        self.i_key.extend_from_slice(bytes);
    }

    fn finalize(mut self) -> Vec<u8> {
        let inner = self.variant.hash(&self.i_key);
        self.o_key.extend_from_slice(&inner);
        self.variant.hash(&self.o_key)
    }

    crate::stateful_hash_helpers!();
}

// https://datatracker.ietf.org/doc/html/rfc4231
crate::stateful_hash_tests!(
    test1_sha256, Hmac::init(&[0x0b; 20], HmacVariant::Sha256),
    b"Hi There",
    "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7";

    test2_sha256, Hmac::init(b"Jefe", HmacVariant::Sha256),
    b"what do ya want for nothing?",
    "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";

    test3_sha256, Hmac::init(&[0xaa; 20], HmacVariant::Sha256),
    &[0xdd; 50],
    "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe";

    test6_sha256, Hmac::init(&[0xaa; 131], HmacVariant::Sha256),
    b"Test Using Larger Than Block-Size Key - Hash Key First",
    "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54";

    test7_sha256, Hmac::init(&[0xaa; 131], HmacVariant::Sha256),
    b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.",
    "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2";
);
