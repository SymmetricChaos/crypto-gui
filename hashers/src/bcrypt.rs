use utils::byte_formatting::ByteFormat;

use crate::{
    blowfish_arrays::{PARRAY, SBOX0, SBOX1, SBOX2, SBOX3},
    errors::HasherError,
    traits::ClassicHasher,
};

pub struct Bcrypt {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    parray: [u32; 18],
    sboxes: [[u32; 256]; 4],
    pub cost: u8,
    pub salt: [u8; 16],
}

impl Default for Bcrypt {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            parray: PARRAY,
            sboxes: [SBOX0, SBOX1, SBOX2, SBOX3],
            cost: 12,
            salt: [0; 16],
        }
    }
}

impl Bcrypt {
    pub fn eks_blowfish_setup(&self, password: &[u8]) {}
    pub fn expand_key(&mut self, password: &[u8]) {
        // Endlessly repeat the key as needed
        let mut key_bytes = password.iter().cycle();

        // Xoring the password into the IV
        for word in self.parray.iter_mut() {
            let mut k = 0u32;
            for _ in 0..4 {
                k <<= 8;
                k |= (*key_bytes.next().unwrap()) as u32;
            }
            *word ^= k;
        }

        let salt0 = u64::from_le_bytes(self.salt[0..8].try_into().unwrap());
        let salt1 = u64::from_le_bytes(self.salt[8..16].try_into().unwrap());
    }
}

impl ClassicHasher for Bcrypt {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
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
