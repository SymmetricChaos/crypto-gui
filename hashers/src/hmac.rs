use crate::{
    md4::Md4,
    md5::Md5,
    sha::{Keccack, Sha0, Sha1, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256},
    traits::{ResettableHasher, StatefulHasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HmacVariant {
    Sha0,
    Sha1,
    Md4,
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

impl HmacVariant {
    pub fn block_size(&self) -> u32 {
        match self {
            Self::Sha0 => 64,
            Self::Sha1 => 64,
            Self::Md4 => 64,
            Self::Md5 => 64,
            Self::Sha224 => 64,
            Self::Sha256 => 64,
            Self::Sha384 => 128,
            Self::Sha512 => 128,
            Self::Sha512_224 => 128,
            Self::Sha512_256 => 128,
            Self::Sha3_224 => 144,
            Self::Sha3_256 => 136,
            Self::Sha3_384 => 104,
            Self::Sha3_512 => 72,
        }
    }

    pub fn output_size(&self) -> u32 {
        match self {
            Self::Sha0 => 20,
            Self::Sha1 => 20,
            Self::Md4 => 16,
            Self::Md5 => 16,
            Self::Sha224 => 28,
            Self::Sha256 => 32,
            Self::Sha384 => 58,
            Self::Sha512 => 64,
            Self::Sha512_224 => 28,
            Self::Sha512_256 => 32,
            Self::Sha3_224 => 28,
            Self::Sha3_256 => 32,
            Self::Sha3_384 => 58,
            Self::Sha3_512 => 64,
        }
    }

    pub fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self {
            Self::Sha0 => Sha0::init().hash(bytes),
            Self::Sha1 => Sha1::init().hash(bytes),
            Self::Md4 => Md4::init().hash(bytes),
            Self::Md5 => Md5::init().hash(bytes),
            Self::Sha224 => Sha224::init().hash(bytes),
            Self::Sha256 => Sha256::init().hash(bytes),
            Self::Sha384 => Sha384::init().hash(bytes),
            Self::Sha512 => Sha512::init().hash(bytes),
            Self::Sha512_224 => Sha512_224::init().hash(bytes),
            Self::Sha512_256 => Sha512_256::init().hash(bytes),
            Self::Sha3_224 => Keccack::sha3_224().hash(bytes),
            Self::Sha3_256 => Keccack::sha3_256().hash(bytes),
            Self::Sha3_384 => Keccack::sha3_384().hash(bytes),
            Self::Sha3_512 => Keccack::sha3_512().hash(bytes),
        }
    }
}

pub struct Hmac {
    i_key: Vec<u8>,
    o_key: Vec<u8>,
    variant: HmacVariant,
    hash_len: u32,
}

impl Hmac {
    pub fn init(variant: HmacVariant, key: &[u8]) -> Self {
        let block_size = variant.block_size() as usize;

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
            hash_len: variant.output_size(),
        }
    }

    pub fn init_var(variant: HmacVariant, hash_len: u32, key: &[u8]) -> Self {
        let block_size = variant.block_size() as usize;

        if hash_len > variant.output_size() {
            panic!("hash length cannot be greater the output size of the haser")
        }

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
            hash_len,
        }
    }
}

impl ResettableHasher for Hmac {
    fn finalize_and_reset(&mut self) -> Vec<u8> {
        let save_i_key = self.i_key[..self.variant.block_size() as usize].to_vec();
        let save_o_key = self.o_key.clone();
        self.o_key
            .extend_from_slice(&self.variant.hash(&self.i_key));
        let mut h = self.variant.hash(&self.o_key);
        h.truncate(self.hash_len as usize);
        self.i_key = save_i_key;
        self.o_key = save_o_key;
        h
    }
}

impl StatefulHasher for Hmac {
    fn update(&mut self, bytes: &[u8]) {
        self.i_key.extend_from_slice(bytes);
    }

    fn finalize(mut self) -> Vec<u8> {
        self.o_key
            .extend_from_slice(&self.variant.hash(&self.i_key));
        let mut h = self.variant.hash(&self.o_key);
        h.truncate(self.hash_len as usize);
        h
    }

    crate::stateful_hash_helpers!();
}

