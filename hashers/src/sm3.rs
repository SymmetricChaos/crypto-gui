// https://datatracker.ietf.org/doc/html/draft-sca-cfrg-sm3
use crate::traits::ClassicHasher;
use itertools::Itertools;
use utils::byte_formatting::{fill_u32s_be, ByteFormat};

fn tj(i: usize) -> u32 {
    if i < 16 {
        0x79cc4519
    } else {
        0x7a879d8a
    }
}

fn ff(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (x & z) | (y & z)
    }
}

fn gg(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (!x & z)
    }
}

fn p0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}

fn p1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}

// Message expansion
fn me(block: &[u8]) -> [u32; 132] {
    let mut out = [0; 132];
    fill_u32s_be(&mut out[0..16], block);
    for j in 16..68 {
        out[j] = p1(out[j - 16] ^ out[j - 9] ^ out[j - 3].rotate_left(15))
            ^ out[j - 13].rotate_left(7)
            ^ out[j - 6];
    }
    for j in 0..64 {
        out[j + 68] = out[j] ^ out[j + 4];
    }
    out
}

// Compression function
fn cf(state: &mut [u32; 8], e: [u32; 132]) {
    let mut s = state.clone();

    for i in 0..64 {
        let ss1 = (s[0]
            .rotate_left(12)
            .wrapping_add(s[4])
            .wrapping_add(tj(i).rotate_left(i as u32 % 32)))
        .rotate_left(7);
        let ss2 = ss1 ^ s[0].rotate_left(12);
        let tt1 = ff(i, s[0], s[1], s[2])
            .wrapping_add(s[3])
            .wrapping_add(ss2)
            .wrapping_add(e[i + 68]);
        let tt2 = gg(i, s[4], s[5], s[6])
            .wrapping_add(s[7])
            .wrapping_add(ss1)
            .wrapping_add(e[i]);
        s[3] = s[2];
        s[2] = s[1].rotate_left(9);
        s[1] = s[0];
        s[0] = tt1;
        s[7] = s[6];
        s[6] = s[5].rotate_left(19);
        s[5] = s[4];
        s[4] = p0(tt2);
    }

    for i in 0..8 {
        state[i] ^= s[i]
    }
}

pub struct Sm3 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Sm3 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl ClassicHasher for Sm3 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Padding and appending length is identical to MD4 and MD5
        // Length in bits before padding
        let b_len = (input.len().wrapping_mul(8)) as u64;

        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length in bits is 448 mod 512
        // equivalently until the length in bytes is 56 mod 64
        while (input.len() % 64) != 56 {
            input.push(0)
        }

        // append length
        for b in b_len.to_be_bytes() {
            input.push(b)
        }

        // Initialization Vector
        let mut v: [u32; 8] = [
            0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d,
            0xb0fb0e4e,
        ];
        for block in input.chunks_exact(64) {
            let e = me(block);
            cf(&mut v, e);
        }

        v.iter().map(|x| x.to_be_bytes()).flatten().collect_vec()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1, Sm3::default(),
    "abc",
    "66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0";
    test2, Sm3::default(),
    "abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd",
    "debe9ff92275b8a138604889c18e5a4d6fdb70e5387e5765293dcba39c0c5732";
);
