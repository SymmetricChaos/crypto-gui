use std::cell::RefCell;

use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{
    errors::HasherError,
    hmac::Hmac,
    sha::Sha1,
    traits::{ClassicHasher, KeyedHasher},
};

pub struct Pbkdf2 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub hmac: RefCell<Hmac>, // RefCell needed because the HMAC is rekeyed in .hash() which is not does not allow mutation
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub output_length: u32, // size of the output in bytes
}

impl Default for Pbkdf2 {
    fn default() -> Self {
        let hmac = Hmac {
            hasher: Box::new(Sha1::default()),
            ..Default::default()
        };
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            hmac: RefCell::new(hmac),
            salt: Vec::new(),
            iterations: 4096,
            output_length: 32,
        }
    }
}

impl Pbkdf2 {
    pub fn hash_block(&self, hmac: &Hmac, block_num: u32) -> Vec<u8> {
        // The salt followed by the block nunber are the initial input
        let mut s = self.salt.clone();
        s.extend(block_num.to_be_bytes());

        // Create the first output block in the chain by hashing the salt (the password has already been set for the hmac)
        let mut block = hmac.hash(&s);
        let mut temp: Vec<u8> = Vec::new();

        // The second iteration uses the block as input as temp is still empty.
        if self.iterations > 1 {
            temp = hmac.hash(&block);
            for (target, new) in block.iter_mut().zip_eq(temp.iter()) {
                *target ^= new
            }
        }

        // After the second iteration the previous temp is hashed, saved, and xored into the block
        for _ in 2..self.iterations {
            temp = hmac.hash(&temp);
            for (target, new) in block.iter_mut().zip_eq(temp.iter()) {
                *target ^= new
            }
        }

        block
    }
}

impl ClassicHasher for Pbkdf2 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(self.iterations != 0);

        let mut out = Vec::new();
        self.hmac.borrow_mut().set_key(bytes);

        let mut block_num = 0;
        while out.len() < self.output_length as usize {
            block_num += 1;
            out.extend(self.hash_block(&self.hmac.borrow(), block_num));
        }

        out.truncate(self.output_length as usize);
        out
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod pbkdf2_tests {
    use super::*;

    #[test]
    fn test1() {
        let mut hasher = Pbkdf2::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.salt = "salt".as_bytes().to_vec();
        hasher.iterations = 1;
        hasher.output_length = 20;
        assert_eq!(
            "0c60c80f961f0e71f3a9b524af6012062fe037a6",
            hasher.hash_bytes_from_string("password").unwrap()
        );
    }

    #[test]
    fn test2() {
        let mut hasher = Pbkdf2::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.salt = "salt".as_bytes().to_vec();
        hasher.iterations = 2;
        hasher.output_length = 20;
        assert_eq!(
            "ea6c014dc72d6f8ccd1ed92ace1d41f0d8de8957",
            hasher.hash_bytes_from_string("password").unwrap()
        );
    }

    #[test]
    fn test3() {
        let mut hasher = Pbkdf2::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.salt = "salt".as_bytes().to_vec();
        hasher.iterations = 4096;
        hasher.output_length = 20;
        assert_eq!(
            "4b007901b765489abead49d926f721d065a429c1",
            hasher.hash_bytes_from_string("password").unwrap()
        );
    }

    #[test]
    fn test4() {
        let mut hasher = Pbkdf2::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.salt = "saltSALTsaltSALTsaltSALTsaltSALTsalt".as_bytes().to_vec();
        hasher.iterations = 4096;
        hasher.output_length = 25;
        assert_eq!(
            "3d2eec4fe41c849b80c8d83662c0e44a8b291a964cf2f07038",
            hasher
                .hash_bytes_from_string("passwordPASSWORDpassword")
                .unwrap()
        );
    }

    #[test]
    fn test5() {
        let mut hasher = Pbkdf2::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.salt = "sa\0lt".as_bytes().to_vec();
        hasher.iterations = 4096;
        hasher.output_length = 16;
        assert_eq!(
            "56fa6aa75548099dcc37d7f03425e0c3",
            hasher.hash_bytes_from_string("pass\0word").unwrap()
        );
    }
}
