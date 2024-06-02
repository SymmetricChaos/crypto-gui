use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, hmac::HmacSha1, traits::ClassicHasher};

#[derive(Debug, Clone)]
pub struct Pbkdf2 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    // hmac: HmacSha1,
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub output_length: u32, // size of the output in bytes
}

impl Default for Pbkdf2 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            // hmac: HmacSha1::default(),
            salt: Vec::new(),
            iterations: 4096,
            output_length: 32,
        }
    }
}

impl Pbkdf2 {
    pub fn hash_block(&self, hmac: &HmacSha1, block_num: u32) -> Vec<u8> {
        // The salt followed by the block nunber are the initial input
        let mut s = self.salt.clone();
        s.extend(block_num.to_be_bytes());

        // Create the first output block in the chain
        let mut out = hmac.hash(&s);

        for _ in 1..self.iterations {
            let t = hmac.hash(&out);
            for (target, new) in out.iter_mut().zip_eq(t.iter()) {
                *target ^= new
            }
        }

        out
    }
}

impl ClassicHasher for Pbkdf2 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(self.iterations != 0);

        let mut out = Vec::new();
        let mut hmac = HmacSha1::default();

        hmac.key = bytes.to_vec();

        let mut block_num = 0;
        while out.len() < self.output_length as usize {
            block_num += 1;
            out.extend(self.hash_block(&hmac, block_num));
        }

        out.truncate(self.output_length as usize);
        out
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
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
        hasher.iterations = 4096;
        hasher.output_length = 20;
        assert_eq!(
            "4b007901b765489abead49d926f721d065a429c1",
            hasher.hash_bytes_from_string("password").unwrap()
        );
    }

    #[test]
    fn test3() {
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
}
