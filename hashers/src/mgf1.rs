use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, sha2::Sha2, traits::ClassicHasher};

#[derive(Debug, Clone)]
pub struct Mgf1 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub output_length: u32,
    pub hasher: Sha2,
}

impl Default for Mgf1 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            output_length: 32,
            hasher: Sha2::default(),
        }
    }
}

impl ClassicHasher for Mgf1 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.output_length as usize);
        let mut ctr = 0_u32;

        loop {
            let mut t_key = bytes.to_vec();
            t_key.extend_from_slice(&ctr.to_be_bytes());
            out.extend_from_slice(&self.hasher.hash(&t_key));
            if out.len() >= self.output_length as usize {
                break;
            }
            ctr += 1;
        }

        out[0..self.output_length as usize].to_vec()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod md5_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Mgf1::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.output_length = 50;
        assert_eq!("382576a7841021cc28fc4c0948753fb8312090cea942ea4c4e735d10dc724b155f9f6069f289d61daca0cb814502ef04eae1", hasher.hash_bytes_from_string("bar").unwrap());
    }
}
