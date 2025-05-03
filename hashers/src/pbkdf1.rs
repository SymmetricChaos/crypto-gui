use crate::{md2::Md2, md5::Md5, sha::Sha1, traits::StatefulHasher};
use strum::{Display, EnumIter, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, VariantNames)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Pbkdf1Variant {
    Md2,
    Md5,
    Sha1,
}

pub struct Pbkdf1 {
    buffer: Vec<u8>,
    variant: Pbkdf1Variant,
    salt: [u8; 8],
    iterations: u32,
    hash_len: u32, // size of the output in bytes
}

impl Pbkdf1 {
    pub fn init(variant: Pbkdf1Variant, iterations: u32, hash_len: u32, salt: &[u8]) -> Self {
        assert!(iterations >= 1);
        assert!(hash_len >= 1);
        assert!(salt.len() == 8);
        Self {
            buffer: Vec::new(),
            salt: salt.try_into().unwrap(),
            variant,
            iterations,
            hash_len,
        }
    }

    pub fn inner_hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.variant {
            Pbkdf1Variant::Sha1 => Sha1::init().hash(bytes),
            Pbkdf1Variant::Md2 => Md2::init().hash(bytes),
            Pbkdf1Variant::Md5 => Md5::init().hash(bytes),
        }
    }
}

impl StatefulHasher for Pbkdf1 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(mut self) -> Vec<u8> {
        self.buffer.extend(self.salt);

        for _ in 0..self.iterations {
            self.buffer = self.inner_hash(&self.buffer);
        }

        self.buffer[..self.hash_len as usize].to_vec()
    }
}

// Wasn't able to find any test vectors for PBKDF1
// crate::stateful_hash_tests!(
//     test1, Pbkdf1::init(Pbkdf1Variant::Md5, 1, 20, b"salt"), b"password", "";
// );
