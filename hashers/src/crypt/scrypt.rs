// based on
// https://datatracker.ietf.org/doc/html/rfc7914.html#section-3

use utils::byte_formatting::{make_u32s_le, u32s_to_bytes_le, ByteFormat};

use crate::{pbkdf2, traits::ClassicHasher};

fn xor_blocks(a: &[u8; 64], b: &[u8; 64]) -> [u8; 64] {
    let mut out = [0; 64];
    for i in 0..64 {
        out[i] = a[i] ^ b[i]
    }
    out
}

macro_rules! salsa {
    ($x: ident, $a: literal, $b: literal, $c: literal, $r: literal) => {
        $x[$a] ^= $x[$b].wrapping_add($x[$c]).rotate_left($r)
    };
}

fn salsa20_8(a: [u8; 64]) -> [u8; 64] {
    let mut x = make_u32s_le::<16>(&a);
    for _ in 0..4 {
        salsa!(x, 4, 0, 12, 7);
        salsa!(x, 8, 4, 0, 9);
        salsa!(x, 12, 8, 4, 13);
        salsa!(x, 0, 12, 8, 18);
        salsa!(x, 9, 5, 1, 7);
        salsa!(x, 13, 9, 5, 9);
        salsa!(x, 1, 13, 9, 13);
        salsa!(x, 5, 1, 13, 18);
        salsa!(x, 14, 10, 6, 7);
        salsa!(x, 2, 14, 10, 9);
        salsa!(x, 6, 2, 14, 13);
        salsa!(x, 10, 6, 2, 18);
        salsa!(x, 3, 15, 11, 7);
        salsa!(x, 7, 3, 15, 9);
        salsa!(x, 11, 7, 3, 13);
        salsa!(x, 15, 11, 7, 18);
        salsa!(x, 1, 0, 3, 7);
        salsa!(x, 2, 1, 0, 9);
        salsa!(x, 3, 2, 1, 13);
        salsa!(x, 0, 3, 2, 18);
        salsa!(x, 6, 5, 4, 7);
        salsa!(x, 7, 6, 5, 9);
        salsa!(x, 4, 7, 6, 13);
        salsa!(x, 5, 4, 7, 18);
        salsa!(x, 11, 10, 9, 7);
        salsa!(x, 8, 11, 10, 9);
        salsa!(x, 9, 8, 11, 13);
        salsa!(x, 10, 9, 8, 18);
        salsa!(x, 12, 15, 14, 7);
        salsa!(x, 13, 12, 15, 9);
        salsa!(x, 14, 13, 12, 13);
        salsa!(x, 15, 14, 13, 18);
    }

    let mut out = [0; 64];
    u32s_to_bytes_le(&mut out, &x);
    for i in 0..64 {
        out[i] = out[i].wrapping_add(a[i])
    }
    out
}

// fn block_mix(block: &[u8]) -> Vec<u8> {
//     let mut even = Vec::new();
//     let mut odd = Vec::new();

//     let mut x = [0u8; 64];
//     x.copy_from_slice(&block[block.len() - 64..]);

//     for (i, chunk) in block.chunks(64).enumerate() {
//         x = salsa20_8(xor_blocks(&x, chunk.try_into().unwrap()));

//         if i.is_even() {
//             even.extend_from_slice(&x);
//         } else {
//             odd.extend_from_slice(&x);
//         }
//     }

//     let mut out = Vec::new();
//     for (e, o) in even.into_iter().zip(odd) {
//         out.push(e);
//         out.push(o);
//     }
//     out
// }

// fn ro_mix() {}

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

#[cfg(test)]
mod scrypt_tests {
    use super::*;

    #[test]
    fn sala_function() {
        let input = [
            0x7e, 0x87, 0x9a, 0x21, 0x4f, 0x3e, 0xc9, 0x86, 0x7c, 0xa9, 0x40, 0xe6, 0x41, 0x71,
            0x8f, 0x26, 0xba, 0xee, 0x55, 0x5b, 0x8c, 0x61, 0xc1, 0xb5, 0x0d, 0xf8, 0x46, 0x11,
            0x6d, 0xcd, 0x3b, 0x1d, 0xee, 0x24, 0xf3, 0x19, 0xdf, 0x9b, 0x3d, 0x85, 0x14, 0x12,
            0x1e, 0x4b, 0x5a, 0xc5, 0xaa, 0x32, 0x76, 0x02, 0x1d, 0x29, 0x09, 0xc7, 0x48, 0x29,
            0xed, 0xeb, 0xc6, 0x8d, 0xb8, 0xb8, 0xc2, 0x5e,
        ];
        let output = [
            0xa4, 0x1f, 0x85, 0x9c, 0x66, 0x08, 0xcc, 0x99, 0x3b, 0x81, 0xca, 0xcb, 0x02, 0x0c,
            0xef, 0x05, 0x04, 0x4b, 0x21, 0x81, 0xa2, 0xfd, 0x33, 0x7d, 0xfd, 0x7b, 0x1c, 0x63,
            0x96, 0x68, 0x2f, 0x29, 0xb4, 0x39, 0x31, 0x68, 0xe3, 0xc9, 0xe6, 0xbc, 0xfe, 0x6b,
            0xc5, 0xb7, 0xa0, 0x6d, 0x96, 0xba, 0xe4, 0x24, 0xcc, 0x10, 0x2c, 0x91, 0x74, 0x5c,
            0x24, 0xad, 0x67, 0x3d, 0xc7, 0x61, 0x8f, 0x81,
        ];
        assert_eq!(
            output,
            salsa20_8(input),
            "\n{:02x?}\n{:02x?}",
            output,
            salsa20_8(input)
        )
    }
}
