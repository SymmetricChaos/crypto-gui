use crate::auxiliary::des_functions::Des;
use utils::byte_formatting::ByteFormat;

pub struct CryptDes {
    pub salt: [bool; 12], // only 12 bits used
}

impl Default for CryptDes {
    fn default() -> Self {
        Self { salt: [false; 12] }
    }
}

impl CryptDes {}

// impl ClassicHasher for CryptDes {
//     fn hash(&self, bytes: &[u8]) -> Vec<u8> {
//         // Load the bytes of the key into a u64
//         let mut key: u64 = 0;
//         for i in 0..8 {
//             key = key << 8;
//             if let Some(byte) = bytes.get(i) {
//                 key |= *byte as u64
//             }
//         }

//         // Setup DES
//         let mut cipher = Des::default();
//         cipher.ksa(key);

//         // Encrypt the block 25 times using the salted block function
//         let mut block = 0;
//         for _ in 0..25 {
//             block = cipher.encrypt_block_salt(block, self.salt);
//         }

//         block.to_be_bytes().to_vec()
//     }

//     crate::hash_bytes_from_string! {}
// }

// crate::basic_hash_tests!(
//     test1,
//     Crypt::default(),
//     "INPUT",
//     "OUTPUT";
// );
