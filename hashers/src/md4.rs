use utils::byte_formatting::fill_u32s_le;

use crate::traits::StatefulHasher;

pub fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

pub fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

pub fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

pub fn r1(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
    *a = (a.wrapping_add(f(b, c, d)).wrapping_add(i)).rotate_left(s)
}

pub fn r2(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
    *a = (a
        .wrapping_add(g(b, c, d))
        .wrapping_add(i)
        .wrapping_add(0x5A827999))
    .rotate_left(s)
}

pub fn r3(a: &mut u32, b: u32, c: u32, d: u32, i: u32, s: u32) {
    *a = (a
        .wrapping_add(h(b, c, d))
        .wrapping_add(i)
        .wrapping_add(0x6ED9EBA1))
    .rotate_left(s)
}

fn compress(state: &mut [u32; 4], chunk: &[u8]) {
    let tstate = state.clone();
    let [mut a, mut b, mut c, mut d] = state;

    let mut x = [0u32; 16];
    fill_u32s_le(&mut x, &chunk);

    // Round 2
    for i in [0, 4, 8, 12] {
        r1(&mut a, b, c, d, x[i], 3);
        r1(&mut d, a, b, c, x[i + 1], 7);
        r1(&mut c, d, a, b, x[i + 2], 11);
        r1(&mut b, c, d, a, x[i + 3], 19);
    }

    // Round 2
    for i in [0, 1, 2, 3] {
        r2(&mut a, b, c, d, x[i], 3);
        r2(&mut d, a, b, c, x[i + 4], 5);
        r2(&mut c, d, a, b, x[i + 8], 9);
        r2(&mut b, c, d, a, x[i + 12], 13);
    }

    // Round 3
    for i in [0, 2, 1, 3] {
        r3(&mut a, b, c, d, x[i], 3);
        r3(&mut d, a, b, c, x[i + 8], 9);
        r3(&mut c, d, a, b, x[i + 4], 11);
        r3(&mut b, c, d, a, x[i + 12], 15);
    }

    state[0] = a.wrapping_add(tstate[0]);
    state[1] = b.wrapping_add(tstate[1]);
    state[2] = c.wrapping_add(tstate[2]);
    state[3] = d.wrapping_add(tstate[3]);
}

#[derive(Debug, Clone)]
pub struct Md4 {
    state: [u32; 4],
    buffer: Vec<u8>,
    bits_taken: u64,
}

impl Default for Md4 {
    fn default() -> Self {
        Self {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            buffer: Vec::new(),
            bits_taken: 0,
        }
    }
}

impl Md4 {
    pub fn init() -> Self {
        Self::default()
    }
}

impl StatefulHasher for Md4 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let chunks = self.buffer.chunks_exact(64);
        let rem = chunks.remainder().to_vec();
        for chunk in chunks {
            self.bits_taken += 512;
            compress(&mut self.state, chunk);
        }
        self.buffer = rem;
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

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1, Md4::init(), b"", "31d6cfe0d16ae931b73c59d7e0c089c0";
    test2, Md4::init(), b"a","bde52cb31de33e46245e05fbdbd6fb24";
    test3, Md4::init(), b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "043f8582f241db351ce627e153e7f0e4";
    test4, Md4::init(), b"12345678901234567890123456789012345678901234567890123456789012345678901234567890","e33b4ddc9c38f2199c3e7b164fcc0536";
);
