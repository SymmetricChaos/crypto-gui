use utils::byte_formatting::ByteFormat;

use crate::{pbkdf2, traits::ClassicHasher};

fn ro_mix() {}

fn xor_blocks(a: &[u8; 64], b: &[u8; 64]) -> [u8; 64] {
    let mut out = [0; 64];
    for i in 0..64 {
        out[i] = a[i] ^ b[i]
    }
    out
}

fn salsa20_8(a: [u8; 64]) -> [u8; 64] {
    todo!()
}

fn block_mix(block: &[u8]) {
    let r = block.len() / 128;

    let mut x = [0u8; 64];
    x.copy_from_slice(&block[block.len() - 64..]);
    for chunk in block.chunks(64) {
        x = salsa20_8(xor_blocks(&x, chunk.try_into().unwrap()));
    }
}

pub struct Scrypt {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub salt: Vec<u8>,
    pub cost: u32,
    pub blocksize_factor: u32,
    pub paralleism: u32,
    pub key_len: u32,
    pub h_len: u32,
    pub mf_len: u32,
}

impl ClassicHasher for Scrypt {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let pbkdf = pbkdf2::Pbkdf2::default()
            .variant(crate::hmac::HmacVariant::Sha256)
            .salt(self.salt.clone())
            .iterations(1)
            .hash_len(128 * self.blocksize_factor * self.paralleism);
        let p = pbkdf.hash(bytes);
        todo!()
    }

    crate::hash_bytes_from_string! {}
}
