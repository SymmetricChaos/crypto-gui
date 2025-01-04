pub mod adler;
pub mod argon2;
pub mod ascon;
pub mod auxiliary;
pub mod bcrypt;
pub mod belt;
pub mod blake;
pub mod checksum;
pub mod cityhash;
pub mod crypt;
pub mod errors;
pub mod fletcher;
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
pub mod scrypt;
pub mod sha;
pub mod shabal;
pub mod siphash;
pub mod skein;
pub mod sm3;
pub mod tiger;
pub mod traits;
pub mod vsh;
pub mod poseidon;

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

// impl HASHERNAME {
//     pub fn input(mut self, input: ByteFormat) -> Self {
//         self.input_format = input;
//         self
//     }

//     pub fn output(mut self, output: ByteFormat) -> Self {
//         self.output_format = output;
//         self
//     }
// }

// impl ClassicHasher for HASHERNAME {
//     fn hash(&self, bytes: &[u8]) -> Vec<u8> {
//         todo!()
//     }

//     crate::hash_bytes_from_string! {}
// }

// crate::basic_hash_tests!(
//     test1,
//     HASHERNAME::default(),
//     "INPUT",
//     "OUTPUT";
// );

// Template
// use crate::traits::StatefulHasher;
// pub struct HASHERNAME {
//     state: []
//     buffer: Vec<u8>
// }
// impl Default for HASHERNAME {
//     fn default() -> Self {
//         Self {
//             state: [],
//             buffer: Vec::new(),
//         }
//     }
// }
// impl HASHERNAME {
// }
// impl StatefulHasher for HASHERNAME {
//     fn update(&mut self, bytes: &[u8]) {
//         self.buffer.extend_from_slice(bytes);
//     }
//     fn finalize(mut self) -> Vec<u8> {
//     }
//     crate::stateful_hash_helpers!();
// }
// crate:stateful_hash_tests!(
//     test1,
//     HASHERNAME::init(),
//     "INPUT",
//     "OUTPUT";
// );
