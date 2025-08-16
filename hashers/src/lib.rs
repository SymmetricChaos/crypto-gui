pub mod adler;
pub mod argon2;
pub mod ascon;
pub mod auxiliary;
pub mod balloon;
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
pub mod groestl;
pub mod haval;
pub mod hkdf;
pub mod hmac;
pub mod ids;
pub mod jh;
pub mod lm;
pub mod lsh;
pub mod md2;
pub mod md4;
pub mod md5;
pub mod md6;
pub mod mgf1;
pub mod murmurhash3;
pub mod one_at_a_time;
pub mod panama;
pub mod pbkdf1;
pub mod pbkdf2;
pub mod pearson;
pub mod poly1305;
pub mod polyval;
pub mod poseidon;
pub mod radio_gatun;
pub mod rapidhash;
pub mod ripemd;
pub mod scrypt;
pub mod sha;
pub mod shabal;
pub mod siphash;
pub mod skein;
pub mod sm3;
pub mod snefru;
pub mod tiger;
pub mod traits;
pub mod vsh;
pub mod wyhash;

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
//
// }
// crate:stateful_hash_tests!(
//     test1,
//     HASHERNAME::init(),
//     "INPUT",
//     "OUTPUT";
// );