// https://datatracker.ietf.org/doc/html/rfc4231
crate::stateful_hash_tests!(
    test2_sha256, Hmac::init(HmacVariant::Sha256, b"Jefe"),
    b"what do ya want for nothing?",
    "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";

    test3_sha256, Hmac::init(HmacVariant::Sha256, &[0xaa; 20]),
    &[0xdd; 50],
    "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe";

    test6_sha256, Hmac::init(HmacVariant::Sha256, &[0xaa; 131]),
    b"Test Using Larger Than Block-Size Key - Hash Key First",
    "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54";

    test7_sha256, Hmac::init(HmacVariant::Sha256, &[0xaa; 131]),
    b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.",
    "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2";


    // https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-224.ipd.pdf
    test1_sha256, Hmac::init_var(HmacVariant::Sha256, 16, &[0xc8, 0xd4, 0x6c, 0xbf, 0x65, 0x27, 0x1f, 0xcc, 0x60, 0xdb, 0x02, 0xe4, 0xd7, 0xcc, 0x4b, 0xd8, 0x75]),
    &[0x06, 0x3f, 0x0b, 0x6e, 0x89, 0x60, 0x82, 0x6c, 0xfb, 0xe3, 0x5e, 0xbd, 0xb0, 0x1b, 0x47, 0xea],
    "6b800744b38d0a9f2b9d64c582f7d6d9";

    test_sha3_224, Hmac::init_var(HmacVariant::Sha3_224, 11, &[0xf8, 0xa7, 0xed, 0x55, 0x62, 0xa7, 0x64, 0x6a, 0x22, 0xb4, 0xdb, 0xb1, 0x4d, 0x3a, 0xd8, 0x91, 0xca, 0x67, 0x78, 0x77, 0xda, 0xe3, 0x78, 0x60, 0x2f, 0x09, 0xce, 0x47, 0x9d, 0x3b, 0x11, 0xe8, 0x1a]),
    &[0x76, 0x27, 0xb1, 0x9c, 0xb5, 0x55, 0x94, 0x58, 0x7e, 0xda, 0xd2, 0xff, 0x0c, 0x22, 0xd2, 0x92],
    "1af28609d217bf6dfb1184";

    test_sha3_256, Hmac::init_var(HmacVariant::Sha3_256, 20, &[0x5f, 0x71, 0x2d, 0x90, 0xe6, 0x10, 0x53, 0x1a, 0xa2, 0x4e, 0x2c, 0x5c, 0xb5, 0x9b, 0x2b, 0x7f, 0x0e, 0x1d, 0x22, 0x98, 0x09, 0xb1, 0x0f, 0x46, 0x20, 0x1e, 0x48, 0xd4, 0x93, 0xeb, 0x67, 0x84, 0xec]),
    &[0x6D, 0x95, 0xCE, 0x1D, 0xEC, 0xC2, 0x21, 0x2A, 0xF7, 0xB3, 0x3A, 0x90, 0xD6, 0x29, 0x7E, 0x02],
    "ed29d0d3923524ae417f0b30dff8a4128dc202ae";

    test_sha3_384, Hmac::init_var(HmacVariant::Sha3_384, 20, &[0x63, 0xe7, 0x02, 0x0d, 0x5e, 0x01, 0x7a, 0xa8, 0xf8, 0x66, 0x18, 0xba, 0x4a, 0x4e, 0xd4, 0xbe, 0x03, 0x29, 0x8e, 0x92, 0xba, 0x8e, 0xf9, 0x7c, 0x73, 0x96, 0xd2, 0x60, 0x61, 0xb1, 0x2d, 0x5d, 0x63, 0x8c, 0x3e, 0x53, 0xff, 0x1b, 0x80, 0x52, 0xb5, 0xe2, 0x17, 0xa9, 0x27, 0xeb, 0x7d, 0x9b, 0x80, 0xce, 0xda, 0xc1, 0xce, 0xb2, 0x27, 0xa1, 0x3a, 0x02, 0x29, 0xdf, 0x54, 0x2f, 0x8b, 0x0f, 0x10, 0x40, 0xa5, 0xc8, 0xe9, 0x55, 0x8c, 0xdd, 0xeb]),
    &[0xc4, 0x22, 0x28, 0x88, 0xaf, 0xab, 0x77, 0xe7, 0xc9, 0x20, 0x6d, 0x28, 0x94, 0x71, 0x4e, 0x9a],
    "0b546df3ef91e1da09e5e7efc7258ca2da57cbe6";

    test_sha3_512, Hmac::init_var(HmacVariant::Sha3_512, 19, &[0xa4, 0x71, 0xb4, 0x61, 0x43, 0xc4, 0x77, 0x22, 0xa4, 0x31, 0x7f, 0x79, 0xc3, 0x60, 0x5f, 0x56, 0x06, 0x21, 0x00, 0x66, 0xf7, 0x60, 0x7f, 0x37, 0xbf, 0xc0, 0x5a, 0xb4, 0x8a, 0xd6, 0x24, 0xec, 0xdd, 0xaa, 0x5f, 0x2b, 0xce, 0x0f, 0x5d, 0x68, 0xcb, 0x90, 0x0a, 0x94, 0x04, 0x1a, 0x38, 0x8c]),
    &[0x67, 0x64, 0x98, 0xa9, 0x15, 0xcc, 0x5b, 0x77, 0x32, 0x75, 0x03, 0x4a, 0x97, 0x2b, 0x55, 0x2a],
    "cf38aa4b510886a34fb3b67f50f8fed59de585";
);
