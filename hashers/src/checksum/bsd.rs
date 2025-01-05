pub struct BsdChecksum {}

impl Default for BsdChecksum {
    fn default() -> Self {
        Self {}
    }
}

impl BsdChecksum {}

// impl ClassicHasher for BsdChecksum {
//     fn hash(&self, bytes: &[u8]) -> Vec<u8> {
//         let mut out = 0_u16;
//         for byte in bytes {
//             out = out.rotate_right(1);
//             out = out.wrapping_add(*byte as u16);
//         }
//         out.to_le_bytes().to_vec()
//     }

//     crate::hash_bytes_from_string! {}
// }
