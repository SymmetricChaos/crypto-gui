use crate::{
    sha256::{Sha224, Sha256},
    sha512::{Sha384, Sha512},
    traits::ClassicHasher,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sha2Variant {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

pub struct Sha2 {
    pub variant: Sha2Variant,
    sha224: Sha224,
    sha256: Sha256,
    sha384: Sha384,
    sha512: Sha512,
}

impl Default for Sha2 {
    fn default() -> Self {
        Self {
            variant: Sha2Variant::Sha256,
            sha224: Default::default(),
            sha256: Default::default(),
            sha384: Default::default(),
            sha512: Default::default(),
        }
    }
}

impl Sha2 {}

impl ClassicHasher for Sha2 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.variant {
            Sha2Variant::Sha224 => self.sha224.hash(bytes),
            Sha2Variant::Sha256 => self.sha256.hash(bytes),
            Sha2Variant::Sha384 => self.sha384.hash(bytes),
            Sha2Variant::Sha512 => self.sha512.hash(bytes),
        }
    }
}

#[cfg(test)]
mod sha2_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Sha2::default();
        hasher.variant = Sha2Variant::Sha256;
        assert_eq!(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            hasher.hash_to_string("".as_bytes())
        );
        hasher.variant = Sha2Variant::Sha224;
        assert_eq!(
            "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f",
            hasher.hash_to_string("".as_bytes())
        );
        hasher.variant = Sha2Variant::Sha512;
        assert_eq!(
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
            hasher.hash_to_string("".as_bytes())
        );
        hasher.variant = Sha2Variant::Sha384;
        assert_eq!(
            "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b",
            hasher.hash_to_string("".as_bytes())
        );
    }
}
