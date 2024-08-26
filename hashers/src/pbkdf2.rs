use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{
    errors::HasherError,
    hmac::{Hmac, HmacVariant},
    traits::ClassicHasher,
};

pub struct Pbkdf2 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub variant: HmacVariant,
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub hash_len: u32, // size of the output in bytes
}

impl Default for Pbkdf2 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: Vec::new(),
            variant: HmacVariant::Sha256,
            iterations: 4096,
            hash_len: 32,
        }
    }
}

impl Pbkdf2 {
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

    pub fn salt(mut self, salt: Vec<u8>) -> Self {
        self.salt = salt;
        self
    }

    pub fn iterations(mut self, iterations: u32) -> Self {
        assert!(self.iterations > 0);
        self.iterations = iterations;
        self
    }

    pub fn hash_len(mut self, hash_len: u32) -> Self {
        assert!(hash_len > 0);
        self.hash_len = hash_len;
        self
    }

    pub fn salt_from_str(mut self, format: ByteFormat, salt_str: &str) -> Self {
        let bytes = format.text_to_bytes(salt_str).expect("invalid key string");
        self.salt = bytes;
        self
    }

    // For changing the key interactively
    pub fn set_salt(&mut self, salt: &[u8]) {
        self.salt = salt.to_vec();
    }

    // Falliable method for changing the salt from a string interactively
    pub fn set_salt_from_str(
        &mut self,
        format: ByteFormat,
        salt_str: &str,
    ) -> Result<(), HasherError> {
        let bytes = format
            .text_to_bytes(salt_str)
            .map_err(|_| HasherError::general("byte format error"))?;
        self.salt = bytes;
        Ok(())
    }

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
        let hmac = Hmac::default().variant(self.variant).key(bytes.to_vec());

        let mut block_num = 0;
        while out.len() < self.hash_len as usize {
            block_num += 1;
            out.extend(self.hash_block(&hmac, block_num));
        }

        out.truncate(self.hash_len as usize);
        out
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    Pbkdf2::default().variant(HmacVariant::Sha1).iterations(1).hash_len(20).salt_from_str(ByteFormat::Utf8, "salt"), test1, "password", "0c60c80f961f0e71f3a9b524af6012062fe037a6";
    Pbkdf2::default().variant(HmacVariant::Sha1).iterations(2).hash_len(20).salt_from_str(ByteFormat::Utf8, "salt"), test2, "password", "ea6c014dc72d6f8ccd1ed92ace1d41f0d8de8957";
    Pbkdf2::default().variant(HmacVariant::Sha1).iterations(4096).hash_len(20).salt_from_str(ByteFormat::Utf8, "salt"), test3, "password", "4b007901b765489abead49d926f721d065a429c1";
    Pbkdf2::default().variant(HmacVariant::Sha1).iterations(4096).hash_len(25).salt_from_str(ByteFormat::Utf8, "saltSALTsaltSALTsaltSALTsaltSALTsalt"), test4, "passwordPASSWORDpassword", "3d2eec4fe41c849b80c8d83662c0e44a8b291a964cf2f07038";
    Pbkdf2::default().variant(HmacVariant::Sha1).iterations(4096).hash_len(16).salt_from_str(ByteFormat::Utf8, "sa\0lt"), test5, "pass\0word", "56fa6aa75548099dcc37d7f03425e0c3";
);
