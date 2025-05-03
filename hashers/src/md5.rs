use crate::traits::StatefulHasher;
use utils::byte_formatting::fill_u32s_le;

const BLOCK_LEN: usize = 64;

pub const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

pub const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

fn compress(state: &mut [u32; 4], chunk: &[u8]) {
    let [mut ta, mut tb, mut tc, mut td] = state.clone();

    let mut x = [0u32; 16];
    fill_u32s_le(&mut x, &chunk);

    for i in 0..64 {
        let mut f = 0;
        let mut g = 0;
        if i < 16 {
            f = (tb & tc) | (!tb & td);
            g = i
        }
        if i >= 16 && i < 32 {
            f = (td & tb) | (!td & tc);
            g = (5 * i + 1) % 16;
        }
        if i >= 32 && i < 48 {
            f = tb ^ tc ^ td;
            g = (3 * i + 5) % 16;
        }
        if i >= 48 {
            f = tc ^ (tb | !td);
            g = (7 * i) % 16;
        }

        f = f.wrapping_add(ta).wrapping_add(K[i]).wrapping_add(x[g]);
        ta = td;
        td = tc;
        tc = tb;
        tb = tb.wrapping_add(f.rotate_left(S[i]))
    }
    state[0] = state[0].wrapping_add(ta);
    state[1] = state[1].wrapping_add(tb);
    state[2] = state[2].wrapping_add(tc);
    state[3] = state[3].wrapping_add(td);
}

#[derive(Debug, Clone)]
pub struct Md5 {
    state: [u32; 4],
    buffer: Vec<u8>,
    bits_taken: u64,
}

impl Md5 {
    pub fn init() -> Self {
        Self {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            buffer: Vec::new(),
            bits_taken: 0,
        }
    }
}

impl StatefulHasher for Md5 {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, BLOCK_LEN, {
            self.bits_taken += 512;
            compress(&mut self.state, &self.buffer);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        self.bits_taken += self.buffer.len() as u64 * 8;
        self.buffer.push(0x80);
        while (self.buffer.len() % 64) != 56 {
            self.buffer.push(0x00)
        }
        self.buffer.extend(self.bits_taken.to_le_bytes());

        // There can be multiple final blocks after padding
        for chunk in self.buffer.chunks_exact(64) {
            compress(&mut self.state, &chunk);
        }

        let mut out = Vec::with_capacity(16);
        for word in self.state {
            out.extend(word.to_le_bytes())
        }
        out
    }
}

crate::stateful_hash_tests!(
    test1, Md5::init(), b"",                                             "d41d8cd98f00b204e9800998ecf8427e";
    test2, Md5::init(), b"The quick brown fox jumps over the lazy dog",  "9e107d9d372bb6826bd81d3542a419d6";
    test3, Md5::init(), b"The quick brown fox jumps over the lazy dog.", "e4d909c290d0fb1ca068ffaddf22cbd0";
    test4, Md5::init(), b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "d174ab98d277d9f5a5611c2c9f419d9f";
    test5, Md5::init(), b"12345678901234567890123456789012345678901234567890123456789012345678901234567890", "57edf4a22be3c955ac49da2e2107b67a";
);
