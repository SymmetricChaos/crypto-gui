use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

// TODO
pub struct Scrypt {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub salt: Vec<u8>,
    pub cost: u32,
    pub blocksize: u32,
    pub paralleism: u32,
    pub key_len: u32,
    pub h_len: u32,
    pub mf_len: u32,
}

impl ClassicHasher for Scrypt {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    crate::hash_bytes_from_string! {}
}
