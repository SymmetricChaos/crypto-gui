use crate::traits::StatefulHasher;
use itertools::Itertools;
use std::ops::Shr;

pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn sigma_0(a: u32) -> u32 {
    (a.rotate_right(7)) ^ (a.rotate_right(18)) ^ (a.shr(3))
}

pub fn sigma_1(a: u32) -> u32 {
    (a.rotate_right(17)) ^ (a.rotate_right(19)) ^ (a.shr(10))
}

pub fn sum_0(a: u32) -> u32 {
    (a.rotate_right(2)) ^ (a.rotate_right(13)) ^ (a.rotate_right(22))
}

pub fn sum_1(a: u32) -> u32 {
    (a.rotate_right(6)) ^ (a.rotate_right(11)) ^ (a.rotate_right(25))
}

pub fn choice(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (!a & c)
}

pub fn majority(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}

fn compress(state: &mut [u32; 8], chunk: &[u8]) {
    // Copy variable values into working variables
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    // Array of 64 words
    let mut x = [0u32; 64];

    // Copy the first words into the array
    // Each word is 4 bytes and 16 are taken in total
    utils::byte_formatting::fill_u32s_be(&mut x[0..16], &chunk);

    // Extend the 16 words already in the array into a total of 64 words
    for i in 16..64 {
        x[i] = x[i - 16]
            .wrapping_add(sigma_0(x[i - 15]))
            .wrapping_add(x[i - 7])
            .wrapping_add(sigma_1(x[i - 2]));
    }

    for i in 0..64 {
        let temp1 = h
            .wrapping_add(sum_1(e))
            .wrapping_add(choice(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(x[i]);
        let temp2 = sum_0(a).wrapping_add(majority(a, b, c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

macro_rules! sha2_256 {
    ($name: ident, $iv: expr, $output_len: literal) => {
        pub struct $name {
            state: [u32; 8],
            buffer: Vec<u8>,
            bits_taken: u64,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    state: $iv,
                    buffer: Vec::new(),
                    bits_taken: 0,
                }
            }
        }

        impl $name {
            pub fn init() -> Self {
                Self::default()
            }
        }

        impl StatefulHasher for $name {
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
                for b in self.bits_taken.to_be_bytes() {
                    self.buffer.push(b)
                }

                // There can be multiple final blocks after padding
                for chunk in self.buffer.chunks_exact(64) {
                    compress(&mut self.state, &chunk);
                }

                let mut out = self
                    .state
                    .iter()
                    .map(|x| x.to_be_bytes())
                    .flatten()
                    .collect_vec();
                out.truncate($output_len);
                out
            }

            crate::stateful_hash_helpers!();
        }
    };
}

sha2_256!(
    Sha2_224,
    [
        0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
        0xbefa4fa4,
    ],
    28
);
sha2_256!(
    Sha2_256,
    [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ],
    32
);
