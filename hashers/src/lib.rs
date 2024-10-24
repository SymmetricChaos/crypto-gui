pub mod argon2;
pub mod ascon;
pub mod auxiliary;
pub mod blake;
pub mod checksum;
pub mod cityhash;
pub mod crypt;
pub mod errors;
pub mod fnv;
pub mod fxhash;
pub mod ghash;
pub mod gost;
pub mod haval;
pub mod hmac;
pub mod ids;
pub mod lm;
pub mod md2;
pub mod md4;
pub mod md5;
pub mod md6;
pub mod mgf1;
pub mod murmurhash3;
pub mod one_at_a_time;
pub mod pbkdf1;
pub mod pbkdf2;
pub mod pearson;
pub mod poly1305;
pub mod radio_gatun;
pub mod ripemd;
pub mod sha;
pub mod siphash;
pub mod skein;
pub mod sm3;
pub mod tiger;
pub mod traits;
pub mod vsh;

// Template
// use utils::byte_formatting::ByteFormat;
// use crate::traits::ClassicHasher;
// pub struct HASHERNAME {
//     pub input_format: ByteFormat,
//     pub output_format: ByteFormat,
// }

// impl Default for HASHERNAME {
//     fn default() -> Self {
//         Self {
//             input_format: ByteFormat::Utf8,
//             output_format: ByteFormat::Hex,
//         }
//     }
// }

// impl ClassicHasher for HASHERNAME {
//     fn hash(&self, bytes: &[u8]) -> Vec<u8> {
//         todo!()
//     }

//     crate::hash_bytes_from_string! {}
// }
