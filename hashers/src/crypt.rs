use crate::{auxiliary::des_functions::Des, traits::ClassicHasher};
use utils::byte_formatting::ByteFormat;

pub struct CryptDes {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub salt: [bool; 12], // only 12 bits used
}

impl Default for CryptDes {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: [false; 12],
        }
    }
}

impl CryptDes {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn salt(mut self, salt: [bool; 12]) -> Self {
        self.salt = salt;
        self
    }
}

impl ClassicHasher for CryptDes {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        // Load the bytes of the key into a u64
        let mut key: u64 = 0;
        for i in 0..8 {
            key = key << 8;
            if let Some(byte) = bytes.get(i) {
                key |= *byte as u64
            }
        }

        // Setup DES
        let mut cipher = Des::default();
        cipher.ksa(key);

        // Encrypt the block 25 times using the salted block function
        let mut block = 0;
        for _ in 0..25 {
            block = cipher.encrypt_block_salt(block, self.salt);
        }

        block.to_be_bytes().to_vec()
    }

    crate::hash_bytes_from_string! {}
}

// crate::basic_hash_tests!(
//     test1,
//     Crypt::default(),
//     "INPUT",
//     "OUTPUT";
// );
